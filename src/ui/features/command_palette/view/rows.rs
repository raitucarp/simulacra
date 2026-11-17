//! A group heading and one result row (icon, highlighted title, detail, shortcut).

use crate::theme::ThemeTokens;
use crate::ui::features::command_palette::model::{PaletteGroup, PaletteItem};
use crate::ui::features::command_palette::search::highlight_ranges;
use crate::ui::keymap::render_chord;
use gpui_kit::component::Icon;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use rust_i18n::t;

/// Element id prefix of each row (suffixed with its flat index).
const ROW_ID_PREFIX: &str = "palette-row-";

pub(super) fn render_group_heading(group: PaletteGroup, theme: &ThemeTokens) -> Div {
    div()
        .px(theme.spacing_sm)
        .pt(theme.spacing_sm)
        .pb(theme.spacing_2xs)
        .text_size(theme.font_size_2xs)
        .font_weight(FontWeight::BOLD)
        .text_color(theme.text_muted)
        .child(t!(group.title_key()).to_string())
}

pub(super) fn render_result_row(
    flat: usize,
    item: &PaletteItem,
    positions: &[usize],
    is_selected: bool,
    theme: &ThemeTokens,
) -> Stateful<Div> {
    let hover_bg = theme.hover_bg;
    let icon_color = if is_selected { theme.accent_brand } else { theme.text_muted };
    let detail = (!item.detail.is_empty()).then(|| {
        div()
            .min_w_0()
            .max_w(theme.contact_list_pane_min)
            .truncate()
            .text_size(theme.font_size_2xs)
            .text_color(theme.text_muted)
            .child(item.detail.clone())
    });
    div()
        .id(SharedString::from(format!("{ROW_ID_PREFIX}{flat}")))
        .h(theme.nav_row_height)
        .px(theme.spacing_sm)
        .flex()
        .flex_none()
        .items_center()
        .gap(theme.inline_gap)
        .rounded(theme.radius_md)
        .cursor_pointer()
        .when(is_selected, |row| row.bg(theme.accent_brand_subtle))
        .when(!is_selected, |row| row.hover(move |style| style.bg(hover_bg)))
        .child(div().flex_none().text_color(icon_color).child(Icon::new(item.icon).size(theme.icon_size_sm)))
        .child(render_title(item, positions, theme))
        .children(detail)
        .when_some(item.shortcut, |row, chord| row.child(render_chord(chord, theme)))
}

fn render_title(item: &PaletteItem, positions: &[usize], theme: &ThemeTokens) -> Div {
    let style = HighlightStyle {
        color: Some(theme.accent_brand.into()),
        font_weight: Some(FontWeight::SEMIBOLD),
        ..Default::default()
    };
    let highlights = highlight_ranges(&item.title, positions).into_iter().map(move |range| (range, style));
    div()
        .flex_1()
        .min_w_0()
        .truncate()
        .text_size(theme.font_size_sm)
        .text_color(theme.text_primary)
        .child(StyledText::new(item.title.clone()).with_highlights(highlights))
}
