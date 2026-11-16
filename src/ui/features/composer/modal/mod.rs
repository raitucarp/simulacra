//! Threads-style composer modal dialog organism: one editor per post of the thread.
//! 100% tokenized via ThemeTokens adhering strictly to AGENTS.md.

mod attachments;
mod draft_row;
mod editor;
mod footer;
mod header;
mod quote_preview;
mod render;
mod reply_context;
mod state;
mod thread_rows;

use crate::domain::{Post, Profile};
use crate::theme::ThemeTokens;
use gpui_kit::component::input::{InputState, TextareaState};
use gpui_kit::*;
use std::rc::Rc;

pub type ModalCloseCallback = Rc<dyn Fn(&mut Window, &mut App) + 'static>;
pub type ModalSubmitCallback = Rc<dyn Fn(&mut Window, &mut App) + 'static>;
pub type ModalActionCallback = Rc<dyn Fn(&mut Window, &mut App) + 'static>;
/// Callback acting on the draft at the given thread position.
pub type DraftActionCallback = Rc<dyn Fn(usize, &mut Window, &mut App) + 'static>;

#[derive(Debug, Clone, PartialEq)]
pub enum ComposerModalMode {
    NewThread,
    Reply(Box<Post>),
    Quote(Box<Post>),
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
    /// The attached image (picked, or a typed path that exists).
    pub image_path: Option<String>,
    pub is_link_input_open: bool,
    pub is_image_input_open: bool,
    /// Publish error or the reason the thread cannot be posted yet.
    pub notice: Option<String>,
    pub can_post: bool,
    pub callbacks: ModalCallbacks,
}
