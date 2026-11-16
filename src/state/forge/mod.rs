//! Agent Forge: the pure data model behind the Stardew/Skyrim-style character creator for
//! agents and personas (no UI here).
//!
//! - [`ForgeStep`] / [`steps_for`]: the wizard pages and which ones each [`ForgeKind`] walks.
//! - [`fields`]: typed handles on every editable field, each owned by exactly one step.
//! - [`AgentDraft`]: the working copy with undo / redo, dirty tracking, commit and autosave.
//! - [`apply_class`] / [`preview_diff`]: start from an [`ArchetypeClass`](crate::domain::ArchetypeClass).
//! - [`validate`], [`roll_random`], [`describe_prompt`] / [`parse_described`], [`voice_sample`]
//!   and the attribute helpers.

pub mod attributes;
mod class;
pub mod describe;
mod draft;
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
pub use class::{apply_class, preview_diff, ChangeValue, FieldChange};
pub use describe::{describe_prompt, parse_described, DraftPatch, ParseError, PatchWarning};
pub use draft::{normalize_handle, AgentDraft, AutosaveError, StepProgress, AUTOSAVE_VERSION, HISTORY_LIMIT};
pub use edit::DraftEdit;
pub use fields::{
    field_label_key, field_step, fields_for_step, AvatarField, CountField, HourField, OptionalTextField,
    ScoreField, TextField, ToggleField, TraitField, TraitValue, TraitVisitor,
};
pub use roll::{roll_into, roll_random};
pub use step::{steps_for, ForgeKind, ForgeStep, FORGE_I18N_ROOT};
pub use validate::{can_save, issues_for_step, suggest_handle, validate, IssueKind, Severity, ValidationIssue};
pub use voice::{voice_sample, voice_sample_for};

#[cfg(test)]
mod tests;
