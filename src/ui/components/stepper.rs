//! Compact numeric steppers built from tiny monospace step buttons.

use crate::theme::ThemeTokens;
use gpui_kit::*;
use std::rc::Rc;

/// Small increment applied by the inner stepper buttons.
pub const STEP_SMALL: u8 = 1;
/// Large increment applied by the outer stepper buttons.
pub const STEP_LARGE: u8 = 5;

/// (element-id suffix, button label) of a stepper button.
pub type StepButton = (&'static str, &'static str);
pub const MINUS_LARGE: StepButton = ("m5", "-5");
pub const MINUS_SMALL: StepButton = ("m1", "-1");
pub const PLUS_SMALL: StepButton = ("p1", "+1");
pub const PLUS_LARGE: StepButton = ("p5", "+5");

/// Value, inclusive bounds and unit suffix of a [`u8_stepper`].
#[derive(Debug, Clone, Copy)]
pub struct StepperSpec<'a> {
    pub value: u8,
    pub min: u8,
    pub max: u8,
    pub unit: &'a str,
}

/// Tiny bordered monospace button (e.g. `-5`, `+1`).
pub fn mini_step_button(
    id: impl Into<ElementId>,
    label: &str,
    theme: &ThemeTokens,
    on_click: impl Fn(&mut Window, &mut App) + 'static,
) -> Stateful<Div> {
    let hover_bg = theme.hover_bg;
    div()
        .id(id)
        .px(theme.spacing_xs)
        .py(theme.spacing_3xs)
        .rounded(theme.radius_xs)
        .bg(theme.bg_card)
        .border(theme.border_width_hairline)
        .border_color(theme.border_subtle)
        .cursor_pointer()
        .hover(move |s| s.bg(hover_bg))
        .on_click(move |_, w, cx| on_click(w, cx))
        .font_family(theme.font_family_mono.clone())
        .text_size(theme.font_size_2xs)
        .text_color(theme.text_secondary)
        .child(label.to_string())
}

/// Builds the `{prefix}-{suffix}` element id used by stepper buttons.
pub fn step_id(id_prefix: &str, suffix: &str) -> String {
    format!("{id_prefix}-{suffix}")
}

/// `-5 -1 [value unit] +1 +5` stepper clamped to `[spec.min, spec.max]`.
pub fn u8_stepper(
    id_prefix: &str,
    spec: StepperSpec<'_>,
    theme: &ThemeTokens,
    on_change: impl Fn(u8, &mut Window, &mut App) + 'static,
) -> Div {
    let cb: Rc<dyn Fn(u8, &mut Window, &mut App)> = Rc::new(on_change);
    let StepperSpec { value, min, max, .. } = spec;
    let button = |(suffix, label): StepButton, next: u8| {
        let cb = cb.clone();
        mini_step_button(step_id(id_prefix, suffix), label, theme, move |w, cx| cb(next, w, cx))
    };
    div()
        .flex()
        .items_center()
        .gap(theme.spacing_2xs)
        .child(button(MINUS_LARGE, value.saturating_sub(STEP_LARGE).max(min)))
        .child(button(MINUS_SMALL, value.saturating_sub(STEP_SMALL).max(min)))
        .child(stepper_value(&spec, theme))
        .child(button(PLUS_SMALL, (value + STEP_SMALL).min(max)))
        .child(button(PLUS_LARGE, (value + STEP_LARGE).min(max)))
}

fn stepper_value(spec: &StepperSpec<'_>, theme: &ThemeTokens) -> Div {
    div()
        .px(theme.spacing_sm)
        .py(theme.spacing_3xs)
        .rounded(theme.radius_sm)
        .bg(theme.bg_app)
        .border(theme.border_width_hairline)
        .border_color(theme.border_strong)
        .font_family(theme.font_family_mono.clone())
        .text_size(theme.font_size_xs)
        .font_weight(FontWeight::BOLD)
        .text_color(theme.accent_brand)
        .child(format!("{}{}", spec.value, spec.unit))
}
