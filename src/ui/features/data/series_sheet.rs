//! N10: series from a post's menu: "Add to a series" with the user's series to pick from, the
//! "New series…" sheet (a title and, optionally, how many parts it is meant to have), and
//! leaving a series.

use super::events::{DataAction, DataEvent};
use super::sheet::{footer, heading, hint, notice, sheet_frame, SheetParts};
use super::text::tr;
use crate::domain::PostId;
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::input::{Input, InputState};
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::*;
use std::rc::Rc;

const SHEET_ID: &str = "data-series";
const SEND_ID: &str = "data-series-send";
const CANCEL_ID: &str = "data-series-cancel";

/// One interaction with a series.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SeriesEvent {
    /// Adds the post to the series with this title as its next part.
    Add(PostId, String),
    /// Opens the "New series…" sheet for the post.
    New(PostId),
    Leave(PostId),
    Submit,
    Close,
}

/// The new series sheet's state.
#[derive(Clone)]
pub struct SeriesDialog {
    pub post_id: PostId,
    pub title: Entity<InputState>,
    pub goal: Entity<InputState>,
    pub problem: Option<String>,
}

fn send(on_event: &DataAction, event: SeriesEvent) -> impl Fn(&ClickEvent, &mut Window, &mut App) + 'static {
    let on_event = on_event.clone();
    move |_, window, cx| on_event(DataEvent::Series(event.clone()), window, cx)
}

fn field(label: &str, input: &Entity<InputState>, theme: &ThemeTokens) -> Div {
    div().flex().flex_col().gap(theme.spacing_xs).child(heading(tr(label), theme)).child(Input::new(input))
}

/// The overlay.
pub fn render_series_sheet(dialog: &SeriesDialog, on_event: DataAction, theme: &ThemeTokens) -> Stateful<Div> {
    let problem = dialog.problem.clone().map(|text| notice(LucideIcon::CircleAlert, text, theme.accent_danger, theme));
    let body = div()
        .flex()
        .flex_col()
        .gap(theme.stack_gap)
        .child(hint(tr("posts.series.explain"), theme))
        .child(field("posts.series.title_label", &dialog.title, theme))
        .child(field("posts.series.goal_label", &dialog.goal, theme))
        .children(problem);
    let buttons = footer(theme)
        .child(Button::new(CANCEL_ID).ghost().small().label(tr("data.cancel")).on_click(send(&on_event, SeriesEvent::Close)))
        .child(Button::new(SEND_ID).primary().small().icon(Icon::new(LucideIcon::ListOrdered)).label(tr("posts.series.start")).on_click(send(&on_event, SeriesEvent::Submit)));
    let closing = on_event.clone();
    let parts = SheetParts {
        id: SHEET_ID,
        icon: LucideIcon::ListOrdered,
        title: tr("posts.series.new_title"),
        subtitle: Some(tr("posts.series.subtitle")),
        body,
        footer: buttons,
        on_close: Rc::new(move |window, cx| closing(DataEvent::Series(SeriesEvent::Close), window, cx)),
    };
    sheet_frame(parts, theme)
}
