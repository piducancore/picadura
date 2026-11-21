//! Monophonic learning, using a bounded YIN-style difference estimator.
//! Reference: de Cheveigné & Kawahara (2002), doi:10.1121/1.1458024.
//! No allocation or locking in the audio callback; the persistent histogram
//! lives in atomic parameters and is captured by the normal host state chunk.
use crate::{
    PicaduraParams,
    params::{LEARNED_IDS, LearnSource},
};
use truce::prelude::*;

const WINDOW: usize = 256;
const FRAME: usize = 512;
const HOP: usize = 128;

pub struct PitchDetector {
    samples: [f32; FRAME],
    count: usize,
    decimation: usize,
    phase: usize,
    rate: f64,
    lowpass: [f32; 2],
    alpha: f32,
    candidate: Option<u8>,
    stable: u8,
}
impl Default for PitchDetector {
    fn default() -> Self {
        Self {
            samples: [0.0; FRAME],
            count: 0,
            decimation: 1,
            phase: 0,
            rate: 0.0,
            lowpass: [0.0; 2],
            alpha: 0.0,
            candidate: None,
            stable: 0,
        }
    }
}
impl PitchDetector {
    pub fn push(&mut self, sample: f32, rate: f64) -> Option<u8> {
        if !rate.is_finite() || !(8000.0..=384000.0).contains(&rate) {
            return None;
        }
        if self.rate != rate {
            *self = Self::default();
            self.rate = rate;
            self.decimation = (rate / 6000.0).floor().max(1.0) as usize;
            self.alpha = (1.0 - (-std::f64::consts::TAU * 1500.0 / rate).exp()) as f32;
        }
        if !sample.is_finite() {
            *self = Self::default();
            return None;
        }
        self.lowpass[0] += self.alpha * (sample - self.lowpass[0]);
        self.lowpass[1] += self.alpha * (self.lowpass[0] - self.lowpass[1]);
        self.phase += 1;
        if self.phase < self.decimation {
            return None;
        }
        self.phase = 0;
        self.samples[self.count] = self.lowpass[1];
        self.count += 1;
        if self.count < FRAME {
            return None;
        }
        let detected = self.estimate(self.rate / self.decimation as f64);
        self.samples.copy_within(HOP.., 0);
        self.count = FRAME - HOP;
        if detected == self.candidate {
            self.stable = self.stable.saturating_add(1);
        } else {
            self.candidate = detected;
            self.stable = 1;
        }
        if self.stable >= 2 { detected } else { None }
    }
    fn estimate(&self, rate: f64) -> Option<u8> {
        let mean = self.samples.iter().sum::<f32>() / FRAME as f32;
        let energy = self.samples.iter().map(|s| (s - mean).powi(2)).sum::<f32>() / FRAME as f32;
        // About -50 dBFS RMS, and reject a non-periodic/noisy window rather
        // than returning a best guess when there is no confident trough.
        if energy < 0.00001 {
            return None;
        }
        let min_lag = (rate / 1200.0).floor().max(2.0) as usize;
        let max_lag = ((rate / 40.0).ceil() as usize).min(WINDOW - 1);
        let mut normalized = [1.0f32; WINDOW];
        let mut sum = 0.0;
        for (lag, value) in normalized.iter_mut().enumerate().take(max_lag + 1).skip(1) {
            let mut difference = 0.0;
            for i in 0..WINDOW {
                difference += (self.samples[i] - self.samples[i + lag]).powi(2);
            }
            sum += difference;
            *value = if sum > 1e-12 {
                difference * lag as f32 / sum
            } else {
                1.0
            };
        }
        let mut lag = min_lag;
        while lag < max_lag {
            if normalized[lag] < 0.12 {
                while lag + 1 < max_lag && normalized[lag + 1] < normalized[lag] {
                    lag += 1;
                }
                let a = normalized[lag - 1];
                let b = normalized[lag];
                let c = normalized[lag + 1];
                let denominator = a - 2.0 * b + c;
                let correction = if denominator.abs() > 1e-9 {
                    (0.5 * (a - c) / denominator).clamp(-0.5, 0.5)
                } else {
                    0.0
                };
                let frequency = rate / (lag as f64 + f64::from(correction));
                let midi = 69.0 + 12.0 * (frequency / 440.0).log2();
                if (40.0..=1200.0).contains(&frequency) && (midi - midi.round()).abs() <= 0.45 {
                    return Some(midi.round().clamp(0.0, 127.0) as u8);
                }
                return None;
            }
            lag += 1;
        }
        None
    }
}

#[derive(Default)]
pub struct InputLearner {
    detector: PitchDetector,
    mode: Option<LearnSource>,
}
impl InputLearner {
    pub fn reset_audio(&mut self) {
        self.detector = PitchDetector::default();
    }
    fn add(params: &PicaduraParams, note: u8) {
        let id: u32 = LEARNED_IDS[note as usize % 12].into();
        let count = params.get_plain(id).unwrap_or(0.0);
        params.set_plain(id, (count + 1.0).min(1_000_000.0));
    }
    pub fn process(
        &mut self,
        params: &PicaduraParams,
        buffer: &AudioBuffer,
        events: &EventList,
        sample_rate: f64,
    ) {
        // A consumed momentary command, excluded from saved state.
        if params
            .clear_learned
            .swap(false, std::sync::atomic::Ordering::AcqRel)
        {
            for id in LEARNED_IDS {
                params.set_plain(id.into(), 0.0);
            }
            self.reset_audio();
        }
        let mode = params.learn.value().then(|| params.learn_source.value());
        if self.mode != mode {
            self.reset_audio();
            self.mode = mode;
        }
        match mode {
            Some(LearnSource::Midi) => {
                for event in events.iter() {
                    if let EventBody::NoteOn { note, velocity, .. } = event.body
                        && velocity > 0
                    {
                        Self::add(params, note);
                    }
                }
            }
            Some(LearnSource::Audio) => {
                // Select the stronger channel instead of summing: anti-phase
                // stereo and right-only sources must not cancel to silence.
                let channel = (0..buffer.num_input_channels()).max_by(|a, b| {
                    let energy = |ch| {
                        buffer
                            .input(ch)
                            .iter()
                            .filter(|s| s.is_finite())
                            .map(|s| f64::from(*s).powi(2))
                            .sum::<f64>()
                    };
                    energy(*a).total_cmp(&energy(*b))
                });
                if let Some(channel) = channel {
                    for sample in buffer.input(channel) {
                        if let Some(note) = self.detector.push(*sample, sample_rate) {
                            Self::add(params, note);
                        }
                    }
                }
            }
            None => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Picadura, PicaduraParamsParamId as P};

    fn tone(note: u8, rate: f64, seconds: f64) -> Vec<f32> {
        let frequency = 440.0 * 2.0f64.powf((f64::from(note) - 69.0) / 12.0);
        (0..(rate * seconds) as usize)
            .map(|i| {
                let phase = std::f64::consts::TAU * frequency * i as f64 / rate;
                (0.25 * phase.sin() + 0.1 * (phase * 2.0).sin() + 0.04 * (phase * 3.0).sin()) as f32
            })
            .collect()
    }
    #[test]
    fn detects_bass_and_melody_at_multiple_sample_rates() {
        for rate in [44100.0, 48000.0, 96000.0] {
            for note in [28, 36, 48, 60, 69, 84] {
                let mut detector = PitchDetector::default();
                let detections: Vec<_> = tone(note, rate, 0.35)
                    .into_iter()
                    .filter_map(|sample| detector.push(sample, rate))
                    .collect();
                assert!(
                    detections.len() >= 3,
                    "no stable pitch for {note} at {rate}"
                );
                assert!(
                    detections.iter().all(|n| *n == note),
                    "{note} at {rate}: {detections:?}"
                );
            }
        }
    }
    #[test]
    fn rejects_silence_dc_and_noise_and_recovers_after_invalid_audio() {
        let mut detector = PitchDetector::default();
        for sample in [0.0, 0.2] {
            for _ in 0..20000 {
                assert_eq!(detector.push(sample, 48000.0), None);
            }
        }
        let mut random = 1u32;
        for _ in 0..48000 {
            random ^= random << 13;
            random ^= random >> 17;
            random ^= random << 5;
            let noise = random as f32 / u32::MAX as f32 - 0.5;
            assert_eq!(detector.push(noise, 48000.0), None);
        }
        assert_eq!(detector.push(f32::NAN, 48000.0), None);
        assert_eq!(detector.push(0.0, f64::NAN), None);
        assert!(
            tone(60, 48000.0, 0.3)
                .into_iter()
                .filter_map(|x| detector.push(x, 48000.0))
                .any(|n| n == 60)
        );
    }
    #[test]
    fn midi_learning_freezes_clears_and_roundtrips_saved_pool() {
        let params = PicaduraParams::default();
        let mut learner = InputLearner::default();
        let buffer = AudioBuffer::from_slices_checked(&[], &mut [], 0);
        let mut events = EventList::with_capacity(4);
        for (note, velocity) in [(60, 100), (64, 100), (67, 100), (61, 0)] {
            events.push(Event::new(
                0,
                EventBody::NoteOn {
                    group: 0,
                    channel: 0,
                    note,
                    velocity,
                },
            ));
        }
        learner.process(&params, &buffer, &events, 48000.0);
        assert_eq!(params.learned(), [0; 12]);
        params.set_plain(P::Learn.into(), 1.0);
        learner.process(&params, &buffer, &events, 48000.0);
        assert_eq!(params.learned(), [1, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0]);
        params.set_plain(P::Learn.into(), 0.0);
        learner.process(&params, &buffer, &events, 48000.0);
        assert_eq!(params.learned().iter().sum::<u32>(), 3);
        let (ids, values) = params.collect_values();
        let restored = PicaduraParams::default();
        restored.restore_values(&ids.into_iter().zip(values).collect::<Vec<_>>());
        learner.process(&restored, &buffer, &events, 48000.0);
        assert_eq!(params.learned(), restored.learned());
        params
            .clear_learned
            .store(true, std::sync::atomic::Ordering::Release);
        learner.process(&params, &buffer, &events, 48000.0);
        assert_eq!(params.learned(), [0; 12]);
        assert!(
            !params
                .clear_learned
                .load(std::sync::atomic::Ordering::Acquire)
        );
    }
    #[test]
    fn audio_learning_is_block_independent_and_uses_right_channel() {
        let samples = tone(57, 48000.0, 0.5);
        let learn = |block: usize| {
            let params = PicaduraParams::default();
            params.set_plain(P::LearnSource.into(), 1.0);
            params.set_plain(P::Learn.into(), 1.0);
            let mut learner = InputLearner::default();
            let events = EventList::default();
            for chunk in samples.chunks(block) {
                let silence = vec![0.0; chunk.len()];
                let channels = [silence.as_slice(), chunk];
                let buffer = AudioBuffer::from_slices_checked(&channels, &mut [], chunk.len());
                learner.process(&params, &buffer, &events, 48000.0);
            }
            params.learned()
        };
        let pool = learn(64);
        assert_eq!(pool, learn(511));
        assert!(pool[9] > 0);
        assert_eq!(pool.iter().sum::<u32>(), pool[9]);
    }
    #[test]
    fn full_processor_learns_while_stopped_and_preserves_input_audio() {
        let params = PicaduraParams::default();
        params.set_plain(P::LearnSource.into(), 1.0);
        params.set_plain(P::Learn.into(), 1.0);
        let mut state = Picadura::default();
        let input = tone(60, 48000.0, 0.3);
        let mut output = vec![0.0; input.len()];
        let channels = [input.as_slice()];
        let mut outputs = [output.as_mut_slice()];
        let mut buffer = AudioBuffer::from_slices_checked(&channels, &mut outputs, input.len());
        let events = EventList::default();
        let mut outgoing = EventList::with_capacity(512);
        let transport = TransportInfo {
            tempo: 120.0,
            ..TransportInfo::default()
        };
        let mut context = ProcessContext::new(&transport, 48000.0, input.len(), &mut outgoing);
        #[cfg(feature = "rt-paranoid")]
        let (_, violations) = truce::core::rt::audit(|| {
            let _guard = truce::core::rt::RtSection::enter();
            Picadura::process(&mut state, &params, &mut buffer, &events, &mut context);
        });
        #[cfg(not(feature = "rt-paranoid"))]
        Picadura::process(&mut state, &params, &mut buffer, &events, &mut context);
        #[cfg(feature = "rt-paranoid")]
        assert_eq!(violations, 0);
        assert_eq!(input, output);
        assert!(outgoing.iter().next().is_none());
        assert!(params.learned()[0] > 0);
    }
}
