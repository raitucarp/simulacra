//! What the shell keeps for post actions (sb C2): the open sheet, its typed-choice field,
//! the running request (its cancel flag and generation) and this session's answers.

use super::view::AppView;
use crate::agent::post_actions::CancelToken;
use crate::domain::post_actions::ActionOutput;
use crate::domain::PostId;
use crate::ui::features::post_actions::ActionSheet;
use gpui_kit::component::input::{InputEvent, InputState};
use gpui_kit::*;
use rust_i18n::t;
use std::collections::HashMap;

/// Answers remembered for this session, by post and request key.
pub type AnswerCache = HashMap<(PostId, String), ActionOutput>;

/// Post actions state of the window.
pub struct PostActionsUi {
    pub sheet: Option<ActionSheet>,
    /// The typed audience, language or concept.
    pub input: Entity<InputState>,
    pub focus: FocusHandle,
    /// The running request's cancel flag.
    pub(super) cancel: Option<CancelToken>,
    /// Bumped on every run; a finished request only lands if it is still the latest.
    pub(super) generation: u64,
    pub(super) answers: AnswerCache,
}

impl PostActionsUi {
    pub(super) fn new(window: &mut Window, cx: &mut Context<AppView>) -> (Self, Subscription) {
        let placeholder = t!("post_actions.typed_placeholder").to_string();
        let input = cx.new(|cx| InputState::new(window, cx).placeholder(placeholder));
        let subscription = cx.subscribe_in(&input, window, |view, _, event: &InputEvent, window, cx| {
            if matches!(event, InputEvent::PressEnter { .. }) {
                view.on_action_sheet_event(crate::ui::features::post_actions::SheetEvent::ApplyTyped, window, cx);
            }
        });
        let ui = Self { sheet: None, input, focus: cx.focus_handle(), cancel: None, generation: 0, answers: HashMap::new() };
        (ui, subscription)
    }

    /// Stops the running request, if any.
    pub(super) fn cancel_running(&mut self) {
        if let Some(cancel) = self.cancel.take() {
            cancel.cancel();
        }
        self.generation += 1;
    }
}
