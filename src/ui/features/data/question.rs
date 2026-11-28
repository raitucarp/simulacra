//! N9: a question's answers under it in the inspector, best first, each with its agent and
//! opening words; the user accepts one (or takes it back) on their own question. A click on
//! an answer opens it.

use super::events::{DataAction, DataEvent};
use super::text::tr;
use crate::domain::PostId;
use crate::i18n::t_count;
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::*;
use std::rc::Rc;

const ACCEPT_ID: &str = "question-accept-";
const ANSWER_ID: &str = "question-answer-";
/// An answer shows enough of its opening to judge it before opening it.
const EXCERPT_LINES: usize = 2;

/// One interaction with a question.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QuestionEvent {
    /// Asks this post of the room.
    Ask(PostId),
    /// Accepts (or takes back) the answer `.1` to the question `.0`.
    Accept(PostId, PostId),
}

/// One answer as listed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnswerRow {
    pub post: PostId,
    pub author: String,
    pub excerpt: String,
    pub accepted: bool,
}

/// Opens a post (the inspector's selection).
pub type OpenAnswer = Rc<dyn Fn(PostId, &mut Window, &mut App) + 'static>;

/// The answers section; `None` while there are no answers.
pub fn render_answers(question: &PostId, rows: &[AnswerRow], own: bool, on_event: &DataAction, on_open: &OpenAnswer, theme: &ThemeTokens) -> Option<Div> {
    if rows.is_empty() {
        return None;
    }
    let heading = div().text_size(theme.font_size_xs).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_primary).child(t_count("posts.question.answers", rows.len()));
    let list = rows.iter().map(|row| render_row(question, row, own, on_event, on_open, theme));
    Some(div().mx(theme.panel_padding).my(theme.spacing_sm).flex().flex_col().gap(theme.spacing_xs).child(heading).children(list))
}

fn render_row(question: &PostId, row: &AnswerRow, own: bool, on_event: &DataAction, on_open: &OpenAnswer, theme: &ThemeTokens) -> Stateful<Div> {
    let author = div().flex_1().min_w_0().truncate().font_weight(FontWeight::MEDIUM).text_color(theme.text_primary).child(row.author.clone());
    let text = div().min_w_0().line_clamp(EXCERPT_LINES).text_ellipsis().text_color(theme.text_secondary).child(row.excerpt.clone());
    let badge = row.accepted.then(|| div().flex().items_center().gap(theme.inline_gap).text_color(theme.accent_success).child(Icon::new(LucideIcon::Check).xsmall().text_color(theme.accent_success)).child(tr("posts.question.accepted")));
    let accept = (own && !row.accepted).then(|| {
        let (on_event, event) = (on_event.clone(), DataEvent::Question(QuestionEvent::Accept(question.clone(), row.post.clone())));
        // Accepting is not opening: the click stays on the button.
        Button::new(SharedString::from(format!("{ACCEPT_ID}{}", row.post))).ghost().xsmall().label(tr("posts.question.accept")).on_click(move |_, window, cx| {
            cx.stop_propagation();
            on_event(event.clone(), window, cx)
        })
    });
    let head = div().flex().items_center().gap(theme.inline_gap).child(author).children(badge).children(accept);
    let (on_open, id, hover) = (on_open.clone(), row.post.clone(), theme.hover_bg);
    div()
        .id(SharedString::from(format!("{ANSWER_ID}{}", row.post)))
        .p(theme.spacing_xs)
        .rounded(theme.radius_sm)
        .bg(theme.bg_callout)
        .cursor_pointer()
        .hover(move |r| r.bg(hover))
        .on_click(move |_, window, cx| on_open(id.clone(), window, cx))
        .flex()
        .flex_col()
        .gap(theme.spacing_3xs)
        .text_size(theme.font_size_xs)
        .child(head)
        .child(text)
}
