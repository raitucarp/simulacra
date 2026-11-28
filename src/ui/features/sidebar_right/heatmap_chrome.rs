//! Header, subtitle, level colors and legend around the cadence grid.

use super::section::{render_count_badge, render_section_header, render_section_title};
use crate::domain::activity::{ActivityHeatmap, MAX_ACTIVITY_LEVEL};
use crate::theme::ThemeTokens;
use crate::ui::features::day_filter::day_summary;
use crate::ui::icons::LucideIcon;
use jiff::civil::Date;
use gpui_kit::*;
use crate::i18n::t_count;
use rust_i18n::t;

/// Cell color of an intensity level: the empty tone, or the success accent at the
/// level's token opacity.
pub(super) fn level_color(level: u8, theme: &ThemeTokens) -> Rgba {
    let opacity = match level {
        0 => return theme.border_subtle,
        1 => theme.cadence_alpha_1,
        2 => theme.cadence_alpha_2,
        3 => theme.cadence_alpha_3,
        _ => theme.cadence_alpha_4,
    };
    Rgba { a: theme.accent_success.a * opacity, ..theme.accent_success }
}

pub(super) fn render_heatmap_header(total: usize, theme: &ThemeTokens) -> Div {
    render_section_header(
        render_section_title(
            LucideIcon::Calendar,
            theme.accent_success,
            t!("sidebar.thought_cadence_title").to_string(),
            theme,
        ),
        render_count_badge(t_count("cadence.badge", total), theme.accent_success, theme)
            .rounded(theme.radius_full),
    )
}

pub(super) fn render_subtitle(heatmap: &ActivityHeatmap, selected: Option<Date>, theme: &ThemeTokens) -> Div {
    let subtitle = match selected {
        Some(day) => day_summary(day, heatmap.count_on(day)),
        None => t!("cadence.hint").to_string(),
    };
    div().text_size(theme.font_size_label).text_color(theme.text_muted).child(subtitle)
}

fn legend_swatch(color: Rgba, theme: &ThemeTokens) -> Div {
    div()
        .w(theme.cadence_cell_size)
        .h(theme.cadence_cell_size)
        .rounded(theme.heatmap_cell_radius)
        .bg(color)
}

pub(super) fn render_legend(theme: &ThemeTokens) -> Div {
    div()
        .flex()
        .items_center()
        .justify_end()
        .gap(theme.spacing_xs)
        .pt(theme.spacing_xs)
        .text_size(theme.font_size_label)
        .text_color(theme.text_muted)
        .child(t!("cadence.less").to_string())
        .children((0..=MAX_ACTIVITY_LEVEL).map(|level| legend_swatch(level_color(level, theme), theme)))
        .child(t!("cadence.more").to_string())
}
