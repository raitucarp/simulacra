//! The overlay shown while files are dragged over the window: drop anywhere to import.

use crate::theme::ThemeTokens;
use crate::ui::illustration::{Illustration, IllustrationKind, ILLUSTRATION_EXT, ILLUSTRATION_ROOT};
use gpui_kit::*;
use rust_i18n::t;

/// The drop card art (`illustrations/import/drop.svg`).
fn drop_art() -> String {
    format!("{ILLUSTRATION_ROOT}/import/drop.{ILLUSTRATION_EXT}")
}

/// The full-window drop hint (it lets the drop through to the window underneath).
pub fn render_drop_overlay(theme: &ThemeTokens) -> Div {
    div()
        .absolute()
        .inset_0()
        .bg(theme.bg_scrim)
        .flex()
        .items_center()
        .justify_center()
        .child(
            div()
                .p(theme.panel_padding)
                .rounded(theme.radius_lg)
                .border(theme.focus_ring_width)
                .border_color(theme.accent_brand)
                .bg(theme.bg_elevated)
                .flex()
                .flex_col()
                .items_center()
                .gap(theme.stack_gap)
                .child(Illustration::asset(drop_art(), IllustrationKind::TraitCard).width(theme.drop_art_width))
                .child(div().text_size(theme.font_size_lg).font_weight(FontWeight::BOLD).text_color(theme.text_primary).child(t!("import.drop_title").to_string()))
                .child(div().text_size(theme.font_size_xs).text_color(theme.text_muted).child(t!("import.drop_hint").to_string())),
        )
}
