//! Header, subtitle, day labels and legend surrounding the cadence heatmap grid.

use super::heatmap::{ACTIVITY_LEVELS, CELL_COUNT};
use super::section::{render_mono_badge, render_section_header, render_section_title};
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::*;
use rust_i18n::t;

const SPARKS_PER_YEAR_BADGE: &str = "128 sparks/yr";
const SPARKS_PER_YEAR_HINT: &str = "128 sparks / yr · Click day to inspect";
/// Weekday row labels shown beside the grid.
const DAY_LABELS: [&str; 3] = ["M", "W", "F"];
const DAY_LABELS_HEIGHT: f32 = 85.0;
const LEGEND_SWATCH_SIZE: f32 = 8.0;
const LEGEND_LESS: &str = "Less";
const LEGEND_MORE: &str = "More";

pub(super) fn render_heatmap_header(theme: &ThemeTokens) -> Div {
    render_section_header(
        render_section_title(
            LucideIcon::Calendar,
            theme.accent_success,
            t!("sidebar.thought_cadence_title").to_string(),
            theme,
        ),
        render_mono_badge(SPARKS_PER_YEAR_BADGE.to_string(), theme.accent_success, theme).rounded_full(),
    )
}

pub(super) fn render_subtitle(selected_day: Option<usize>, theme: &ThemeTokens) -> Div {
    let subtitle = match selected_day {
        Some(day) => format!("Day #{}: {} sparks", day + 1, ACTIVITY_LEVELS[day % CELL_COUNT]),
        None => SPARKS_PER_YEAR_HINT.to_string(),
    };
    div()
        .font_family(theme.font_family_mono.clone())
        .text_xs()
        .text_color(theme.text_muted)
        .child(subtitle)
}

pub(super) fn render_day_labels(theme: &ThemeTokens) -> Div {
    div()
        .flex()
        .flex_col()
        .justify_between()
        .h(px(DAY_LABELS_HEIGHT))
        .text_xs()
        .text_color(theme.text_muted)
        .children(DAY_LABELS.iter().map(|label| div().child(*label)))
}

fn legend_swatch(color: Rgba) -> Div {
    div()
        .w(px(LEGEND_SWATCH_SIZE))
        .h(px(LEGEND_SWATCH_SIZE))
        .rounded_sm()
        .bg(color)
}

pub(super) fn render_legend(theme: &ThemeTokens) -> Div {
    div()
        .flex()
        .items_center()
        .justify_end()
        .gap_1p5()
        .pt_1()
        .text_xs()
        .text_color(theme.text_muted)
        .child(LEGEND_LESS)
        .child(legend_swatch(theme.bg_card).border_1().border_color(theme.border))
        .child(legend_swatch(theme.cadence_level_1))
        .child(legend_swatch(theme.cadence_level_2))
        .child(legend_swatch(theme.cadence_level_3))
        .child(legend_swatch(theme.cadence_level_4))
        .child(LEGEND_MORE)
}
