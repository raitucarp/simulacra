//! N14: retracting or correcting a post from its menu: the sheet asks for the reason (what no
//! longer stands, or what is right instead); withdrawing the mark needs no sheet.

use super::events::{DataAction, DataEvent};
use super::sheet::{footer, heading, hint, notice, sheet_frame, SheetParts};
use super::text::tr;
use crate::domain::post::correction::CorrectionKind;
use crate::domain::PostId;
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::input::{Input, InputState};
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::*;
use std::rc::Rc;

const SHEET_ID: &str = "data-correction";

/// One interaction with a retraction or correction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CorrectionEvent {
    /// Opens the sheet for the post, to mark it so.
    Open(PostId, CorrectionKind),
    /// Takes the mark off the post.
    Withdraw(PostId),
    Submit,
    Close,
}

/// The sheet's state.
#[derive(Clone)]
pub struct CorrectionDialog {
    pub post_id: PostId,
    pub kind: CorrectionKind,
    pub reason: Entity<InputState>,
    pub problem: Option<String>,
}

fn send(on_event: &DataAction, event: CorrectionEvent) -> impl Fn(&ClickEvent, &mut Window, &mut App) + 'static {
    let on_event = on_event.clone();
    move |_, window, cx| on_event(DataEvent::Correction(event.clone()), window, cx)
}

/// The overlay.
pub fn render_correction_sheet(dialog: &CorrectionDialog, on_event: DataAction, theme: &ThemeTokens) -> Stateful<Div> {
    let key = |part: &str| tr(&format!("posts.correction.{}_{part}", dialog.kind.key()));
    let problem = dialog.problem.clone().map(|text| notice(LucideIcon::CircleAlert, text, theme.accent_danger, theme));
    let reason = div().flex().flex_col().gap(theme.spacing_xs).child(heading(key("reason_label"), theme)).child(Input::new(&dialog.reason));
    let body = div().flex().flex_col().gap(theme.stack_gap).child(hint(key("explain"), theme)).child(reason).children(problem);
    let icon = match dialog.kind {
        CorrectionKind::Retracted => LucideIcon::CircleX,
        CorrectionKind::Corrected => LucideIcon::Pencil,
    };
    let buttons = footer(theme)
        .child(Button::new("data-correction-cancel").ghost().small().label(tr("data.cancel")).on_click(send(&on_event, CorrectionEvent::Close)))
        .child(Button::new("data-correction-send").primary().small().icon(Icon::new(icon)).label(key("send")).on_click(send(&on_event, CorrectionEvent::Submit)));
    let closing = on_event.clone();
    let parts = SheetParts {
        id: SHEET_ID,
        icon,
        title: key("title"),
        subtitle: Some(tr("posts.correction.subtitle")),
        body,
        footer: buttons,
        on_close: Rc::new(move |window, cx| closing(DataEvent::Correction(CorrectionEvent::Close), window, cx)),
    };
    sheet_frame(parts, theme)
}
