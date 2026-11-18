//! The allowed options of every choice parameter (stable keys), their label keys, and the
//! options the draft currently has.

use super::super::engine_param::EngineParam;
use super::super::param::Param;
use crate::agent::llm::LlmProvider;
use crate::domain::engine::{
    HttpMethod, ImageEngineKind, ImageSize, OutputKind, RequestKind, SchemaPreset, SdFlavor, SearchBackend, TextSource, ToolKind,
    TriggerKind,
};
use crate::domain::TraitOption;

/// Keys of a switch.
pub const ON: &str = "on";
pub const OFF: &str = "off";
const SWITCH_LABELS: [(&str, &str); 2] = [(ON, "forge.ui.on"), (OFF, "forge.ui.off")];
/// Kinds of avatar and their label keys.
pub const AVATAR_PORTRAIT: &str = "pcg";
pub const AVATAR_FORM: &str = "form";
const AVATAR_LABELS: [(&str, &str); 4] =
    [(AVATAR_PORTRAIT, "portrait.mode_pcg"), (AVATAR_FORM, "portrait.mode_form"), ("emblem", "portrait.mode_emblem"), ("photo", "portrait.mode_photo")];

fn keys_of<T: TraitOption>() -> Vec<&'static str> {
    T::ALL.iter().map(|option| option.key()).collect()
}

fn label_of<T: TraitOption>(key: &str) -> Option<String> {
    T::ALL.iter().find(|option| option.key() == key).map(|option| option.label_key())
}

/// `(keys, label lookup)` of an Engine choice, `None` for Engine settings that are not choices.
fn engine_options(param: EngineParam) -> Option<(Vec<&'static str>, fn(&str) -> Option<String>)> {
    use EngineParam as E;
    Some(match param {
        E::Source => (keys_of::<TextSource>(), label_of::<TextSource>),
        E::Provider => (keys_of::<LlmProvider>(), label_of::<LlmProvider>),
        E::Request => (keys_of::<RequestKind>(), label_of::<RequestKind>),
        E::Method => (keys_of::<HttpMethod>(), label_of::<HttpMethod>),
        E::Output => (keys_of::<OutputKind>(), label_of::<OutputKind>),
        E::SchemaPreset => (keys_of::<SchemaPreset>(), label_of::<SchemaPreset>),
        E::Image => (keys_of::<ImageEngineKind>(), label_of::<ImageEngineKind>),
        E::ImageSize => (keys_of::<ImageSize>(), label_of::<ImageSize>),
        E::SdFlavor => (keys_of::<SdFlavor>(), label_of::<SdFlavor>),
        E::Trigger => (keys_of::<TriggerKind>(), label_of::<TriggerKind>),
        E::Tools => (keys_of::<ToolKind>(), label_of::<ToolKind>),
        E::SearchBackend => (keys_of::<SearchBackend>(), label_of::<SearchBackend>),
        _ => return None,
    })
}

fn table_label(table: &[(&str, &str)], key: &str) -> Option<String> {
    table.iter().find(|(candidate, _)| *candidate == key).map(|(_, label)| label.to_string())
}

/// Every allowed option of `param` (empty when it is not a choice).
pub fn option_keys(param: Param) -> Vec<&'static str> {
    match param {
        Param::Trait(field) => field.option_keys(),
        Param::Set(field) => field.option_keys(),
        Param::Toggle(_) => vec![ON, OFF],
        Param::Avatar(_) => AVATAR_LABELS.iter().map(|(key, _)| *key).collect(),
        Param::Engine(engine) => engine_options(engine).map(|(keys, _)| keys).unwrap_or_default(),
        _ => Vec::new(),
    }
}

/// The label key of option `key` of `param`.
pub fn option_label_key(param: Param, key: &str) -> Option<String> {
    match param {
        Param::Trait(field) => field.parse(key).map(|value| value.label_key()),
        Param::Set(field) => field.parse(&[key]).0.label_keys().into_iter().next(),
        Param::Toggle(_) => table_label(&SWITCH_LABELS, key),
        Param::Avatar(_) => table_label(&AVATAR_LABELS, key),
        Param::Engine(engine) => engine_options(engine).and_then(|(_, label)| label(key)),
        _ => None,
    }
}

/// Whether `param` is a single choice or a multi-select.
pub fn is_choice(param: Param) -> bool {
    !option_keys(param).is_empty()
}
