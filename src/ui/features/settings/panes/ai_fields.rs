//! Input field sections of the AI Intelligence pane.

use super::super::consts::{INPUT_HEIGHT, TEXT_BODY, TEXT_SMALL};
use super::common::{field_group, field_header, field_title};
use crate::agent::LlmProvider;
use crate::theme::ThemeTokens;
use gpui_kit::component::input::{Input, InputState};
use gpui_kit::{div, px, Div, Entity, ParentElement, Stateful, Styled};

/// Full-width settings text input at the standard field height.
fn settings_input(state: &Entity<InputState>) -> Input {
    Input::new(state).w_full().h(px(INPUT_HEIGHT)).text_size(px(TEXT_BODY))
}

/// API key label row and input.
pub(super) fn render_api_key_section(
    theme: &ThemeTokens,
    active_provider: LlmProvider,
    api_key_input: &Entity<InputState>,
) -> Div {
    field_group()
        .child(field_header(
            theme,
            format!("{} API Key", active_provider),
            "Persisted in local SQLite vault",
        ))
        .child(settings_input(api_key_input))
}

/// Custom endpoint URL label and input (Custom provider only).
pub(super) fn render_endpoint_section(theme: &ThemeTokens, endpoint_input: &Entity<InputState>) -> Div {
    field_group()
        .child(field_title(theme, "Custom Endpoint URL"))
        .child(settings_input(endpoint_input))
}

/// Model identifier input plus clickable preset chips.
pub(super) fn render_model_section(
    theme: &ThemeTokens,
    model_input: &Entity<InputState>,
    preset_chips: Vec<Stateful<Div>>,
) -> Div {
    field_group()
        .child(field_header(theme, "Model Identifier", "Click preset to select"))
        .child(settings_input(model_input))
        .child(
            div()
                .flex()
                .items_center()
                .gap_2()
                .mt_1()
                .child(div().text_size(px(TEXT_SMALL)).text_color(theme.text_muted).child("Presets:"))
                .child(div().flex().flex_wrap().gap_1p5().children(preset_chips)),
        )
}
