//! Applying a palette to a draft (one undo step) and previewing what that would change.

use super::change::{describe, FieldChange};
use super::draft::AgentDraft;
use super::fields::field_step;
use super::step::ForgeStep;
use crate::domain::palette::Palette;

/// The step a step palette belongs to: its step key, or the step owning the field of that name
/// (the `engine` palettes follow wherever the engine field is edited).
pub fn palette_step(palette: &Palette) -> Option<ForgeStep> {
    let key = palette.step_key()?;
    ForgeStep::from_key(key).or_else(|| field_step(key))
}

/// The step palettes offered on `step`, in strip order.
pub fn step_palettes(step: ForgeStep) -> Vec<&'static Palette> {
    Palette::all().iter().filter(|palette| palette_step(palette) == Some(step)).collect()
}

/// Every field `palette` would change on the draft, grouped by step in walking order.
pub fn preview_palette(draft: &AgentDraft, palette: &Palette) -> Vec<FieldChange> {
    let current = &draft.profile().config;
    let target = palette.applied(current);
    let mut changes: Vec<FieldChange> =
        palette.changes_for(current).into_iter().flat_map(|change| describe(change, current, &target)).collect();
    changes.sort_by_key(|change| change.step);
    changes
}

/// Applies `palette` to the draft as one undo step; returns whether anything changed. A full
/// palette also records itself as the draft's palette.
pub fn apply_palette(draft: &mut AgentDraft, palette: &Palette) -> bool {
    draft.transform(|profile| palette.apply_to(&mut profile.config))
}

/// The full palette the draft started from: its palette, else its class's.
pub fn draft_palette(draft: &AgentDraft) -> Option<&'static Palette> {
    let cfg = &draft.profile().config;
    let key = cfg.palette.clone().or_else(|| cfg.archetype_class.map(|class| class.key().to_string()))?;
    Palette::find(&key).filter(|palette| palette.is_full())
}

/// Forgets which palette (and class) the draft started from.
pub fn clear_palette(draft: &mut AgentDraft) -> bool {
    draft.transform(|profile| {
        profile.config.palette = None;
        profile.config.archetype_class = None;
    })
}
