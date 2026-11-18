//! The accessible summary line under every chart: the chart's point in one sentence, so it
//! reads without seeing the picture.

use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::Icon;
use gpui_kit::*;

/// A muted sentence with an info glyph.
pub fn summary_line(text: String, theme: &ThemeTokens) -> Div {
    div()
        .w_full()
        .flex()
        .items_start()
        .gap(theme.inline_gap)
        .child(Icon::new(LucideIcon::Info).size(theme.icon_size_xs).text_color(theme.text_muted).flex_none())
        .child(div().flex_1().min_w_0().text_size(theme.font_size_xs).line_height(relative(theme.line_height_body)).text_color(theme.text_muted).child(text))
}
