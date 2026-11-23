//! A5: the status of the run the user started from an agent's profile, under its actions:
//! queued, writing (both with Cancel), posted (with "Open the post"), not posted or
//! cancelled (both with Dismiss).

use super::recommended::OpenPostCallback;
use crate::agent::schedule::RunStatus;
use crate::domain::{Profile, ProfileId};
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::*;
use rust_i18n::t;
use std::rc::Rc;

const CANCEL_ID: &str = "roster-run-cancel";
const DISMISS_ID: &str = "roster-run-dismiss";
const OPEN_ID: &str = "roster-run-open";

/// What the status line asks the shell to do.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RunEvent {
    Cancel(ProfileId),
    Dismiss(ProfileId),
}

pub type RunAction = Rc<dyn Fn(RunEvent, &mut Window, &mut App)>;

/// The icon and words of `status` for `name`.
pub fn status_line(status: &RunStatus, name: &str) -> (LucideIcon, String) {
    match status {
        RunStatus::Queued => (LucideIcon::Clock, t!("trigger.status.queued", name = name).to_string()),
        RunStatus::Writing => (LucideIcon::Feather, t!("trigger.status.writing", name = name).to_string()),
        RunStatus::Done { .. } => (LucideIcon::Check, t!("trigger.status.done").to_string()),
        RunStatus::Failed => (LucideIcon::Info, t!("trigger.status.failed").to_string()),
        RunStatus::Cancelled => (LucideIcon::CircleStop, t!("trigger.status.cancelled").to_string()),
    }
}

fn button(id: &'static str, key: &str, run: impl Fn(&mut Window, &mut App) + 'static) -> Button {
    Button::new(id).ghost().xsmall().label(t!(key).to_string()).on_click(move |_, window, cx| run(window, cx))
}

/// The status line of `profile`'s run.
pub(super) fn render_run_status(profile: &Profile, status: &RunStatus, on_run: &RunAction, on_open: &OpenPostCallback, theme: &ThemeTokens) -> Div {
    let (icon, text) = status_line(status, &profile.name);
    let send = |event: RunEvent| {
        let on_run = on_run.clone();
        move |window: &mut Window, cx: &mut App| on_run(event.clone(), window, cx)
    };
    let id = profile.id.clone();
    let mut actions = Vec::new();
    match status {
        RunStatus::Queued | RunStatus::Writing => actions.push(button(CANCEL_ID, "trigger.status.cancel", send(RunEvent::Cancel(id)))),
        RunStatus::Done { post } => {
            if let Some(post) = post.clone() {
                let open = on_open.clone();
                actions.push(button(OPEN_ID, "trigger.status.open", move |window, cx| open(post.clone(), window, cx)));
            }
            actions.push(button(DISMISS_ID, "trigger.status.dismiss", send(RunEvent::Dismiss(id))));
        }
        RunStatus::Failed | RunStatus::Cancelled => actions.push(button(DISMISS_ID, "trigger.status.dismiss", send(RunEvent::Dismiss(id)))),
    }
    let color = if status.is_active() { theme.accent_agent } else { theme.text_muted };
    div()
        .w_full()
        .flex()
        .items_center()
        .justify_between()
        .gap(theme.control_gap)
        .px(theme.spacing_sm)
        .py(theme.spacing_2xs)
        .rounded(theme.radius_md)
        .bg(theme.bg_callout)
        .child(div().flex().items_center().gap(theme.spacing_xs).text_size(theme.font_size_xs).text_color(theme.text_secondary).child(Icon::new(icon).xsmall().text_color(color)).child(text))
        .child(div().flex().gap(theme.spacing_3xs).children(actions))
}
