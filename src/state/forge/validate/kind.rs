//! The stable id of every validation message.

use crate::state::forge::step::FORGE_I18N_ROOT;
use serde::{Deserialize, Serialize};

const ISSUE_GROUP: &str = "issue";

/// Stable id of every validation message; the i18n key is `forge.issue.{key}`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum IssueKind {
    NameRequired,
    NameTooShort,
    NameTooLong,
    NameJunk,
    HandleRequired,
    HandleFormat,
    HandleLength,
    HandleTaken,
    LanguageRequired,
    HoursOutOfRange,
    HoursAroundTheClock,
    TemplateEmpty,
    TemplateUnknownPlaceholder,
    TemplateUnclosed,
    BioEmpty,
    AgeUnusual,
    FormKindMismatch,
    NoActiveDays,
    EngineCredentialMissing,
    EngineCredentialGone,
    EngineUrlEmpty,
    EngineBodyInvalid,
    EngineSchemaInvalid,
    EngineMappingContent,
    EngineMappingUnknown,
    EngineImageNeverAsked,
    EngineToolName,
    EngineToolParams,
}

impl IssueKind {
    pub const ALL: [Self; 28] = [
        Self::NameRequired, Self::NameTooShort, Self::NameTooLong, Self::NameJunk,
        Self::HandleRequired, Self::HandleFormat, Self::HandleLength, Self::HandleTaken,
        Self::LanguageRequired, Self::HoursOutOfRange, Self::HoursAroundTheClock, Self::TemplateEmpty,
        Self::TemplateUnknownPlaceholder, Self::TemplateUnclosed, Self::BioEmpty, Self::AgeUnusual,
        Self::FormKindMismatch, Self::NoActiveDays, Self::EngineCredentialMissing, Self::EngineCredentialGone,
        Self::EngineUrlEmpty, Self::EngineBodyInvalid, Self::EngineSchemaInvalid, Self::EngineMappingContent,
        Self::EngineMappingUnknown, Self::EngineImageNeverAsked, Self::EngineToolName, Self::EngineToolParams,
    ];

    pub const fn key(self) -> &'static str {
        match self {
            Self::NameRequired => "name_required",
            Self::NameTooShort => "name_too_short",
            Self::NameTooLong => "name_too_long",
            Self::NameJunk => "name_junk",
            Self::HandleRequired => "handle_required",
            Self::HandleFormat => "handle_format",
            Self::HandleLength => "handle_length",
            Self::HandleTaken => "handle_taken",
            Self::LanguageRequired => "language_required",
            Self::HoursOutOfRange => "hours_out_of_range",
            Self::HoursAroundTheClock => "hours_around_the_clock",
            Self::TemplateEmpty => "template_empty",
            Self::TemplateUnknownPlaceholder => "template_unknown_placeholder",
            Self::TemplateUnclosed => "template_unclosed",
            Self::BioEmpty => "bio_empty",
            Self::AgeUnusual => "age_unusual",
            Self::FormKindMismatch => "form_kind_mismatch",
            Self::NoActiveDays => "no_active_days",
            Self::EngineCredentialMissing => "engine_credential_missing",
            Self::EngineCredentialGone => "engine_credential_gone",
            Self::EngineUrlEmpty => "engine_url_empty",
            Self::EngineBodyInvalid => "engine_body_invalid",
            Self::EngineSchemaInvalid => "engine_schema_invalid",
            Self::EngineMappingContent => "engine_mapping_content",
            Self::EngineMappingUnknown => "engine_mapping_unknown",
            Self::EngineImageNeverAsked => "engine_image_never_asked",
            Self::EngineToolName => "engine_tool_name",
            Self::EngineToolParams => "engine_tool_params",
        }
    }

    /// Whether the issue only says a field is still empty (shown once the user moves on).
    pub const fn is_blank(self) -> bool {
        matches!(self, Self::NameRequired | Self::HandleRequired | Self::LanguageRequired | Self::BioEmpty | Self::TemplateEmpty | Self::EngineUrlEmpty)
    }

    /// i18n key of the message (interpolate the issue's `args`).
    pub fn message_key(self) -> String {
        format!("{FORGE_I18N_ROOT}.{ISSUE_GROUP}.{}", self.key())
    }
}

