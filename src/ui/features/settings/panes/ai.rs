//! AI & Models provider pane: provider grid, API key, endpoint and model presets.

use super::super::props::AiPaneProps;
use super::super::provider_grid::{render_preset_chips, render_provider_rows};
use super::ai_fields::{render_api_key_section, render_endpoint_section, render_model_section};
use super::common::{section, section_label, spaced_row};
use crate::agent::LlmProvider;
use crate::theme::ThemeTokens;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::{div, Div, IntoElement, ParentElement, Stateful, Styled};
use std::rc::Rc;
use rust_i18n::t;

/// The provider grid, API key, custom endpoint and default model of the AI & Models category.
pub(in crate::ui::features::settings) fn render_ai_pane(theme: &ThemeTokens, props: AiPaneProps) -> impl IntoElement {
    let AiPaneProps { settings, api_key, model, endpoint, on_select_provider, on_select_preset } = props;
    let (current_settings, api_key_input, model_input, endpoint_input) = (&settings, &api_key, &model, &endpoint);
    let active_provider = current_settings.provider;
    let is_custom = active_provider == LlmProvider::Custom;
    let (row1_buttons, row2_buttons) = render_provider_rows(theme, active_provider, Rc::new(on_select_provider));
    let preset_chips = render_preset_chips(theme, current_settings, Rc::new(on_select_preset));

    div()
        .flex()
        .flex_col()
        .gap(theme.section_gap)
        // 1. Provider Grid Header & Selection
        .child(render_provider_section(theme, active_provider, row1_buttons, row2_buttons))
        // 2. API Key Configuration
        .child(render_api_key_section(theme, active_provider, api_key_input))
        // 3. Custom Endpoint (if applicable)
        .when(is_custom, |this| this.child(render_endpoint_section(theme, endpoint_input)))
        // 4. Model Identifier & Quick Presets
        .child(render_model_section(theme, model_input, preset_chips))
}

/// Provider grid header ("SELECT LLM PROVIDER" + active readout) above two button rows.
fn render_provider_section(
    theme: &ThemeTokens,
    active_provider: LlmProvider,
    row1_buttons: Vec<Stateful<Div>>,
    row2_buttons: Vec<Stateful<Div>>,
) -> Div {
    section(
        spaced_row().child(section_label(theme, t!("settings.provider_title").to_string())).child(
            div()
                .text_size(theme.font_size_hint)
                .text_color(theme.accent_brand)
                .child(t!("settings.provider_active", provider = active_provider).to_string()),
        ),
        div()
            .flex()
            .flex_col()
            .gap(theme.spacing_sm)
            .child(div().flex().gap(theme.spacing_sm).children(row1_buttons))
            .child(div().flex().gap(theme.spacing_sm).children(row2_buttons)),
            theme,
    )
}
