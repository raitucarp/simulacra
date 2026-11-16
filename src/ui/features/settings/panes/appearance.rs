//! Appearance tab pane: theme palette presets and layout density toggles.

use super::super::callbacks::SelectThemeCallback;
use super::super::consts::{STATUS_ENABLED, TEXT_SMALL};
use super::super::theme_card::{render_theme_card, THEME_CARDS};
use super::common::{body_text, card, section, section_label, spaced_row};
use crate::theme::ThemeTokens;
use gpui_kit::{div, px, Div, FontWeight, IntoElement, ParentElement, Styled};
use std::rc::Rc;

/// Layout & feed density features shown as always-enabled rows.
const DENSITY_FEATURES: [&str; 3] = [
    "Threads Continuous Threadlines",
    "Human-Readable Wikilink Excerpts",
    "Subpixel Text Rendering",
];

/// Helper function to render Appearance Tab Pane
pub(in crate::ui::features::settings) fn render_appearance_pane(
    theme: &ThemeTokens,
    on_select_theme: SelectThemeCallback,
) -> impl IntoElement {
    let cb_theme = Rc::new(on_select_theme);
    let cards: Vec<Div> = THEME_CARDS
        .iter()
        .map(|spec| render_theme_card(theme, spec, cb_theme.clone()))
        .collect();

    div()
        .flex()
        .flex_col()
        .gap_5()
        // Theme Presets
        .child(section(
            section_label(theme, "THEME PALETTES"),
            div().flex().flex_col().gap_2().children(cards),
        ))
        // Layout & Density
        .child(render_density_section(theme))
}

/// Layout & feed density card listing the enabled rendering features.
fn render_density_section(theme: &ThemeTokens) -> Div {
    section(
        section_label(theme, "LAYOUT & FEED DENSITY"),
        card(theme)
            .gap_3()
            .children(DENSITY_FEATURES.iter().map(|&label| render_enabled_row(theme, label))),
    )
}

/// Feature label with an emerald "Enabled" status.
fn render_enabled_row(theme: &ThemeTokens, label: &'static str) -> Div {
    spaced_row().child(body_text(theme, label)).child(
        div()
            .text_size(px(TEXT_SMALL))
            .font_weight(FontWeight::SEMIBOLD)
            .text_color(theme.accent_success)
            .child(STATUS_ENABLED),
    )
}
