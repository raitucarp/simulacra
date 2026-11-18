//! The Journal page entity: the open tab, the entry being written, the check-in form, the
//! focus run and the capsule being written or read, with their text fields. Changes to the
//! records go out as [`JournalEvent`]s; the shell stores them and hands back [`JournalData`].

mod capsules;
mod checkin;
mod entries;
mod fields;
mod focus;

pub use capsules::CapsuleForm;
pub use fields::JournalFields;
pub use focus::{now_ms as focus_now_ms, FocusRun};

use super::data::JournalData;
use super::events::JournalEvent;
use super::tab::JournalTab;
use crate::domain::journal::mood::CheckinDraft;
use crate::domain::journal::prompts::Prompt;
use crate::domain::journal::templates::Template;
use crate::domain::journal::RecordId;
use crate::theme::ThemeTokens;
use gpui_kit::{Bounds, Context, EventEmitter, FocusHandle, Pixels, Subscription, Task, WeakEntity, Window};
use std::cell::Cell;
use std::rc::Rc;

/// The entry open in the editor.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct EntryEditor {
    /// The entry being edited; `None` writes a new one.
    pub id: Option<RecordId>,
    pub prompt: Option<Prompt>,
    pub template: Option<Template>,
    /// The preview (rendered wikilinks) instead of the text field.
    pub preview: bool,
}

/// The Journal page.
pub struct JournalPanel {
    pub(super) theme: ThemeTokens,
    pub(super) data: JournalData,
    pub(super) tab: JournalTab,
    pub(super) fields: JournalFields,
    /// The editor while an entry is open.
    pub(super) editor: Option<EntryEditor>,
    /// Today's prompt, or the one picked with "another prompt".
    pub(super) prompt: Option<Prompt>,
    pub(super) draft: CheckinDraft,
    /// Where the affect grid was painted last (clicks are read against it).
    pub(super) grid_bounds: Rc<Cell<Option<Bounds<Pixels>>>>,
    pub(super) focus_run: FocusRun,
    pub(super) capsule: CapsuleForm,
    pub(super) focus: FocusHandle,
    pub(super) ticker: Option<Task<()>>,
    pub(super) weak: WeakEntity<Self>,
    _subscriptions: Vec<Subscription>,
}

impl EventEmitter<JournalEvent> for JournalPanel {}

impl JournalPanel {
    pub fn new(data: JournalData, theme: ThemeTokens, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let (fields, subscriptions) = JournalFields::new(window, cx);
        Self {
            theme,
            data,
            tab: JournalTab::Today,
            fields,
            editor: None,
            prompt: None,
            draft: CheckinDraft::default(),
            grid_bounds: Rc::default(),
            focus_run: FocusRun { preset: Some(crate::domain::journal::focus::PRESET_MINUTES[0]), ..FocusRun::default() },
            capsule: CapsuleForm::default(),
            focus: cx.focus_handle(),
            ticker: None,
            weak: cx.entity().downgrade(),
            _subscriptions: subscriptions,
        }
    }

    /// Swaps in fresh records; a new day forgets the picked prompt.
    pub fn set_data(&mut self, data: JournalData, cx: &mut Context<Self>) {
        if data.today != self.data.today {
            self.prompt = None;
        }
        if data != self.data {
            self.data = data;
            cx.notify();
        }
    }

    pub fn set_theme(&mut self, theme: &ThemeTokens, cx: &mut Context<Self>) {
        if &self.theme != theme {
            self.theme = theme.clone();
            cx.notify();
        }
    }

    /// Shows `tab`.
    pub fn show(&mut self, tab: JournalTab, cx: &mut Context<Self>) {
        self.tab = tab;
        cx.notify();
    }

    /// Whether focus mode is on (the shell hides the feed and side panels).
    pub fn is_focus_mode(&self) -> bool {
        self.focus_run.focus_mode
    }

    /// The prompt shown today.
    pub(super) fn current_prompt(&self) -> Prompt {
        self.prompt.unwrap_or_else(|| crate::domain::journal::prompts::prompt_for_day(self.data.today))
    }
}
