//! The draft edit that accepting a suggested value makes.

use super::value::SuggestedValue;
use super::super::choice::ON;
use super::super::engine_param::EngineParam;
use super::super::model::HUNDREDTHS;
use super::super::param::Param;
use crate::domain::engine::{
    EngineConfig, ImageSize, ImageTrigger, OutputKind, SchemaPreset, TextModel, ToolKind, TriggerKind, DEFAULT_IMAGE_CHANCE,
};
use crate::domain::traits::OptionSet;
use crate::domain::{Profile, TraitOption};
use crate::state::forge::engine::mutate::{apply_preset, set_output_kind, set_tools, set_trigger};
use crate::state::forge::DraftEdit;

fn find<T: TraitOption>(key: &str) -> Option<T> {
    T::ALL.iter().copied().find(|option| option.key() == key)
}

/// `engine` with `param` set to `value`; `None` when the value does not fit the setting.
fn engine_with(param: EngineParam, value: &SuggestedValue, mut engine: EngineConfig) -> Option<EngineConfig> {
    use EngineParam as E;
    use SuggestedValue as V;
    match (param, value) {
        (E::Temperature, V::Number(hundredths)) => match &mut engine.text {
            TextModel::Own(own) => own.temperature = f32::from(*hundredths) / HUNDREDTHS,
            TextModel::UseGlobal => return None,
        },
        (E::MaxTokens, V::Number(tokens)) => match &mut engine.text {
            TextModel::Own(own) => own.max_tokens = u32::from(*tokens),
            TextModel::UseGlobal => return None,
        },
        (E::Output, V::Option(key)) => set_output_kind(&mut engine, find::<OutputKind>(key)?),
        (E::SchemaPreset, V::Option(key)) => apply_preset(&mut engine, find::<SchemaPreset>(key)?),
        (E::ImageSize, V::Option(key)) => engine.image.size = find::<ImageSize>(key)?.dimensions().to_string(),
        (E::StylePrefix, V::Text(text)) => engine.image.style_prefix = text.clone(),
        (E::Trigger, V::Option(key)) => set_trigger(&mut engine, find::<TriggerKind>(key)?),
        (E::ImageChance, V::Number(percent)) => {
            engine.image.trigger = ImageTrigger::Chance { percent: u8::try_from(*percent).unwrap_or(DEFAULT_IMAGE_CHANCE) }
        }
        (E::Tools, V::Options(keys)) => set_tools(&mut engine, &keys.iter().filter_map(|key| find::<ToolKind>(key)).collect::<OptionSet<_>>()),
        (E::MaxToolSteps, V::Number(steps)) => engine.max_tool_steps = u8::try_from(*steps).ok()?,
        _ => return None,
    }
    Some(engine)
}

/// The edit that sets `param` to `value` on `profile` (`None` when they do not match).
pub fn value_edit(param: Param, value: &SuggestedValue, profile: &Profile) -> Option<DraftEdit> {
    use SuggestedValue as V;
    match (param, value) {
        (Param::Text(field), V::Text(text)) => Some(DraftEdit::Text(field, text.clone())),
        (Param::Score(field), V::Number(score)) => Some(DraftEdit::Score(field, u8::try_from(*score).ok()?)),
        (Param::Hour(field), V::Number(hour)) => Some(DraftEdit::Hour(field, u8::try_from(*hour).ok()?)),
        (Param::Count(field), V::Number(count)) => Some(DraftEdit::Count(field, *count)),
        (Param::Toggle(field), V::Option(key)) => Some(DraftEdit::Toggle(field, *key == ON)),
        (Param::Trait(field), V::Option(key)) => field.parse(key).map(DraftEdit::Trait),
        (Param::Set(field), V::Options(keys)) => Some(DraftEdit::TraitSet(field.parse(keys).0)),
        (Param::Engine(engine), _) => {
            engine_with(engine, value, profile.config.engine.clone()).map(|engine| DraftEdit::Engine(Box::new(engine), None))
        }
        _ => None,
    }
}
