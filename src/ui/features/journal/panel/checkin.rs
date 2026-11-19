//! The check-in form: a point on the affect grid (read from a click against the grid's
//! painted bounds), the energy level, the "I'd like pointers to support" tick, and saving.

use super::{JournalFields, JournalPanel};
use crate::domain::journal::mood::{parse_tags, Affect, CheckinDraft};
use crate::domain::journal::{journal_day, RecordId};
use crate::ui::features::journal::events::JournalEvent;
use gpui_kit::{Context, Pixels, Point, Window};

impl JournalPanel {
    /// A click at `position` (window coordinates) on the grid sets the point.
    pub(in crate::ui::features::journal) fn click_grid(&mut self, position: Point<Pixels>, cx: &mut Context<Self>) {
        let Some(bounds) = self.grid_bounds.get() else { return };
        let (width, height) = (bounds.size.width.as_f32().max(1.0), bounds.size.height.as_f32().max(1.0));
        let x = (position.x - bounds.origin.x).as_f32() / width;
        let y = (position.y - bounds.origin.y).as_f32() / height;
        self.draft.affect = Some(Affect::from_grid(x, y));
        cx.notify();
    }

    pub(in crate::ui::features::journal) fn set_energy(&mut self, energy: u8, cx: &mut Context<Self>) {
        self.draft.energy = energy;
        cx.notify();
    }

    pub(in crate::ui::features::journal) fn toggle_support(&mut self, cx: &mut Context<Self>) {
        self.draft.wants_support = !self.draft.wants_support;
        cx.notify();
    }

    /// Saves the check-in (once a point is chosen) and clears the form.
    pub(in crate::ui::features::journal) fn save_checkin(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let now = crate::time::local_now();
        let sleep = JournalFields::text(&self.fields.sleep, cx).replace(',', ".").parse::<f32>().ok();
        let draft = CheckinDraft {
            sleep_hours: sleep,
            tags: parse_tags(&JournalFields::text(&self.fields.tags, cx)),
            note: JournalFields::text(&self.fields.note, cx),
            ..self.draft.clone()
        };
        let Some(checkin) = draft.into_checkin(RecordId::random(), now.clone(), journal_day(&now)) else { return };
        cx.emit(JournalEvent::SaveCheckin(checkin));
        self.draft = CheckinDraft::default();
        for field in [&self.fields.note, &self.fields.tags, &self.fields.sleep] {
            field.update(cx, |state, cx| state.set_value("", window, cx));
        }
        cx.notify();
    }

    pub(in crate::ui::features::journal) fn delete_checkin(&mut self, id: RecordId, cx: &mut Context<Self>) {
        cx.emit(JournalEvent::DeleteCheckin(id));
    }
}
