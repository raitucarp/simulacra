//! Global settings modal dialog organism featuring vertical category tabs
//! (General, AI Intelligence, Appearance, Knowledge Vault) with true Threads OLED styling.

use crate::agent::{LlmProvider, LlmSettings};
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::input::{Input, InputState};
use gpui_kit::component::scroll::ScrollableElement;
use gpui_kit::component::Icon;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::{
    div, px, AnyElement, App, Entity, InteractiveElement, IntoElement, MouseButton,
    ParentElement, RenderOnce, Styled, Window,
};
use rust_i18n::t;
use std::rc::Rc;

/// Categorical tabs for the vertical settings navigation rail.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettingsTab {
    General,
    AiIntelligence,
    Appearance,
    KnowledgeVault,
}

impl SettingsTab {
    pub const ALL: [SettingsTab; 4] = [
        SettingsTab::General,
        SettingsTab::AiIntelligence,
        SettingsTab::Appearance,
        SettingsTab::KnowledgeVault,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Self::General => "General",
            Self::AiIntelligence => "AI Intelligence",
            Self::Appearance => "Appearance",
            Self::KnowledgeVault => "Knowledge Vault",
        }
    }

    pub fn icon(self) -> LucideIcon {
        match self {
            Self::General => LucideIcon::Sliders,
            Self::AiIntelligence => LucideIcon::Bot,
            Self::Appearance => LucideIcon::Palette,
            Self::KnowledgeVault => LucideIcon::Brain,
        }
    }

    pub fn title(self) -> &'static str {
        match self {
            Self::General => "General Preferences",
            Self::AiIntelligence => "AI Intelligence & Providers",
            Self::Appearance => "Theme & Appearance",
            Self::KnowledgeVault => "Knowledge Vault & RAG",
        }
    }

    pub fn description(self) -> &'static str {
        match self {
            Self::General => "Local SQLite database, autonomous thought cycle, and interface locale",
            Self::AiIntelligence => "Configure LLM provider models (Google Gemini recommended) and API keys",
            Self::Appearance => "Display density, high-contrast OLED black themes, and layout aesthetics",
            Self::KnowledgeVault => "Local vector embedding engine, resonance thresholds, and epistemic grounding",
        }
    }
}

pub type ModalCloseCallback = Box<dyn Fn(&mut Window, &mut App) + 'static>;
pub type SelectProviderCallback = Box<dyn Fn(LlmProvider, &mut Window, &mut App) + 'static>;
pub type SelectPresetModelCallback = Box<dyn Fn(&'static str, &mut Window, &mut App) + 'static>;
pub type SaveSettingsCallback = Box<dyn Fn(&mut Window, &mut App) + 'static>;
pub type SelectTabCallback = Box<dyn Fn(SettingsTab, &mut Window, &mut App) + 'static>;
pub type SelectThemeCallback = Box<dyn Fn(ThemeTokens, &mut Window, &mut App) + 'static>;

/// Comprehensive Vertical-Tabbed Settings Modal Organism.
#[derive(IntoElement)]
pub struct SettingsModal {
    theme: ThemeTokens,
    current_settings: LlmSettings,
    active_tab: SettingsTab,
    api_key_input: Entity<InputState>,
    model_input: Entity<InputState>,
    endpoint_input: Entity<InputState>,
    on_select_tab: SelectTabCallback,
    on_select_theme: SelectThemeCallback,
    on_select_provider: SelectProviderCallback,
    on_select_preset_model: SelectPresetModelCallback,
    on_save: SaveSettingsCallback,
    on_close: ModalCloseCallback,
}

impl SettingsModal {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        theme: ThemeTokens,
        current_settings: LlmSettings,
        active_tab: SettingsTab,
        api_key_input: Entity<InputState>,
        model_input: Entity<InputState>,
        endpoint_input: Entity<InputState>,
        on_select_tab: impl Fn(SettingsTab, &mut Window, &mut App) + 'static,
        on_select_theme: impl Fn(ThemeTokens, &mut Window, &mut App) + 'static,
        on_select_provider: impl Fn(LlmProvider, &mut Window, &mut App) + 'static,
        on_select_preset_model: impl Fn(&'static str, &mut Window, &mut App) + 'static,
        on_save: impl Fn(&mut Window, &mut App) + 'static,
        on_close: impl Fn(&mut Window, &mut App) + 'static,
    ) -> Self {
        Self {
            theme,
            current_settings,
            active_tab,
            api_key_input,
            model_input,
            endpoint_input,
            on_select_tab: Box::new(on_select_tab),
            on_select_theme: Box::new(on_select_theme),
            on_select_provider: Box::new(on_select_provider),
            on_select_preset_model: Box::new(on_select_preset_model),
            on_save: Box::new(on_save),
            on_close: Box::new(on_close),
        }
    }
}

impl RenderOnce for SettingsModal {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let on_close = Rc::new(self.on_close);
        let on_close_backdrop = on_close.clone();
        let on_close_x = on_close.clone();
        let on_close_cancel = on_close.clone();

        let on_save = Rc::new(self.on_save);
        let on_save_click = on_save.clone();

        let on_tab = Rc::new(self.on_select_tab);
        let active_tab = self.active_tab;

        // Build Left Rail Tab List
        let tab_buttons: Vec<_> = SettingsTab::ALL
            .iter()
            .map(|&tab| {
                let is_active = tab == active_tab;
                let cb = on_tab.clone();
                let hover_bg = self.theme.hover_bg;

                let bg_color = if is_active {
                    self.theme.bg_tab_active
                } else {
                    self.theme.bg_transparent
                };

                let text_color = if is_active {
                    self.theme.text_primary
                } else {
                    self.theme.text_secondary
                };

                let icon_color = if is_active {
                    self.theme.accent_brand
                } else {
                    self.theme.text_muted
                };

                div()
                    .id(format!("settings-tab-btn-{:?}", tab))
                    .w_full()
                    .h(self.theme.titlebar_height)
                    .px_3()
                    .rounded_lg()
                    .bg(bg_color)
                    .hover(move |s| s.bg(hover_bg))
                    .cursor_pointer()
                    .flex()
                    .items_center()
                    .justify_between()
                    .on_mouse_down(MouseButton::Left, move |_ev, window, cx| {
                        cx.stop_propagation();
                        (cb)(tab, window, cx);
                    })
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2p5()
                            .child(Icon::new(tab.icon()).size(px(15.0)).text_color(icon_color))
                            .child(
                                div()
                                    .text_size(px(13.0))
                                    .font_weight(if is_active {
                                        gpui_kit::FontWeight::SEMIBOLD
                                    } else {
                                        gpui_kit::FontWeight::NORMAL
                                    })
                                    .text_color(text_color)
                                    .child(tab.label()),
                            ),
                    )
                    .when(is_active, |this| {
                        this.child(
                            div()
                                .w(px(3.0))
                                .h(px(16.0))
                                .rounded_full()
                                .bg(self.theme.accent_brand),
                        )
                    })
            })
            .collect();

        // Dispatch tab content to individual helper functions to prevent stack frame exhaustion
        let tab_content: AnyElement = match active_tab {
            SettingsTab::General => render_general_pane(&self.theme).into_any_element(),
            SettingsTab::AiIntelligence => render_ai_pane(
                &self.theme,
                &self.current_settings,
                &self.api_key_input,
                &self.model_input,
                &self.endpoint_input,
                self.on_select_provider,
                self.on_select_preset_model,
            )
            .into_any_element(),
            SettingsTab::Appearance => render_appearance_pane(&self.theme, self.on_select_theme).into_any_element(),
            SettingsTab::KnowledgeVault => render_vault_pane(&self.theme).into_any_element(),
        };

        // Render full dialog modal layout
        div()
            .id("settings-modal-backdrop")
            .absolute()
            .inset_0()
            .bg(self.theme.bg_scrim) // Deep opaque scrim to eliminate background bleed
            .flex()
            .items_center()
            .justify_center()
            .on_mouse_down(MouseButton::Left, move |_ev, window, cx| {
                (on_close_backdrop)(window, cx);
            })
            .child(
                // Modal Container: tokenized dimensions
                div()
                    .id("settings-modal-dialog")
                    .w(self.theme.modal_width)
                    .h(self.theme.modal_height)
                    .rounded_xl()
                    .border_1()
                    .border_color(self.theme.border)
                    .bg(self.theme.bg_elevated) // solid OLED dark surface
                    .shadow_2xl()
                    .flex()
                    .overflow_hidden()
                    .on_mouse_down(MouseButton::Left, |_ev, _window, cx| {
                        cx.stop_propagation();
                    })
                    // Left Navigation Rail
                    .child(
                        div()
                            .w(self.theme.modal_rail_width)
                            .h_full()
                            .bg(self.theme.bg_sidebar)
                            .border_r_1()
                            .border_color(self.theme.border)
                            .p_4()
                            .flex()
                            .flex_col()
                            .justify_between()
                            // Rail Top: Title & Tabs
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_4()
                                    .child(
                                        div()
                                            .flex()
                                            .items_center()
                                            .gap_2p5()
                                            .child(
                                                div()
                                                    .w(px(26.0))
                                                    .h(px(26.0))
                                                    .rounded_lg()
                                                    .bg(self.theme.accent_brand_subtle)
                                                    .flex()
                                                    .items_center()
                                                    .justify_center()
                                                    .child(
                                                        Icon::new(LucideIcon::Sliders)
                                                            .size(px(14.0))
                                                            .text_color(self.theme.accent_brand),
                                                    ),
                                            )
                                            .child(
                                                div()
                                                    .flex()
                                                    .flex_col()
                                                    .child(
                                                        div()
                                                            .text_size(px(14.0))
                                                            .font_weight(gpui_kit::FontWeight::BOLD)
                                                            .text_color(self.theme.text_primary)
                                                            .child("Settings"),
                                                    )
                                                    .child(
                                                        div()
                                                            .text_size(px(10.0))
                                                            .text_color(self.theme.text_muted)
                                                            .child("Simulacra System"),
                                                    ),
                                            ),
                                    )
                                    // Tabs List
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap_1()
                                            .children(tab_buttons),
                                    ),
                            )
                            // Rail Bottom: Epistemic status footer
                            .child(
                                div()
                                    .p_2p5()
                                    .rounded_lg()
                                    .bg(self.theme.hover_bg)
                                    .border_1()
                                    .border_color(self.theme.border)
                                    .flex()
                                    .items_center()
                                    .gap_2()
                                    .child(
                                        Icon::new(LucideIcon::Lock)
                                            .size(px(12.0))
                                            .text_color(self.theme.text_muted),
                                    )
                                    .child(
                                        div()
                                            .text_size(px(10.0))
                                            .text_color(self.theme.text_muted)
                                            .child("Local Epistemic Sovereignty"),
                                    ),
                            ),
                    )
                    // Right Content Pane (620px flex_1)
                    .child(
                        div()
                            .flex_1()
                            .h_full()
                            .flex()
                            .flex_col()
                            // Header of Active Tab
                            .child(
                                div()
                                    .h(px(64.0))
                                    .px_6()
                                    .border_b_1()
                                    .border_color(self.theme.border)
                                    .flex()
                                    .items_center()
                                    .justify_between()
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap_0p5()
                                            .child(
                                                div()
                                                    .text_size(px(15.0))
                                                    .font_weight(gpui_kit::FontWeight::BOLD)
                                                    .text_color(self.theme.text_primary)
                                                    .child(active_tab.title()),
                                            )
                                            .child(
                                                div()
                                                    .text_size(px(11.0))
                                                    .text_color(self.theme.text_muted)
                                                    .child(active_tab.description()),
                                            ),
                                    )
                                    .child(
                                        Button::new("btn-close-settings-x")
                                            .ghost()
                                            .icon(Icon::new(LucideIcon::X).size(px(14.0)))
                                            .on_click(move |_, window, cx| {
                                                (on_close_x)(window, cx);
                                            }),
                                    ),
                            )
                            // Scrollable Tab Body
                            .child(
                                div()
                                    .flex_1()
                                    .overflow_y_scrollbar()
                                    .p_6()
                                    .child(tab_content),
                            )
                            // Persistent Footer Action Bar
                            .child(
                                div()
                                    .h(self.theme.modal_footer_height)
                                    .px_6()
                                    .border_t_1()
                                    .border_color(self.theme.border)
                                    .bg(self.theme.bg_elevated)
                                    .flex()
                                    .items_center()
                                    .justify_between()
                                    .child(
                                        Button::new("btn-cancel-settings")
                                            .ghost()
                                            .label("Cancel")
                                            .on_click(move |_, window, cx| {
                                                (on_close_cancel)(window, cx);
                                            }),
                                    )
                                    .child(
                                        Button::new("btn-save-settings")
                                            .primary()
                                            .icon(Icon::new(LucideIcon::Check).size(px(13.0)))
                                            .label("Save & Apply Settings")
                                            .on_click(move |_, window, cx| {
                                                (on_save_click)(window, cx);
                                            }),
                                    ),
                            ),
                    ),
            )
    }
}

/// Helper function to render General Tab Pane
fn render_general_pane(theme: &ThemeTokens) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .gap_5()
        // Section 1: Autonomous Dialectic Engine
        .child(
            div()
                .flex()
                .flex_col()
                .gap_2()
                .child(
                    div()
                        .flex()
                        .items_center()
                        .justify_between()
                        .child(
                            div()
                                .text_size(px(12.0))
                                .font_weight(gpui_kit::FontWeight::BOLD)
                                .text_color(theme.text_muted)
                                .child("AUTONOMOUS DIALECTIC ENGINE"),
                        )
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap_1p5()
                                .px_2()
                                .py_0p5()
                                .rounded_full()
                                .bg(theme.accent_success_subtle)
                                .border_1()
                                .border_color(theme.accent_success_border)
                                .child(
                                    Icon::new(LucideIcon::Activity)
                                        .size(px(11.0))
                                        .text_color(theme.accent_success),
                                )
                                .child(
                                    div()
                                        .text_size(px(11.0))
                                        .font_weight(gpui_kit::FontWeight::MEDIUM)
                                        .text_color(theme.accent_success)
                                        .child("Active Loop"),
                                ),
                        ),
                )
                .child(
                    div()
                        .p_3p5()
                        .rounded_lg()
                        .bg(theme.bg_card)
                        .border_1()
                        .border_color(theme.border)
                        .flex()
                        .flex_col()
                        .gap_2()
                        .child(
                            div()
                                .text_size(px(12.0))
                                .text_color(theme.text_primary)
                                .child("Autonomous Background Heartbeat: 15-minute cycle"),
                        )
                        .child(
                            div()
                                .text_size(px(11.0))
                                .text_color(theme.text_muted)
                                .child("Simulacra's autonomous loop ticks on new thoughts and scheduled intervals, enabling each network's human-like member agents to dialectically question and link memory nodes."),
                        ),
                ),
        )
        // Section 2: Local SQLite Vault Storage
        .child(
            div()
                .flex()
                .flex_col()
                .gap_2()
                .child(
                    div()
                        .text_size(px(12.0))
                        .font_weight(gpui_kit::FontWeight::BOLD)
                        .text_color(theme.text_muted)
                        .child("LOCAL STORAGE & EPISTEMIC VAULT"),
                )
                .child(
                    div()
                        .p_3p5()
                        .rounded_lg()
                        .bg(theme.bg_card)
                        .border_1()
                        .border_color(theme.border)
                        .flex()
                        .flex_col()
                        .gap_3()
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .justify_between()
                                .child(
                                    div()
                                        .flex()
                                        .flex_col()
                                        .gap_0p5()
                                        .child(
                                            div()
                                                .text_size(px(12.0))
                                                .font_weight(gpui_kit::FontWeight::MEDIUM)
                                                .text_color(theme.text_primary)
                                                .child("Database Path"),
                                        )
                                        .child(
                                            div()
                                                .text_size(px(11.0))
                                                .font_family("Consolas, monospace")
                                                .text_color(theme.text_secondary)
                                                .child("~/.simulacra/knowledge.db (WAL Mode)"),
                                        ),
                                )
                                .child(
                                    div()
                                        .px_2()
                                        .py_1()
                                        .rounded_md()
                                        .bg(theme.hover_bg)
                                        .border_1()
                                        .border_color(theme.border)
                                        .text_size(px(11.0))
                                        .text_color(theme.text_secondary)
                                        .child("Schema v1.4"),
                                ),
                        )
                        .child(
                            div()
                                .text_size(px(11.0))
                                .text_color(theme.text_muted)
                                .child("All dialectic posts, wikilinks, vectors, and API keys are stored in your encrypted local SQLite database. No telemetry is sent to any centralized cloud."),
                        ),
                ),
        )
        // Section 3: Interface Language & Locale
        .child(
            div()
                .flex()
                .flex_col()
                .gap_2()
                .child(
                    div()
                        .text_size(px(12.0))
                        .font_weight(gpui_kit::FontWeight::BOLD)
                        .text_color(theme.text_muted)
                        .child("INTERFACE LOCALE"),
                )
                .child(
                    div()
                        .p_3()
                        .rounded_lg()
                        .bg(theme.bg_card)
                        .border_1()
                        .border_color(theme.border)
                        .flex()
                        .items_center()
                        .justify_between()
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap_2()
                                .child(Icon::new(LucideIcon::Globe).size(px(14.0)).text_color(theme.text_muted))
                                .child(
                                    div()
                                        .text_size(px(12.0))
                                        .text_color(theme.text_primary)
                                        .child("Dialectic Language Support"),
                                ),
                        )
                        .child(
                            div()
                                .text_size(px(11.0))
                                .text_color(theme.text_secondary)
                                .child("English (US) / Indonesian (Bilingual)"),
                        ),
                ),
        )
}

/// Helper function to render AI Intelligence Tab Pane
fn render_ai_pane(
    theme: &ThemeTokens,
    current_settings: &LlmSettings,
    api_key_input: &Entity<InputState>,
    model_input: &Entity<InputState>,
    endpoint_input: &Entity<InputState>,
    on_select_provider: SelectProviderCallback,
    on_select_preset_model: SelectPresetModelCallback,
) -> impl IntoElement {
    let on_provider = Rc::new(on_select_provider);
    let on_preset = Rc::new(on_select_preset_model);
    let active_provider = current_settings.provider;
    let is_custom = active_provider == LlmProvider::Custom;

    let mut row1_buttons = Vec::new();
    let mut row2_buttons = Vec::new();
    for (idx, &provider) in LlmProvider::ALL.iter().enumerate() {
        let is_selected = provider == active_provider;
        let cb = on_provider.clone();

        let border_color = if is_selected {
            theme.accent_brand
        } else {
            theme.border
        };

        let btn_hover_bg = theme.hover_bg;
        let bg_color = if is_selected {
            theme.accent_brand_subtle
        } else {
            theme.bg_card
        };

        let text_color = if is_selected {
            theme.text_primary
        } else {
            theme.text_secondary
        };

        let icon_view = match provider {
            LlmProvider::Gemini => LucideIcon::Sparkles,
            LlmProvider::OpenAi => LucideIcon::Cpu,
            LlmProvider::Anthropic => LucideIcon::Bot,
            LlmProvider::OpenRouter => LucideIcon::Compass,
            LlmProvider::Groq => LucideIcon::Zap,
            LlmProvider::Custom => LucideIcon::Sliders,
        };

        let btn = div()
            .id(format!("provider-btn-{:?}", provider))
            .flex_1()
            .h(px(40.0))
            .px_3()
            .rounded_lg()
            .border_1()
            .border_color(border_color)
            .bg(bg_color)
            .hover(move |s| s.bg(btn_hover_bg))
            .cursor_pointer()
            .flex()
            .items_center()
            .justify_center()
            .gap_2()
            .on_mouse_down(MouseButton::Left, move |_ev, window, cx| {
                (cb)(provider, window, cx);
            })
            .child(
                Icon::new(icon_view)
                    .size(px(14.0))
                    .text_color(if is_selected { theme.accent_brand } else { theme.text_muted }),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .child(
                        div()
                            .text_size(px(12.0))
                            .font_weight(if is_selected { gpui_kit::FontWeight::SEMIBOLD } else { gpui_kit::FontWeight::NORMAL })
                            .text_color(text_color)
                            .child(provider.to_string()),
                    )
                    .when(provider == LlmProvider::Gemini, |this| {
                        this.child(
                            div()
                                .text_size(px(9.0))
                                .text_color(theme.accent_brand)
                                .child("★ Recommended"),
                        )
                    }),
            );

        if idx < 3 {
            row1_buttons.push(btn);
        } else {
            row2_buttons.push(btn);
        }
    }

    let presets = active_provider.preset_models();
    let preset_chips: Vec<_> = presets
        .iter()
        .map(|&p| {
            let cb = on_preset.clone();
            let is_current = current_settings.model == p;
            let chip_bg = if is_current {
                theme.bg_tab_active
            } else {
                theme.hover_bg
            };
            let chip_border = if is_current {
                theme.text_primary
            } else {
                theme.border
            };
            let chip_hover = theme.hover_bg;

            div()
                .id(format!("preset-{}", p))
                .px_2()
                .py_1()
                .rounded_md()
                .border_1()
                .border_color(chip_border)
                .bg(chip_bg)
                .hover(move |s| s.bg(chip_hover))
                .cursor_pointer()
                .text_size(px(11.0))
                .text_color(if is_current { theme.text_primary } else { theme.text_secondary })
                .on_mouse_down(MouseButton::Left, move |_ev, window, cx| {
                    (cb)(p, window, cx);
                })
                .child(p)
        })
        .collect();

    div()
        .flex()
        .flex_col()
        .gap_4()
        // 1. Provider Grid Header & Selection
        .child(
            div()
                .flex()
                .flex_col()
                .gap_2()
                .child(
                    div()
                        .flex()
                        .items_center()
                        .justify_between()
                        .child(
                            div()
                                .text_size(px(12.0))
                                .font_weight(gpui_kit::FontWeight::BOLD)
                                .text_color(theme.text_muted)
                                .child("SELECT LLM PROVIDER"),
                        )
                        .child(
                            div()
                                .text_size(px(11.0))
                                .text_color(theme.accent_brand)
                                .child(format!("Active: {}", active_provider)),
                        ),
                )
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap_2()
                        .child(div().flex().gap_2().children(row1_buttons))
                        .child(div().flex().gap_2().children(row2_buttons)),
                ),
        )
        // 2. API Key Configuration
        .child(
            div()
                .flex()
                .flex_col()
                .gap_1p5()
                .child(
                    div()
                        .flex()
                        .items_center()
                        .justify_between()
                        .child(
                            div()
                                .text_size(px(12.0))
                                .font_weight(gpui_kit::FontWeight::MEDIUM)
                                .text_color(theme.text_primary)
                                .child(format!("{} API Key", active_provider)),
                        )
                        .child(
                            div()
                                .text_size(px(10.0))
                                .text_color(theme.text_muted)
                                .child("Persisted in local SQLite vault"),
                        ),
                )
                .child(
                    Input::new(api_key_input)
                        .w_full()
                        .h(px(34.0))
                        .text_size(px(12.0)),
                ),
        )
        // 3. Custom Endpoint (if applicable)
        .when(is_custom, |this| {
            this.child(
                div()
                    .flex()
                    .flex_col()
                    .gap_1p5()
                    .child(
                        div()
                            .text_size(px(12.0))
                            .font_weight(gpui_kit::FontWeight::MEDIUM)
                            .text_color(theme.text_primary)
                            .child("Custom Endpoint URL"),
                    )
                    .child(
                        Input::new(endpoint_input)
                            .w_full()
                            .h(px(34.0))
                            .text_size(px(12.0)),
                    ),
            )
        })
        // 4. Model Identifier & Quick Presets
        .child(
            div()
                .flex()
                .flex_col()
                .gap_1p5()
                .child(
                    div()
                        .flex()
                        .items_center()
                        .justify_between()
                        .child(
                            div()
                                .text_size(px(12.0))
                                .font_weight(gpui_kit::FontWeight::MEDIUM)
                                .text_color(theme.text_primary)
                                .child("Model Identifier"),
                        )
                        .child(
                            div()
                                .text_size(px(10.0))
                                .text_color(theme.text_muted)
                                .child("Click preset to select"),
                        ),
                )
                .child(
                    Input::new(model_input)
                        .w_full()
                        .h(px(34.0))
                        .text_size(px(12.0)),
                )
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap_2()
                        .mt_1()
                        .child(
                            div()
                                .text_size(px(11.0))
                                .text_color(theme.text_muted)
                                .child("Presets:"),
                        )
                        .child(div().flex().flex_wrap().gap_1p5().children(preset_chips)),
                ),
        )
        // 5. Epistemic Status Banner
        .child(
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
                        .child(
                            Icon::new(LucideIcon::Users)
                                .size(theme.font_size_sm)
                                .text_color(theme.accent_brand),
                        )
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
                        .font_weight(gpui_kit::FontWeight::SEMIBOLD)
                        .text_color(theme.accent_brand)
                        .child("Ready"),
                ),
        )
}

/// Helper function to render Appearance Tab Pane
fn render_appearance_pane(
    theme: &ThemeTokens,
    on_select_theme: SelectThemeCallback,
) -> impl IntoElement {
    let cb_theme = Rc::new(on_select_theme);
    let cb_oled = cb_theme.clone();
    let cb_zinc = cb_theme.clone();
    let cb_slate = cb_theme.clone();

    let oled_preset = crate::theme::minimalist_dark();
    let zinc_preset = crate::theme::obsidian_zinc();
    let slate_preset = crate::theme::midnight_slate();
    let is_oled = theme.bg_app == oled_preset.bg_app;
    let is_zinc = theme.bg_app == zinc_preset.bg_app;
    let is_slate = theme.bg_app == slate_preset.bg_app;
    let theme_hover_bg = theme.hover_bg;

    div()
        .flex()
        .flex_col()
        .gap_5()
        // Theme Presets
        .child(
            div()
                .flex()
                .flex_col()
                .gap_2()
                .child(
                    div()
                        .text_size(px(12.0))
                        .font_weight(gpui_kit::FontWeight::BOLD)
                        .text_color(theme.text_muted)
                        .child("THEME PALETTES"),
                )
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap_2()
                        // 1. True Threads OLED Black
                        .child(
                            div()
                                .p_3p5()
                                .rounded_lg()
                                .bg(if is_oled { oled_preset.bg_app } else { theme.bg_card })
                                .border_1()
                                .border_color(if is_oled { theme.accent_brand } else { theme.border })
                                .cursor_pointer()
                                .hover(move |s| s.bg(theme_hover_bg))
                                .on_mouse_down(MouseButton::Left, move |_ev, window, cx| {
                                    (cb_oled)(crate::theme::minimalist_dark(), window, cx);
                                })
                                .flex()
                                .items_center()
                                .justify_between()
                                .child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .gap_3()
                                        .child(
                                            div()
                                                .w(px(20.0))
                                                .h(px(20.0))
                                                .rounded_full()
                                                .bg(oled_preset.bg_app)
                                                .border_1()
                                                .border_color(oled_preset.border),
                                        )
                                        .child(
                                            div()
                                                .flex()
                                                .flex_col()
                                                .gap_0p5()
                                                .child(
                                                    div()
                                                        .text_size(px(13.0))
                                                        .font_weight(gpui_kit::FontWeight::SEMIBOLD)
                                                        .text_color(theme.text_primary)
                                                        .child("True Threads OLED Black (Default)"),
                                                )
                                                .child(
                                                    div()
                                                        .text_size(px(11.0))
                                                        .text_color(theme.text_muted)
                                                        .child("Deep #000000 background, hairline neutral borders, zero bleed-through"),
                                                ),
                                        ),
                                )
                                .child(
                                    div()
                                        .px_2()
                                        .py_0p5()
                                        .rounded_full()
                                        .bg(if is_oled { theme.accent_brand_subtle } else { theme.bg_transparent })
                                        .text_size(px(10.0))
                                        .font_weight(gpui_kit::FontWeight::SEMIBOLD)
                                        .text_color(if is_oled { theme.accent_brand } else { theme.text_muted })
                                        .child(if is_oled { "Selected" } else { "Available" }),
                                ),
                        )
                        // 2. Obsidian Deep Zinc
                        .child(
                            div()
                                .p_3p5()
                                .rounded_lg()
                                .bg(if is_zinc { zinc_preset.bg_elevated } else { theme.bg_card })
                                .border_1()
                                .border_color(if is_zinc { theme.accent_brand } else { theme.border })
                                .cursor_pointer()
                                .hover(move |s| s.bg(theme_hover_bg))
                                .on_mouse_down(MouseButton::Left, move |_ev, window, cx| {
                                    (cb_zinc)(crate::theme::obsidian_zinc(), window, cx);
                                })
                                .flex()
                                .items_center()
                                .justify_between()
                                .child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .gap_3()
                                        .child(
                                            div()
                                                .w(px(20.0))
                                                .h(px(20.0))
                                                .rounded_full()
                                                .bg(zinc_preset.bg_app)
                                                .border_1()
                                                .border_color(zinc_preset.border),
                                        )
                                        .child(
                                            div()
                                                .flex()
                                                .flex_col()
                                                .gap_0p5()
                                                .child(
                                                    div()
                                                        .text_size(px(13.0))
                                                        .font_weight(gpui_kit::FontWeight::MEDIUM)
                                                        .text_color(theme.text_primary)
                                                        .child("Obsidian Deep Zinc"),
                                                )
                                                .child(
                                                    div()
                                                        .text_size(px(11.0))
                                                        .text_color(theme.text_muted)
                                                        .child("Subtle zinc warmth with soft contrast for long dialectic sessions"),
                                                ),
                                        ),
                                )
                                .child(
                                    div()
                                        .px_2()
                                        .py_0p5()
                                        .rounded_full()
                                        .bg(if is_zinc { theme.accent_brand_subtle } else { theme.bg_transparent })
                                        .text_size(px(10.0))
                                        .font_weight(gpui_kit::FontWeight::SEMIBOLD)
                                        .text_color(if is_zinc { theme.accent_brand } else { theme.text_muted })
                                        .child(if is_zinc { "Selected" } else { "Available" }),
                                ),
                        )
                        // 3. Midnight Slate
                        .child(
                            div()
                                .p_3p5()
                                .rounded_lg()
                                .bg(if is_slate { slate_preset.bg_elevated } else { theme.bg_card })
                                .border_1()
                                .border_color(if is_slate { theme.accent_brand } else { theme.border })
                                .cursor_pointer()
                                .hover(move |s| s.bg(theme_hover_bg))
                                .on_mouse_down(MouseButton::Left, move |_ev, window, cx| {
                                    (cb_slate)(crate::theme::midnight_slate(), window, cx);
                                })
                                .flex()
                                .items_center()
                                .justify_between()
                                .child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .gap_3()
                                        .child(
                                            div()
                                                .w(px(20.0))
                                                .h(px(20.0))
                                                .rounded_full()
                                                .bg(slate_preset.bg_app)
                                                .border_1()
                                                .border_color(slate_preset.border),
                                        )
                                        .child(
                                            div()
                                                .flex()
                                                .flex_col()
                                                .gap_0p5()
                                                .child(
                                                    div()
                                                        .text_size(px(13.0))
                                                        .font_weight(gpui_kit::FontWeight::MEDIUM)
                                                        .text_color(theme.text_primary)
                                                        .child("Midnight Slate"),
                                                )
                                                .child(
                                                    div()
                                                        .text_size(px(11.0))
                                                        .text_color(theme.text_muted)
                                                        .child("Cold deep navy tint with oceanic epistemic accents"),
                                                ),
                                        ),
                                )
                                .child(
                                    div()
                                        .px_2()
                                        .py_0p5()
                                        .rounded_full()
                                        .bg(if is_slate { theme.accent_brand_subtle } else { theme.bg_transparent })
                                        .text_size(px(10.0))
                                        .font_weight(gpui_kit::FontWeight::SEMIBOLD)
                                        .text_color(if is_slate { theme.accent_brand } else { theme.text_muted })
                                        .child(if is_slate { "Selected" } else { "Available" }),
                                ),
                        ),
                ),
        )
        // Layout & Density
        .child(
            div()
                .flex()
                .flex_col()
                .gap_2()
                .child(
                    div()
                        .text_size(px(12.0))
                        .font_weight(gpui_kit::FontWeight::BOLD)
                        .text_color(theme.text_muted)
                        .child("LAYOUT & FEED DENSITY"),
                )
                .child(
                    div()
                        .p_3p5()
                        .rounded_lg()
                        .bg(theme.bg_card)
                        .border_1()
                        .border_color(theme.border)
                        .flex()
                        .flex_col()
                        .gap_3()
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .justify_between()
                                .child(
                                    div()
                                        .text_size(px(12.0))
                                        .text_color(theme.text_primary)
                                        .child("Threads Continuous Threadlines"),
                                )
                                .child(
                                    div()
                                        .text_size(px(11.0))
                                        .font_weight(gpui_kit::FontWeight::SEMIBOLD)
                                        .text_color(theme.accent_success)
                                        .child("Enabled"),
                                ),
                        )
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .justify_between()
                                .child(
                                    div()
                                        .text_size(px(12.0))
                                        .text_color(theme.text_primary)
                                        .child("Human-Readable Wikilink Excerpts"),
                                )
                                .child(
                                    div()
                                        .text_size(px(11.0))
                                        .font_weight(gpui_kit::FontWeight::SEMIBOLD)
                                        .text_color(theme.accent_success)
                                        .child("Enabled"),
                                ),
                        )
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .justify_between()
                                .child(
                                    div()
                                        .text_size(px(12.0))
                                        .text_color(theme.text_primary)
                                        .child("Subpixel Text Rendering"),
                                )
                                .child(
                                    div()
                                        .text_size(px(11.0))
                                        .font_weight(gpui_kit::FontWeight::SEMIBOLD)
                                        .text_color(theme.accent_success)
                                        .child("Enabled"),
                                ),
                        ),
                ),
        )
}

/// Helper function to render Knowledge Vault Tab Pane
fn render_vault_pane(theme: &ThemeTokens) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .gap_5()
        // Vector RAG Engine
        .child(
            div()
                .flex()
                .flex_col()
                .gap_2()
                .child(
                    div()
                        .text_size(px(12.0))
                        .font_weight(gpui_kit::FontWeight::BOLD)
                        .text_color(theme.text_muted)
                        .child("DENSE VECTOR EMBEDDING ENGINE"),
                )
                .child(
                    div()
                        .p_3p5()
                        .rounded_lg()
                        .bg(theme.bg_card)
                        .border_1()
                        .border_color(theme.border)
                        .flex()
                        .flex_col()
                        .gap_3()
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .justify_between()
                                .child(
                                    div()
                                        .flex()
                                        .flex_col()
                                        .gap_0p5()
                                        .child(
                                            div()
                                                .text_size(px(12.0))
                                                .font_weight(gpui_kit::FontWeight::MEDIUM)
                                                .text_color(theme.text_primary)
                                                .child("Embedding Engine"),
                                        )
                                        .child(
                                            div()
                                                .text_size(px(11.0))
                                                .font_family(theme.font_family_mono.clone())
                                                .text_color(theme.text_secondary)
                                                .child("nano-vectordb (384-dimensional dense vectors)"),
                                        ),
                                )
                                .child(
                                    div()
                                        .px_2()
                                        .py_0p5()
                                        .rounded_full()
                                        .bg(theme.accent_success_subtle)
                                        .text_size(px(10.0))
                                        .font_weight(gpui_kit::FontWeight::SEMIBOLD)
                                        .text_color(theme.accent_success)
                                        .child("Optimized"),
                                ),
                        )
                        .child(
                            div()
                                .text_size(px(11.0))
                                .text_color(theme.text_muted)
                                .child("Embeddings run strictly on local memory for fast similarity matching without cloud latency."),
                        ),
                ),
        )
        // Semantic Resonance
        .child(
            div()
                .flex()
                .flex_col()
                .gap_2()
                .child(
                    div()
                        .text_size(px(12.0))
                        .font_weight(gpui_kit::FontWeight::BOLD)
                        .text_color(theme.text_muted)
                        .child("SEMANTIC RESONANCE & THRESHOLD"),
                )
                .child(
                    div()
                        .p_3p5()
                        .rounded_lg()
                        .bg(theme.bg_card)
                        .border_1()
                        .border_color(theme.border)
                        .flex()
                        .flex_col()
                        .gap_3()
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .justify_between()
                                .child(
                                    div()
                                        .text_size(px(12.0))
                                        .text_color(theme.text_primary)
                                        .child("Grounding Resonance Threshold"),
                                )
                                .child(
                                    div()
                                        .px_2()
                                        .py_0p5()
                                        .rounded_md()
                                        .bg(theme.accent_dialectic_subtle)
                                        .border_1()
                                        .border_color(theme.accent_dialectic_border)
                                        .text_size(px(11.0))
                                        .font_weight(gpui_kit::FontWeight::BOLD)
                                        .text_color(theme.accent_dialectic)
                                        .child("94% Strict Resonance"),
                                ),
                        )
                        .child(
                            div()
                                .text_size(px(11.0))
                                .text_color(theme.text_muted)
                                .child("Dialectic sparks are filtered to ensure high epistemic coherence against existing vault memories."),
                        ),
                ),
        )
}
