//! Deleting a journal entry, a mood check-in or a time capsule asks first (sb P16), with the
//! app's confirm dialog, as deleting a post or a vault document does.

use super::super::panel::JournalPanel;
use super::super::text::tr;
use crate::domain::journal::RecordId;
use gpui_kit::component::button::ButtonVariant;
use gpui_kit::component::WindowExt;
use gpui_kit::*;

/// What a delete button removes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeleteKind {
    Entry,
    Checkin,
    Capsule,
}

impl DeleteKind {
    #[cfg(test)]
    pub const ALL: [Self; 3] = [Self::Entry, Self::Checkin, Self::Capsule];

    const fn key(self) -> &'static str {
        match self {
            Self::Entry => "entry",
            Self::Checkin => "checkin",
            Self::Capsule => "capsule",
        }
    }

    /// The confirm's question.
    pub fn title_key(self) -> String {
        format!("journal.delete.{}_title", self.key())
    }

    /// What deleting it means.
    pub fn description_key(self) -> String {
        format!("journal.delete.{}_desc", self.key())
    }
}

impl JournalPanel {
    /// The delete button's click: a confirm, then the delete once it is accepted.
    pub(super) fn confirm_delete(&self, kind: DeleteKind, id: RecordId, cx: &mut Context<Self>) -> impl Fn(&ClickEvent, &mut Window, &mut App) + 'static {
        let panel = cx.entity().downgrade();
        move |_, window, cx| {
            let (panel, id) = (panel.clone(), id.clone());
            window.open_alert_dialog(cx, move |alert, _, _| {
                let (panel, id) = (panel.clone(), id.clone());
                alert
                    .confirm()
                    .title(tr(&kind.title_key()))
                    .description(tr(&kind.description_key()))
                    .ok_text(tr("journal.entry.delete"))
                    .ok_variant(ButtonVariant::Danger)
                    .cancel_text(tr("composer.cancel"))
                    .on_ok(move |_, _, cx| {
                        let _ = panel.update(cx, |this, cx| this.delete_record(kind, id.clone(), cx));
                        true
                    })
            });
        }
    }

    fn delete_record(&mut self, kind: DeleteKind, id: RecordId, cx: &mut Context<Self>) {
        match kind {
            DeleteKind::Entry => self.delete_entry(id, cx),
            DeleteKind::Checkin => self.delete_checkin(id, cx),
            DeleteKind::Capsule => self.delete_capsule(id, cx),
        }
    }
}
