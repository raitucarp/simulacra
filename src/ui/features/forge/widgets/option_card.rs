//! One illustrated, selectable card: art on top, a label and an optional short description,
//! a brand ring when selected and the theme's focus ring when the keyboard cursor is on it.

use super::focus_ring::focus_ring_shadow;
use super::suggest::suggested_badge;
use crate::theme::ThemeTokens;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;

/// Content and state of an [`option_card`].
pub struct OptionCardSpec {
    pub id: ElementId,
    /// The illustration (any element; usually an `Illustration` sized to `width`).
    pub art: AnyElement,
    pub label: String,
    pub description: Option<String>,
    pub selected: bool,
    pub focused: bool,
    pub disabled: bool,
    /// A good starting pick: a "Suggested" badge over the art.
    pub suggested: bool,
    pub width: Pixels,
}

impl OptionCardSpec {
    /// A plain card with no description, unselected and enabled.
    pub fn new(id: impl Into<ElementId>, art: impl IntoElement, label: impl Into<String>, width: Pixels) -> Self {
        Self {
            id: id.into(),
            art: art.into_any_element(),
            label: label.into(),
            description: None,
            selected: false,
            focused: false,
            disabled: false,
            suggested: false,
            width,
        }
    }
}

/// Border color and width for the card state (the keyboard cursor wins over selection).
fn ring(spec: &OptionCardSpec, theme: &ThemeTokens) -> (Rgba, Pixels) {
    if spec.focused {
        (theme.focus_ring, theme.border_width_threadline)
    } else if spec.selected {
        (theme.accent_brand, theme.border_width_threadline)
    } else {
        (theme.border_subtle, theme.border_width_hairline)
    }
}

/// The card; clicking it (unless disabled) runs `on_click`.
pub fn option_card(
    spec: OptionCardSpec,
    theme: &ThemeTokens,
    on_click: impl Fn(&mut Window, &mut App) + 'static,
) -> Stateful<Div> {
    let (ring_color, ring_width) = ring(&spec, theme);
    let (disabled, selected, focused) = (spec.disabled, spec.selected, spec.focused);
    let hover_bg = theme.bg_elevated;
    div()
        .id(spec.id)
        .w(spec.width)
        .flex_shrink_0()
        .flex()
        .flex_col()
        .rounded(theme.radius_lg)
        .overflow_hidden()
        .border(ring_width)
        .border_color(ring_color)
        .bg(if selected { theme.accent_brand_subtle } else { theme.bg_card })
        .when(focused, |card| card.shadow(focus_ring_shadow(theme)))
        .when(disabled, |card| card.opacity(theme.forge_disabled_opacity))
        .when(!disabled, |card| {
            card.cursor_pointer()
                .hover(move |style| style.bg(hover_bg))
                .on_click(move |_, window, cx| on_click(window, cx))
        })
        .child(div().relative().w_full().flex().justify_center().bg(theme.bg_elevated).child(spec.art).children(spec.suggested.then(|| suggested_badge(theme, spec.width < theme.forge_option_card_width))))
        .child(caption(spec.label, spec.description, selected, theme))
}

fn caption(label: String, description: Option<String>, selected: bool, theme: &ThemeTokens) -> Div {
    let title = div()
        .text_size(theme.font_size_sm)
        .font_weight(if selected { FontWeight::BOLD } else { FontWeight::SEMIBOLD })
        .text_color(if selected { theme.accent_brand_text } else { theme.text_primary })
        .child(label);
    let body = description.map(|text| {
        div()
            .text_size(theme.font_size_xs)
            .text_color(theme.text_muted)
            .line_height(relative(theme.line_height_body))
            .child(text)
    });
    div().p(theme.spacing_sm).flex().flex_col().gap(theme.spacing_2xs).child(title).children(body)
}
