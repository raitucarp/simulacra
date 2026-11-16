//! LLM provider selection grid and model preset chips of the AI Intelligence pane.

use super::callbacks::{SelectPresetModelCallback, SelectProviderCallback};
use super::consts::{PRESET_CHIP_ID_PREFIX, PROVIDERS_PER_ROW, PROVIDER_BUTTON_ID_PREFIX};
use crate::agent::{LlmProvider, LlmSettings};
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::Icon;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::{
    div, Div, FontWeight, InteractiveElement, MouseButton, ParentElement, Stateful, Styled,
};
use std::rc::Rc;

/// Lucide glyph representing each provider.
fn provider_icon(provider: LlmProvider) -> LucideIcon {
    match provider {
        LlmProvider::Gemini => LucideIcon::Sparkles,
        LlmProvider::OpenAi => LucideIcon::Cpu,
        LlmProvider::Anthropic => LucideIcon::Bot,
        LlmProvider::OpenRouter => LucideIcon::Compass,
        LlmProvider::Groq => LucideIcon::Zap,
        LlmProvider::Custom => LucideIcon::Sliders,
    }
}

/// Builds the provider buttons split into the first row and the remainder.
pub(super) fn render_provider_rows(
    theme: &ThemeTokens,
    active_provider: LlmProvider,
    on_provider: Rc<SelectProviderCallback>,
) -> (Vec<Stateful<Div>>, Vec<Stateful<Div>>) {
    let mut row1_buttons = Vec::new();
    let mut row2_buttons = Vec::new();
    for (idx, &provider) in LlmProvider::ALL.iter().enumerate() {
        let btn = render_provider_button(theme, provider, provider == active_provider, on_provider.clone());
        if idx < PROVIDERS_PER_ROW {
            row1_buttons.push(btn);
        } else {
            row2_buttons.push(btn);
        }
    }
    (row1_buttons, row2_buttons)
}

/// One selectable provider button.
fn render_provider_button(
    theme: &ThemeTokens,
    provider: LlmProvider,
    is_selected: bool,
    cb: Rc<SelectProviderCallback>,
) -> Stateful<Div> {
    let border_color = if is_selected { theme.accent_brand } else { theme.border };
    let btn_hover_bg = theme.hover_bg;
    let bg_color = if is_selected { theme.accent_brand_subtle } else { theme.bg_card };
    let icon_color = if is_selected { theme.accent_brand } else { theme.text_muted };
    div()
        .id(format!("{}{:?}", PROVIDER_BUTTON_ID_PREFIX, provider))
        .flex_1()
        .h(theme.settings_provider_button_height)
        .px(theme.spacing_md)
        .rounded(theme.radius_md)
        .border(theme.border_width_hairline)
        .border_color(border_color)
        .bg(bg_color)
        .hover(move |s| s.bg(btn_hover_bg))
        .cursor_pointer()
        .flex()
        .items_center()
        .justify_center()
        .gap(theme.spacing_sm)
        .on_mouse_down(MouseButton::Left, move |_ev, window, cx| {
            (cb)(provider, window, cx);
        })
        .child(Icon::new(provider_icon(provider)).size(theme.icon_size_lg).text_color(icon_color))
        .child(render_provider_label(theme, provider, is_selected))
}

/// Provider name with the "Recommended" hint for Gemini.
fn render_provider_label(theme: &ThemeTokens, provider: LlmProvider, is_selected: bool) -> Div {
    let text_color = if is_selected { theme.text_primary } else { theme.text_secondary };
    let weight = if is_selected { FontWeight::SEMIBOLD } else { FontWeight::NORMAL };
    div()
        .flex()
        .flex_col()
        .child(
            div()
                .text_size(theme.font_size_label)
                .font_weight(weight)
                .text_color(text_color)
                .child(provider.to_string()),
        )
        .when(provider == LlmProvider::Gemini, |this| {
            this.child(div().text_size(theme.font_size_micro).text_color(theme.accent_brand).child("★ Recommended"))
        })
}

/// Clickable chips for the active provider's preset model identifiers.
pub(super) fn render_preset_chips(
    theme: &ThemeTokens,
    current_settings: &LlmSettings,
    on_preset: Rc<SelectPresetModelCallback>,
) -> Vec<Stateful<Div>> {
    current_settings
        .provider
        .preset_models()
        .iter()
        .map(|&preset| render_preset_chip(theme, preset, current_settings.model == preset, on_preset.clone()))
        .collect()
}

/// One preset model chip.
fn render_preset_chip(
    theme: &ThemeTokens,
    preset: &'static str,
    is_current: bool,
    cb: Rc<SelectPresetModelCallback>,
) -> Stateful<Div> {
    let chip_bg = if is_current { theme.bg_tab_active } else { theme.hover_bg };
    let chip_border = if is_current { theme.text_primary } else { theme.border };
    let chip_hover = theme.hover_bg;
    div()
        .id(format!("{}{}", PRESET_CHIP_ID_PREFIX, preset))
        .px(theme.spacing_sm)
        .py(theme.spacing_xs)
        .rounded(theme.radius_sm)
        .border(theme.border_width_hairline)
        .border_color(chip_border)
        .bg(chip_bg)
        .hover(move |s| s.bg(chip_hover))
        .cursor_pointer()
        .text_size(theme.font_size_hint)
        .text_color(if is_current { theme.text_primary } else { theme.text_secondary })
        .on_mouse_down(MouseButton::Left, move |_ev, window, cx| {
            (cb)(preset, window, cx);
        })
        .child(preset)
}
