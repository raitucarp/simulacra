//! The composer's schedule picker state (Y3): whether it is open and the typed moment. An
//! empty or closed picker posts at once; a readable future moment schedules the thread; any
//! other text blocks posting until it is fixed.

use super::super::schedule_picker::{example_moment, ScheduleView};
use super::ComposerModalPanel;
use crate::domain::post::schedule::{format_moment, parse_moment, SchedulePreset, ScheduleProblem};
use gpui_kit::component::input::InputState;
use gpui_kit::*;
use jiff::Timestamp;

/// The picker's state.
pub(super) struct SchedulePicker {
    pub open: bool,
    pub input: Entity<InputState>,
}

impl SchedulePicker {
    pub fn new(window: &mut Window, cx: &mut Context<ComposerModalPanel>) -> Self {
        let input = cx.new(|cx| InputState::new(window, cx).placeholder(example_moment()));
        Self { open: false, input }
    }
}

impl ComposerModalPanel {
    /// Opens the picker (focusing its field) or closes it, forgetting the moment.
    pub(super) fn toggle_schedule(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.schedule.open {
            return self.clear_schedule(window, cx);
        }
        self.schedule.open = true;
        self.schedule.input.update(cx, |state, cx| state.focus(window, cx));
        cx.notify();
    }

    /// Fills the field with `preset`'s moment.
    pub(super) fn pick_preset(&mut self, preset: SchedulePreset, window: &mut Window, cx: &mut Context<Self>) {
        let now = crate::time::local_now();
        if let Some(at) = preset.moment(&now) {
            let text = format_moment(at.timestamp(), at.time_zone());
            self.schedule.input.update(cx, |state, cx| state.set_value(text, window, cx));
        }
        cx.notify();
    }

    /// Opens the picker on `at` (editing a waiting post, Y4).
    pub(super) fn open_schedule_at(&mut self, at: Timestamp, window: &mut Window, cx: &mut Context<Self>) {
        self.schedule.open = true;
        let text = format_moment(at, &jiff::tz::TimeZone::system());
        self.schedule.input.update(cx, |state, cx| state.set_value(text, window, cx));
    }

    /// Closes the picker: the thread posts at once again.
    pub(super) fn clear_schedule(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.reset_schedule(window, cx);
        cx.notify();
    }

    /// Closes and empties the picker (opening and clearing the composer).
    pub(super) fn reset_schedule(&mut self, window: &mut Window, cx: &mut App) {
        self.schedule.open = false;
        self.schedule.input.update(cx, |state, cx| state.set_value("", window, cx));
    }

    /// The typed moment, or why it cannot be used; `None` when the thread posts at once.
    pub(super) fn schedule_choice(&self, cx: &App) -> Option<Result<Timestamp, ScheduleProblem>> {
        let text = self.schedule.input.read(cx).value().to_string();
        (self.schedule.open && !text.trim().is_empty()).then(|| parse_moment(&text, &crate::time::local_now()))
    }

    /// The open picker as the dialog renders it.
    pub(super) fn schedule_view(&self, cx: &mut Context<Self>) -> Option<ScheduleView> {
        if !self.schedule.open {
            return None;
        }
        let (preset_entity, clear_entity) = (cx.entity(), cx.entity());
        Some(ScheduleView {
            input: self.schedule.input.clone(),
            choice: self.schedule_choice(cx),
            on_preset: std::rc::Rc::new(move |preset, window, cx| preset_entity.update(cx, |this, cx| this.pick_preset(preset, window, cx))),
            on_clear: std::rc::Rc::new(move |window, cx| clear_entity.update(cx, |this, cx| this.clear_schedule(window, cx))),
        })
    }
}
