//! The UpSet matrix for four or more sets: one column per exclusive intersection (largest
//! first) with its size as a bar, a dot per member set below it, and each set's size on its
//! row; plus the Jaccard similarity table.

use crate::domain::insights::sets::{jaccard_matrix, NamedSet, Region};
use crate::theme::ThemeTokens;
use crate::ui::features::insights::chart::columns::ColumnClick;
use crate::ui::features::insights::chart::palette::{sequential, series};
use crate::ui::features::insights::chart::scale::LinearScale;
use crate::ui::features::insights::chart::ticks::{compact, percent};
use gpui_kit::component::tooltip::Tooltip;
use gpui_kit::*;

fn names(sets: &[NamedSet], region: &Region) -> String {
    region.sets().iter().filter_map(|&i| sets.get(i)).map(|s| s.label.as_str()).collect::<Vec<_>>().join(" ∩ ")
}

fn bar_column(index: usize, region: &Region, sets: &[NamedSet], scale: LinearScale, on_open: &ColumnClick, theme: &ThemeTokens) -> Stateful<Div> {
    let tip = format!("{}: {}", names(sets, region), crate::i18n::t_count("insights.posts", region.members.len()));
    let open = on_open.clone();
    let bar = div().w_full().h(relative(scale.share(region.members.len() as f64))).rounded_t(theme.radius_xs).bg(theme.accent_brand);
    let value = div().text_size(theme.font_size_2xs).text_color(theme.text_muted).child(compact(region.members.len() as f64));
    div()
        .id(ElementId::NamedInteger("insights-upset".into(), index as u64))
        .w(theme.insights_upset_cell)
        .flex_none()
        .px(theme.spacing_2xs)
        .h_full()
        .flex()
        .flex_col()
        .items_center()
        .justify_end()
        .gap(theme.spacing_3xs)
        .cursor_pointer()
        .hover(|d| d.bg(theme.hover_bg))
        .tooltip(move |window, cx| Tooltip::new(tip.clone()).build(window, cx))
        .on_click(move |_, window, cx| open(index, window, cx))
        .child(value)
        .child(bar)
}

fn set_row(i: usize, set: &NamedSet, regions: &[Region], theme: &ThemeTokens) -> Div {
    let name = div().w(theme.insights_upset_label_width).flex_none().truncate().text_size(theme.font_size_xs).text_color(theme.text_secondary).child(format!("{} · {}", set.label, compact(set.len() as f64)));
    let dots = regions.iter().map(|region| {
        let member = region.mask & (1 << i) != 0;
        let dot = div().size(theme.insights_upset_dot).rounded(theme.radius_full).bg(if member { series(i, theme) } else { theme.border_subtle });
        div().w(theme.insights_upset_cell).flex_none().flex().justify_center().child(dot)
    });
    div().flex().items_center().child(name).children(dots)
}

/// The matrix; `on_open` opens the first post of a column.
pub fn upset_matrix(sets: &[NamedSet], regions: &[Region], on_open: ColumnClick, theme: &ThemeTokens) -> Stateful<Div> {
    let max = regions.iter().map(|r| r.members.len()).max().unwrap_or(1) as f64;
    let scale = LinearScale::new((0.0, max), (0.0, 1.0));
    let bars = regions.iter().enumerate().map(|(i, r)| bar_column(i, r, sets, scale, &on_open, theme));
    let header = div().flex().h(theme.insights_chart_height).child(div().w(theme.insights_upset_label_width).flex_none()).children(bars);
    let rows = sets.iter().enumerate().map(|(i, set)| set_row(i, set, regions, theme));
    div().id("insights-upset-scroll").w_full().overflow_x_scroll().child(div().flex().flex_col().gap(theme.spacing_xs).child(header).children(rows))
}

/// A column of the Jaccard table: as wide as the card allows, between a value's and a tile's width.
fn jaccard_column(theme: &ThemeTokens) -> Div {
    div().flex_1().min_w(theme.insights_rank_value_width).max_w(theme.insights_tile_width)
}

/// The Jaccard similarity of every pair of sets, as a shaded table with the sets named over
/// its columns as well as beside its rows.
pub fn jaccard_table(sets: &[NamedSet], theme: &ThemeTokens) -> Stateful<Div> {
    let matrix = jaccard_matrix(sets);
    let cell = |value: f64| {
        let text = if value >= 0.995 { theme.on_accent } else { theme.text_primary };
        jaccard_column(theme).py(theme.spacing_3xs).flex().justify_center().rounded(theme.radius_xs).bg(sequential(value as f32, theme)).text_size(theme.font_size_2xs).text_color(text).child(percent(value))
    };
    let label = |text: &str| div().w(theme.insights_upset_label_width).flex_none().truncate().text_size(theme.font_size_xs).text_color(theme.text_secondary).child(text.to_string());
    let heading = |text: &str| jaccard_column(theme).flex().justify_center().overflow_hidden().child(div().truncate().text_size(theme.font_size_2xs).text_color(theme.text_muted).child(text.to_string()));
    let header = div().flex().items_center().gap(theme.spacing_3xs).child(label("")).children(sets.iter().map(|set| heading(&set.label)));
    let rows = sets.iter().zip(&matrix).map(|(set, row)| div().flex().items_center().gap(theme.spacing_3xs).child(label(&set.label)).children(row.iter().map(|v| cell(*v))));
    div().id("insights-jaccard-scroll").w_full().overflow_x_scroll().child(div().w_full().flex().flex_col().gap(theme.spacing_3xs).child(header).children(rows))
}
