//! Stateful composer modal entity.
//!
//! Owns the modal's draft inputs, attachment toggles and mode, re-rendering itself on
//! keystrokes. The host only opens it and handles [`ComposerModalEvent::Submit`], since
//! publishing needs the application state.

mod actions;
mod render;

use super::consts::{MODAL_COMPOSER_ROWS, MODAL_LINK_PLACEHOLDER};
use super::modal::ComposerModalMode;
use crate::domain::Profile;
use crate::theme::ThemeTokens;
use gpui_kit::component::input::{InputState, TextareaState};
use gpui_kit::*;
use rust_i18n::t;

/// A draft the user asked to publish.
#[derive(Debug, Clone, PartialEq)]
pub struct ComposerSubmission {
    pub mode: ComposerModalMode,
    pub content: String,
    pub link: Option<String>,
    pub image: Option<String>,
}

/// Events emitted by [`ComposerModalPanel`].
#[derive(Debug, Clone, PartialEq)]
pub enum ComposerModalEvent {
    Submit(ComposerSubmission),
}

/// Composer modal visibility, mode, draft inputs and attachments.
pub struct ComposerModalPanel {
    is_open: bool,
    mode: ComposerModalMode,
    composer_input: Entity<TextareaState>,
    link_input: Entity<InputState>,
    image_input: Entity<InputState>,
    image_path: Option<String>,
    is_link_input_open: bool,
    is_image_input_open: bool,
    theme: ThemeTokens,
    profile: Option<Profile>,
    _subscriptions: Vec<Subscription>,
}

impl EventEmitter<ComposerModalEvent> for ComposerModalPanel {}

impl ComposerModalPanel {
    pub fn new(
        theme: ThemeTokens,
        profile: Option<Profile>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let (min_rows, max_rows) = MODAL_COMPOSER_ROWS;
        let composer_placeholder = t!("composer.placeholder").to_string();
        let composer_input = cx.new(|cx| {
            TextareaState::new(window, cx)
                .placeholder(composer_placeholder)
                .auto_grow(min_rows, max_rows)
        });
        let link_input = cx.new(|cx| InputState::new(window, cx).placeholder(MODAL_LINK_PLACEHOLDER));
        let image_placeholder = t!("composer.image_path_placeholder").to_string();
        let image_input = cx.new(|cx| InputState::new(window, cx).placeholder(image_placeholder));
        let mut panel = Self {
            is_open: false,
            mode: ComposerModalMode::NewThread,
            composer_input,
            link_input,
            image_input,
            image_path: None,
            is_link_input_open: false,
            is_image_input_open: false,
            theme,
            profile,
            _subscriptions: Vec::new(),
        };
        panel._subscriptions = panel.subscribe(window, cx);
        panel
    }

    pub fn is_open(&self) -> bool {
        self.is_open
    }

    /// Opens the modal in `mode` with an empty, focused draft.
    pub fn open(&mut self, mode: ComposerModalMode, window: &mut Window, cx: &mut Context<Self>) {
        self.mode = mode;
        self.is_open = true;
        self.clear(window, cx);
        self.composer_input.update(cx, |state, cx| state.focus(window, cx));
        cx.notify();
    }

    /// Closes the modal and clears its draft.
    pub fn close(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.is_open = false;
        self.clear(window, cx);
        cx.notify();
    }

    pub fn set_profile(&mut self, profile: Option<&Profile>, cx: &mut Context<Self>) {
        if self.profile.as_ref() != profile {
            self.profile = profile.cloned();
            cx.notify();
        }
    }

    pub fn set_theme(&mut self, theme: &ThemeTokens, cx: &mut Context<Self>) {
        if &self.theme != theme {
            self.theme = theme.clone();
            cx.notify();
        }
    }
}
