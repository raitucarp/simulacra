//! Threads-style inline composer molecule.
//! Fully tokenized via ThemeTokens.
//! Features isolated View rendering for 120 FPS sub-millisecond typing latency.

mod fallback;
mod layout;
mod render;

use super::keys::{SubmitPostDraft, INPUT_KEY_CONTEXT, SUBMIT_BINDING_CMD, SUBMIT_BINDING_CTRL};
use crate::domain::Profile;
use crate::theme::ThemeTokens;
use gpui_kit::component::input::{InputEvent, TextareaState};
use gpui_kit::*;

pub use fallback::render_post_composer;

pub type SubmitPostCallback = Box<dyn Fn(String, &mut Window, &mut App) + 'static>;

/// Isolated GPUI View component for the composer.
/// Renders independently without triggering full-window `AppView` re-renders on keystrokes.
pub struct PostComposerView {
    profile: Option<Profile>,
    composer_input: Entity<TextareaState>,
    theme: ThemeTokens,
    draft_length: usize,
    on_submit: SubmitPostCallback,
    _sub: Subscription,
}

impl PostComposerView {
    pub fn new(
        profile: Option<Profile>,
        composer_input: Entity<TextareaState>,
        theme: ThemeTokens,
        on_submit: impl Fn(String, &mut Window, &mut App) + 'static,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let initial_len = composer_input.read(cx).value().chars().count();
        cx.bind_keys([
            KeyBinding::new(SUBMIT_BINDING_CTRL, SubmitPostDraft, Some(INPUT_KEY_CONTEXT)),
            KeyBinding::new(SUBMIT_BINDING_CMD, SubmitPostDraft, Some(INPUT_KEY_CONTEXT)),
        ]);
        let sub = cx.subscribe_in(&composer_input, window, Self::on_input_event);
        Self {
            profile,
            composer_input,
            theme,
            draft_length: initial_len,
            on_submit: Box::new(on_submit),
            _sub: sub,
        }
    }

    /// Tracks the draft length and submits on secondary (Ctrl/Cmd) Enter.
    fn on_input_event(
        &mut self,
        entity: &Entity<TextareaState>,
        event: &InputEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match event {
            InputEvent::Change => {
                let new_len = entity.read(cx).value().chars().count();
                if self.draft_length != new_len {
                    self.draft_length = new_len;
                    cx.notify();
                }
            }
            InputEvent::PressEnter { secondary: true, .. } => self.submit(window, cx),
            _ => {}
        }
    }

    /// Submits the current draft content.
    pub fn submit(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let content = self.composer_input.read(cx).value().to_string();
        if !content.trim().is_empty() {
            (self.on_submit)(content, window, cx);
        }
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

    pub fn reset(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.composer_input.update(cx, |state, cx| {
            state.set_value("", window, cx);
            state.focus(window, cx);
        });
        self.draft_length = 0;
        cx.notify();
    }
}
