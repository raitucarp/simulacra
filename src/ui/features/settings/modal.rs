//! Settings modal root element: state holder, backdrop and dialog frame.

use super::callbacks::{
    ModalCloseCallback, SaveSettingsCallback, SelectPresetModelCallback, SelectProviderCallback,
    SelectTabCallback,
};
use super::general_props::GeneralPaneProps;
use super::content::{render_backdrop, render_content_header, render_content_pane, render_footer_bar};
use super::credentials::{render_credentials_section, CredentialsPaneProps};
use super::panes::{render_ai_pane, render_general_pane, render_vault_pane, AppearancePanel};
use super::rail::render_rail;
use super::tabs::SettingsTab;
use crate::agent::{LlmProvider, LlmSettings};
use crate::theme::ThemeTokens;
use gpui_kit::{AnyElement, App, Entity, IntoElement, ParentElement, RenderOnce, Styled, Window};
use gpui_kit::component::input::InputState;
use std::rc::Rc;

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
    appearance: Entity<AppearancePanel>,
    on_select_provider: SelectProviderCallback,
    on_select_preset_model: SelectPresetModelCallback,
    on_save: SaveSettingsCallback,
    on_close: ModalCloseCallback,
    general: GeneralPaneProps,
    credentials: Option<CredentialsPaneProps>,
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
        appearance: Entity<AppearancePanel>,
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
            appearance,
            on_select_provider: Box::new(on_select_provider),
            on_select_preset_model: Box::new(on_select_preset_model),
            on_save: Box::new(on_save),
            on_close: Box::new(on_close),
            general: GeneralPaneProps::default(),
            credentials: None,
        }
    }

    /// Supplies the General pane's live scheduler status, vault path and switch callback.
    pub fn general(mut self, general: GeneralPaneProps) -> Self {
        self.general = general;
        self
    }

    /// Supplies the AI pane's credential list, editor and callbacks.
    pub fn credentials(mut self, credentials: CredentialsPaneProps) -> Self {
        self.credentials = Some(credentials);
        self
    }

    /// The AI pane followed by the credentials section.
    fn ai_content(theme: &ThemeTokens, ai: impl IntoElement, credentials: Option<CredentialsPaneProps>) -> AnyElement {
        let column = gpui_kit::div().flex().flex_col().gap(theme.spacing_xl).child(ai);
        match credentials {
            Some(props) => column.child(render_credentials_section(theme, props)).into_any_element(),
            None => column.into_any_element(),
        }
    }

    /// Dispatches tab content to individual pane helpers to prevent stack frame exhaustion.
    fn tab_content(
        theme: &ThemeTokens,
        active_tab: SettingsTab,
        settings: &LlmSettings,
        inputs: [&Entity<InputState>; 3],
        callbacks: (SelectProviderCallback, SelectPresetModelCallback),
        appearance: Entity<AppearancePanel>,
        (general, credentials): (GeneralPaneProps, Option<CredentialsPaneProps>),
    ) -> AnyElement {
        let [api_key_input, model_input, endpoint_input] = inputs;
        let (on_provider, on_preset) = callbacks;
        match active_tab {
            SettingsTab::General => render_general_pane(theme, general).into_any_element(),
            SettingsTab::AiIntelligence => {
                let ai = render_ai_pane(theme, settings, api_key_input, model_input, endpoint_input, on_provider, on_preset);
                Self::ai_content(theme, ai, credentials)
            }
            SettingsTab::Appearance => appearance.into_any_element(),
            SettingsTab::KnowledgeVault => render_vault_pane(theme).into_any_element(),
        }
    }
}

impl RenderOnce for SettingsModal {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let on_close = Rc::new(self.on_close);
        let theme = &self.theme;
        let rail = render_rail(theme, self.active_tab, Rc::new(self.on_select_tab));
        let header = render_content_header(theme, self.active_tab, on_close.clone());
        let footer = render_footer_bar(theme, on_close.clone(), Rc::new(self.on_save));
        let tab_content = Self::tab_content(
            theme,
            self.active_tab,
            &self.current_settings,
            [&self.api_key_input, &self.model_input, &self.endpoint_input],
            (self.on_select_provider, self.on_select_preset_model),
            self.appearance,
            (self.general, self.credentials),
        );
        let content = render_content_pane(header, tab_content, footer, theme);
        render_backdrop(theme, on_close, rail, content)
    }
}
