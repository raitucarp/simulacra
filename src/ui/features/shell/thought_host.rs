//! N16–N20 in the shell's data host: the open sheets of the lens fan (N16) and the glossary
//! (N17), the open questions page (N18), and their text fields.

use super::view::AppView;
use crate::ui::features::data::{GlossaryDialog, LensDialog};
use gpui_kit::component::input::InputState;
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
}

impl ThoughtSheets {
    /// Whether one of them is open.
    pub fn any_open(&self) -> bool {
        self.lens.is_some() || self.glossary.is_some() || self.questions_open
    }
}

/// The text fields of the N16–N20 sheets.
pub struct ThoughtInputs {
    /// N17: the term and its definition.
    pub glossary_term: Entity<InputState>,
    pub glossary_text: Entity<InputState>,
}

fn field(placeholder: &str, window: &mut Window, cx: &mut Context<AppView>) -> Entity<InputState> {
    let placeholder = t!(placeholder).to_string();
    cx.new(|cx| InputState::new(window, cx).placeholder(placeholder))
}

impl ThoughtInputs {
    pub(super) fn new(window: &mut Window, cx: &mut Context<AppView>) -> Self {
        Self { glossary_term: field("posts.glossary.term_placeholder", window, cx), glossary_text: field("posts.glossary.definition_placeholder", window, cx) }
    }

    /// Every field.
    pub fn all(&self) -> [&Entity<InputState>; 2] {
        [&self.glossary_term, &self.glossary_text]
    }
}
