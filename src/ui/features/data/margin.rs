//! N2: the "Reply in the margin" sheet: the post's passages (its sentences and lines), one
//! picked, and a short reply answering exactly that passage. A locked post never opens it.

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

const SHEET_ID: &str = "data-margin";
const PASSAGE_ID: &str = "data-margin-passage-";
const SEND_ID: &str = "data-margin-send";
const CANCEL_ID: &str = "data-margin-cancel";

/// The margin sheet's state.
#[derive(Clone)]
pub struct MarginDialog {
    pub post_id: PostId,
    /// Each passage: its characters in the post and its words.
    pub passages: Vec<(Range<usize>, String)>,
    pub picked: Option<usize>,
    pub reply: Entity<InputState>,
    /// Why the last send failed.
    pub problem: Option<String>,
}

impl MarginDialog {
    pub fn new(post_id: PostId, passages: Vec<(Range<usize>, String)>, reply: Entity<InputState>) -> Self {
        let picked = (passages.len() == 1).then_some(0);
        Self { post_id, passages, picked, reply, problem: None }
    }

    /// The characters of the picked passage.
    pub fn picked_range(&self) -> Option<Range<usize>> {
        self.picked.and_then(|at| self.passages.get(at)).map(|(range, _)| range.clone())
    }
}

fn send(on_event: &DataAction, event: DataEvent) -> impl Fn(&ClickEvent, &mut Window, &mut App) + 'static {
    let on_event = on_event.clone();
    move |_, window, cx| on_event(event.clone(), window, cx)
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
        .on_click(send(on_event, DataEvent::PickPassage(at)))
        .child(text.to_string())
}

/// The overlay.
pub fn render_margin_sheet(dialog: &MarginDialog, on_event: DataAction, theme: &ThemeTokens) -> Stateful<Div> {
    let rows = dialog.passages.iter().enumerate().map(|(at, (_, text))| passage_row(at, text, dialog.picked == Some(at), &on_event, theme));
    let list = div().flex().flex_col().gap(theme.spacing_3xs).children(rows);
    let problem = dialog.problem.clone().map(|text| notice(LucideIcon::CircleAlert, text, theme.accent_danger, theme));
    let field = div().flex().flex_col().gap(theme.spacing_xs).child(heading(tr("post.margin.reply_label"), theme)).child(Input::new(&dialog.reply));
    let body = div().flex().flex_col().gap(theme.stack_gap).child(hint(tr("post.margin.explain"), theme)).child(heading(tr("post.margin.passages"), theme)).child(list).child(field).children(problem);
    let buttons = footer(theme)
        .child(Button::new(CANCEL_ID).ghost().small().label(tr("data.cancel")).on_click(send(&on_event, DataEvent::CloseMargin)))
        .child(Button::new(SEND_ID).primary().small().icon(Icon::new(LucideIcon::MessageSquareReply)).label(tr("post.margin.send")).disabled(dialog.picked.is_none()).on_click(send(&on_event, DataEvent::SubmitMargin)));
    let closing = on_event.clone();
    let parts = SheetParts {
        id: SHEET_ID,
        icon: LucideIcon::MessageSquareReply,
        title: tr("post.margin.title"),
        subtitle: Some(tr("post.margin.subtitle")),
        body,
        footer: buttons,
        on_close: Rc::new(move |window, cx| closing(DataEvent::CloseMargin, window, cx)),
    };
    sheet_frame(parts, theme)
}
