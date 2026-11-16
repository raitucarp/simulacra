//! Threads-style composer modal dialog organism.
//! 100% tokenized via ThemeTokens adhering strictly to AGENTS.md.

mod attachments;
mod editor;
mod footer;
mod header;
mod quote_preview;
mod render;
mod reply_context;
mod state;

use crate::domain::{Post, Profile};
use crate::theme::ThemeTokens;
use gpui_kit::component::input::{InputState, TextareaState};
use gpui_kit::*;
use std::rc::Rc;

pub type ModalCloseCallback = Rc<dyn Fn(&mut Window, &mut App) + 'static>;
pub type ModalSubmitCallback = Rc<dyn Fn(&mut Window, &mut App) + 'static>;
pub type ModalActionCallback = Rc<dyn Fn(&mut Window, &mut App) + 'static>;

#[derive(Debug, Clone, PartialEq)]
pub enum ComposerModalMode {
    NewThread,
    Reply(Box<Post>),
    Quote(Box<Post>),
}

#[derive(IntoElement)]
pub struct ComposerModalView {
    theme: ThemeTokens,
    profile: Option<Profile>,
    mode: ComposerModalMode,
    composer_input: Entity<TextareaState>,
    link_input: Entity<InputState>,
    image_input: Entity<InputState>,
    image_path: Option<String>,
    is_link_input_open: bool,
    is_image_input_open: bool,
    draft_length: usize,
    on_close: ModalCloseCallback,
    on_submit: ModalSubmitCallback,
    on_toggle_link: ModalActionCallback,
    on_toggle_image: ModalActionCallback,
    on_pick_image: ModalActionCallback,
    on_clear_image: ModalActionCallback,
    on_clear_link: ModalActionCallback,
}

impl ComposerModalView {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        theme: ThemeTokens,
        profile: Option<Profile>,
        mode: ComposerModalMode,
        composer_input: Entity<TextareaState>,
        link_input: Entity<InputState>,
        image_input: Entity<InputState>,
        image_path: Option<String>,
        is_link_input_open: bool,
        is_image_input_open: bool,
        draft_length: usize,
        on_close: impl Fn(&mut Window, &mut App) + 'static,
        on_submit: impl Fn(&mut Window, &mut App) + 'static,
        on_toggle_link: impl Fn(&mut Window, &mut App) + 'static,
        on_toggle_image: impl Fn(&mut Window, &mut App) + 'static,
        on_pick_image: impl Fn(&mut Window, &mut App) + 'static,
        on_clear_image: impl Fn(&mut Window, &mut App) + 'static,
        on_clear_link: impl Fn(&mut Window, &mut App) + 'static,
    ) -> Self {
        Self {
            theme,
            profile,
            mode,
            composer_input,
            link_input,
            image_input,
            image_path,
            is_link_input_open,
            is_image_input_open,
            draft_length,
            on_close: Rc::new(on_close),
            on_submit: Rc::new(on_submit),
            on_toggle_link: Rc::new(on_toggle_link),
            on_toggle_image: Rc::new(on_toggle_image),
            on_pick_image: Rc::new(on_pick_image),
            on_clear_image: Rc::new(on_clear_image),
            on_clear_link: Rc::new(on_clear_link),
        }
    }
}
