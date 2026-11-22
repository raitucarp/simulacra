//! The "Unlock post" sheet (X4): one masked password field. The right password opens the
//! post for this session only (it locks again after a while without activity); a wrong one
//! says so and the field is emptied.

use super::sheet::{footer, hint, notice, sheet_frame, SheetParts};
use super::text::tr;
use crate::domain::PostId;
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::base::Disableable;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::input::{Input, InputState};
use gpui_kit::component::Sizable;
use gpui_kit::component::Icon;
use gpui_kit::*;
use std::rc::Rc;

const SHEET_ID: &str = "data-unlock";
const UNLOCK_ID: &str = "data-unlock-open";
const CANCEL_ID: &str = "data-unlock-cancel";

/// What a locked card or the unlock sheet asks of the shell.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnlockEvent {
    /// Opens the sheet for this post.
    Open(PostId),
    Submit,
    Close,
    /// Locks this unlocked post again.
    Relock(PostId),
}

/// Sends an unlock event to the shell.
pub type UnlockAction = Rc<dyn Fn(UnlockEvent, &mut Window, &mut App) + 'static>;

/// The unlock sheet's state.
#[derive(Clone)]
pub struct UnlockDialog {
    pub post_id: PostId,
    pub password: Entity<InputState>,
    /// The key is being derived.
    pub busy: bool,
    /// Why the last attempt failed.
    pub problem: Option<String>,
}

impl UnlockDialog {
    pub fn new(post_id: PostId, password: Entity<InputState>) -> Self {
        Self { post_id, password, busy: false, problem: None }
    }
}

fn send(on_event: &UnlockAction, event: UnlockEvent) -> impl Fn(&ClickEvent, &mut Window, &mut App) + 'static {
    let on_event = on_event.clone();
    move |_, window, cx| on_event(event.clone(), window, cx)
}

/// The overlay.
pub fn render_unlock_sheet(dialog: &UnlockDialog, on_event: UnlockAction, theme: &ThemeTokens) -> Stateful<Div> {
    let problem = dialog.problem.clone().map(|text| notice(LucideIcon::CircleAlert, text, theme.accent_danger, theme));
    let label = div().text_size(theme.font_size_xs).text_color(theme.text_secondary).child(tr("posts.lock.password"));
    let field = div().flex().flex_col().gap(theme.spacing_xs).child(label).child(Input::new(&dialog.password).mask_toggle());
    let body = div().flex().flex_col().gap(theme.stack_gap).child(hint(tr("posts.lock.explain"), theme)).child(field).children(problem);
    let action = if dialog.busy { tr("posts.lock.unlocking") } else { tr("posts.lock.unlock") };
    let buttons = footer(theme)
        .child(Button::new(CANCEL_ID).ghost().small().label(tr("data.cancel")).on_click(send(&on_event, UnlockEvent::Close)))
        .child(Button::new(UNLOCK_ID).primary().small().icon(Icon::new(LucideIcon::LockOpen)).label(action).disabled(dialog.busy).on_click(send(&on_event, UnlockEvent::Submit)));
    let closing = on_event.clone();
    let parts = SheetParts {
        id: SHEET_ID,
        icon: LucideIcon::LockKeyhole,
        title: tr("posts.lock.title"),
        subtitle: Some(tr("posts.lock.subtitle")),
        body,
        footer: buttons,
        on_close: Rc::new(move |w, cx| closing(UnlockEvent::Close, w, cx)),
    };
    sheet_frame(parts, theme)
}
