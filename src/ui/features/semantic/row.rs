//! A scored result row (icon, title, detail, excerpt, similarity bar) shared by the
//! palette's semantic mode and the "find similar" sheet.

use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::Icon;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;

/// Element id prefix of each row (suffixed with its index).
const ROW_ID_PREFIX: &str = "semantic-row-";
/// Percent per unit of similarity.
const PERCENT: f32 = 100.0;

/// One clickable scored row.
#[allow(clippy::too_many_arguments)]
pub fn render_scored_row(
    index: usize,
    icon: LucideIcon,
    title: &str,
    detail: &str,
    excerpt: &str,
    score: f32,
    is_selected: bool,
    theme: &ThemeTokens,
) -> Stateful<Div> {
    let hover_bg = theme.hover_bg;
    let icon_color = if is_selected { theme.accent_brand } else { theme.text_muted };
    let heading = render_heading(title, detail, score, theme);
    let body = div()
        .flex_1()
        .min_w_0()
        .flex()
        .flex_col()
        .gap(theme.spacing_3xs)
        .child(heading)
        .when(!excerpt.is_empty() && excerpt != title, |col| {
            col.child(div().text_size(theme.font_size_xs).text_color(theme.text_secondary).line_clamp(2).child(excerpt.to_string()))
        });
    div()
        .id(SharedString::from(format!("{ROW_ID_PREFIX}{index}")))
        .px(theme.spacing_sm)
        .py(theme.spacing_xs_plus)
        .flex()
        .flex_none()
        .items_start()
        .gap(theme.inline_gap)
        .rounded(theme.radius_md)
        .cursor_pointer()
        .when(is_selected, |row| row.bg(theme.accent_brand_subtle))
        .when(!is_selected, |row| row.hover(move |style| style.bg(hover_bg)))
        .child(div().flex_none().pt(theme.spacing_3xs).text_color(icon_color).child(Icon::new(icon).size(theme.icon_size_sm)))
        .child(body)
}

/// Title, detail and the similarity readout on one line.
fn render_heading(title: &str, detail: &str, score: f32, theme: &ThemeTokens) -> Div {
    div()
        .flex()
        .items_center()
        .gap(theme.inline_gap)
        .child(div().flex_1().min_w_0().truncate().text_size(theme.font_size_sm).text_color(theme.text_primary).child(title.to_string()))
        .when(!detail.is_empty(), |row| row.child(div().flex_none().text_size(theme.font_size_2xs).text_color(theme.text_muted).child(detail.to_string())))
        .child(render_similarity(score, theme))
}

/// A short bar filled to `score` and the percentage beside it.
pub fn render_similarity(score: f32, theme: &ThemeTokens) -> Div {
    let score = score.clamp(0.0, 1.0);
    let track = div()
        .w(theme.similarity_bar_width)
        .h(theme.similarity_bar_height)
        .rounded(theme.radius_full)
        .bg(theme.border_subtle)
        .overflow_hidden()
        .child(div().h_full().w(relative(score)).rounded(theme.radius_full).bg(theme.accent_brand));
    let percent = format!("{:.0}%", score * PERCENT);
    div()
        .flex_none()
        .flex()
        .items_center()
        .gap(theme.spacing_xs)
        .child(track)
        .child(div().min_w(theme.similarity_readout_width).text_size(theme.font_size_2xs).text_color(theme.text_muted).child(percent))
}
