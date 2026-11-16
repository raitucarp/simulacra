//! Clickable title/subtitle row with an ON/OFF status pill.

use crate::theme::ThemeTokens;
use gpui_kit::*;

const LABEL_ON: &str = "ON";
const LABEL_OFF: &str = "OFF";

/// Title, subtitle and current state of a [`toggle_row`].
#[derive(Debug, Clone, Copy)]
pub struct ToggleSpec<'a> {
    pub title: &'a str,
    pub subtitle: &'a str,
    pub enabled: bool,
}

/// A full-width row that flips a boolean setting when clicked.
pub fn toggle_row(
    id: impl Into<ElementId>,
    spec: ToggleSpec<'_>,
    theme: &ThemeTokens,
    on_toggle: impl Fn(&mut Window, &mut App) + 'static,
) -> Stateful<Div> {
    let hover_bg = theme.hover_bg;
    div()
        .id(id)
        .w_full()
        .px(theme.spacing_sm)
        .py(theme.spacing_xs)
        .rounded(theme.radius_md)
        .bg(theme.bg_app)
        .border(theme.border_width_hairline)
        .border_color(if spec.enabled { theme.accent_brand_border } else { theme.border_subtle })
        .cursor_pointer()
        .hover(move |s| s.bg(hover_bg))
        .on_click(move |_, w, cx| on_toggle(w, cx))
        .flex()
        .items_center()
        .justify_between()
        .gap(theme.spacing_sm)
        .child(toggle_text(&spec, theme))
        .child(toggle_pill(spec.enabled, theme))
}

fn toggle_text(spec: &ToggleSpec<'_>, theme: &ThemeTokens) -> Div {
    div()
        .flex_1()
        .min_w_0()
        .flex()
        .flex_col()
        .gap(theme.spacing_3xs)
        .child(
            div()
                .text_size(theme.font_size_xs)
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(theme.text_primary)
                .truncate()
                .child(spec.title.to_string()),
        )
        .child(
            div()
                .text_size(theme.font_size_2xs)
                .text_color(theme.text_muted)
                .truncate()
                .child(spec.subtitle.to_string()),
        )
}

fn toggle_pill(enabled: bool, theme: &ThemeTokens) -> Div {
    let (bg, border, text) = if enabled {
        (theme.accent_success_subtle, theme.accent_success_border, theme.accent_success)
    } else {
        (theme.hover_bg, theme.border_subtle, theme.text_muted)
    };
    div()
        .flex_shrink_0()
        .px(theme.spacing_xs)
        .py(theme.spacing_3xs)
        .rounded(theme.radius_full)
        .bg(bg)
        .border(theme.border_width_hairline)
        .border_color(border)
        .font_family(theme.font_family_mono.clone())
        .text_size(theme.font_size_2xs)
        .font_weight(FontWeight::BOLD)
        .text_color(text)
        .child(if enabled { LABEL_ON } else { LABEL_OFF })
}
