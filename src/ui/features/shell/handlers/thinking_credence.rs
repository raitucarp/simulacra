//! Credences in the shell (sb E1): the controls every card of the user's own posts gets,
//! and the popover's events (set, nudge, date, resolve, reopen, remove), each saved to the
//! vault as it happens.

use super::super::binder::Binder;
use super::super::view::AppView;
use crate::domain::thinking::credence::{clamp_percent, Credence, Outcome};
use crate::domain::PostId;
use crate::ui::features::thinking::credence::{CredenceChip, CredenceEvent, CredencePane, CredenceSource};
use jiff::Timestamp;
use gpui_kit::*;
use std::rc::Rc;

impl AppView {
    /// What the feed needs to draw credence chips and popovers.
    pub(in crate::ui::features::shell) fn credence_source(&self, bind: &Binder) -> CredenceSource {
        let today = crate::time::local_now().date();
        let chips = self.thinking.credences.iter().map(|(id, credence)| (id.as_str().to_string(), CredenceChip::of(credence, today))).collect();
        CredenceSource { chips: Rc::new(chips), open: self.thinking.credence_open.clone(), pane: CredencePane::Feed, note: self.thinking.note.clone(), on_event: Rc::new(bind.with(Self::on_credence_event)) }
    }

    pub(in crate::ui::features::shell) fn on_credence_event(&mut self, event: CredenceEvent, window: &mut Window, cx: &mut Context<Self>) {
        let today = crate::time::local_now().date();
        match event {
            CredenceEvent::Open(id, pane) => {
                self.thinking.credence_open = Some((id, pane));
                self.thinking.note.update(cx, |input, cx| input.set_value("", window, cx));
            }
            CredenceEvent::Close => self.thinking.credence_open = None,
            CredenceEvent::SetPercent(id, percent) => self.change_credence(&id, |c| Some(c.with_percent(percent))),
            CredenceEvent::Nudge(id, points) => self.change_credence(&id, |c| Some(c.clone().with_percent(clamp_percent(i32::from(c.percent) + i32::from(points))))),
            CredenceEvent::SetDue(id, preset) => self.change_credence(&id, |c| Some(c.with_due(preset.map(|p| p.date_from(today))))),
            CredenceEvent::Resolve(id, outcome) => {
                let note = self.thinking.note.read(cx).value().trim().to_string();
                self.change_credence(&id, |c| Some(c.resolved(outcome, &note, Timestamp::now())));
                self.thinking.note.update(cx, |input, cx| input.set_value("", window, cx));
            }
            CredenceEvent::Reopen(id) => self.change_credence(&id, |c| Some(c.reopened())),
            CredenceEvent::Remove(id) => {
                self.change_credence(&id, |_| None);
                self.thinking.credence_open = None;
            }
        }
        cx.notify();
    }

    /// Resolves an open prediction from the Calibration view's list (no note).
    pub(super) fn resolve_prediction(&mut self, id: PostId, outcome: Outcome, cx: &mut Context<Self>) {
        self.change_credence(&id, |c| Some(c.resolved(outcome, "", Timestamp::now())));
        cx.notify();
    }

    /// Applies `change` to the post's credence (a new one at even odds when there is none);
    /// `None` removes it. The result is saved at once.
    fn change_credence(&mut self, id: &PostId, change: impl FnOnce(Credence) -> Option<Credence>) {
        let current = self.thinking.credences.get(id).cloned().unwrap_or_else(|| Credence::new(id.clone(), EVEN_ODDS, Timestamp::now()));
        let vault = self.state.db.as_ref();
        match change(current) {
            Some(next) => {
                if let Some(vault) = vault {
                    let _ = vault.save_credence(next.clone());
                }
                self.thinking.credences.insert(id.clone(), next);
            }
            None => {
                if let Some(vault) = vault {
                    let _ = vault.delete_credence(id);
                }
                self.thinking.credences.remove(id);
            }
        }
    }
}

/// The percentage of a credence created by a date or a nudge before any percentage.
const EVEN_ODDS: u8 = 50;
