//! Agent Forge: the pure data model behind the Stardew/Skyrim-style character creator for
//! agents and personas (no UI here).
//!
//! - [`ForgeStep`] / [`steps_for`]: the wizard pages and which ones each [`ForgeKind`] walks.
//! - [`fields`]: typed handles on every editable field, each owned by exactly one step.
//! - [`AgentDraft`]: the working copy with undo / redo, dirty tracking, commit and autosave.
//! - [`apply_palette`] / [`preview_palette`]: start from (or borrow a step of) a
//!   [`Palette`](crate::domain::Palette); [`apply_class`] / [`preview_diff`] for the thinker classes.
//! - [`validate`], [`roll_random`], [`describe_prompt`] / [`parse_described`], [`voice_sample`]
//!   and the attribute helpers.

pub mod attributes;
mod change;
mod class;
pub mod describe;
mod draft;
mod palette;
mod edit;
pub mod fields;
pub mod roll;
mod step;
pub mod validate;
mod voice;

pub use attributes::{
    budget_remaining, budget_spent, can_raise, intelligence_radar, is_balanced_build,
    personality_radar, QuotientLevel, RadarPoint, PERSONALITY_POINT_BUDGET,
};
pub use change::{engine_change_keys, ChangeValue, FieldChange};
pub use class::{apply_class, preview_diff, set_class};
pub use palette::{apply_palette, clear_palette, draft_palette, palette_step, preview_palette, step_palettes};
pub use describe::{describe_prompt, parse_described, DraftPatch, ParseError, PatchWarning};
pub use draft::{normalize_handle, AgentDraft, AutosaveError, StepProgress, AUTOSAVE_VERSION, HISTORY_LIMIT};
pub use edit::DraftEdit;
pub use fields::{
    field_label_key, field_step, fields_for_step, AvatarField, CountField, HourField, OptionalTextField,
    ScoreField, TextField, ToggleField, TraitField, TraitSet, TraitSetValue, TraitSetVisitor, TraitValue,
    TraitVisitor,
};
pub use roll::{avatar_edits, avatar_for_body, body_edits, body_for_form, body_for_kind, roll_being, roll_into, roll_random, BeingScope};
pub use step::{steps_for, ForgeKind, ForgeStep, FORGE_I18N_ROOT};
pub use validate::{can_save, issues_for_step, suggest_handle, validate, IssueKind, Severity, ValidationIssue};
pub use voice::{voice_sample, voice_sample_for};

#[cfg(test)]
mod tests;
