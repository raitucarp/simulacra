//! "Fill the rest": the edits that give every blank text (and empty multi-select) of a step its
//! first suggestion and every setting still at its starting value the suggested one. Deterministic and pure;
//! the forge applies the result as one undo step.

use super::choice::ON;
use super::context::SuggestContext;
use super::model::{FieldSuggestions, Phrase};
use super::param::Param;
use super::{range_hint, suggestions_for, EngineParam};
use crate::agent::llm::{DEFAULT_MAX_TOKENS, DEFAULT_TEMPERATURE};
use crate::domain::agent_config::AgentConfig;
use crate::domain::engine::{EngineConfig, TextModel};
use crate::domain::Profile;
use crate::state::forge::attributes::{is_balanced_build, BUDGET_FIELDS};
use crate::state::forge::{suggest_handle, DraftEdit, ForgeStep, TextField};

/// Hundredths per temperature unit.
const TEMPERATURE_SCALE: f32 = 100.0;

/// The first example of text `field`, when the field is blank.
fn text_fill(field: TextField, ctx: &SuggestContext, existing: &[Profile], resolve: &dyn Fn(&Phrase) -> String) -> Option<DraftEdit> {
    if !field.get(ctx.profile).trim().is_empty() {
        return None;
    }
    let value = match (field, suggestions_for(Param::Text(field), ctx)) {
        (TextField::Handle, _) if !ctx.profile.name.trim().is_empty() => suggest_handle(&ctx.profile.name, &ctx.profile.id, existing),
        (_, FieldSuggestions::Text(hints)) => resolve(&hints.examples.first()?.value),
        _ => return None,
    };
    Some(DraftEdit::Text(field, value))
}

/// The suggested value of a setting that still has its starting value (`start` is the draft
/// with a fresh config) and sits outside the suggestions: a number outside its typical band,
/// an option that is not one of the picks.
fn setting_fill(param: Param, ctx: &SuggestContext, start: &Profile) -> Option<DraftEdit> {
    let (now, fresh) = (ctx.profile, start);
    let untouched = super::choice::current_keys(param, now) == super::choice::current_keys(param, fresh);
    let number = |value: u16| range_hint(param, ctx).filter(|hint| value < hint.typical.0 || value > hint.typical.1).map(|hint| hint.recommended);
    let picks = super::picks_for(param, ctx);
    let pick = || picks.first().copied().filter(|_| untouched && !super::choice::current_keys(param, now).iter().any(|key| picks.contains(key)));
    match param {
        Param::Score(field) if field.get(now) == field.get(fresh) => Some(DraftEdit::Score(field, u8::try_from(number(u16::from(*field.get(now)))?).ok()?)),
        Param::Hour(field) if field.get(now) == field.get(fresh) => Some(DraftEdit::Hour(field, u8::try_from(number(u16::from(*field.get(now)))?).ok()?)),
        Param::Count(field) if field.get(now) == field.get(fresh) => Some(DraftEdit::Count(field, number(*field.get(now))?)),
        Param::Toggle(field) => Some(DraftEdit::Toggle(field, pick()? == ON)),
        Param::Trait(field) => field.parse(pick()?).map(DraftEdit::Trait),
        Param::Set(field) if field.read(&now.config).keys().is_empty() => Some(DraftEdit::TraitSet(field.parse(&picks).0)),
        _ => None,
    }
}

/// The engine with a blank model named, untouched sampling at the suggested values and a
/// blank image size or style set.
fn engine_fill(ctx: &SuggestContext, resolve: &dyn Fn(&Phrase) -> String) -> Option<DraftEdit> {
    let mut engine: EngineConfig = ctx.config().engine.clone();
    let temperature = range_hint(Param::Engine(EngineParam::Temperature), ctx).map(|hint| f32::from(hint.recommended) / TEMPERATURE_SCALE);
    let tokens = range_hint(Param::Engine(EngineParam::MaxTokens), ctx).map(|hint| u32::from(hint.recommended));
    if let TextModel::Own(own) = &mut engine.text {
        if own.model.trim().is_empty() {
            own.model = own.provider.default_model().to_string();
        }
        if own.temperature == DEFAULT_TEMPERATURE {
            own.temperature = temperature.unwrap_or(own.temperature);
        }
        if own.max_tokens == DEFAULT_MAX_TOKENS {
            own.max_tokens = tokens.unwrap_or(own.max_tokens);
        }
    }
    if engine.image.is_enabled() && engine.image.style_prefix.trim().is_empty() {
        if let FieldSuggestions::Text(hints) = suggestions_for(Param::Engine(EngineParam::StylePrefix), ctx) {
            engine.image.style_prefix = hints.examples.first().map(|example| resolve(&example.value)).unwrap_or_default();
        }
    }
    if engine.image.size.trim().is_empty() {
        engine.image.size = EngineConfig::default().image.size;
    }
    (engine != ctx.config().engine).then(|| DraftEdit::Engine(Box::new(engine), None))
}

/// The edits that fill `step`: blank texts get their first suggestion (a handle from the name),
/// settings still at their starting value get the suggested one. Personality scores are left
/// alone when filling them would break the balanced-build budget.
pub fn fill_edits(step: ForgeStep, ctx: &SuggestContext, existing: &[Profile], resolve: &dyn Fn(&Phrase) -> String) -> Vec<DraftEdit> {
    let start = Profile { config: AgentConfig::default(), ..ctx.profile.clone() };
    let fill_one = |param: Param, now: &SuggestContext| match param {
        Param::Text(field) => text_fill(field, now, existing, resolve),
        Param::Engine(_) => None,
        other => setting_fill(other, now, &start),
    };
    // Each fill sees the ones before it (a handle follows the name just filled).
    let (filled, mut edits) = Param::for_step(step).into_iter().fold((ctx.profile.clone(), Vec::new()), |(mut working, mut edits), param| {
        let edit = fill_one(param, &SuggestContext { profile: &working, ..*ctx });
        if let Some(edit) = edit.filter(|edit| edit.apply(&mut working)) {
            edits.push(edit);
        }
        (working, edits)
    });
    edits.extend((step == ForgeStep::Engine).then(|| engine_fill(ctx, resolve)).flatten());
    if !is_balanced_build(&filled.config) {
        edits.retain(|edit| !matches!(edit, DraftEdit::Score(field, _) if BUDGET_FIELDS.contains(field)));
    }
    edits
}
