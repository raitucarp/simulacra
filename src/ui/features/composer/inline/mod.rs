//! Threads-style inline composer molecule.
//! Fully tokenized via ThemeTokens.
//! Features isolated View rendering for 120 FPS sub-millisecond typing latency.

mod layout;
mod render;

use super::keys::{SubmitPostDraft, INPUT_KEY_CONTEXT, SUBMIT_BINDING_CMD, SUBMIT_BINDING_CTRL};
use super::messages::thread_problem_message;
use crate::domain::{first_thread_problem, Profile};
use crate::theme::ThemeTokens;
use gpui_kit::component::input::{InputEvent, TextareaState};
use gpui_kit::*;

/// Publishes the draft; an `Err` carries the message shown under the editor.
pub type SubmitPostCallback =
    Box<dyn Fn(String, &mut Window, &mut App) -> Result<(), String> + 'static>;

/// Isolated GPUI View component for the composer.
/// Renders independently without triggering full-window `AppView` re-renders on keystrokes.
pub struct PostComposerView {
    profile: Option<Profile>,
    composer_input: Entity<TextareaState>,
    theme: ThemeTokens,
    /// Trimmed character count of the draft.
    draft_length: usize,
    /// Why the last submission failed, cleared on the next edit.
    error: Option<String>,
    on_submit: SubmitPostCallback,
    _sub: Subscription,
}

impl PostComposerView {
    pub fn new(
        profile: Option<Profile>,
        composer_input: Entity<TextareaState>,
        theme: ThemeTokens,
        on_submit: impl Fn(String, &mut Window, &mut App) -> Result<(), String> + 'static,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let initial_len = trimmed_len(&composer_input.read(cx).value());
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
            error: None,
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
                let new_len = trimmed_len(&entity.read(cx).value());
                if self.draft_length != new_len || self.error.is_some() {
                    self.draft_length = new_len;
                    self.error = None;
                    cx.notify();
                }
            }
            InputEvent::PressEnter { secondary: true, .. } => self.submit(window, cx),
            _ => {}
        }
    }

    /// Submits the current draft: resets on success, otherwise shows why it was rejected.
    pub fn submit(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let content = self.composer_input.read(cx).value().to_string();
        if content.trim().is_empty() {
            return;
        }
        match (self.on_submit)(content, window, cx) {
            Ok(()) => self.reset(window, cx),
            Err(message) => {
                self.error = Some(message);
                cx.notify();
            }
        }
    }

    /// The submit error, or the live reason the draft cannot be posted (e.g. too long).
    fn notice(&self, cx: &App) -> Option<String> {
        let text = self.composer_input.read(cx).value();
        let live = first_thread_problem(&[text], false)
            .filter(|_| self.draft_length > 0)
            .map(|problem| thread_problem_message(&problem));
        self.error.clone().or(live)
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
        self.error = None;
        cx.notify();
    }
}

fn trimmed_len(text: &str) -> usize {
    text.trim().chars().count()
}
