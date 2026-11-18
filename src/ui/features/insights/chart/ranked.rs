//! A ranked list with horizontal bars: label, a track filled to the value's share of the
//! largest, and the value. Rows open their item on click or Enter / Space.

use super::scale::LinearScale;
use crate::theme::ThemeTokens;
use crate::ui::features::forge::widgets::KeyboardStop;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use std::rc::Rc;

/// Opens row `index`.
pub type RowAction = Rc<dyn Fn(usize, &mut Window, &mut App)>;

/// One ranked row.
#[derive(Debug, Clone, PartialEq)]
pub struct RankedRow {
    pub label: String,
    pub value: f64,
    pub value_text: String,
    pub colour: Rgba,
    /// Highlighted (the picked item).
    pub active: bool,
}

/// The list; `on_open` makes the rows interactive.
pub fn ranked_list(id: &str, rows: &[RankedRow], on_open: Option<RowAction>, theme: &ThemeTokens) -> Div {
    let max = rows.iter().map(|r| r.value).fold(f64::MIN_POSITIVE, f64::max);
    let scale = LinearScale::new((0.0, max), (0.0, 1.0));
    let items = rows.iter().enumerate().map(|(index, row)| {
        let track = div()
            .flex_1()
            .min_w_0()
            .h(theme.insights_rank_bar_height)
            .rounded(theme.radius_full)
            .bg(theme.border_subtle)
            .child(div().h_full().w(relative(scale.share(row.value))).rounded(theme.radius_full).bg(row.colour));
        let label = div().w(theme.insights_rank_label_width).flex_none().overflow_hidden().whitespace_nowrap().text_ellipsis().text_size(theme.font_size_xs).text_color(theme.text_primary).child(row.label.clone());
        let value = div().w(theme.insights_rank_value_width).flex_none().text_right().text_size(theme.font_size_xs).text_color(theme.text_muted).child(row.value_text.clone());
        let line = div()
            .id(ElementId::NamedInteger(SharedString::from(id.to_string()), index as u64))
            .flex()
            .items_center()
            .gap(theme.control_gap)
            .px(theme.spacing_xs)
            .py(theme.spacing_3xs)
            .rounded(theme.radius_sm)
            .border(theme.border_width_hairline)
            .border_color(if row.active { theme.accent_brand_border } else { theme.bg_transparent })
            .when(row.active, |d| d.bg(theme.accent_brand_subtle))
            .child(label)
            .child(track)
            .child(value);
        match on_open.clone() {
            Some(open) => {
                let activate = Rc::clone(&open);
                line.cursor_pointer()
                    .hover(|s| s.bg(theme.hover_bg))
                    .on_click(move |_, window, cx| open(index, window, cx))
                    .keyboard_stop(theme, Rc::new(move |window, cx| activate(index, window, cx)))
                    .into_any_element()
            }
            None => line.into_any_element(),
        }
    });
    div().w_full().flex().flex_col().gap(theme.spacing_3xs).children(items)
}
