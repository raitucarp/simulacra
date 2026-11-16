//! Stateful composer modal entity.
//!
//! Owns the thread being composed (one text area per post), attachment toggles and the mode,
//! re-rendering itself on keystrokes. The host only opens it and handles
//! [`ComposerModalEvent::Submit`], reporting store rejections back through [`ComposerModalPanel::set_error`].

mod actions;
mod drafts;
mod render;

use super::consts::MODAL_LINK_PLACEHOLDER;
use super::modal::ComposerModalMode;
use crate::domain::Profile;
use crate::theme::ThemeTokens;
use gpui_kit::component::input::{InputState, TextareaState};
use gpui_kit::*;
use rust_i18n::t;

/// A thread the user asked to publish: one text per post, attachments on the first post.
#[derive(Debug, Clone, PartialEq)]
pub struct ComposerSubmission {
    pub mode: ComposerModalMode,
    pub drafts: Vec<String>,
    pub link: Option<String>,
    pub image: Option<String>,
}

/// Events emitted by [`ComposerModalPanel`].
#[derive(Debug, Clone, PartialEq)]
pub enum ComposerModalEvent {
    Submit(ComposerSubmission),
}

/// One post editor of the thread, with the subscription that keeps the panel in sync.
struct DraftSlot {
    input: Entity<TextareaState>,
    _subscription: Subscription,
}

/// Composer modal visibility, mode, thread drafts and attachments.
pub struct ComposerModalPanel {
    is_open: bool,
    mode: ComposerModalMode,
    drafts: Vec<DraftSlot>,
    link_input: Entity<InputState>,
    image_input: Entity<InputState>,
    image_path: Option<String>,
    is_link_input_open: bool,
    is_image_input_open: bool,
    /// Last publish failure reported by the host, cleared on the next edit.
    error: Option<String>,
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
        let link_input = cx.new(|cx| InputState::new(window, cx).placeholder(MODAL_LINK_PLACEHOLDER));
        let image_placeholder = t!("composer.image_path_placeholder").to_string();
        let image_input = cx.new(|cx| InputState::new(window, cx).placeholder(image_placeholder));
        let mut panel = Self {
            is_open: false,
            mode: ComposerModalMode::NewThread,
            drafts: Vec::new(),
            link_input,
            image_input,
            image_path: None,
            is_link_input_open: false,
            is_image_input_open: false,
            error: None,
            theme,
            profile,
            _subscriptions: Vec::new(),
        };
        panel._subscriptions = panel.subscribe(cx);
        panel.drafts.push(panel.new_draft_slot(window, cx));
        panel
    }

    pub fn is_open(&self) -> bool {
        self.is_open
    }

    /// Opens the modal in `mode` with a single empty, focused draft.
    pub fn open(&mut self, mode: ComposerModalMode, window: &mut Window, cx: &mut Context<Self>) {
        self.mode = mode;
        self.is_open = true;
        self.clear(window, cx);
        self.focus_draft(0, window, cx);
        cx.notify();
    }

    /// Closes the modal and clears its drafts.
    pub fn close(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.is_open = false;
        self.clear(window, cx);
        cx.notify();
    }

    /// Shows why the host could not publish the submitted thread.
    pub fn set_error(&mut self, message: String, cx: &mut Context<Self>) {
        self.error = Some(message);
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
