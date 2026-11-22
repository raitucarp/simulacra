//! The composer's to-do helpers: whether the syntax help card is open (D1), and completing
//! the to-do token at the cursor of the active draft (D11) from what the host hands in.

use super::super::todo_suggest::TodoSuggestView;
use super::ComposerModalPanel;
use crate::domain::todo::complete::{complete, suggestions, typing_at, CompletionSources};
use gpui_kit::*;
use std::rc::Rc;

/// The to-do helpers' state.
#[derive(Debug, Clone, Default)]
pub(super) struct TodoAssist {
    /// The syntax help card is shown.
    pub help: bool,
    /// Contexts in use and agents' handles, for completion.
    pub sources: CompletionSources,
}

impl ComposerModalPanel {
    /// Opens or closes the to-do syntax help card.
    pub(super) fn toggle_todo_help(&mut self, _: &mut Window, cx: &mut Context<Self>) {
        self.todo.help = !self.todo.help;
        cx.notify();
    }

    /// Whether the syntax help card is open.
    pub(super) fn todo_help_open(&self) -> bool {
        self.todo.help
    }

    /// What completion may offer besides the fixed phrases (D11).
    pub fn set_completion_sources(&mut self, sources: CompletionSources) {
        self.todo.sources = sources;
    }

    /// The suggestions for the word at the cursor of the active draft; `None` when there are none.
    pub(super) fn todo_suggest_view(&self, cx: &mut Context<Self>) -> Option<TodoSuggestView> {
        let input = self.drafts.get(self.active_draft)?.input.read(cx);
        let typing = typing_at(input.value().as_ref(), input.cursor())?;
        let today = crate::time::local_now().date();
        let items = suggestions(&typing, &self.todo.sources, today);
        let entity = cx.entity();
        (!items.is_empty()).then(|| TodoSuggestView { items, today, on_pick: Rc::new(move |token, window, cx| entity.update(cx, |this, cx| this.pick_completion(&token, window, cx))) })
    }

    /// Puts `token` in place of the word being typed in the active draft.
    fn pick_completion(&mut self, token: &str, window: &mut Window, cx: &mut Context<Self>) {
        let Some(input) = self.drafts.get(self.active_draft).map(|slot| slot.input.clone()) else { return };
        input.update(cx, |state, cx| {
            let text = state.value().to_string();
            let Some(typing) = typing_at(&text, state.cursor()) else { return };
            let (next, cursor) = complete(&text, &typing, token);
            state.set_value(next, window, cx);
            state.set_selected_range(cursor..cursor, cx);
            state.focus(window, cx);
        });
        cx.notify();
    }
}
