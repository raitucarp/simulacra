//! Mind source: the global model from Settings › AI, or the agent's own provider, model,
//! sampling, endpoint and key.

use super::super::super::super::common::{card, muted};
use super::credential::readiness_badge;
use super::kit::{engine_id, option_line};
use crate::agent::credentials::CredentialKind;
use crate::agent::llm::LlmProvider;
use crate::domain::engine::{OwnModel, TextModel, TextSource};
use crate::domain::EngineConfig;
use crate::state::forge::engine::{mutate, CredentialStatus};
use crate::state::forge::EngineSection;
use crate::ui::features::forge::text::{tr, tr_with};
use crate::ui::features::forge::view::{ForgeHandle, ForgeView};
use crate::ui::features::forge::widgets::NumberSlider;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::Sizable;
use gpui_kit::*;

const GLOBAL_KEY: &str = "forge.ui.engine.global_hint";
const GLOBAL_STATUS_KEY: &str = "forge.ui.engine.global_status";
const TEMPERATURE_KEY: &str = "forge.ui.engine.temperature";
const MAX_TOKENS_KEY: &str = "forge.ui.engine.max_tokens";
const PRESETS_KEY: &str = "forge.ui.engine.model_presets";
const ENDPOINT_HINT_KEY: &str = "forge.ui.engine.endpoint_hint";
const ARG_PROVIDER: &str = "provider";
const ARG_MODEL: &str = "model";
/// Temperature slider in hundredths: (min, max, step).
const TEMPERATURE_RANGE: (u16, u16, u16) = (0, 200, 5);
const TEMPERATURE_SCALE: f32 = 100.0;
/// Max-tokens slider: (min, max, step).
const MAX_TOKENS_RANGE: (u16, u16, u16) = (64, 8192, 64);
const SECTION: EngineSection = EngineSection::Model;

/// Mutable access to the own model, when the agent has one.
fn own(engine: &mut EngineConfig) -> Option<&mut OwnModel> {
    match &mut engine.text {
        TextModel::Own(own) => Some(own),
        TextModel::UseGlobal => None,
    }
}

impl ForgeView {
    pub(super) fn model_section(&self, handle: &ForgeHandle) -> Div {
        let theme = &self.theme;
        let source = TextSource::of(&self.engine().text);
        let grid = self.engine_cards("source", (Some(source), &TextSource::ALL), handle, mutate::set_text_source);
        let head = card(theme).child(self.engine_field("engine_source", grid)).children(option_line(source, theme));
        let body = match &self.engine().text {
            TextModel::UseGlobal => self.global_card(),
            TextModel::Own(own) => self.own_model_card(own, handle),
        };
        div().flex().flex_col().gap(theme.spacing_lg).child(head).child(body)
    }

    /// What "use the global model" means right now.
    fn global_card(&self) -> Div {
        let theme = &self.theme;
        let llm = &self.context.llm;
        let args = [(ARG_PROVIDER, llm.provider.to_string()), (ARG_MODEL, llm.model.clone())];
        let status = if llm.has_api_key() { CredentialStatus::NotNeeded } else { CredentialStatus::Missing };
        let badge = match status {
            CredentialStatus::Missing => readiness_badge(&status, &llm.provider.to_string(), theme),
            _ => muted(tr_with(GLOBAL_STATUS_KEY, &args), theme),
        };
        card(theme).child(muted(tr(GLOBAL_KEY), theme)).child(badge)
    }

    fn own_model_card(&self, model: &OwnModel, handle: &ForgeHandle) -> Div {
        let theme = &self.theme;
        let providers = self.engine_cards("provider", (Some(model.provider), &LlmProvider::ALL), handle, mutate::set_provider).compact();
        let model_input = self.engine_input("model", &model.model, (SECTION, handle), |e, text| {
            if let Some(own) = own(e) {
                own.model = text;
            }
        });
        let endpoint = self.engine_input("endpoint", &model.endpoint, (SECTION, handle), |e, text| {
            if let Some(own) = own(e) {
                own.endpoint = text;
            }
        });
        let kind = CredentialKind::from_provider(model.provider);
        let key = self.credential_picker("model", (model.credential.as_deref(), Some(kind)), handle, |e, id| {
            if let Some(own) = own(e) {
                own.credential = id;
            }
        });
        card(theme)
            .gap(theme.spacing_lg)
            .child(self.engine_field("engine_provider", providers))
            .child(self.engine_field("engine_model", div().flex().flex_col().gap(theme.spacing_xs).child(model_input).child(self.model_presets(model, handle))))
            .child(self.engine_field("engine_sampling", self.sampling(model, handle)))
            .child(self.engine_field("engine_endpoint", endpoint.placeholder(tr(ENDPOINT_HINT_KEY))))
            .child(self.engine_field("engine_credential", key))
    }

    /// One chip per suggested model of the provider.
    fn model_presets(&self, model: &OwnModel, handle: &ForgeHandle) -> Div {
        let theme = &self.theme;
        let chips = model.provider.preset_models().iter().enumerate().map(|(at, &preset)| {
            let pick = handle.engine(move |e: &mut EngineConfig, ()| {
                if let Some(own) = own(e) {
                    own.model = preset.to_string();
                }
            });
            let chip = Button::new(engine_id(format!("model-preset-{at}"))).xsmall().label(preset);
            let chip = if model.model.trim() == preset { chip.primary() } else { chip.outline() };
            chip.on_click(move |_, w, cx| pick((), w, cx))
        });
        let hint = div().text_size(theme.font_size_2xs).text_color(theme.text_muted).child(tr(PRESETS_KEY));
        div().flex().flex_col().gap(theme.spacing_xs).child(hint).child(div().flex().flex_wrap().gap(theme.spacing_xs).children(chips))
    }

    fn sampling(&self, model: &OwnModel, handle: &ForgeHandle) -> Div {
        let theme = &self.theme;
        let temperature = (model.temperature * TEMPERATURE_SCALE).round() as u16;
        let set_temperature = handle.engine(|e: &mut EngineConfig, value: u16| {
            if let Some(own) = own(e) {
                own.temperature = f32::from(value) / TEMPERATURE_SCALE;
            }
        });
        let set_tokens = handle.engine(|e: &mut EngineConfig, value: u16| {
            if let Some(own) = own(e) {
                own.max_tokens = u32::from(value);
            }
        });
        let tokens = model.max_tokens.min(u32::from(MAX_TOKENS_RANGE.1)) as u16;
        let temperature = NumberSlider::new(engine_id("temperature"), tr(TEMPERATURE_KEY), temperature, TEMPERATURE_RANGE, theme, move |v, w, cx| set_temperature(v, w, cx))
            .format(|value| format!("{:.2}", f32::from(value) / TEMPERATURE_SCALE));
        let tokens = NumberSlider::new(engine_id("max-tokens"), tr(MAX_TOKENS_KEY), tokens, MAX_TOKENS_RANGE, theme, move |v, w, cx| set_tokens(v, w, cx));
        div().flex().flex_col().gap(theme.spacing_md).child(temperature).child(tokens)
    }
}
