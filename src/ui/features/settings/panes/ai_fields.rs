//! Input field sections of the AI Intelligence pane.

use super::common::{field_group, field_header, field_title};
use crate::agent::LlmProvider;
use crate::theme::ThemeTokens;
use gpui_kit::component::input::{Input, InputState};
use gpui_kit::{div, Div, Entity, ParentElement, Stateful, Styled};
use rust_i18n::t;

/// Full-width settings text input at the standard field height.
fn settings_input(state: &Entity<InputState>, theme: &ThemeTokens) -> Input {
    Input::new(state).w_full().h(theme.settings_input_height).text_size(theme.font_size_label)
}

/// API key label row and input.
pub(super) fn render_api_key_section(
    theme: &ThemeTokens,
    active_provider: LlmProvider,
    api_key_input: &Entity<InputState>,
) -> Div {
    field_group(theme)
        .child(field_header(
            theme,
            t!("settings.api_key_label", provider = active_provider).to_string(),
            t!("settings.api_key_note").to_string(),
        ))
        .child(settings_input(api_key_input, theme).mask_toggle())
}

/// Custom endpoint URL label and input (Custom provider only).
pub(super) fn render_endpoint_section(theme: &ThemeTokens, endpoint_input: &Entity<InputState>) -> Div {
    field_group(theme)
        .child(field_title(theme, t!("settings.endpoint_label").to_string()))
        .child(settings_input(endpoint_input, theme))
}

/// Model identifier input plus clickable preset chips.
pub(super) fn render_model_section(
    theme: &ThemeTokens,
    model_input: &Entity<InputState>,
    preset_chips: Vec<Stateful<Div>>,
) -> Div {
    field_group(theme)
        .child(field_title(theme, t!("settings.model_label").to_string()))
        .child(settings_input(model_input, theme))
        .child(
            div()
                .flex()
                .items_center()
                .gap(theme.spacing_sm)
                .mt(theme.spacing_xs)
                .child(div().text_size(theme.font_size_hint).text_color(theme.text_muted).child(t!("settings.presets_label").to_string()))
                .child(div().flex().flex_wrap().gap(theme.control_gap).children(preset_chips)),
        )
}
