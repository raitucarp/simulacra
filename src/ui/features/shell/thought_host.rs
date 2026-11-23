//! N16–N20 in the shell's data host: the open sheets of the lens fan (N16) and the glossary
//! (N17), the open questions page (N18), the change of mind sheet (N19), and their text fields.

use super::view::AppView;
use crate::ui::features::data::{GlossaryDialog, LensDialog, MindDialog, WordsDialog};
use gpui_kit::component::input::{InputEvent, InputState};
use gpui_kit::*;
use rust_i18n::t;

/// The open N16–N20 sheets and what is under way.
#[derive(Default)]
pub struct ThoughtSheets {
    /// N16: picking the lenses of a fan.
    pub lens: Option<LensDialog>,
    /// N17: defining a term.
    pub glossary: Option<GlossaryDialog>,
    /// N18: the page of every open question is open.
    pub questions_open: bool,
    /// N19: marking a change of mind.
    pub mind: Option<MindDialog>,
    /// N20: the posts shown translated, the one being translated (and the ticket of that
    /// request), and the words sheet.
    pub translated: Vec<crate::domain::PostId>,
    pub translating: Option<crate::domain::PostId>,
    pub translate_ticket: u64,
    pub words: Option<WordsDialog>,
}

impl ThoughtSheets {
    /// Whether one of them is open.
    pub fn any_open(&self) -> bool {
        self.lens.is_some() || self.glossary.is_some() || self.questions_open || self.mind.is_some() || self.words.is_some()
    }
}

/// The text fields of the N16–N20 sheets.
pub struct ThoughtInputs {
    /// N17: the term and its definition.
    pub glossary_term: Entity<InputState>,
    pub glossary_text: Entity<InputState>,
    /// N19: what the user used to think, and the search for the post it revises.
    pub mind_was: Entity<InputState>,
    pub mind_find: Entity<InputState>,
}

fn field(placeholder: &str, window: &mut Window, cx: &mut Context<AppView>) -> Entity<InputState> {
    let placeholder = t!(placeholder).to_string();
    cx.new(|cx| InputState::new(window, cx).placeholder(placeholder))
}

impl ThoughtInputs {
    pub(super) fn new(window: &mut Window, cx: &mut Context<AppView>) -> Self {
        Self {
            glossary_term: field("posts.glossary.term_placeholder", window, cx),
            glossary_text: field("posts.glossary.definition_placeholder", window, cx),
            mind_was: field("posts.mind.was_placeholder", window, cx),
            mind_find: field("posts.mind.find_placeholder", window, cx),
        }
    }

    /// Every field.
    pub fn all(&self) -> [&Entity<InputState>; 4] {
        [&self.glossary_term, &self.glossary_text, &self.mind_was, &self.mind_find]
    }

    /// N19: the search follows what is typed.
    pub(super) fn subscribe(&self, window: &mut Window, cx: &mut Context<AppView>) -> Vec<Subscription> {
        let search = cx.subscribe_in(&self.mind_find, window, |this: &mut AppView, _, event: &InputEvent, _, cx| {
            if matches!(event, InputEvent::Change) {
                this.refresh_mind_candidates(cx);
                cx.notify();
            }
        });
        vec![search]
    }
}
