//! Threads-style composer modal dialog organism: one editor per post of the thread.
//! 100% tokenized via ThemeTokens adhering strictly to AGENTS.md.

mod attachments;
mod draft_row;
mod editor;
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
}

/// One post editor of the thread with its current (trimmed) character count.
#[derive(Clone)]
pub struct DraftEditor {
    pub input: Entity<TextareaState>,
    pub length: usize,
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
    pub on_remove_draft: DraftActionCallback,
    pub on_split_draft: DraftActionCallback,
    /// Removes the attached image at an index (sb C3).
    pub on_remove_image: DraftActionCallback,
    pub on_paste: PasteCallback,
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
    pub is_link_input_open: bool,
    pub is_image_input_open: bool,
    /// Publish error or the reason the thread cannot be posted yet.
    pub notice: Option<String>,
    pub can_post: bool,
    pub callbacks: ModalCallbacks,
}
