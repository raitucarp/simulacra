//! Theme gallery: search field, Dark/Light filter chips and the preview card grid.

use super::super::common::{muted_note, section_label};
use super::consts::{FILTER_ID_PREFIX, ID_SEPARATOR};
use super::filter::{is_visible, ModeFilter};
use super::handlers::{act, PanelHandle};
use super::preview_card::{self, CardState};
use super::AppearancePanel;
use crate::theme::derive::derive_default_tokens;
use crate::theme::{ThemePalette, ThemeRegistry, ThemeTokens};
use crate::ui::components::{chip_row, choice_chip};
use gpui_kit::component::input::Input;
use gpui_kit::*;
use rust_i18n::t;

/// Section title, search field and mode filter chips.
pub(super) fn header(pane: &AppearancePanel, handle: &PanelHandle, theme: &ThemeTokens) -> Div {
    let chips = ModeFilter::ALL.iter().map(|&filter| {
        choice_chip(
            SharedString::from(format!("{FILTER_ID_PREFIX}{ID_SEPARATOR}{}", filter.id())),
            &t!(filter.label_key()),
            pane.filter == filter,
            theme,
            act(handle, move |pane, _, cx| pane.set_filter(filter, cx)),
        )
    });
    div()
        .flex()
        .flex_col()
        .gap(theme.spacing_sm)
        .child(section_label(theme, t!("appearance.gallery_title").to_string()))
        .child(Input::new(&pane.search))
        .child(chip_row(theme).children(chips))
}

/// Custom palettes, then bundled palettes, each as a wrapping grid of preview cards.
pub(super) fn body(pane: &AppearancePanel, handle: &PanelHandle, cx: &App) -> Div {
    let theme = &pane.tokens;
    let query = pane.search.read(cx).value().to_string();
    let visible = |p: &&ThemePalette| is_visible(p, &query, pane.filter);
    let custom: Vec<&ThemePalette> = pane.settings.custom.iter().filter(visible).collect();
    let bundled: Vec<&ThemePalette> = ThemeRegistry::global().all().iter().filter(visible).collect();
    let mut body = div().flex().flex_col().gap(theme.spacing_md);
    if custom.is_empty() && bundled.is_empty() {
        return body.items_center().child(crate::ui::components::art_state(crate::ui::components::ArtState::hint(crate::ui::illustration::Illustration::empty(crate::ui::illustration::EmptyKey::NoResults).width(theme.art_notice_width), t!("appearance.no_results").to_string()), theme));
    }
    for (title, palettes) in [("appearance.your_themes", custom), ("appearance.bundled_themes", bundled)] {
        if !palettes.is_empty() {
            body = body.child(muted_note(theme, t!(title).to_string())).child(grid(pane, handle, &palettes));
        }
    }
    body
}

/// Wrapping grid of cards for `palettes`.
fn grid(pane: &AppearancePanel, handle: &PanelHandle, palettes: &[&ThemePalette]) -> Div {
    let theme = &pane.tokens;
    let cards = palettes.iter().map(|palette| {
        let preview = if palette.id == pane.settings.preset_id { theme.clone() } else { derive_default_tokens(palette) };
        let state = CardState {
            active: palette.id == pane.settings.preset_id,
            custom: pane.settings.is_custom(&palette.id),
        };
        let id = palette.id.clone();
        preview_card::render(palette, &preview, state, theme, act(handle, move |pane, window, cx| pane.select(id.clone(), window, cx)))
    });
    div().flex().flex_wrap().gap(theme.spacing_md).children(cards)
}
