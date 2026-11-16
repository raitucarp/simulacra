//! Selectable theme palette preset cards of the Appearance pane.

use super::callbacks::SelectThemeCallback;
use super::consts::{BADGE_AVAILABLE, BADGE_SELECTED, TEXT_CAPTION, TEXT_LABEL, TEXT_SMALL, THEME_SWATCH_SIZE};
use crate::theme::ThemeTokens;
use gpui_kit::{div, px, Div, FontWeight, InteractiveElement, MouseButton, ParentElement, Rgba, Styled};
use std::rc::Rc;

/// Which preset surface tints the card background while it is selected.
pub(super) enum SelectedSurface {
    App,
    Elevated,
}

/// Declarative description of one theme preset card.
pub(super) struct ThemeCardSpec {
    pub build: fn() -> ThemeTokens,
    pub title: &'static str,
    pub description: &'static str,
    pub title_weight: FontWeight,
    pub selected_surface: SelectedSurface,
}

/// The preset palettes offered in the Appearance pane, in display order.
pub(super) const THEME_CARDS: [ThemeCardSpec; 3] = [
    ThemeCardSpec {
        build: crate::theme::minimalist_dark,
        title: "True Threads OLED Black (Default)",
        description: "Deep #000000 background, hairline neutral borders, zero bleed-through",
        title_weight: FontWeight::SEMIBOLD,
        selected_surface: SelectedSurface::App,
    },
    ThemeCardSpec {
        build: crate::theme::obsidian_zinc,
        title: "Obsidian Deep Zinc",
        description: "Subtle zinc warmth with soft contrast for long dialectic sessions",
        title_weight: FontWeight::MEDIUM,
        selected_surface: SelectedSurface::Elevated,
    },
    ThemeCardSpec {
        build: crate::theme::midnight_slate,
        title: "Midnight Slate",
        description: "Cold deep navy tint with oceanic epistemic accents",
        title_weight: FontWeight::MEDIUM,
        selected_surface: SelectedSurface::Elevated,
    },
];

/// Renders one clickable preset card; selection is detected by matching `bg_app`.
pub(super) fn render_theme_card(
    theme: &ThemeTokens,
    spec: &ThemeCardSpec,
    on_select: Rc<SelectThemeCallback>,
) -> Div {
    let preset = (spec.build)();
    let is_selected = theme.bg_app == preset.bg_app;
    let selected_bg: Rgba = match spec.selected_surface {
        SelectedSurface::App => preset.bg_app,
        SelectedSurface::Elevated => preset.bg_elevated,
    };
    let hover_bg = theme.hover_bg;
    let build = spec.build;
    div()
        .p_3p5()
        .rounded_lg()
        .bg(if is_selected { selected_bg } else { theme.bg_card })
        .border_1()
        .border_color(if is_selected { theme.accent_brand } else { theme.border })
        .cursor_pointer()
        .hover(move |s| s.bg(hover_bg))
        .on_mouse_down(MouseButton::Left, move |_ev, window, cx| {
            (on_select)(build(), window, cx);
        })
        .flex()
        .items_center()
        .justify_between()
        .child(render_card_identity(theme, spec, &preset))
        .child(render_selection_badge(theme, is_selected))
}

/// Swatch dot plus title and description.
fn render_card_identity(theme: &ThemeTokens, spec: &ThemeCardSpec, preset: &ThemeTokens) -> Div {
    div()
        .flex()
        .items_center()
        .gap_3()
        .child(
            div()
                .w(px(THEME_SWATCH_SIZE))
                .h(px(THEME_SWATCH_SIZE))
                .rounded_full()
                .bg(preset.bg_app)
                .border_1()
                .border_color(preset.border),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap_0p5()
                .child(
                    div()
                        .text_size(px(TEXT_LABEL))
                        .font_weight(spec.title_weight)
                        .text_color(theme.text_primary)
                        .child(spec.title),
                )
                .child(div().text_size(px(TEXT_SMALL)).text_color(theme.text_muted).child(spec.description)),
        )
}

/// "Selected" / "Available" status pill.
fn render_selection_badge(theme: &ThemeTokens, is_selected: bool) -> Div {
    div()
        .px_2()
        .py_0p5()
        .rounded_full()
        .bg(if is_selected { theme.accent_brand_subtle } else { theme.bg_transparent })
        .text_size(px(TEXT_CAPTION))
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(if is_selected { theme.accent_brand } else { theme.text_muted })
        .child(if is_selected { BADGE_SELECTED } else { BADGE_AVAILABLE })
}
