//! N7: the "Quote a passage" sheet of a vault document: its passages (sentences and lines),
//! narrowed by a search, one picked, and a post commenting on it that pins the passage
//! verbatim. Only the first passages that match are listed, so a long book stays quick.

use super::events::{DataAction, DataEvent};
use super::sheet::{footer, heading, hint, notice, sheet_frame, SheetParts};
use super::text::tr;
use crate::domain::PostId;
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::base::Disableable;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::input::{Input, InputState};
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use std::ops::Range;
use std::rc::Rc;

const SHEET_ID: &str = "data-quote";
const PASSAGE_ID: &str = "data-quote-passage-";
const SEND_ID: &str = "data-quote-send";
const CANCEL_ID: &str = "data-quote-cancel";
/// How many matching passages the sheet lists.
pub const MAX_LISTED_PASSAGES: usize = 60;

/// One interaction with the quote sheet, or with a pinned quote.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QuoteEvent {
    /// Opens the sheet on this vault document.
    Open(String),
    Pick(usize),
    Submit,
    Close,
    /// Re-pins this post's quote to its document as it is now.
    Repin(PostId),
}

/// The quote sheet's state.
#[derive(Clone)]
pub struct QuoteDialog {
    pub doc_id: String,
    pub title: String,
    /// Each passage: its characters in the document and its words.
    pub passages: Vec<(Range<usize>, String)>,
    /// What the search holds (lower case).
    pub query: String,
    pub picked: Option<usize>,
    pub find: Entity<InputState>,
    pub comment: Entity<InputState>,
    pub problem: Option<String>,
}

impl QuoteDialog {
    /// The passages the search keeps, at most [`MAX_LISTED_PASSAGES`], with their indices.
    pub fn listed(&self) -> Vec<(usize, &str)> {
        listed_passages(&self.passages, &self.query)
    }

    /// The characters of the picked passage.
    pub fn picked_range(&self) -> Option<Range<usize>> {
        self.picked.and_then(|at| self.passages.get(at)).map(|(range, _)| range.clone())
    }
}

/// The passages holding `query` (lower case; empty keeps all), at most [`MAX_LISTED_PASSAGES`].
pub fn listed_passages<'a>(passages: &'a [(Range<usize>, String)], query: &str) -> Vec<(usize, &'a str)> {
    let keeps = |text: &str| query.is_empty() || text.to_lowercase().contains(query);
    passages.iter().enumerate().filter(|(_, (_, text))| keeps(text)).take(MAX_LISTED_PASSAGES).map(|(at, (_, text))| (at, text.as_str())).collect()
}

fn send(on_event: &DataAction, event: QuoteEvent) -> impl Fn(&ClickEvent, &mut Window, &mut App) + 'static {
    let on_event = on_event.clone();
    move |_, window, cx| on_event(DataEvent::Quote(event.clone()), window, cx)
}

fn passage_row(at: usize, text: &str, picked: bool, on_event: &DataAction, theme: &ThemeTokens) -> Stateful<Div> {
    let (edge, hover) = (if picked { theme.accent_dialectic } else { theme.border_subtle }, theme.hover_bg);
    div()
        .id(SharedString::from(format!("{PASSAGE_ID}{at}")))
        .w_full()
        .px(theme.spacing_sm)
        .py(theme.spacing_xs)
        .rounded(theme.radius_sm)
        .border_l(theme.border_width_threadline)
        .border_color(edge)
        .when(picked, |row| row.bg(theme.bg_callout))
        .hover(move |row| row.bg(hover))
        .cursor_pointer()
        .text_size(theme.font_size_sm)
        .text_color(if picked { theme.text_primary } else { theme.text_secondary })
        .on_click(send(on_event, QuoteEvent::Pick(at)))
        .child(text.to_string())
}

/// The overlay.
pub fn render_quote_sheet(dialog: &QuoteDialog, on_event: DataAction, theme: &ThemeTokens) -> Stateful<Div> {
    let rows = dialog.listed().into_iter().map(|(at, text)| passage_row(at, text, dialog.picked == Some(at), &on_event, theme));
    let list = div().flex().flex_col().gap(theme.spacing_3xs).children(rows);
    let problem = dialog.problem.clone().map(|text| notice(LucideIcon::CircleAlert, text, theme.accent_danger, theme));
    let comment = div().flex().flex_col().gap(theme.spacing_xs).child(heading(tr("posts.pinned.comment_label"), theme)).child(Input::new(&dialog.comment));
    let body = div().flex().flex_col().gap(theme.stack_gap).child(hint(tr("posts.pinned.explain"), theme)).child(Input::new(&dialog.find)).child(heading(tr("posts.pinned.passages"), theme)).child(list).child(comment).children(problem);
    let buttons = footer(theme)
        .child(Button::new(CANCEL_ID).ghost().small().label(tr("data.cancel")).on_click(send(&on_event, QuoteEvent::Close)))
        .child(Button::new(SEND_ID).primary().small().icon(Icon::new(LucideIcon::Quote)).label(tr("posts.pinned.send")).disabled(dialog.picked.is_none()).on_click(send(&on_event, QuoteEvent::Submit)));
    let closing = on_event.clone();
    let parts = SheetParts {
        id: SHEET_ID,
        icon: LucideIcon::Quote,
        title: tr("posts.pinned.title"),
        subtitle: Some(dialog.title.clone()),
        body,
        footer: buttons,
        on_close: Rc::new(move |window, cx| closing(DataEvent::Quote(QuoteEvent::Close), window, cx)),
    };
    sheet_frame(parts, theme)
}
