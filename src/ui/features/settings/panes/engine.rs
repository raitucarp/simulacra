//! General pane, engine section: the autonomous-scheduler switch, status and last LLM error.

use super::super::consts::{ICON_XS, SCHEDULER_SWITCH_ID, TEXT_SMALL};
use super::super::general_props::GeneralPaneProps;
use super::common::{body_text, card, muted_note, section, section_label, spaced_row};
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::switch::Switch;
use gpui_kit::component::Icon;
use gpui_kit::{div, px, Div, FontWeight, ParentElement, Styled};
use rust_i18n::t;

/// Section 1: Autonomous Dialectic Engine (scheduler switch, queue status, last error).
pub(super) fn render_engine_section(theme: &ThemeTokens, props: &mut GeneralPaneProps) -> Div {
    let enabled = props.scheduler_enabled;
    let queue = t!("scheduler.queue_status", running = props.running, queued = props.queued);
    let hint = t!("scheduler.toggle_hint", seconds = props.heartbeat_seconds);
    section(
        spaced_row()
            .child(section_label(theme, t!("scheduler.section_title")))
            .child(render_status_badge(theme, enabled)),
        card(theme)
            .gap_2()
            .child(
                spaced_row()
                    .child(body_text(theme, t!("scheduler.toggle_label")))
                    .child(render_switch(theme, props)),
            )
            .child(muted_note(theme, hint))
            .child(muted_note(theme, queue))
            .children(props.last_error.as_ref().map(|err| render_last_error(theme, err))),
    )
}

/// The controlled on/off switch; forwards the requested value to the owner.
fn render_switch(theme: &ThemeTokens, props: &mut GeneralPaneProps) -> Switch {
    let switch = Switch::new(SCHEDULER_SWITCH_ID)
        .checked(props.scheduler_enabled)
        .color(theme.accent_success);
    match props.on_toggle_scheduler.take() {
        Some(on_toggle) => switch.on_click(move |checked, window, cx| on_toggle(*checked, window, cx)),
        None => switch,
    }
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
        .gap_1p5()
        .px_2()
        .py_0p5()
        .rounded_full()
        .bg(bg)
        .border_1()
        .border_color(border)
        .child(Icon::new(icon).size(px(ICON_XS)).text_color(fg))
        .child(
            div()
                .text_size(px(TEXT_SMALL))
                .font_weight(FontWeight::MEDIUM)
                .text_color(fg)
                .child(label.to_string()),
        )
}

/// Danger-tinted line describing the most recent LLM failure.
fn render_last_error(theme: &ThemeTokens, (provider, message): &(String, String)) -> Div {
    div()
        .text_size(px(TEXT_SMALL))
        .text_color(theme.accent_danger)
        .child(t!("scheduler.last_error", provider = provider, message = message).to_string())
}
