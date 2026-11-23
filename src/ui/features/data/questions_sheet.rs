//! N18: open questions from a post's menu (make it a question, mark it answered, reopen it,
//! mark a reply as the answer) and the page of every question still open in the network.

use super::events::{DataAction, DataEvent};
use super::sheet::{footer, hint, sheet_frame, SheetParts};
use super::text::tr;
use super::thought_events::ThoughtEvent;
use crate::domain::PostId;
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::Sizable;
use gpui_kit::*;
use std::rc::Rc;

const SHEET_ID: &str = "data-open-questions";

/// One interaction with open questions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QuestionsEvent {
    /// Makes the post a question (`true`) or an ordinary thought again.
    SetQuestion(PostId, bool),
    /// Marks the question `.0` answered, by the post `.1` when one answers it.
    Resolve(PostId, Option<PostId>),
    Reopen(PostId),
    /// The page of every open question.
    ShowAll,
    /// Opens one of them.
    Open(PostId),
    Close,
}

fn send(on_event: &DataAction, event: QuestionsEvent) -> impl Fn(&ClickEvent, &mut Window, &mut App) + 'static {
    let on_event = on_event.clone();
    move |_, window, cx| on_event(DataEvent::Thought(ThoughtEvent::Questions(event.clone())), window, cx)
}

/// The page over the list of open questions (`list`: their view, or why it is empty).
pub fn render_questions_sheet(list: AnyElement, count: usize, on_event: DataAction, theme: &ThemeTokens) -> Stateful<Div> {
    let body = div().flex().flex_col().gap(theme.stack_gap).child(hint(tr("posts.open_question.page_explain"), theme)).child(list);
    let buttons = footer(theme).child(Button::new("data-open-questions-close").ghost().small().label(tr("data.close")).on_click(send(&on_event, QuestionsEvent::Close)));
    let closing = on_event.clone();
    let parts = SheetParts {
        id: SHEET_ID,
        icon: LucideIcon::HelpCircle,
        title: tr("posts.open_question.page_title"),
        subtitle: Some(crate::i18n::t_count("posts.open_question.page_count", count)),
        body,
        footer: buttons,
        on_close: Rc::new(move |window, cx| closing(DataEvent::Thought(ThoughtEvent::Questions(QuestionsEvent::Close)), window, cx)),
    };
    sheet_frame(parts, theme)
}
