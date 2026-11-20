use crate::{
    PicaduraParams, PicaduraParamsParamId as P, engine,
    params::{LOCK_IDS, NOTE_IDS, TIMING_IDS},
};
use std::{cell::Cell, rc::Rc, sync::Arc};
use truce::core::editor::PluginContextReadF32;
use truce::prelude::*;
use truce_slint::{
    SlintEditor,
    slint::{Model, ModelRc, SharedString, VecModel, include_modules},
};
include_modules!();
const CONTROLS: [P; 10] = [
    P::Root,
    P::Scale,
    P::Length,
    P::Octaves,
    P::Density,
    P::Accent,
    P::Gate,
    P::Slide,
    P::Swing,
    P::Movement,
];
const LABELS: [&str; 10] = [
    "ROOT", "SCALE", "STEPS", "OCTAVES", "DENSITY", "ACCENT", "GATE", "SLIDE", "SWING", "MOVEMENT",
];
fn note_name(note: u8) -> String {
    format!(
        "{}{}",
        [
            "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"
        ][note as usize % 12],
        note as i32 / 12 - 1
    )
}
// One gesture owns one parameter. Closing the editor or switching pages also
// closes the host gesture, so dragging never leaves automation latched.
struct Gesture {
    state: PluginContext<PicaduraParams>,
    active: Cell<Option<u32>>,
}
impl Gesture {
    fn begin(&self, id: u32) {
        self.end();
        self.state.begin_edit(id);
        self.active.set(Some(id));
    }
    fn end(&self) {
        if let Some(id) = self.active.take() {
            self.state.end_edit(id);
        }
    }
    fn set(&self, value: f64) {
        if let Some(id) = self.active.get() {
            self.state.set_param(id, value.clamp(0.0, 1.0));
        }
    }
}
impl Drop for Gesture {
    fn drop(&mut self) {
        self.end();
    }
}

pub fn editor(params: Arc<PicaduraParams>) -> Box<dyn Editor> {
    Box::new(
        SlintEditor::new(params, (1120, 660), |state| {
            let ui = PicaduraUi::new().expect("create Slint editor");
            let page = Rc::new(Cell::new(0usize));
            let gesture = Rc::new(Gesture {
                state: state.clone(),
                active: Cell::new(None),
            });
            let controls = Rc::new(VecModel::from(vec![ControlView::default(); 10]));
            let steps = Rc::new(VecModel::from(vec![StepView::default(); 16]));
            ui.set_controls(ModelRc::from(controls.clone()));
            ui.set_steps(ModelRc::from(steps.clone()));
            {
                let g = gesture.clone();
                ui.on_control_began(move |index| {
                    if let Some(id) = CONTROLS.get(index as usize) {
                        g.begin((*id).into());
                    }
                });
            }
            {
                let g = gesture.clone();
                ui.on_control_changed(move |_, value| g.set(f64::from(value)));
            }
            {
                let g = gesture.clone();
                ui.on_control_ended(move |_| g.end());
            }
            {
                let g = gesture.clone();
                let p = page.clone();
                ui.on_step_began(move |index, axis| {
                    let slot = p.get() * 16 + index as usize;
                    if slot < 64 && g.state.params().settings().locks[slot] < 0 {
                        g.begin(
                            if axis == 0 {
                                NOTE_IDS[slot]
                            } else {
                                TIMING_IDS[slot]
                            }
                            .into(),
                        );
                    }
                });
            }
            {
                let g = gesture.clone();
                ui.on_step_changed(move |_, axis, value| {
                    g.set(if axis == 0 {
                        (f64::from(value.round()) + 2.0) / 129.0
                    } else {
                        (f64::from(value) + 25.0) / 50.0
                    });
                });
            }
            {
                let g = gesture.clone();
                ui.on_step_ended(move |_, _| g.end());
            }
            {
                let p = page.clone();
                let cx = state.clone();
                ui.on_step_toggle(move |index| {
                    let slot = p.get() * 16 + index as usize;
                    let s = cx.params().settings();
                    if slot < s.length && s.locks[slot] < 0 {
                        let step = engine::pattern(&s)[slot];
                        let note = if step.on { -1 } else { i32::from(step.note) };
                        cx.automate(NOTE_IDS[slot], f64::from(note + 2) / 129.0);
                    }
                });
            }
            {
                let p = page.clone();
                let cx = state.clone();
                ui.on_step_lock(move |index| {
                    let slot = p.get() * 16 + index as usize;
                    let s = cx.params().settings();
                    if slot < s.length {
                        let value = if s.locks[slot] >= 0 {
                            -1
                        } else {
                            engine::pattern(&s)[slot].pack()
                        };
                        cx.automate(LOCK_IDS[slot], f64::from(value + 1) / 1024.0);
                    }
                });
            }
            {
                let p = page.clone();
                let g = gesture.clone();
                ui.on_page_changed(move |value| {
                    g.end();
                    p.set(value.clamp(0, 3) as usize);
                });
            }
            {
                let cx = state.clone();
                ui.on_action(move |action| match action {
                    0..=3 => {
                        let seeds = cx.params().settings().seeds;
                        for (lane, id) in [P::NotesSeed, P::RhythmSeed, P::ExpressionSeed]
                            .iter()
                            .enumerate()
                        {
                            if action == 0 || action as usize == lane + 1 {
                                cx.automate(*id, f64::from((seeds[lane] + 1) % 65536) / 65535.0);
                            }
                        }
                    }
                    4 => {
                        let s = cx.params().settings();
                        for (slot, id) in NOTE_IDS.iter().enumerate() {
                            if s.locks[slot] < 0 {
                                cx.automate(*id, 0.0);
                            }
                        }
                    }
                    5 => {
                        let s = cx.params().settings();
                        for (slot, id) in TIMING_IDS.iter().enumerate() {
                            if s.locks[slot] < 0 {
                                cx.automate(*id, 0.5);
                            }
                        }
                    }
                    6 => cx.automate(P::Run, if cx.params().run.value() { 0.0 } else { 1.0 }),
                    7 => {
                        let s = cx.params().settings();
                        let seed = (cx.params().mutation_seed.value_i32() as u32 + 1) % 65536;
                        let changes = engine::mutations(&s, seed);
                        if changes.iter().any(Option::is_some) {
                            cx.automate(P::MutationSeed, f64::from(seed) / 65535.0);
                            for (slot, note) in changes.iter().enumerate() {
                                if let Some(note) = note {
                                    cx.automate(NOTE_IDS[slot], (f64::from(*note) + 2.0) / 129.0);
                                }
                            }
                        }
                    }
                    _ => {}
                });
            }
            Box::new(move |cx: &PluginContext<PicaduraParams>| {
                let s = cx.params().settings();
                let pattern = engine::pattern(&s);
                let running = cx.params().run.value();
                let transport = cx.transport();
                let current = transport
                    .filter(|t| t.playing && running)
                    .map(|t| engine::playing_step(&s, t.position_beats));
                ui.set_running(running);
                ui.set_page(page.get() as i32);
                ui.set_page_text(
                    format!("{} STEPS  /  PAGE {} OF 4", s.length, page.get() + 1).into(),
                );
                ui.set_status(if !running {
                    "MUTED".into()
                } else if let Some(t) = transport.filter(|t| t.playing) {
                    format!("{:.1} BPM  /  MIDI CH 1", t.tempo).into()
                } else {
                    "WAITING FOR TRANSPORT".into()
                });
                ui.set_seed_text(
                    format!("SEED {} / {} / {}", s.seeds[0], s.seeds[1], s.seeds[2]).into(),
                );
                for (i, id) in CONTROLS.iter().enumerate() {
                    controls.set_row_data(
                        i,
                        ControlView {
                            label: LABELS[i].into(),
                            value: cx.get_param(*id),
                            value_text: if i == 0 {
                                note_name(s.root as u8).into()
                            } else {
                                cx.format_param(*id).into()
                            },
                        },
                    );
                }
                for i in 0..16 {
                    let slot = page.get() * 16 + i;
                    let step = pattern[slot];
                    let next = pattern[(slot + 1) % s.length];
                    let timing = s.timings[slot] * 100.0;
                    steps.set_row_data(
                        i,
                        StepView {
                            number: format!("{:02}", slot + 1).into(),
                            note: note_name(step.note).into(),
                            pitch: i32::from(step.note),
                            timing: timing as f32,
                            timing_text: SharedString::from(if timing.abs() < 0.05 {
                                "0%".into()
                            } else {
                                format!("{timing:+.1}%")
                            }),
                            locked: s.locks[slot] >= 0,
                            on: step.on,
                            enabled: slot < s.length,
                            accent: step.accent,
                            slide: step.slide && next.on && next.note != step.note,
                            edited: s.overrides[slot] != -2 || timing.abs() > 0.05,
                            playing: current == Some(slot),
                        },
                    );
                }
            })
        })
        .resizable(true)
        .min_size((1040, 620)),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use slint::{
        LogicalPosition, PhysicalSize,
        platform::{PointerEventButton, WindowEvent},
    };
    use std::cell::RefCell;

    #[test]
    fn pointer_drags_choose_pitch_or_timing_and_never_toggle_a_note() {
        truce_slint::platform::ensure_platform();
        let window = truce_slint::platform::create_slint_window();
        window.set_size(PhysicalSize::new(1120, 660));
        let ui = PicaduraUi::new().unwrap();
        ui.set_controls(ModelRc::new(VecModel::from(vec![
            ControlView::default();
            10
        ])));
        ui.set_steps(ModelRc::new(VecModel::from(vec![
            StepView {
                enabled: true,
                on: true,
                pitch: 36,
                ..StepView::default()
            };
            16
        ])));
        let log = Rc::new(RefCell::new(Vec::<(i32, i32, f32)>::new()));
        {
            let l = log.clone();
            ui.on_step_began(move |_, axis| l.borrow_mut().push((0, axis, 0.0)));
        }
        {
            let l = log.clone();
            ui.on_step_changed(move |_, axis, v| l.borrow_mut().push((1, axis, v)));
        }
        {
            let l = log.clone();
            ui.on_step_ended(move |_, axis| l.borrow_mut().push((2, axis, 0.0)));
        }
        {
            let l = log.clone();
            ui.on_step_toggle(move |_| l.borrow_mut().push((3, 0, 0.0)));
        }
        {
            let l = log.clone();
            ui.on_step_lock(move |slot| l.borrow_mut().push((4, slot, 0.0)));
        }
        truce_slint::platform::render_to_rgba(&window, 1120, 660, &mut vec![], &mut vec![]);
        let drag = |dx: f32, dy: f32| {
            let start = LogicalPosition::new(45.0, 400.0);
            let end = LogicalPosition::new(45.0 + dx, 400.0 + dy);
            window.dispatch_event(WindowEvent::PointerPressed {
                position: start,
                button: PointerEventButton::Left,
            });
            window.dispatch_event(WindowEvent::PointerMoved { position: end });
            window.dispatch_event(WindowEvent::PointerReleased {
                position: end,
                button: PointerEventButton::Left,
            });
        };
        drag(0.0, -30.0);
        assert_eq!(*log.borrow(), [(0, 0, 0.0), (1, 0, 41.0), (2, 0, 0.0)]);
        log.borrow_mut().clear();
        drag(30.0, 0.0);
        assert_eq!(*log.borrow(), [(0, 1, 0.0), (1, 1, 15.0), (2, 1, 0.0)]);
        log.borrow_mut().clear();
        drag(-30.0, 0.0);
        assert_eq!(*log.borrow(), [(0, 1, 0.0), (1, 1, -15.0), (2, 1, 0.0)]);
        log.borrow_mut().clear();
        drag(0.0, 0.0);
        assert_eq!(*log.borrow(), [(3, 0, 0.0)]);
        log.borrow_mut().clear();
        // A locked card rejects note/timing gestures, but its lock toggle works.
        ui.set_steps(ModelRc::new(VecModel::from(vec![
            StepView {
                enabled: true,
                locked: true,
                on: true,
                pitch: 36,
                ..StepView::default()
            };
            16
        ])));
        truce_slint::platform::render_to_rgba(&window, 1120, 660, &mut vec![], &mut vec![]);
        drag(0.0, -30.0);
        drag(0.0, 0.0);
        assert!(log.borrow().is_empty());
        let position = LogicalPosition::new(60.0, 332.0);
        window.dispatch_event(WindowEvent::PointerPressed {
            position,
            button: PointerEventButton::Left,
        });
        window.dispatch_event(WindowEvent::PointerReleased {
            position,
            button: PointerEventButton::Left,
        });
        assert_eq!(*log.borrow(), [(4, 0, 0.0)]);
    }
}
