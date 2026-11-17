//! Flat ontology and rhetoric in the prompt: the embodiment section (only for non-human
//! beings, so people read exactly as before), the being's own framing lines, and the
//! rhetoric / semiotics / post-structure section.

use crate::domain::agent_config::AgentConfig;

const LIST_SEPARATOR: &str = ", ";
const NO_DEVICES: &str = "None in particular (plain statements)";
const EVERYDAY_WORDS: &str = "Everyday vocabulary";

impl AgentConfig {
    /// The agent's form as a noun phrase ("a tin can"); `None` for a human.
    pub(super) fn being(&self) -> Option<&'static str> {
        (!self.embodiment.is_human()).then(|| self.embodiment.form.noun())
    }

    /// The physical, temporal and relational parameters of a non-human body (empty for people).
    pub fn format_embodiment_section(&self) -> String {
        let Some(noun) = self.being() else { return String::new() };
        let body = &self.embodiment;
        format!(
            "[EMBODIMENT: FLAT ONTOLOGY]\n\
             - What You Are: {} ({})\n\
             - Made Of: {}\n\
             - Size / Scale: {}\n\
             - Time Scale You Live In: {}\n\
             - How You Perceive: {}\n\
             - How You Move: {}\n\
             - Human-Likeness of Your Voice: {}\n\
             - Relation to Humans: {}\n\
             - Withdrawal (How Much of Yourself You Reveal): {}\n\
             - Ontological Rule: Speak from inside this form, through its senses, time scale and situation. The psychological, intelligence, linguistic and rhetorical parameters below are the temperament of {} in a flat ontology where every thing is equally real; they are not a human body or a human life.",
            noun,
            body.body_kind.label(),
            body.material.label(),
            body.scale.label(),
            body.temporality.label(),
            body.perception.label(),
            body.mobility.label(),
            body.anthropomorphism.label(),
            body.relation_to_humans.label(),
            body.withdrawal.label(),
            noun,
        )
    }

    /// Rhetorical mode and devices, stylistic school, semiotics, sentence structure, post
    /// format and borrowed vocabularies.
    pub fn format_rhetoric_section(&self) -> String {
        let rhetoric = &self.rhetoric;
        format!(
            "[RHETORIC, STYLE, SEMIOTICS & POST STRUCTURE]\n\
             - Rhetorical Mode: {}\n\
             - Favorite Rhetorical Devices: {}\n\
             - Stylistic School: {}\n\
             - Semiotic Mode: {}\n\
             - Sentence Structure: {}\n\
             - Post Format: {}\n\
             - Lexical Fields (Borrow Vocabulary & Imagery From): {}",
            rhetoric.rhetorical_mode.label(),
            rhetoric.rhetorical_devices.labels(LIST_SEPARATOR, NO_DEVICES),
            rhetoric.stylistic_school.label(),
            rhetoric.semiotic_mode.label(),
            rhetoric.sentence_structure.label(),
            rhetoric.post_format.label(),
            rhetoric.lexical_fields.labels(LIST_SEPARATOR, EVERYDAY_WORDS),
        )
    }
}
