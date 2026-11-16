//! Gallery card: a miniature feed drawn entirely with one palette's derived tokens, over a
//! footer (name, credit, badges) drawn with the app's current tokens.

use super::consts::CARD_ID_PREFIX;
use crate::theme::{ThemePalette, ThemeTokens};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use rust_i18n::t;

/// What a card shows besides its palette.
pub(super) struct CardState {
    pub active: bool,
    pub custom: bool,
}

/// One clickable gallery card.
pub(super) fn render(
    palette: &ThemePalette,
    preview: &ThemeTokens,
    state: CardState,
    chrome: &ThemeTokens,
    on_click: impl Fn(&mut Window, &mut App) + 'static,
) -> Stateful<Div> {
    let hover_border = chrome.border_strong;
    div()
        .id(SharedString::from(format!("{CARD_ID_PREFIX}{}", palette.id)))
        .w(chrome.theme_preview_card_width)
        .flex()
        .flex_col()
        .rounded(chrome.radius_lg)
        .overflow_hidden()
        .bg(chrome.bg_card)
        .cursor_pointer()
        .border(if state.active { chrome.border_width_threadline } else { chrome.border_width_hairline })
        .border_color(if state.active { chrome.accent_brand } else { chrome.border })
        .when(!state.active, |card| card.hover(move |s| s.border_color(hover_border)))
        .on_click(move |_, window, cx| on_click(window, cx))
        .child(mini_feed(preview))
        .child(footer(palette, &state, chrome))
}

/// The palette's app background holding one fake post.
fn mini_feed(p: &ThemeTokens) -> Div {
    div().bg(p.bg_app).p(p.spacing_sm).child(
        div()
            .flex()
            .flex_col()
            .gap(p.spacing_xs)
            .p(p.spacing_sm)
            .rounded(p.radius_md)
            .bg(p.bg_card)
            .border(p.border_width_hairline)
            .border_color(p.border)
            .child(author_row(p))
            .child(div().text_size(p.font_size_2xs).text_color(p.text_primary).child(t!("appearance.preview_body").to_string()))
            .child(action_row(p)),
    )
}

/// Avatar dot, display name and muted handle/timestamp.
fn author_row(p: &ThemeTokens) -> Div {
    div()
        .flex()
        .items_center()
        .gap(p.spacing_xs)
        .text_size(p.font_size_2xs)
        .child(div().size(p.theme_preview_avatar_size).rounded(p.radius_full).bg(p.accent_agent))
        .child(div().font_weight(FontWeight::SEMIBOLD).text_color(p.text_primary).child(t!("appearance.preview_name").to_string()))
        .child(div().truncate().text_color(p.text_muted).child(t!("appearance.preview_meta").to_string()))
}

/// A wikilink chip and a filled accent button.
fn action_row(p: &ThemeTokens) -> Div {
    let pill = |bg: Rgba, ink: Rgba, label: String| {
        div().px(p.spacing_sm).rounded(p.radius_full).bg(bg).text_color(ink).child(label)
    };
    div()
        .flex()
        .items_center()
        .justify_between()
        .text_size(p.font_size_2xs)
        .child(pill(p.accent_brand_subtle, p.accent_brand, t!("appearance.preview_chip").to_string()))
        .child(pill(p.accent_brand, p.on_accent, t!("composer.post_button").to_string()).font_weight(FontWeight::SEMIBOLD))
}

/// Name, credit and the Active / Custom badges, in the app's own chrome colors.
fn footer(palette: &ThemePalette, state: &CardState, chrome: &ThemeTokens) -> Div {
    let badge = |label: String, ink: Rgba| div().text_size(chrome.font_size_2xs).font_weight(FontWeight::SEMIBOLD).text_color(ink).child(label);
    div()
        .flex()
        .flex_col()
        .gap(chrome.spacing_3xs)
        .px(chrome.spacing_sm)
        .py(chrome.spacing_xs)
        .border_t(chrome.border_width_hairline)
        .border_color(chrome.border_subtle)
        .child(
            div()
                .flex()
                .items_center()
                .justify_between()
                .gap(chrome.spacing_xs)
                .child(div().truncate().text_size(chrome.font_size_xs).font_weight(FontWeight::MEDIUM).text_color(chrome.text_primary).child(palette.name.clone()))
                .when(state.active, |row| row.child(badge(t!("appearance.active").to_string(), chrome.accent_brand)))
                .when(state.custom && !state.active, |row| row.child(badge(t!("appearance.custom_badge").to_string(), chrome.text_muted))),
        )
        .child(div().truncate().text_size(chrome.font_size_2xs).text_color(chrome.text_muted).child(palette.credit.clone()))
}
