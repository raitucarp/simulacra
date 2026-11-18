//! The Beliefs tab of an agent's profile (sb F1): the agent's propositions with their
//! credence, where each came from, its history as a sparkline and its revision log, and the
//! user's controls (pin, edit, forget), the automatic-update switch with today's count, and
//! "Read the open thread".
//!
//! [`BeliefsPanel`] is the tab entity; the shell owns the beliefs, applies its
//! [`BeliefsEvent`]s and hands back [`BeliefsData`].

pub mod data;
pub mod events;
mod render;
pub mod spark;

#[cfg(test)]
mod tests;

pub use data::{BeliefRow, BeliefsData, HISTORY_ROWS};
pub use events::BeliefsEvent;

use crate::domain::beliefs::{BeliefId, Credence};
use crate::theme::ThemeTokens;
use gpui_kit::component::input::InputState;
use gpui_kit::{AppContext, Context, Entity, EventEmitter, Window};
use rust_i18n::t;

/// The belief being edited and its credence so far.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BeliefEdit {
    pub id: BeliefId,
    pub credence: Credence,
}

/// The Beliefs tab.
pub struct BeliefsPanel {
    pub(crate) theme: ThemeTokens,
    pub(crate) data: BeliefsData,
    pub(crate) editing: Option<BeliefEdit>,
    pub(crate) wording: Entity<InputState>,
    /// The belief whose revision log is open.
    pub(crate) expanded: Option<BeliefId>,
}

impl EventEmitter<BeliefsEvent> for BeliefsPanel {}

impl BeliefsPanel {
    pub fn new(theme: ThemeTokens, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let placeholder = t!("beliefs.edit_hint").to_string();
        let wording = cx.new(|cx| InputState::new(window, cx).placeholder(placeholder));
        Self { theme, data: BeliefsData::default(), editing: None, wording, expanded: None }
    }

    /// Swaps in fresh data; another agent closes any edit.
    pub fn set_data(&mut self, data: BeliefsData, cx: &mut Context<Self>) {
        if data == self.data {
            return;
        }
        if data.agent != self.data.agent {
            self.editing = None;
            self.expanded = None;
        }
        self.data = data;
        cx.notify();
    }

    pub fn set_theme(&mut self, theme: &ThemeTokens, cx: &mut Context<Self>) {
        if &self.theme != theme {
            self.theme = theme.clone();
            cx.notify();
        }
    }
}
