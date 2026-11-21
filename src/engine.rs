//! Original, deterministic acid sequencer. No Max runtime or copied patch assets.
use crate::Scale;
use truce::prelude::ParamEnum;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Settings {
    pub learned: [u32; 12],
    pub influence: f64,
    pub strict: bool,
    pub follow_root: bool,
    pub movement: f64,
    pub locks: [i32; 64], // -1 unlocked; packed note/on/accent/slide snapshot
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
            learned: [0; 12],
            influence: 1.0,
            strict: false,
            follow_root: false,
            movement: 1.0,
            locks: [-1; 64],
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
/// Build a fixed-size pitch pool. With no learning (or zero influence),
/// the legacy scale path is left bit-for-bit unchanged.
struct PitchPool {
    offsets: [i32; 12],
    weights: [f64; 12],
    len: usize,
    root: i32,
    guided: bool,
}
impl PitchPool {
    fn new(s: &Settings) -> Self {
        let scale = Scale::from_index(s.scale).intervals();
        let total: u32 = s.learned.iter().sum();
        let guided = total > 0 && (s.influence > 0.0 || s.strict);
        let mut pool = Self {
            offsets: [0; 12],
            weights: [0.0; 12],
            len: 0,
            root: s.root,
            guided,
        };
        if !guided {
            pool.offsets[..scale.len()].copy_from_slice(scale);
            pool.len = scale.len();
            return pool;
        }
        if s.follow_root {
            let dominant = (0..12).max_by_key(|i| s.learned[*i]).unwrap_or(0);
            pool.root = s.root.div_euclid(12) * 12 + dominant as i32;
        }
        for offset in 0..12 {
            let pc = (pool.root + offset).rem_euclid(12) as usize;
            let learned = f64::from(s.learned[pc]) / f64::from(total);
            let base = if scale.contains(&offset) {
                0.75 / scale.len() as f64 + if offset == 0 { 0.25 } else { 0.0 }
            } else {
                0.0
            };
            let weight = if s.strict {
                learned
            } else {
                (1.0 - s.influence) * base + s.influence * learned
            };
            if weight > 0.0 {
                pool.offsets[pool.len] = offset;
                pool.weights[pool.len] = weight;
                pool.len += 1;
            }
        }
        pool
    }
    fn choose(&self, choice: u32) -> usize {
        let mut remaining = f64::from(choice) / (f64::from(u32::MAX) + 1.0);
        for i in 0..self.len {
            remaining -= self.weights[i];
            if remaining < 0.0 {
                return i;
            }
        }
        self.len - 1
    }
}

pub fn pattern(s: &Settings) -> [Step; 64] {
    let pool = PitchPool::new(s);
    let scale = &pool.offsets[..pool.len];
    let mut previous_degree = 0i32;
    std::array::from_fn(|i| {
        // Repeated roots supply an anchor; other steps explore scale tones.
        let choice = random(s.seeds[0], i, 15);
        let degree = if pool.guided {
            pool.choose(choice)
        } else if choice.is_multiple_of(4) {
            0
        } else {
            // The root decision already consumed the low two bits. Reusing
            // them with 8/12-note scales would exclude degrees 4 and 8.
            let degree_choice = if scale.len().is_multiple_of(4) {
                choice / 4
            } else {
                choice
            };
            degree_choice as usize % scale.len()
        };
        let octave = (random(s.seeds[0], i, 32) % s.octaves as u32) as i32;
        let target = degree as i32 + octave * scale.len() as i32;
        let position = if s.movement >= 1.0 {
            target // Original generator at 100%, including saved projects.
        } else if s.movement <= 0.0 {
            0
        } else {
            let reach = (s.movement * (scale.len() as i32 * s.octaves - 1) as f64).ceil() as i32;
            if chance(s.seeds[0], i, 93, (1.0 - s.movement) * 0.5) {
                previous_degree
            } else {
                previous_degree + (target - previous_degree).clamp(-reach, reach)
            }
        };
        // Overrides and locks do not feed back into the contour: mutating one
        // note must not silently regenerate every following note.
        previous_degree = position;
        let note = (pool.root
            + scale[position as usize % scale.len()]
            + position / scale.len() as i32 * 12)
            .clamp(0, 127) as u8;
        let forced = s.overrides[i];
        let generated = Step {
            note: if forced >= 0 { forced as u8 } else { note },
            on: i < s.length
                && match forced {
                    -2 => chance(s.seeds[1], i, 41, s.density),
                    -1 => false,
                    _ => true,
                },
            accent: chance(s.seeds[2], i, 56, s.accent),
            slide: chance(s.seeds[2], i, 77, s.slide),
        };
        if s.locks[i] >= 0 {
            let mut locked = Step::unpack(s.locks[i]);
            locked.on &= i < s.length;
            locked
        } else {
            generated
        }
    })
}

impl Step {
    pub fn pack(self) -> i32 {
        i32::from(self.note)
            | ((self.on as i32) << 7)
            | ((self.accent as i32) << 8)
            | ((self.slide as i32) << 9)
    }
    fn unpack(value: i32) -> Self {
        Self {
            note: (value & 127) as u8,
            on: value & 128 != 0,
            accent: value & 256 != 0,
            slide: value & 512 != 0,
        }
    }
}

/// Change exactly ceil(eligible/4) sounding notes, selected without replacement.
/// Pitch overrides preserve rhythm, expression and every unselected step.
pub fn mutations(s: &Settings, seed: u32) -> [Option<u8>; 64] {
    let steps = pattern(s);
    let pool = PitchPool::new(s);
    let scale = &pool.offsets[..pool.len];
    let mut eligible = [0usize; 64];
    let mut count = 0;
    for (i, step) in steps.iter().enumerate().take(s.length) {
        if step.on && s.locks[i] < 0 {
            eligible[count] = i;
            count += 1;
        }
    }
    let mut result = [None; 64];
    for n in 0..count.div_ceil(4) {
        let pick = n + random(seed, n, 114) as usize % (count - n);
        eligible.swap(n, pick);
        let slot = eligible[n];
        let mut candidates = [0u8; 36];
        let mut len = 0;
        let mut nearest = i32::MAX;
        let reach = (s.movement * 36.0).ceil().max(2.0) as i32;
        // First establish the nearest different scale note for off-scale edits.
        for octave in 0..s.octaves {
            for degree in scale {
                let note = (pool.root + degree + octave * 12).clamp(0, 127);
                let distance = (note - i32::from(steps[slot].note)).abs();
                if distance > 0 {
                    nearest = nearest.min(distance);
                }
            }
        }
        for octave in 0..s.octaves {
            for degree in scale {
                let note = (pool.root + degree + octave * 12).clamp(0, 127);
                let distance = (note - i32::from(steps[slot].note)).abs();
                if distance > 0 && distance <= reach.max(nearest) {
                    candidates[len] = note as u8;
                    len += 1;
                }
            }
        }
        if len > 0 {
            result[slot] = Some(candidates[random(seed, slot, 129) as usize % len]);
        }
    }
    result
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
    fn movement_reduces_jumps_without_changing_other_lanes() {
        let mut total = [0u64; 2];
        for scale in 0..Scale::variant_count() {
            for seed in 0..32 {
                let s = Settings {
                    scale,
                    seeds: [seed, 7, 11],
                    length: 64,
                    octaves: 3,
                    ..Settings::default()
                };
                let wide = pattern(&s);
                let close = pattern(&Settings { movement: 0.1, ..s });
                let repeat = pattern(&Settings { movement: 0.0, ..s });
                for i in 0..64 {
                    assert_eq!(repeat[i].note, s.root as u8);
                    assert_eq!(
                        (wide[i].on, wide[i].accent, wide[i].slide),
                        (close[i].on, close[i].accent, close[i].slide)
                    );
                    assert!(
                        Scale::from_index(scale)
                            .intervals()
                            .contains(&((i32::from(close[i].note) - s.root) % 12))
                    );
                    if i > 0 {
                        total[0] += u64::from(wide[i].note.abs_diff(wide[i - 1].note));
                        total[1] += u64::from(close[i].note.abs_diff(close[i - 1].note));
                    }
                }
            }
        }
        assert!(
            total[1] < total[0] / 2,
            "low movement should materially reduce leaps"
        );
    }

    #[test]
    fn mutation_is_local_repeatable_and_respects_locks_and_rests() {
        for scale in 0..Scale::variant_count() {
            let mut s = Settings {
                scale,
                movement: 0.2,
                density: 1.0,
                length: 64,
                ..Settings::default()
            };
            s.overrides[1] = -1;
            s.overrides[2] = 127; // Off-scale manual edits can be brought back in range.
            let before = pattern(&s);
            s.locks[0] = before[0].pack();
            s.locks[63] = before[63].pack();
            let changes = mutations(&s, 123);
            assert_eq!(changes, mutations(&s, 123));
            assert_eq!(changes.iter().flatten().count(), 61usize.div_ceil(4));
            assert!(changes[0].is_none() && changes[1].is_none() && changes[63].is_none());
            for (i, note) in changes.iter().enumerate() {
                if let Some(note) = note {
                    assert_ne!(*note, before[i].note);
                    assert!(
                        Scale::from_index(scale)
                            .intervals()
                            .contains(&((i32::from(*note) - s.root) % 12))
                    );
                    s.overrides[i] = i32::from(*note);
                }
            }
            let after = pattern(&s);
            for i in 0..64 {
                if changes[i].is_none() {
                    assert_eq!(before[i], after[i]);
                }
                assert_eq!(
                    (before[i].on, before[i].accent, before[i].slide),
                    (after[i].on, after[i].accent, after[i].slide)
                );
            }
            s.locks = before.map(Step::pack);
            assert!(mutations(&s, 456).iter().all(Option::is_none));
            s.locks = [-1; 64];
            s.overrides = [-1; 64];
            assert!(mutations(&s, 456).iter().all(Option::is_none));
        }
    }

    #[test]
    fn locks_and_movement_survive_parameter_save_restore() {
        use crate::{PicaduraParams, PicaduraParamsParamId as P, params::LOCK_IDS};
        use truce::prelude::*;
        let params = PicaduraParams::default();
        params.set_plain(P::Movement.into(), 25.0);
        params.set_plain(P::Length.into(), 64.0);
        let original = pattern(&params.settings());
        for slot in [0, 31, 63] {
            params.set_normalized(
                LOCK_IDS[slot].into(),
                f64::from(original[slot].pack() + 1) / 1024.0,
            );
        }
        params.set_plain(P::NotesSeed.into(), 1000.0);
        params.set_plain(P::ExpressionSeed.into(), 2000.0);
        params.set_plain(P::Density.into(), 0.0);
        params.set_plain(P::Root.into(), 60.0);
        let (ids, values) = params.collect_values();
        let restored = PicaduraParams::default();
        restored.restore_values(&ids.into_iter().zip(values).collect::<Vec<_>>());
        assert_eq!(params.settings(), restored.settings());
        let held = pattern(&restored.settings());
        for slot in [0, 31, 63] {
            assert_eq!(held[slot], original[slot]);
        }
        restored.set_plain(P::Length.into(), 16.0);
        assert!(!pattern(&restored.settings())[63].on);
    }

    #[test]
    fn learned_notes_blend_obey_strict_and_preserve_manual_edits() {
        let base = Settings {
            length: 64,
            density: 1.0,
            ..Settings::default()
        };
        let mut learned = [0; 12];
        learned[1] = 9;
        learned[6] = 1; // C# and F#, outside C natural minor.
        let mut s = Settings {
            learned,
            influence: 0.0,
            ..base
        };
        assert_eq!(pattern(&s), pattern(&base));
        s.influence = 1.0;
        let mut counts = [0; 12];
        for seed in 0..100 {
            s.seeds[0] = seed;
            for step in pattern(&s) {
                let pc = step.note as usize % 12;
                assert!([1, 6].contains(&pc));
                counts[pc] += 1;
            }
        }
        assert!(counts[1] > counts[6] * 5);
        s.influence = 0.5;
        assert!(
            pattern(&s)
                .iter()
                .any(|step| ![1, 6].contains(&(step.note % 12)))
        );
        s.strict = true;
        s.influence = 0.0;
        s.movement = 0.1;
        assert!(
            pattern(&s)
                .iter()
                .all(|step| [1, 6].contains(&(step.note % 12)))
        );
        for note in mutations(&s, 57).into_iter().flatten() {
            assert!([1, 6].contains(&(note % 12)));
        }
        s.overrides[0] = 64;
        s.locks[1] = Step {
            note: 67,
            on: true,
            accent: true,
            slide: false,
        }
        .pack();
        assert_eq!(pattern(&s)[0].note, 64);
        assert_eq!(pattern(&s)[1].note, 67);
        s.learned = [0; 12];
        s.overrides = [-2; 64];
        s.locks = [-1; 64];
        s.movement = 1.0;
        assert_eq!(
            pattern(&s),
            pattern(&Settings {
                seeds: s.seeds,
                ..base
            })
        );
    }

    #[test]
    fn follow_root_uses_dominant_pitch_and_single_note_pool_can_repeat() {
        let mut s = Settings {
            learned: [0; 12],
            strict: true,
            follow_root: true,
            octaves: 1,
            density: 1.0,
            ..Settings::default()
        };
        s.learned[9] = 20;
        assert!(pattern(&s).iter().all(|step| step.note == 45));
        assert!(mutations(&s, 1).iter().all(Option::is_none));
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
    fn every_scale_is_reachable_and_generates_its_pitch_classes() {
        use crate::PicaduraParams;
        use truce::prelude::*;
        let expected: &[&[i32]] = &[
            &[0, 2, 3, 5, 7, 8, 10],
            &[0, 2, 3, 5, 7, 9, 10],
            &[0, 1, 3, 5, 7, 8, 10],
            &[0, 2, 4, 5, 7, 9, 11],
            &[0, 3, 5, 7, 10],
            &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
            &[0, 2, 4, 6, 7, 9, 11],
            &[0, 2, 4, 5, 7, 9, 10],
            &[0, 1, 3, 5, 6, 8, 10],
            &[0, 2, 3, 5, 7, 8, 11],
            &[0, 2, 3, 5, 7, 9, 11],
            &[0, 2, 4, 7, 9],
            &[0, 3, 5, 6, 7, 10],
            &[0, 2, 4, 6, 8, 10],
            &[0, 2, 3, 5, 6, 8, 9, 11],
            &[0, 1, 3, 4, 6, 7, 9, 10],
        ];
        assert_eq!(Scale::variant_count(), expected.len());
        let params = PicaduraParams::default();
        for (index, intervals) in expected.iter().enumerate() {
            params.set_normalized(
                crate::PicaduraParamsParamId::Scale.into(),
                index as f64 / (expected.len() - 1) as f64,
            );
            let mut settings = params.settings();
            assert_eq!(settings.scale, index);
            let mut seen = [false; 12];
            for seed in 0..100 {
                settings.seeds[0] = seed;
                for step in pattern(&settings) {
                    let degree = (i32::from(step.note) - settings.root) % 12;
                    assert!(
                        intervals.contains(&degree),
                        "scale {index}, degree {degree}"
                    );
                    seen[degree as usize] = true;
                }
            }
            for degree in *intervals {
                assert!(
                    seen[*degree as usize],
                    "scale {index} missing degree {degree}"
                );
            }
        }
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
            movement: 0.2,
            locks: [Step {
                note: 36,
                on: true,
                accent: true,
                slide: true,
            }
            .pack(); 64],
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
