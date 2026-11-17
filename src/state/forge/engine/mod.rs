//! The Engine step's pure model: its five sections, the engine edits its cards make
//! ([`mutate`]), whether the referenced keys exist ([`readiness`]) and the live rendered-post
//! preview of a structured answer ([`sample`]). Validation lives in `validate::engine`.

mod json;
pub mod mutate;
pub mod readiness;
pub mod sample;
mod tool_grants;

#[cfg(test)]
mod tests;

pub use json::{body_error, insert_token, object_problem, JsonProblem};
pub use readiness::{credential_status, image_credential_kind, search_credential_kind, CredentialStatus};
pub use sample::{preview, sample_answer, PreviewError, SampleTexts};

use crate::state::forge::step::FORGE_I18N_ROOT;
use serde::{Deserialize, Serialize};

/// Prefix of the validation-issue field of a section (`engine.<key>`).
const ISSUE_FIELD_PREFIX: &str = "engine.";
const SECTION_GROUP: &str = "ui.engine.section";

/// One section of the Engine step, in page order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum EngineSection {
    /// Which model answers: the global one or the agent's own.
    #[default]
    Model,
    /// The provider's native API or a hand-built request.
    Request,
    /// Free text or a JSON schema mapped onto the post.
    Output,
    /// The image engine and when it draws.
    Image,
    /// The tools the agent may call.
    Tools,
}

impl EngineSection {
    pub const ALL: [Self; 5] = [Self::Model, Self::Request, Self::Output, Self::Image, Self::Tools];

    pub const fn key(self) -> &'static str {
        match self {
            Self::Model => "model",
            Self::Request => "request",
            Self::Output => "output",
            Self::Image => "image",
            Self::Tools => "tools",
        }
    }

    /// The `field` of this section's validation issues.
    pub const fn issue_field(self) -> &'static str {
        match self {
            Self::Model => "engine.model",
            Self::Request => "engine.request",
            Self::Output => "engine.output",
            Self::Image => "engine.image",
            Self::Tools => "engine.tools",
        }
    }

    /// The section an issue `field` belongs to.
    pub fn of_field(field: &str) -> Option<Self> {
        let key = field.strip_prefix(ISSUE_FIELD_PREFIX)?;
        Self::ALL.into_iter().find(|section| section.key() == key)
    }

    /// i18n key of the section title: `forge.ui.engine.section.{key}`.
    pub fn label_key(self) -> String {
        format!("{FORGE_I18N_ROOT}.{SECTION_GROUP}.{}", self.key())
    }

    /// i18n key of the one-line section intro: `forge.ui.engine.section.{key}_intro`.
    pub fn intro_key(self) -> String {
        format!("{}_intro", self.label_key())
    }
}
