//! Agents & Simulation status row: the scheduler status pill, heartbeat, queue and the last
//! model failure (the switch itself is the "Autonomous posting" row above it).

use super::super::props::SchedulerStatus;
use super::common::muted_note;
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::Icon;
use gpui_kit::{div, Div, FontWeight, ParentElement, Styled};
use rust_i18n::t;

/// Status pill, heartbeat and queue lines, and the last error when there is one.
pub(in crate::ui::features::settings) fn render_scheduler_status(theme: &ThemeTokens, status: &SchedulerStatus) -> Div {
    let queue = t!("scheduler.queue_status", running = status.running, queued = status.queued);
    let hint = t!("scheduler.toggle_hint", seconds = status.heartbeat_seconds);
    div()
        .w_full()
        .py(theme.spacing_sm)
        .flex()
        .flex_col()
        .items_start()
        .gap(theme.spacing_xs)
        .child(render_status_badge(theme, status.enabled))
        .child(muted_note(theme, hint))
        .child(muted_note(theme, queue))
        .children(status.last_error.as_ref().map(|err| render_last_error(theme, err)))
}

/// Status pill: emerald "Scheduler On" or muted "Scheduler Off".
fn render_status_badge(theme: &ThemeTokens, enabled: bool) -> Div {
    let (bg, border, fg, icon, label) = if enabled {
        let label = t!("scheduler.status_on");
        (theme.accent_success_subtle, theme.accent_success_border, theme.accent_success, LucideIcon::Activity, label)
    } else {
        (theme.hover_bg, theme.border, theme.text_muted, LucideIcon::Clock, t!("scheduler.status_off"))
    };
    div()
        .flex()
        .items_center()
        .gap(theme.spacing_xs_plus)
        .px(theme.spacing_sm)
        .py(theme.spacing_2xs)
        .rounded(theme.radius_full)
        .bg(bg)
        .border(theme.border_width_hairline)
        .border_color(border)
        .child(Icon::new(icon).size(theme.icon_size_xs).text_color(fg))
        .child(div().text_size(theme.font_size_hint).font_weight(FontWeight::MEDIUM).text_color(fg).child(label.to_string()))
}

/// Danger-tinted line describing the most recent model failure.
fn render_last_error(theme: &ThemeTokens, (provider, message): &(String, String)) -> Div {
    div()
        .text_size(theme.font_size_hint)
        .text_color(theme.accent_danger)
        .child(t!("scheduler.last_error", provider = provider, message = message).to_string())
}
