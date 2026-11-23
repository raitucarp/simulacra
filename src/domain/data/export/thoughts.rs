//! N16–N20 in exports: the lens a reply was written through, the term a post defines, whether
//! a question is open or answered, a change of mind with the old view, and a translation, as
//! Markdown. Nothing for a locked post, which travels as its seal only.

use crate::domain::post::open_question::{question_state, QuestionState};
use crate::domain::Post;

/// `text` as a Markdown quotation continued over its lines.
fn quoted(text: &str) -> String {
    text.replace('\n', "\n> ")
}

/// The Markdown of `post`'s N16–N20 marks.
pub(super) fn thought_block(post: &Post) -> String {
    if post.is_locked() {
        return String::new();
    }
    let extras = &post.extras;
    let lens = extras.lens.map(|lens| format!("*Through the lens of {}*\n\n", lens.key()));
    let defines = extras.defines.as_ref().map(|definition| format!("> **Defines “{}”:** {}\n\n", definition.term, quoted(&definition.text)));
    let question = question_state(post).map(|state| match state {
        QuestionState::Open => "**Open question**\n\n".to_string(),
        QuestionState::Answered => "**Answered question**\n\n".to_string(),
    });
    let mind = extras.mind_change.as_ref().map(|change| match change.was.as_str() {
        "" => "> **Changed my mind.**\n\n".to_string(),
        was => format!("> **Changed my mind.** Before: {}\n\n", quoted(was)),
    });
    let translation = extras.translation.as_ref().map(|kept| format!("> **Translation ({}):** {}\n\n", kept.language, quoted(&kept.text)));
    [lens, defines, question, mind, translation].into_iter().flatten().collect()
}
