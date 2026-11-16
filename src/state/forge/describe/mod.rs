//! "Describe in words": an LLM prompt for a strict-JSON character sheet, and a tolerant parser
//! that turns the reply into a [`DraftPatch`]. No network here: the UI sends the prompt through
//! the configured LLM client and passes the reply text back in.

mod parse;
mod prompt;

pub use parse::{parse_described, ParseError};
pub use prompt::describe_prompt;

use super::draft::AgentDraft;
use super::edit::DraftEdit;
use super::fields::{CountField, ScoreField, TextField, TraitField};
use super::step::{ForgeStep, FORGE_I18N_ROOT};

pub(crate) const DESCRIBE_GROUP: &str = "describe";

/// Free-text fields the LLM may fill.
pub const DESCRIBABLE_TEXT: [TextField; 13] = [
    TextField::Name,
    TextField::Description,
    TextField::Occupation,
    TextField::Bio,
    TextField::Pronouns,
    TextField::LocationCulture,
    TextField::Hobbies,
    TextField::CoreValues,
    TextField::DailyRoutine,
    TextField::CurrentMood,
    TextField::PrimaryLanguage,
    TextField::SignaturePhrases,
    TextField::PreferredTopics,
];

/// Whole-number fields the LLM may fill.
pub const DESCRIBABLE_COUNTS: [CountField; 1] = [CountField::Age];

/// Score fields the LLM may fill (all of them).
pub fn describable_scores() -> &'static [ScoreField] {
    ScoreField::ALL
}

/// Trait fields the LLM may fill: everything except the advanced Grimoire settings.
pub fn describable_traits() -> Vec<TraitField> {
    TraitField::ALL.iter().copied().filter(|f| f.step() != ForgeStep::Grimoire).collect()
}

/// Something in the LLM reply that was skipped.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PatchWarning {
    /// A key that is not a describable field.
    UnknownField(String),
    /// A choice value that is not one of the field's options.
    InvalidOption { field: TraitField, value: String },
    /// A value of the wrong JSON type for its field.
    WrongType(String),
    /// A number outside its field's range (it was clamped).
    OutOfRange { field: String, value: String },
}

impl PatchWarning {
    pub const fn key(&self) -> &'static str {
        match self {
            Self::UnknownField(_) => "warn_unknown_field",
            Self::InvalidOption { .. } => "warn_invalid_option",
            Self::WrongType(_) => "warn_wrong_type",
            Self::OutOfRange { .. } => "warn_out_of_range",
        }
    }

    /// i18n key of the message (interpolates `%{field}` and `%{value}`).
    pub fn message_key(&self) -> String {
        format!("{FORGE_I18N_ROOT}.{DESCRIBE_GROUP}.{}", self.key())
    }

    /// The offending field name.
    pub fn field(&self) -> &str {
        match self {
            Self::UnknownField(field) | Self::WrongType(field) | Self::OutOfRange { field, .. } => field,
            Self::InvalidOption { field, .. } => field.path(),
        }
    }
}

/// Edits parsed from an LLM reply, plus whatever had to be skipped.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DraftPatch {
    pub edits: Vec<DraftEdit>,
    pub warnings: Vec<PatchWarning>,
}

impl AgentDraft {
    /// Applies every edit of `patch` as a single undo step.
    pub fn apply_patch(&mut self, patch: &DraftPatch) -> bool {
        self.edit_all(&patch.edits)
    }
}
