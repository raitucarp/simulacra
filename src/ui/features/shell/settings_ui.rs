//! Settings window state owned by the root view: the search box, the text inputs of the
//! text preferences, the rail focus and the pending "reset all" confirmation.

use super::view::AppView;
use crate::domain::preferences::{EmbeddingField, ImageField, Preferences, SettingField, SettingValue, DEFAULT_EMBEDDING_MODEL, DEFAULT_LOCAL_ENDPOINT};
use crate::ui::features::settings::SettingsInputs;
use gpui_kit::component::input::{InputEvent, InputState};
use gpui_kit::*;
use rust_i18n::t;

/// Inputs and transient flags of the settings window.
pub struct SettingsUi {
    pub search: Entity<InputState>,
    pub embedding_model: Entity<InputState>,
    pub embedding_endpoint: Entity<InputState>,
    pub style_prefix: Entity<InputState>,
    pub rail_focus: FocusHandle,
    /// "Reset all settings" was clicked once.
    pub confirm_reset: bool,
}

impl SettingsUi {
    /// The inputs, filled from `prefs`, and the subscriptions that apply their edits.
    pub(super) fn new(prefs: &Preferences, window: &mut Window, cx: &mut Context<AppView>) -> (Self, Vec<Subscription>) {
        let mut input = |value: &str, placeholder: String| {
            let value = value.to_string();
            cx.new(|cx| InputState::new(window, cx).default_value(value).placeholder(placeholder))
        };
        let search = input("", t!("prefs.ui.search_placeholder").to_string());
        let embedding_model = input(&prefs.embeddings.model, DEFAULT_EMBEDDING_MODEL.to_string());
        let embedding_endpoint = input(&prefs.embeddings.endpoint, DEFAULT_LOCAL_ENDPOINT.to_string());
        let style_prefix = input(&prefs.images.style_prefix, t!("prefs.ui.style_prefix_placeholder").to_string());
        let subscriptions = vec![
            cx.subscribe_in(&search, window, |_, _, _: &InputEvent, _, cx| cx.notify()),
            Self::bind_text(cx, window, &embedding_model, SettingField::Embeddings(EmbeddingField::Model)),
            Self::bind_text(cx, window, &embedding_endpoint, SettingField::Embeddings(EmbeddingField::Endpoint)),
            Self::bind_text(cx, window, &style_prefix, SettingField::Images(ImageField::StylePrefix)),
        ];
        let ui = Self { search, embedding_model, embedding_endpoint, style_prefix, rail_focus: cx.focus_handle(), confirm_reset: false };
        (ui, subscriptions)
    }

    /// Applies every edit of `input` to `field`.
    fn bind_text(cx: &mut Context<AppView>, window: &mut Window, input: &Entity<InputState>, field: SettingField) -> Subscription {
        cx.subscribe_in(input, window, move |view, input, event: &InputEvent, window, cx| {
            if matches!(event, InputEvent::Change) {
                let value = input.read(cx).value().to_string();
                view.edit_preference(field, SettingValue::Text(value), window, cx);
            }
        })
    }

    /// The handles the settings window renders.
    pub(super) fn inputs(&self) -> SettingsInputs {
        SettingsInputs {
            search: self.search.clone(),
            embedding_model: self.embedding_model.clone(),
            embedding_endpoint: self.embedding_endpoint.clone(),
            style_prefix: self.style_prefix.clone(),
            rail_focus: self.rail_focus.clone(),
        }
    }

    /// The search box text.
    pub(super) fn query(&self, cx: &App) -> String {
        self.search.read(cx).value().to_string()
    }

    /// Refills the text inputs from `prefs` (after a reset), leaving matching inputs alone.
    pub(super) fn sync_inputs(&self, prefs: &Preferences, window: &mut Window, cx: &mut App) {
        let fields = [
            (&self.embedding_model, prefs.embeddings.model.clone()),
            (&self.embedding_endpoint, prefs.embeddings.endpoint.clone()),
            (&self.style_prefix, prefs.images.style_prefix.clone()),
        ];
        for (input, value) in fields {
            if input.read(cx).value().trim() != value {
                input.update(cx, |state, cx| state.set_value(value, window, cx));
            }
        }
    }

    /// Empties the search box.
    pub(super) fn clear_search(&self, window: &mut Window, cx: &mut App) {
        self.search.update(cx, |state, cx| state.set_value("", window, cx));
    }
}
