//! Input field sections and status banner of the AI Intelligence pane.

use super::super::consts::{INPUT_HEIGHT, TEXT_BODY, TEXT_SMALL};
use super::common::{field_group, field_header, field_title};
use crate::agent::LlmProvider;
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::input::{Input, InputState};
use gpui_kit::component::Icon;
use gpui_kit::{div, px, Div, Entity, FontWeight, ParentElement, Stateful, Styled};
use rust_i18n::t;

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

/// Epistemic agent status banner (fully tokenized).
pub(super) fn render_status_banner(theme: &ThemeTokens) -> Div {
    div()
        .p(theme.spacing_md)
        .rounded(theme.radius_md)
        .bg(theme.accent_brand_subtle)
        .border(theme.border_width_hairline)
        .border_color(theme.accent_brand_border)
        .flex()
        .items_center()
        .justify_between()
        .child(
            div()
                .flex()
                .items_center()
                .gap(theme.spacing_sm)
                .child(Icon::new(LucideIcon::Users).size(theme.font_size_sm).text_color(theme.accent_brand))
                .child(
                    div()
                        .font_family(theme.font_family_ui.clone())
                        .text_size(theme.font_size_xs)
                        .text_color(theme.text_secondary)
                        .child(t!("settings.agent_status").to_string()),
                ),
        )
        .child(
            div()
                .font_family(theme.font_family_mono.clone())
                .text_size(theme.font_size_2xs)
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(theme.accent_brand)
                .child("Ready"),
        )
}
