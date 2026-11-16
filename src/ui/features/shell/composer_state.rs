//! State of the Threads-style composer modal: mode, draft inputs and attachments.

use super::consts::{MODAL_COMPOSER_ROWS, MODAL_LINK_PLACEHOLDER};
use super::view::AppView;
use crate::ui::features::composer::ComposerModalMode;
use gpui_kit::component::input::{InputEvent, InputState, TextareaState};
use gpui_kit::*;
use rust_i18n::t;

/// Composer modal visibility, mode, inputs and attachment toggles.
pub struct ComposerModalState {
    pub is_open: bool,
    pub mode: ComposerModalMode,
    pub composer_input: Entity<TextareaState>,
    pub link_input: Entity<InputState>,
    pub image_input: Entity<InputState>,
    pub image_path: Option<String>,
    pub is_link_input_open: bool,
    pub is_image_input_open: bool,
}

impl ComposerModalState {
    pub(super) fn new(window: &mut Window, cx: &mut Context<AppView>) -> Self {
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
        Self {
            is_open: false,
            mode: ComposerModalMode::NewThread,
            composer_input,
            link_input,
            image_input,
            image_path: None,
            is_link_input_open: false,
            is_image_input_open: false,
        }
    }

    /// Re-renders on every draft change and submits on secondary (Ctrl/Cmd) Enter.
    pub(super) fn subscribe(&self, window: &mut Window, cx: &mut Context<AppView>) -> Vec<Subscription> {
        let composer = cx.subscribe_in(
            &self.composer_input,
            window,
            |this, _, event: &InputEvent, window, cx| match event {
                InputEvent::Change => cx.notify(),
                InputEvent::PressEnter { secondary: true, .. } => this.submit_composer_modal(window, cx),
                _ => {}
            },
        );
        let link = cx.subscribe(&self.link_input, notify_on_change);
        let image = cx.subscribe(&self.image_input, notify_on_change);
        vec![composer, link, image]
    }

    /// Clears the draft inputs and attachment state (the modal stays in its current mode).
    pub(super) fn clear(&mut self, window: &mut Window, cx: &mut App) {
        self.is_link_input_open = false;
        self.is_image_input_open = false;
        self.image_path = None;
        for input in [&self.link_input, &self.image_input] {
            input.update(cx, |state, cx| state.set_value("", window, cx));
        }
        self.composer_input.update(cx, |state, cx| state.set_value("", window, cx));
    }
}

fn notify_on_change(
    _: &mut AppView,
    _: Entity<InputState>,
    event: &InputEvent,
    cx: &mut Context<AppView>,
) {
    if matches!(event, InputEvent::Change) {
        cx.notify();
    }
}
