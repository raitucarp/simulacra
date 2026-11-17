//! Header, subtitle, month/weekday labels, level colors and legend around the cadence grid.

use super::heatmap::cell_slot;
use super::section::{render_mono_badge, render_section_header, render_section_title};
use crate::domain::activity::{ActivityHeatmap, HEATMAP_WEEKS, MAX_ACTIVITY_LEVEL};
use crate::theme::ThemeTokens;
use crate::ui::features::day_filter::{day_summary, month_label};
use crate::ui::icons::LucideIcon;
use chrono::NaiveDate;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use crate::i18n::t_count;
use rust_i18n::t;

/// Weekday rows (Sunday = 0) that carry a label, with their translation keys.
const LABELED_ROWS: [(usize, &str); 3] =
    [(1, "cadence.weekday_mon"), (3, "cadence.weekday_wed"), (5, "cadence.weekday_fri")];
/// Divisor centering a label on its row.
const HALF: f32 = 2.0;

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
        render_mono_badge(t_count("cadence.badge", total), theme.accent_success, theme)
            .rounded(theme.radius_full),
    )
}

pub(super) fn render_subtitle(heatmap: &ActivityHeatmap, selected: Option<NaiveDate>, theme: &ThemeTokens) -> Div {
    let subtitle = match selected {
        Some(day) => day_summary(day, heatmap.count_on(day)),
        None => t!("cadence.hint").to_string(),
    };
    div().font_family(theme.font_family_mono.clone()).text_size(theme.font_size_label).text_color(theme.text_muted).child(subtitle)
}

/// Small muted label absolutely placed in a slot, vertically centered on a `slot_height` row.
fn slot_label(text: String, slot_height: Pixels, theme: &ThemeTokens) -> Div {
    div()
        .absolute()
        .left_0()
        .top((slot_height - theme.font_size_2xs) / HALF)
        .text_size(theme.font_size_2xs)
        .line_height(theme.font_size_2xs)
        .whitespace_nowrap()
        .text_color(theme.text_muted)
        .child(text)
}

/// Fixed-width label column cell of `row` (Mon / Wed / Fri, otherwise blank).
pub(super) fn render_weekday_label(row: usize, theme: &ThemeTokens) -> Div {
    let key = LABELED_ROWS.iter().find(|(r, _)| *r == row).map(|(_, key)| *key);
    div()
        .flex_none()
        .w(theme.heatmap_label_width)
        .h(theme.heatmap_cell_height)
        .relative()
        .when_some(key, |slot, key| slot.child(slot_label(t!(key).to_string(), theme.heatmap_cell_height, theme)))
}

/// Month names above the first week column of each month, aligned with the grid.
pub(super) fn render_month_row(heatmap: &ActivityHeatmap, theme: &ThemeTokens) -> Div {
    let labels = heatmap.month_labels();
    let slots = (0..HEATMAP_WEEKS).map(|week| {
        let label = labels.iter().find(|l| l.week == week).map(|l| month_label(l.month));
        cell_slot(theme)
            .h(theme.heatmap_month_row_height)
            .relative()
            .when_some(label, |slot, text| slot.child(slot_label(text, theme.heatmap_month_row_height, theme)))
    });
    div()
        .w_full()
        .flex()
        .child(div().flex_none().w(theme.heatmap_label_width))
        .child(div().flex_1().min_w_0().flex().gap(theme.heatmap_cell_gap).children(slots))
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
