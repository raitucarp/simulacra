//! N6: the "Fork a what-if…" sheet: what the branch supposes and its first post, published as
//! a reply that starts the branch. A locked post never opens it.

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

const SHEET_ID: &str = "data-branch";
const SEND_ID: &str = "data-branch-send";
const CANCEL_ID: &str = "data-branch-cancel";

/// One interaction with the branch sheet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BranchEvent {
    /// Opens the sheet forking at this post.
    Fork(PostId),
    Submit,
    Close,
}

/// The branch sheet's state.
#[derive(Clone)]
pub struct BranchDialog {
    pub post_id: PostId,
    pub label: Entity<InputState>,
    pub text: Entity<InputState>,
    /// Why the last send failed.
    pub problem: Option<String>,
}

fn send(on_event: &DataAction, event: BranchEvent) -> impl Fn(&ClickEvent, &mut Window, &mut App) + 'static {
    let on_event = on_event.clone();
    move |_, window, cx| on_event(DataEvent::Branch(event.clone()), window, cx)
}

fn field(label: &str, input: &Entity<InputState>, theme: &ThemeTokens) -> Div {
    div().flex().flex_col().gap(theme.spacing_xs).child(heading(tr(label), theme)).child(Input::new(input))
}

/// The overlay.
pub fn render_branch_sheet(dialog: &BranchDialog, on_event: DataAction, theme: &ThemeTokens) -> Stateful<Div> {
    let problem = dialog.problem.clone().map(|text| notice(LucideIcon::CircleAlert, text, theme.accent_danger, theme));
    let body = div()
        .flex()
        .flex_col()
        .gap(theme.stack_gap)
        .child(hint(tr("posts.branch.explain"), theme))
        .child(field("posts.branch.label", &dialog.label, theme))
        .child(field("posts.branch.first_post", &dialog.text, theme))
        .children(problem);
    let buttons = footer(theme)
        .child(Button::new(CANCEL_ID).ghost().small().label(tr("data.cancel")).on_click(send(&on_event, BranchEvent::Close)))
        .child(Button::new(SEND_ID).primary().small().icon(Icon::new(LucideIcon::GitBranch)).label(tr("posts.branch.send")).on_click(send(&on_event, BranchEvent::Submit)));
    let closing = on_event.clone();
    let parts = SheetParts {
        id: SHEET_ID,
        icon: LucideIcon::GitBranch,
        title: tr("posts.branch.title"),
        subtitle: Some(tr("posts.branch.subtitle")),
        body,
        footer: buttons,
        on_close: Rc::new(move |window, cx| closing(DataEvent::Branch(BranchEvent::Close), window, cx)),
    };
    sheet_frame(parts, theme)
}
