//! Original, deterministic acid sequencer. No Max runtime or copied patch assets.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Settings {
    pub root: i32,
    pub scale: usize,
    pub length: usize,
    pub density: f64,
    pub accent: f64,
    pub gate: f64,
    pub slide: f64,
    pub swing: f64,
    pub octaves: i32,
    pub seeds: [u32; 3],
    pub timings: [f64; 64],   // fractions of one sixteenth, -0.25..0.25
    pub overrides: [i32; 64], // -2 generated, -1 rest, 0..127 explicit note
}
impl Default for Settings {
    fn default() -> Self {
        Self {
            root: 36,
            scale: 0,
            length: 16,
            density: 0.7,
            accent: 0.3,
            gate: 0.55,
            slide: 0.2,
            swing: 0.0,
            octaves: 2,
            seeds: [1, 7, 11],
            overrides: [-2; 64],
            timings: [0.0; 64],
        }
    }
}
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Step {
    pub note: u8,
    pub on: bool,
    pub accent: bool,
    pub slide: bool,
}

fn random(seed: u32, index: usize, lane: u32) -> u32 {
    let mut x = seed.wrapping_add((index as u32).wrapping_mul(0x9e3779b9)) ^ lane;
    x ^= x >> 16;
    x = x.wrapping_mul(0x7feb352d);
    x ^= x >> 15;
    x = x.wrapping_mul(0x846ca68b);
    x ^ (x >> 16)
}
fn chance(seed: u32, index: usize, lane: u32, amount: f64) -> bool {
    f64::from(random(seed, index, lane)) / (f64::from(u32::MAX) + 1.0) < amount
}
pub fn pattern(s: &Settings) -> [Step; 64] {
    let scale: &[i32] = match s.scale {
        1 => &[0, 2, 3, 5, 7, 9, 10],
        2 => &[0, 1, 3, 5, 7, 8, 10],
        3 => &[0, 2, 4, 5, 7, 9, 11],
        4 => &[0, 3, 5, 7, 10],
        5 => &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
        _ => &[0, 2, 3, 5, 7, 8, 10],
    };
    std::array::from_fn(|i| {
        // Repeated roots supply an anchor; other steps explore scale tones.
        let choice = random(s.seeds[0], i, 15);
        let degree = if choice.is_multiple_of(4) {
            0
        } else {
            choice as usize % scale.len()
        };
        let octave = (random(s.seeds[0], i, 32) % s.octaves as u32) as i32;
        let note = (s.root + scale[degree] + octave * 12).clamp(0, 127) as u8;
        let forced = s.overrides[i];
        Step {
            note: if forced >= 0 { forced as u8 } else { note },
            on: i < s.length
                && match forced {
                    -2 => chance(s.seeds[1], i, 41, s.density),
                    -1 => false,
                    _ => true,
                },
            accent: chance(s.seeds[2], i, 56, s.accent),
            slide: chance(s.seeds[2], i, 77, s.slide),
        }
    })
}

/// Index of the sixteenth containing `beat`, including delayed offbeats.
pub fn step_at(beat: f64, swing: f64) -> i64 {
    let pair = (beat / 0.5).floor();
    let second = beat - pair * 0.5 + 1e-10 >= 0.25 + swing * 0.125;
    pair as i64 * 2 + i64::from(second)
}
fn boundary(index: i64, swing: f64) -> f64 {
    index as f64 * 0.25
        + if index.rem_euclid(2) == 1 {
            swing * 0.125
        } else {
            0.0
        }
}
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Midi {
    On(u8, u8),
    Off(u8),
}
#[derive(Clone, Copy)]
struct Voice {
    note: u8,
    until: f64,
}
#[derive(Default)]
pub struct Sequencer {
    voices: [Option<Voice>; 2],
    expected: Option<f64>,
    next_step: i64,
    swing: f64,
}
fn onset(index: i64, s: &Settings) -> f64 {
    boundary(index, s.swing) + s.timings[index.rem_euclid(s.length as i64) as usize] * 0.25
}
pub fn playing_step(s: &Settings, beat: f64) -> usize {
    let mut index = step_at(beat, s.swing);
    if onset(index, s) > beat {
        index -= 1;
    } else if onset(index + 1, s) <= beat {
        index += 1;
    }
    index.rem_euclid(s.length as i64) as usize
}

impl Sequencer {
    pub fn reset(&mut self) {
        self.expected = None;
    }
    fn flush(&mut self, offset: u32, emit: &mut impl FnMut(u32, Midi)) {
        for v in &mut self.voices {
            if let Some(v) = v.take() {
                emit(offset, Midi::Off(v.note));
            }
        }
    }
    pub fn stop(&mut self, emit: &mut impl FnMut(u32, Midi)) {
        self.flush(0, emit);
        self.expected = None;
    }
    // Scalars keep the engine independent of any host transport type.
    #[allow(clippy::too_many_arguments)]
    pub fn process(
        &mut self,
        s: &Settings,
        beat: f64,
        tempo: f64,
        sample_rate: f64,
        frames: usize,
        playing: bool,
        mut emit: impl FnMut(u32, Midi),
    ) {
        if frames == 0 {
            return;
        }
        if !playing
            || !beat.is_finite()
            || !tempo.is_finite()
            || tempo <= 0.0
            || !sample_rate.is_finite()
            || sample_rate <= 0.0
        {
            self.stop(&mut emit);
            return;
        }
        let delta = tempo / (60.0 * sample_rate);
        let jump = self
            .expected
            .is_none_or(|v| (v - beat).abs() > delta * 2.0 + 1e-7);
        if jump || self.swing != s.swing {
            self.flush(0, &mut emit);
            // Look behind the nominal grid for delayed notes, and ahead for early notes.
            // A note before a fresh transport start is skipped: no retroactive event.
            self.next_step = step_at(beat, s.swing) - 1;
            while onset(self.next_step, s) < beat - 1e-10 {
                self.next_step += 1;
            }
        }
        self.swing = s.swing;
        let steps = pattern(s);
        for frame in 0..frames {
            let now = beat + frame as f64 * delta;
            let offset = frame as u32;
            for voice in &mut self.voices {
                if voice.is_some_and(|v| now + 1e-10 >= v.until) {
                    emit(offset, Midi::Off(voice.take().unwrap().note));
                }
            }
            if now + 1e-10 < onset(self.next_step, s) {
                continue;
            }
            let index = self.next_step;
            self.next_step += 1;
            let slot = index.rem_euclid(s.length as i64) as usize;
            let step = steps[slot];
            if !step.on {
                self.flush(offset, &mut emit);
                continue;
            }
            for v in &mut self.voices {
                if v.is_some_and(|v| v.note == step.note) {
                    emit(offset, Midi::Off(v.take().unwrap().note));
                }
            }
            let start = onset(index, s);
            let end = onset(index + 1, s);
            let next = steps[(slot + 1) % s.length];
            // Offset bounds keep onsets ordered even at maximum swing. Gates stretch
            // to the next shifted onset; slides still overlap the following note.
            let until = if step.slide && next.on && next.note != step.note {
                end + 0.01
            } else {
                start + (end - start) * s.gate
            };
            // Automation can move an onset into the past; release stale voices before
            // emitting a late event. Never panic or exceed the fixed voice capacity.
            let position = self
                .voices
                .iter()
                .position(Option::is_none)
                .unwrap_or_else(|| {
                    self.flush(offset, &mut emit);
                    0
                });
            self.voices[position] = Some(Voice {
                note: step.note,
                until: until.max(now + delta),
            });
            emit(
                offset,
                Midi::On(step.note, if step.accent { 120 } else { 78 }),
            );
        }
        self.expected = Some(beat + frames as f64 * delta);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn render(s: &Settings, frames: usize, block: usize) -> Vec<(u32, Midi)> {
        let mut engine = Sequencer::default();
        let mut out = vec![];
        for start in (0..frames).step_by(block) {
            engine.process(
                s,
                start as f64 / 24000.0,
                120.0,
                48000.0,
                block.min(frames - start),
                true,
                |offset, event| out.push((start as u32 + offset, event)),
            );
        }
        out
    }
    #[test]
    fn timing_is_block_independent() {
        let s = Settings {
            density: 1.0,
            swing: 0.57,
            slide: 1.0,
            ..Settings::default()
        };
        assert_eq!(render(&s, 192000, 1), render(&s, 192000, 511));
        assert_eq!(render(&s, 192000, 64), render(&s, 192000, 8192));
    }
    #[test]
    fn sixteenths_and_gate_are_sample_accurate() {
        let s = Settings {
            density: 1.0,
            gate: 0.5,
            slide: 0.0,
            ..Settings::default()
        };
        let events = render(&s, 24000, 512);
        let on: Vec<_> = events
            .iter()
            .filter_map(|(t, e)| matches!(e, Midi::On(..)).then_some(*t))
            .collect();
        let off: Vec<_> = events
            .iter()
            .filter_map(|(t, e)| matches!(e, Midi::Off(..)).then_some(*t))
            .collect();
        assert_eq!(on, [0, 6000, 12000, 18000]);
        assert_eq!(off, [3000, 9000, 15000, 21000]);
    }
    #[test]
    fn zero_density_is_silent_and_seeds_are_repeatable() {
        let mut s = Settings {
            density: 0.0,
            ..Settings::default()
        };
        assert!(render(&s, 96000, 256).is_empty());
        s.density = 1.0;
        assert_eq!(pattern(&s), pattern(&s));
        let a = pattern(&s);
        s.seeds[0] += 1;
        assert_ne!(a, pattern(&s));
        for (a, b) in a.iter().zip(pattern(&s)) {
            assert_eq!((a.on, a.accent, a.slide), (b.on, b.accent, b.slide));
        }
    }
    #[test]
    fn stop_and_seek_release_notes() {
        let s = Settings {
            density: 1.0,
            ..Settings::default()
        };
        let mut q = Sequencer::default();
        let mut out = vec![];
        q.process(&s, 0.0, 120.0, 48000.0, 32, true, |t, e| out.push((t, e)));
        q.process(&s, 8.1, 120.0, 48000.0, 32, true, |t, e| out.push((t, e)));
        assert!(matches!(
            out.as_slice(),
            [(0, Midi::On(..)), (0, Midi::Off(..))]
        ));
        q.process(&s, 0.0, 120.0, 48000.0, 32, true, |_, _| {});
        out.clear();
        q.process(&s, 0.0, 120.0, 48000.0, 32, false, |t, e| out.push((t, e)));
        assert!(matches!(out.as_slice(), [(0, Midi::Off(..))]));
    }
    #[test]
    fn slides_overlap_and_all_notes_release() {
        let mut s = Settings {
            density: 1.0,
            slide: 1.0,
            accent: 0.0,
            ..Settings::default()
        };
        s.overrides = std::array::from_fn(|i| 36 + i as i32);
        let events = render(&s, 12500, 256);
        assert_eq!(events[0], (0, Midi::On(36, 78)));
        assert!(
            events
                .iter()
                .any(|(t, e)| *t == 6000 && matches!(e, Midi::On(37, _)))
        );
        assert!(
            events
                .iter()
                .any(|(t, e)| *t == 6240 && *e == Midi::Off(36))
        );
    }
    #[test]
    fn notes_stay_in_scale_and_overrides_win() {
        let mut s = Settings::default();
        for seed in 0..100 {
            s.seeds[0] = seed;
            for p in pattern(&s) {
                assert!([0, 2, 3, 5, 7, 8, 10].contains(&((p.note as i32 - s.root) % 12)));
            }
        }
        s.overrides[0] = 61;
        s.overrides[1] = -1;
        assert_eq!(pattern(&s)[0].note, 61);
        assert!(pattern(&s)[0].on);
        assert!(!pattern(&s)[1].on);
    }
    #[test]
    fn lifecycle_never_leaves_or_duplicates_a_note() {
        let mut q = Sequencer::default();
        let mut s = Settings {
            density: 1.0,
            slide: 1.0,
            ..Settings::default()
        };
        let mut held = [false; 128];
        let mut check = |_: u32, e: Midi| match e {
            Midi::On(n, _) => {
                assert!(!held[n as usize], "duplicate note on");
                held[n as usize] = true;
            }
            Midi::Off(n) => {
                assert!(held[n as usize], "unmatched note off");
                held[n as usize] = false;
            }
        };
        let mut beat = 0.0;
        for i in 0..2000 {
            let tempo = if i % 200 < 100 { 120.0 } else { 173.0 };
            if i % 157 == 0 {
                beat = 0.0;
            }
            if i % 113 == 0 {
                q.reset();
            }
            s.seeds[0] = (i / 41) as u32;
            s.length = 1 + i % 16;
            q.process(&s, beat, tempo, 48000.0, 511, i % 191 != 0, &mut check);
            beat += 511.0 * tempo / (60.0 * 48000.0);
        }
        q.stop(&mut check);
        assert!(held.iter().all(|v| !*v));
    }
    #[test]
    fn invalid_transport_releases_active_notes() {
        let mut q = Sequencer::default();
        let s = Settings {
            density: 1.0,
            ..Settings::default()
        };
        q.process(&s, 0.0, 120.0, 48000.0, 32, true, |_, _| {});
        let mut out = vec![];
        q.process(&s, f64::NAN, 120.0, 48000.0, 32, true, |t, e| {
            out.push((t, e))
        });
        assert!(matches!(out.as_slice(), [(0, Midi::Off(..))]));
    }
    #[test]
    fn timing_moves_notes_early_and_late_across_blocks() {
        let mut s = Settings {
            density: 1.0,
            slide: 0.0,
            ..Settings::default()
        };
        s.timings[0] = 0.25;
        s.timings[1] = -0.25;
        let out = render(&s, 14000, 511);
        let on: Vec<_> = out
            .iter()
            .filter_map(|(t, e)| matches!(e, Midi::On(..)).then_some(*t))
            .collect();
        assert_eq!(on, [1500, 4500, 12000]);
        assert_eq!(out, render(&s, 14000, 64));
    }
    #[test]
    fn early_first_step_plays_before_pattern_wrap() {
        let mut s = Settings {
            length: 64,
            density: 1.0,
            slide: 0.0,
            ..Settings::default()
        };
        s.timings[0] = -0.25;
        s.overrides = std::array::from_fn(|i| 36 + i as i32);
        let events = render(&s, 384000, 511);
        assert!(
            events
                .iter()
                .any(|(t, e)| *t == 382500 && matches!(e, Midi::On(36, _)))
        );
        assert!(
            !events
                .iter()
                .any(|(t, e)| *t == 0 && matches!(e, Midi::On(..)))
        );
        let notes: Vec<_> = events
            .iter()
            .filter_map(|(_, e)| {
                if let Midi::On(n, _) = e {
                    Some(*n)
                } else {
                    None
                }
            })
            .collect();
        assert_eq!(notes.len(), 64);
        assert_eq!(notes[62], 99);
    }
    #[test]
    fn shifted_patterns_are_independent_of_block_size() {
        let mut s = Settings {
            length: 64,
            density: 1.0,
            swing: 0.6,
            slide: 1.0,
            ..Settings::default()
        };
        s.timings = std::array::from_fn(|i| if i % 2 == 0 { 0.25 } else { -0.25 });
        assert_eq!(render(&s, 400000, 127), render(&s, 400000, 8192));
        let events = render(&s, 400000, 511);
        let mut previous = None;
        for (time, event) in events {
            if matches!(event, Midi::On(..)) {
                if let Some(p) = previous {
                    assert!(time > p);
                }
                previous = Some(time);
            }
        }
    }
    #[test]
    fn slides_follow_shifted_onsets() {
        let mut s = Settings {
            density: 1.0,
            slide: 1.0,
            accent: 0.0,
            ..Settings::default()
        };
        s.overrides[0] = 36;
        s.overrides[1] = 40;
        s.timings[1] = -0.25;
        let out = render(&s, 6000, 256);
        assert!(out.contains(&(4500, Midi::On(40, 78))));
        assert!(out.contains(&(4740, Midi::Off(36))));
    }
    #[cfg(feature = "rt-paranoid")]
    #[test]
    fn sixty_four_step_processing_does_not_allocate() {
        let mut q = Sequencer::default();
        let s = Settings {
            length: 64,
            density: 1.0,
            slide: 1.0,
            ..Settings::default()
        };
        let (_, violations) = truce::core::rt::audit(|| {
            let _section = truce::core::rt::RtSection::enter();
            q.process(&s, 0.0, 120.0, 48000.0, 192000, true, |_, _| {});
        });
        assert_eq!(violations, 0);
    }
}
