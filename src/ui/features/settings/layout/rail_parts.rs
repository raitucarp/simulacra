//! Pieces of the category rail: the search box, a button's label, the match-count badge and
//! the privacy footer.

use super::super::props::SettingsInputs;
use super::super::tabs::SettingsTab;
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::input::Input;
use gpui_kit::component::Icon;
use gpui_kit::*;
use rust_i18n::t;

/// The settings search box.
pub(in crate::ui::features::settings) fn search_box(theme: &ThemeTokens, inputs: &SettingsInputs) -> Input {
    Input::new(&inputs.search).cleanable(true).prefix(Icon::new(LucideIcon::Search).size(theme.icon_size_sm)).text_size(theme.font_size_label)
}

/// The category name of a full-width button.
pub(super) fn tab_label(theme: &ThemeTokens, tab: SettingsTab, active: bool) -> Div {
    div()
        .flex_1()
        .min_w_0()
        .truncate()
        .text_size(theme.font_size_sm)
        .font_weight(if active { FontWeight::SEMIBOLD } else { FontWeight::NORMAL })
        .text_color(if active { theme.text_primary } else { theme.text_secondary })
        .child(tab.title())
}

/// Matches in a category while searching.
pub(super) fn count_badge(theme: &ThemeTokens, count: usize) -> Div {
    div()
        .px(theme.spacing_xs)
        .rounded(theme.radius_full)
        .bg(theme.accent_dialectic_subtle)
        .text_size(theme.font_size_2xs)
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(theme.accent_dialectic)
        .child(count.to_string())
}

/// "Everything stays on this computer."
pub(super) fn render_footer(theme: &ThemeTokens) -> Div {
    div()
        .p(theme.card_padding)
        .rounded(theme.radius_md)
        .bg(theme.hover_bg)
        .flex()
        .items_center()
        .gap(theme.inline_gap)
        .child(Icon::new(LucideIcon::Lock).size(theme.icon_size_sm).text_color(theme.text_muted))
        .child(div().flex_1().min_w_0().text_size(theme.font_size_2xs).text_color(theme.text_muted).child(t!("settings.rail_footer").to_string()))
}
