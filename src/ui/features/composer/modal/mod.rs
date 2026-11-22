//! Composer modal dialog organism: one editor per post of the thread.
//! Every size and color comes from `ThemeTokens`.

mod attachments;
mod draft_row;
mod editor;
mod file_tray;
mod footer;
mod header;
mod media_tray;
mod quote_preview;
mod render;
mod reply_context;
mod state;
mod thread_rows;

use crate::domain::{AuthoredPost, Profile};
use crate::theme::ThemeTokens;
use gpui_kit::component::input::{InputState, TextareaState};
use gpui_kit::*;
use std::rc::Rc;

pub type ModalCloseCallback = Rc<dyn Fn(&mut Window, &mut App) + 'static>;
pub type ModalSubmitCallback = Rc<dyn Fn(&mut Window, &mut App) + 'static>;
pub type ModalActionCallback = Rc<dyn Fn(&mut Window, &mut App) + 'static>;
/// Callback acting on the draft at the given thread position.
pub type DraftActionCallback = Rc<dyn Fn(usize, &mut Window, &mut App) + 'static>;
/// A paste into a draft; `true` when it attached images instead of inserting text (sb C3).
pub type PasteCallback = Rc<dyn Fn(&ClipboardItem, &mut Window, &mut App) -> bool + 'static>;

#[derive(Debug, Clone, PartialEq)]
pub enum ComposerModalMode {
    NewThread,
    Reply(Box<AuthoredPost>),
    Quote(Box<AuthoredPost>),
    /// sb F2: rewriting the text of one of the user's own posts.
    Edit(Box<AuthoredPost>),
}

/// One post editor of the thread with its current (trimmed) character count.
#[derive(Clone)]
pub struct DraftEditor {
    pub input: Entity<TextareaState>,
    pub length: usize,
    /// The draft text (what the preview renders).
    pub text: String,
}

/// Every interaction the dialog dispatches back to its owner.
#[derive(Clone)]
pub struct ModalCallbacks {
    pub on_close: ModalCloseCallback,
    pub on_submit: ModalSubmitCallback,
    pub on_toggle_link: ModalActionCallback,
    pub on_toggle_image: ModalActionCallback,
    pub on_pick_image: ModalActionCallback,
    pub on_clear_image: ModalActionCallback,
    pub on_clear_link: ModalActionCallback,
    pub on_add_draft: ModalActionCallback,
    /// Inserts the hashtag sigil at the cursor of the active draft.
    pub on_insert_tag: ModalActionCallback,
    /// sb D3: opens image ideas for the draft.
    pub on_image_ideas: ModalActionCallback,
    pub on_remove_draft: DraftActionCallback,
    pub on_split_draft: DraftActionCallback,
    /// Removes the attached image at an index (sb C3).
    pub on_remove_image: DraftActionCallback,
    /// Removes the attached file at an index (L12).
    pub on_remove_file: DraftActionCallback,
    pub on_paste: PasteCallback,
    /// Toggles a mark on the selection of the last focused draft.
    pub on_format: super::format::FormatCallback,
    /// Switches between the editors and the preview.
    pub on_toggle_preview: ModalActionCallback,
    /// Picks the thread's container tint (sb C3).
    pub on_pick_tint: super::tint_picker::TintCallback,
    /// Opens or closes the "Schedule…" picker (Y3).
    pub on_toggle_schedule: ModalActionCallback,
    /// Toggles a symbol, and opens or closes the symbol search (S5).
    pub on_pick_symbol: super::symbol_picker::SymbolCallback,
    pub on_toggle_symbol_search: ModalActionCallback,
    /// Switches free compose on or off (T2).
    pub on_toggle_free: ModalActionCallback,
    /// Opens or closes the to-do syntax help card (D1).
    pub on_toggle_todo_help: ModalActionCallback,
    /// Opens or closes "Lock with password" (X3).
    pub on_toggle_lock: ModalActionCallback,
}

/// Stateless dialog rendering a composer snapshot.
#[derive(IntoElement)]
pub struct ComposerModalView {
    pub theme: ThemeTokens,
    pub profile: Option<Profile>,
    pub mode: ComposerModalMode,
    pub drafts: Vec<DraftEditor>,
    pub link_input: Entity<InputState>,
    pub image_input: Entity<InputState>,
    /// Attached images and their description fields (sb C3).
    pub images: Vec<(String, Entity<InputState>)>,
    /// Images still being copied into the media folder.
    pub pending_images: usize,
    /// The other files attached (L12), and how many are still being read.
    pub files: Vec<crate::domain::media::FileRef>,
    pub pending_files: usize,
    pub is_link_input_open: bool,
    pub is_image_input_open: bool,
    /// Publish error or the reason the thread cannot be posted yet.
    pub notice: Option<String>,
    pub can_post: bool,
    /// Drafts are shown as they will read once posted.
    pub preview: bool,
    /// The container tint every post of the thread (or the edited post) gets (sb C3).
    pub tint: Option<crate::domain::post::Tint>,
    /// The open "Schedule…" picker (Y3); `None` while closed.
    pub schedule: Option<super::schedule_picker::ScheduleView>,
    /// The open "Lock with password" row (X3); `None` while closed.
    pub lock: Option<super::lock_picker::LockView>,
    /// The symbol picker (S5).
    pub symbols: super::symbol_picker::SymbolPickerView,
    /// The version row while a published post is edited (V3).
    pub version: Option<super::version_picker::VersionView>,
    /// The preview of the split while composing freely (T2).
    pub free: Option<super::free_compose::FreeComposeView>,
    /// The to-do syntax help card is open (D1).
    pub todo_help: bool,
    /// What the to-do word at the cursor could be (D11); `None` while there is nothing to offer.
    pub todo_suggest: Option<super::todo_suggest::TodoSuggestView>,
    pub callbacks: ModalCallbacks,
    /// sb E1: the atomic-note hint under one draft.
    pub atomic_hint: Option<AtomicHintView>,
    /// sb F3: the readability meter under one draft.
    pub readability: Option<super::panel::readability_meter::ReadabilityMeterView>,
}

/// sb E1: the atomic-note hint of draft `draft`, with its split and dismiss actions.
#[derive(Clone)]
pub struct AtomicHintView {
    pub draft: usize,
    pub report: crate::domain::thinking::atomic::AtomicReport,
    pub on_split: ModalActionCallback,
    pub on_dismiss: ModalActionCallback,
}
