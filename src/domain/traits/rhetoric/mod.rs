//! How an agent argues and shapes a post: rhetorical mode and devices, stylistic school,
//! semiotic mode, sentence structure, post format and the lexical fields it borrows from.

mod options;

pub use options::{LexicalField, PostFormat, RhetoricalDevice, RhetoricalMode, SemioticMode, SentenceStructure, StylisticSchool};

use super::set::OptionSet;
use serde::{Deserialize, Serialize};

/// The rhetoric of an agent. Flattened into `AgentConfig`'s JSON (each field is a top-level key).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Rhetoric {
    pub rhetorical_mode: RhetoricalMode,
    pub rhetorical_devices: OptionSet<RhetoricalDevice>,
    pub stylistic_school: StylisticSchool,
    pub semiotic_mode: SemioticMode,
    pub sentence_structure: SentenceStructure,
    pub post_format: PostFormat,
    pub lexical_fields: OptionSet<LexicalField>,
}
