use truce::prelude::*;
mod engine;
mod params;
mod ui;
use engine::{Midi, Sequencer, Settings};
pub use params::{PicaduraParams, PicaduraParamsParamId, Scale};

impl PicaduraParams {
    pub(crate) fn settings(&self) -> Settings {
        Settings {
            root: self.root.value_i32(),
            scale: self.scale.value().to_index(),
            length: self.length.value_i32() as usize,
            density: f64::from(self.density.value()) / 100.0,
            accent: f64::from(self.accent.value()) / 100.0,
            gate: f64::from(self.gate.value()) / 100.0,
            slide: f64::from(self.slide.value()) / 100.0,
            swing: f64::from(self.swing.value()) / 100.0,
            octaves: self.octaves.value_i32(),
            seeds: [
                self.notes_seed.value_i32() as u32,
                self.rhythm_seed.value_i32() as u32,
                self.expression_seed.value_i32() as u32,
            ],
            overrides: self.overrides(),
            timings: self.timings(),
            movement: f64::from(self.movement.value()) / 100.0,
            locks: self.locks(),
        }
    }
}
#[derive(Default)]
pub struct Picadura {
    sequencer: Sequencer,
}
impl PluginLogic for Picadura {
    type Params = PicaduraParams;
    type DspState = Self;
    fn reset(state: &mut Self, _: &PicaduraParams, _: &AudioConfig) {
        // Keep voices until the next callback can deliver their note-offs.
        state.sequencer.reset();
    }
    fn process(
        state: &mut Self,
        params: &PicaduraParams,
        buffer: &mut AudioBuffer,
        _: &EventList,
        context: &mut ProcessContext,
    ) -> ProcessStatus {
        // Some hosts supply audio buses for a MIDI effect; preserve that audio.
        for ch in 0..buffer.num_output_channels() {
            if ch < buffer.num_input_channels() {
                let (input, output) = buffer.io_pair(ch, ch);
                output.copy_from_slice(input);
            } else {
                buffer.output(ch).fill(0.0);
            }
        }
        let settings = params.settings();
        let transport = context.transport;
        state.sequencer.process(
            &settings,
            transport.position_beats,
            transport.tempo,
            context.sample_rate,
            context.block_size,
            transport.playing && params.run.value(),
            |offset, midi| {
                let body = match midi {
                    Midi::On(note, velocity) => EventBody::NoteOn {
                        group: 0,
                        channel: 0,
                        note,
                        velocity,
                    },
                    Midi::Off(note) => EventBody::NoteOff {
                        group: 0,
                        channel: 0,
                        note,
                        velocity: 0,
                    },
                };
                context.output_events.push(Event::new(offset, body));
            },
        );
        ProcessStatus::Normal
    }
    fn editor(params: Arc<PicaduraParams>) -> Box<dyn Editor> {
        ui::editor(params)
    }
}
truce::plugin! { logic: Picadura, params: PicaduraParams, }
truce::enable_rt_paranoid!();
