//! N9: "Answers" under a question in the inspector: the asked agents' replies, best first,
//! with Accept on the user's own question. A locked question lists none.

use super::inspector::InspectorContext;
use crate::domain::views::rapid_log::one_line;
use crate::domain::Post;
use crate::state::AppState;
use crate::ui::features::data::{render_answers, AnswerRow, OpenAnswer};
use gpui_kit::Div;
use std::rc::Rc;

/// The rows of `question`'s answers, best first (none for a locked question).
pub(crate) fn answer_rows(state: &AppState, question: &Post) -> Vec<AnswerRow> {
    let accepted = question.extras.question.as_ref().and_then(|q| q.accepted.clone());
    let rows = state.ranked_answers(&question.id).into_iter().filter_map(|answer| {
        let reply = state.get_post(&answer.post)?;
        let author = state.author_of(reply).name.clone();
        let excerpt = if reply.is_locked() { rust_i18n::t!("posts.lock.locked").to_string() } else { one_line(reply) };
        Some(AnswerRow { accepted: accepted.as_ref() == Some(&answer.post), excerpt, post: answer.post, author })
    });
    rows.collect()
}

/// The section of the focused `post`; `None` unless it is a question with answers.
pub(super) fn render_question_answers(ctx: &InspectorContext<'_>, post: &Post) -> Option<Div> {
    let source = ctx.callbacks.data.as_ref()?;
    post.extras.question.as_ref()?;
    let rows = answer_rows(ctx.state, post);
    let own = ctx.state.author_of(post).is_human_persona();
    let select = ctx.callbacks.on_select.clone();
    let on_open: OpenAnswer = Rc::new(move |id, window, cx| select(id, window, cx));
    render_answers(&post.id, &rows, own, &source.on_event, &on_open, ctx.theme)
}
