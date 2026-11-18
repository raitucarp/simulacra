//! What the shell keeps for the image ideas sheet (sb D3): the open sheet, its prompt editor,
//! the running language-model request and the generation counter that drops stale results.

use super::view::AppView;
use crate::agent::post_actions::CancelToken;
use crate::ui::features::image_prompt::IdeasSheet;
use gpui_kit::component::input::TextareaState;
use gpui_kit::*;

/// Rows of the prompt editor (shortest, tallest).
const PROMPT_ROWS: (usize, usize) = (4, 8);

/// Image ideas state of the window.
pub struct ImageIdeasUi {
    pub sheet: Option<IdeasSheet>,
    /// The editable prompt (rebuilt when a pick changes).
    pub prompt: Entity<TextareaState>,
    pub focus: FocusHandle,
    /// The running "Write it for me" request.
    pub(super) cancel: Option<CancelToken>,
    /// Bumped on every request, cancel and close; a finished request only lands if its
    /// number is still current (one counter for ideas, one for drawings).
    pub(super) writing: u64,
    pub(super) drawing: u64,
}

impl ImageIdeasUi {
    pub(super) fn new(window: &mut Window, cx: &mut Context<AppView>) -> Self {
        let prompt = cx.new(|cx| TextareaState::new(window, cx).auto_grow(PROMPT_ROWS.0, PROMPT_ROWS.1));
        Self { sheet: None, prompt, focus: cx.focus_handle(), cancel: None, writing: 0, drawing: 0 }
    }

    /// Stops the running request and forgets pending results.
    pub(super) fn cancel_running(&mut self) {
        if let Some(cancel) = self.cancel.take() {
            cancel.cancel();
        }
        self.writing += 1;
        self.drawing += 1;
    }
}
