//! The Arena page entity: the open tab, the debate setup form and the debate on show, the
//! lab's scenario form and the experiment on show, with their text fields. Requests go out
//! as [`ArenaEvent`]s; the shell runs them and hands back [`ArenaData`].

mod actions;
mod debate_draft;
mod fields;
mod lab_form;
mod prefill;

pub use debate_draft::DebateDraft;
pub use fields::ArenaFields;
pub use lab_form::LabDraft;

use super::data::ArenaData;
use super::events::ArenaEvent;
use super::tab::ArenaTab;
use crate::domain::arena::ArenaId;
use crate::theme::ThemeTokens;
use gpui_kit::{Context, EventEmitter, FocusHandle, Subscription, Window};

/// The Arena page.
pub struct ArenaPanel {
    pub(super) theme: ThemeTokens,
    pub(super) data: ArenaData,
    pub(super) tab: ArenaTab,
    pub(super) fields: ArenaFields,
    pub(super) draft: DebateDraft,
    /// The debate on show; `None` shows the setup form.
    pub(super) open_debate: Option<ArenaId>,
    pub(super) lab: LabDraft,
    /// The experiment on show (its results beside the form).
    pub(super) open_experiment: Option<ArenaId>,
    pub(super) focus: FocusHandle,
    _subscriptions: Vec<Subscription>,
}

impl EventEmitter<ArenaEvent> for ArenaPanel {}

impl ArenaPanel {
    pub fn new(data: ArenaData, theme: ThemeTokens, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let (fields, subscriptions) = ArenaFields::new(window, cx);
        Self {
            theme,
            data,
            tab: ArenaTab::Debate,
            fields,
            draft: DebateDraft::default(),
            open_debate: None,
            lab: LabDraft::default(),
            open_experiment: None,
            focus: cx.focus_handle(),
            _subscriptions: subscriptions,
        }
    }

    /// Swaps in fresh data; a record that was deleted is no longer shown.
    pub fn set_data(&mut self, data: ArenaData, cx: &mut Context<Self>) {
        if data == self.data {
            return;
        }
        if self.open_debate.as_ref().is_some_and(|id| data.debate(id).is_none()) {
            self.open_debate = None;
        }
        if self.open_experiment.as_ref().is_some_and(|id| data.experiment(id).is_none()) {
            self.open_experiment = None;
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

    /// Shows `tab`.
    pub fn show(&mut self, tab: ArenaTab, cx: &mut Context<Self>) {
        self.tab = tab;
        cx.notify();
    }

    /// Shows the debate `id` (the shell opens a debate it just started).
    pub fn open_debate(&mut self, id: ArenaId, cx: &mut Context<Self>) {
        self.tab = ArenaTab::Debate;
        self.open_debate = Some(id);
        cx.notify();
    }

    /// Shows the experiment `id`.
    pub fn open_experiment(&mut self, id: ArenaId, cx: &mut Context<Self>) {
        self.tab = ArenaTab::Lab;
        self.open_experiment = Some(id);
        cx.notify();
    }
}
