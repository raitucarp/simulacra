//! Linguistics and social-schedule sections, plus the combined human dossier.

use crate::domain::agent_config::AgentConfig;
use crate::domain::profile::Profile;

/// Upper bound on the number of dossier sections (network grounding is injected separately).
const DOSSIER_SECTION_CAPACITY: usize = 7;
/// Separator placed between consecutive dossier sections.
pub(super) const SECTION_SEPARATOR: &str = "\n\n";

impl AgentConfig {
    /// Formats the linguistic, grammatical, lexical diction, and typing stylistics section.
    pub fn format_linguistics_section(&self) -> String {
        format!(
            "[LINGUISTICS, GRAMMAR, DICTION & TYPING STYLISTICS]\n\
             - Primary Language: {}\n\
             - Secondary / Known Languages: {}\n\
             - Code-Switching Habit: {}\n\
             - Grammar & Syntax Style: {}\n\
             - Diction & Lexical Register: {} (Vocabulary Complexity: {}%)\n\
             - Sentence Rhythm & Length: {}\n\
             - Typing Capitalization: {}\n\
             - Punctuation Habit: {}\n\
             - Emoji / Symbol Usage: {}\n\
             - Specific Typing Quirks: {}\n\
             - Characteristic Expressions: {}\n\
             - Strictly Forbidden / Avoided Words: {}",
            self.primary_language,
            self.secondary_languages,
            self.code_switching.label(),
            self.grammar_style.label(),
            self.diction_register.label(),
            self.vocabulary_complexity,
            self.sentence_length.label(),
            self.capitalization.label(),
            self.punctuation_habit.label(),
            self.emoji_usage.label(),
            self.typing_quirks,
            self.signature_phrases,
            self.avoided_words,
        )
    }

    /// Formats the posting schedule, circadian chronotype, and `@mention` social attitude section.
    pub fn format_social_schedule_section(&self) -> String {
        format!(
            "[POSTING SCHEDULE, CHRONOTYPE & @MENTION ATTITUDE]\n\
             - Network Activity Cadence: {}\n\
             - Circadian Chronotype: {} (Active Window: {:02}:00–{:02}:00, Spontaneity: {}%)\n\
             - Core Topics of Interest: {}\n\
             - Attitude When @Mentioned: {}\n\
             - Disagreement & Conflict Style: {}\n\
             - Custom @Mention Protocol: {}",
            self.posting_cadence.label(),
            self.chronotype.label(),
            self.active_hours_start,
            self.active_hours_end,
            self.spontaneity_rate,
            self.preferred_topics,
            self.mention_attitude.label(),
            self.conflict_handling.label(),
            self.custom_mention_instruction,
        )
    }

    /// Compiles the complete dossier combining all enabled sections. A non-human being always
    /// opens with its embodiment; rhetoric travels with the linguistic section.
    pub fn format_full_human_dossier(&self, profile: &Profile) -> String {
        let mut sections = Vec::with_capacity(DOSSIER_SECTION_CAPACITY);
        if !self.embodiment.is_human() {
            sections.push(self.format_embodiment_section());
        }
        if self.inject_identity_and_bio {
            sections.push(self.format_identity_section(profile));
        }
        if self.inject_psychological_profile {
            sections.push(self.format_psychology_section());
        }
        if self.inject_intelligence_profile {
            sections.push(self.format_intelligence_section());
        }
        if self.inject_linguistic_stylistics {
            sections.push(self.format_linguistics_section());
            sections.push(self.format_rhetoric_section());
        }
        if self.inject_social_and_schedule {
            sections.push(self.format_social_schedule_section());
        }
        sections.join(SECTION_SEPARATOR)
    }
}
