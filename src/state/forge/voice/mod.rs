//! A deterministic, template-based sample post showing how the draft's voice settings sound.
//!
//! Pipeline: register clauses → sentence length → grammar → signature phrase →
//! capitalization → punctuation → emoji. No LLM involved; the sample is English.

mod register;
mod style;

use super::draft::AgentDraft;
use crate::domain::agent_config::AgentConfig;
use crate::domain::{CapitalizationStyle, GrammarStyle};
use register::{first_topic, sample, TOPIC_SLOT};

const PHRASE_SEPARATOR: char = ',';
/// Punctuation stripped from a signature phrase before it is re-punctuated.
const PHRASE_TRAILING: [char; 4] = ['.', '?', '!', '…'];

/// The first signature phrase, without trailing punctuation.
fn signature(phrases: &str) -> Option<String> {
    let first = phrases.split(PHRASE_SEPARATOR).next()?.trim().trim_end_matches(PHRASE_TRAILING).trim();
    (!first.is_empty()).then(|| first.to_string())
}

/// The sample post for a bare config.
pub fn voice_sample_for(cfg: &AgentConfig) -> String {
    let register = sample(cfg.diction_register);
    let topic = first_topic(&cfg.preferred_topics);
    let clauses = register.clauses.map(|clause| clause.replace(TOPIC_SLOT, &topic));
    let mut sentences = style::shape(clauses, cfg.sentence_length);
    style::grammar(&mut sentences, cfg.grammar_style);
    sentences.extend(signature(&cfg.signature_phrases));
    let capitalization = if cfg.grammar_style == GrammarStyle::MinimalistLowercase {
        CapitalizationStyle::AllLowercase
    } else {
        cfg.capitalization
    };
    style::capitalize(&mut sentences, capitalization, register.emphasis);
    let mut post = style::punctuate(&sentences, cfg.punctuation_habit);
    post.push_str(style::emoji_tail(cfg.emoji_usage));
    post
}

/// The sample post for the draft's current voice settings.
pub fn voice_sample(draft: &AgentDraft) -> String {
    voice_sample_for(&draft.profile().config)
}
