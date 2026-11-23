//! N12: the drawing sheet ("New drawing"): a small pad to draw on by hand, the pen's ink and
//! width, Undo and Clear, a caption if you like, and Post (the drawing is also saved as SVG).

use super::events::{DataAction, DataEvent};
use super::ink_pad::InkPad;
use super::sheet::{footer, heading, hint, notice, sheet_frame, SheetParts};
use super::text::tr;
use crate::domain::post::ink::{Ink, PEN_WIDTHS};
use crate::theme::ThemeTokens;
use crate::ui::components::ink_figure::ink_colour;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::input::{Input, InputState};
use gpui_kit::component::{Icon, Selectable, Sizable};
use gpui_kit::*;
use std::rc::Rc;

const SHEET_ID: &str = "data-ink";

/// One interaction with the drawing sheet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InkEvent {
    Open,
    Pen(Ink),
    Width(u8),
    Undo,
    Clear,
    Submit,
    Close,
}

/// The sheet's state.
#[derive(Clone)]
pub struct InkDialog {
    pub pad: Entity<InkPad>,
    pub caption: Entity<InputState>,
    /// The pen, as the pad has it.
    pub ink: Ink,
    pub width: u8,
    pub problem: Option<String>,
}

fn send(on_event: &DataAction, event: InkEvent) -> impl Fn(&ClickEvent, &mut Window, &mut App) + 'static {
    let on_event = on_event.clone();
    move |_, window, cx| on_event(DataEvent::Ink(event.clone()), window, cx)
}

/// The pen row: an ink swatch per colour, the widths, Undo and Clear.
fn pens(dialog: &InkDialog, on_event: &DataAction, theme: &ThemeTokens) -> Div {
    let swatch = |ink: Ink| {
        let ring = if ink == dialog.ink { theme.text_primary } else { theme.border_subtle };
        div().id(SharedString::from(format!("ink-pen-{ink:?}"))).size(theme.icon_size_md).rounded(theme.radius_full).bg(ink_colour(ink, theme)).border(theme.border_width_threadline).border_color(ring).cursor_pointer().on_click(send(on_event, InkEvent::Pen(ink)))
    };
    let width = |value: u8| Button::new(SharedString::from(format!("ink-width-{value}"))).xsmall().ghost().label(format!("{value}")).selected(value == dialog.width).on_click(send(on_event, InkEvent::Width(value)));
    div()
        .flex()
        .flex_wrap()
        .items_center()
        .gap(theme.inline_gap)
        .children(Ink::ALL.map(swatch))
        .children(PEN_WIDTHS.map(width))
        .child(Button::new("ink-undo").xsmall().ghost().icon(Icon::new(LucideIcon::Undo2)).label(tr("posts.ink.undo")).on_click(send(on_event, InkEvent::Undo)))
        .child(Button::new("ink-clear").xsmall().ghost().icon(Icon::new(LucideIcon::Eraser)).label(tr("posts.ink.clear")).on_click(send(on_event, InkEvent::Clear)))
}

/// The overlay.
pub fn render_ink_sheet(dialog: &InkDialog, on_event: DataAction, theme: &ThemeTokens) -> Stateful<Div> {
    let problem = dialog.problem.clone().map(|text| notice(LucideIcon::CircleAlert, text, theme.accent_danger, theme));
    let caption = div().flex().flex_col().gap(theme.spacing_xs).child(heading(tr("posts.ink.caption_label"), theme)).child(Input::new(&dialog.caption));
    let body = div().flex().flex_col().gap(theme.stack_gap).child(hint(tr("posts.ink.explain"), theme)).child(pens(dialog, &on_event, theme)).child(dialog.pad.clone()).child(caption).children(problem);
    let buttons = footer(theme)
        .child(Button::new("data-ink-cancel").ghost().small().label(tr("data.cancel")).on_click(send(&on_event, InkEvent::Close)))
        .child(Button::new("data-ink-send").primary().small().icon(Icon::new(LucideIcon::Send)).label(tr("posts.ink.post")).on_click(send(&on_event, InkEvent::Submit)));
    let closing = on_event.clone();
    let parts = SheetParts {
        id: SHEET_ID,
        icon: LucideIcon::PenTool,
        title: tr("posts.ink.title"),
        subtitle: Some(tr("posts.ink.subtitle")),
        body,
        footer: buttons,
        on_close: Rc::new(move |window, cx| closing(DataEvent::Ink(InkEvent::Close), window, cx)),
    };
    sheet_frame(parts, theme)
}
