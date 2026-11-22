//! Offline heuristic post text (replies and new posts), used directly or as the LLM fallback.

use super::voices::{CannedVoice, CANNED_VOICES, PARENT_REF_PLACEHOLDER};
use super::DialecticEngine;
use crate::domain::post::ResolvedSeed;
use crate::domain::{CapitalizationStyle, Post, Profile};

/// Lead phrase used when the persona has no usable signature phrase.
const DEFAULT_LEAD: &str = "Reflecting on this";
/// Subject of an unseeded new post when the persona lists no hobbies.
const DEFAULT_SUBJECT: &str = "an open question";
/// Separator between hobbies in the persona config.
const HOBBY_SEPARATOR: char = ',';
/// Separator between signature phrases in the persona config.
const SIGNATURE_PHRASE_SEPARATOR: char = ';';
/// Separator between variants inside one signature phrase.
const SIGNATURE_VARIANT_SEPARATOR: char = ',';
/// Quote character stripped from the chosen signature phrase.
const SIGNATURE_QUOTE: char = '"';
/// Lowercased primary-language fragments that select the Indonesian template.
const INDONESIAN_LANGUAGE_MARKERS: &[&str] = &["indonesian", "indonesia", "jawa"];

impl DialecticEngine {
    /// Generates heuristic fallback response text that respects the agent's human identity,
    /// language, capitalization style, and signature voice.
    pub(super) fn generate_heuristic_text(parent_post: &Post, agent_profile: &Profile) -> String {
        let parent_ref = format!("[[{}]]", parent_post.id);
        let handle = agent_profile.handle.as_str();

        let raw = match CANNED_VOICES.iter().find(|v| v.handles.contains(&handle)) {
            Some(voice) => canned_text(voice, &parent_post.content.to_lowercase(), &parent_ref),
            None => generic_text(agent_profile, &parent_ref),
        };

        styled(agent_profile, raw)
    }

    /// Heuristic text opening a new conversation about the seed (or, with no seed, the
    /// agent's first hobby), in the agent's language and capitalization style.
    pub(super) fn generate_heuristic_root_text(agent_profile: &Profile, seed: &ResolvedSeed) -> String {
        let subject = root_subject(agent_profile, seed);
        let lead = signature_lead(agent_profile);
        let occupation = &agent_profile.config.occupation;
        let mbti = agent_profile.config.mbti.code();
        let raw = if speaks_indonesian(agent_profile) {
            format!("{lead} — sebagai {occupation} ({mbti}), aku terus kembali memikirkan {subject}. Apa yang perlu kita pahami lebih dulu? #Dialektika")
        } else {
            format!("{lead} — as a {occupation} ({mbti}), I keep coming back to {subject}. What would it take to see it clearly? #Dialectic")
        };
        styled(agent_profile, raw)
    }
}

/// `raw` in the agent's capitalization style.
pub(super) fn styled(agent_profile: &Profile, raw: String) -> String {
    match agent_profile.config.capitalization {
        CapitalizationStyle::AllLowercase => raw.to_lowercase(),
        _ => raw,
    }
}

/// The seed topic (citing the seed document as a `[[wikilink]]`), else the agent's first hobby.
fn root_subject(agent_profile: &Profile, seed: &ResolvedSeed) -> String {
    let cite = seed.document.as_ref().map(|doc| format!("[[{}]]", doc.title));
    match (seed.topic.as_str(), cite) {
        ("", Some(cite)) => cite,
        ("", None) => {
            let hobby = agent_profile.config.hobbies.split(HOBBY_SEPARATOR).next().unwrap_or_default().trim();
            if hobby.is_empty() { DEFAULT_SUBJECT.to_string() } else { hobby.to_lowercase() }
        }
        (topic, Some(cite)) => format!("{topic} ({cite})"),
        (topic, None) => topic.to_string(),
    }
}

fn speaks_indonesian(agent_profile: &Profile) -> bool {
    let lang_lower = agent_profile.config.primary_language.to_lowercase();
    INDONESIAN_LANGUAGE_MARKERS.iter().any(|marker| lang_lower.contains(marker))
}

/// Picks the first keyword-matching template of `voice` (or its fallback) and fills it in.
fn canned_text(voice: &CannedVoice, content_lower: &str, parent_ref: &str) -> String {
    let template = voice
        .rules
        .iter()
        .find(|rule| rule.keywords.iter().any(|kw| content_lower.contains(kw)))
        .map_or(voice.fallback, |rule| rule.template);
    template.replace(PARENT_REF_PLACEHOLDER, parent_ref)
}

/// The first signature phrase variant of the persona, or [`DEFAULT_LEAD`].
fn signature_lead(agent_profile: &Profile) -> &str {
    let phrases = &agent_profile.config.signature_phrases;
    if phrases.trim().is_empty() {
        return DEFAULT_LEAD;
    }
    phrases
        .split(SIGNATURE_PHRASE_SEPARATOR)
        .next()
        .unwrap_or(DEFAULT_LEAD)
        .split(SIGNATURE_VARIANT_SEPARATOR)
        .next()
        .unwrap_or(DEFAULT_LEAD)
        .trim()
        .trim_matches(SIGNATURE_QUOTE)
}

/// Generic persona reply built from signature phrase, occupation, MBTI and language.
fn generic_text(agent_profile: &Profile, parent_ref: &str) -> String {
    let lead = signature_lead(agent_profile);
    let occupation = &agent_profile.config.occupation;
    let mbti = agent_profile.config.mbti.code();

    if speaks_indonesian(agent_profile) {
        format!(
            "{lead} — menarik melihat perspektif di {parent_ref} dari sudut pandang {occupation} ({mbti}). Bagaimana implikasi jangka panjangnya menurutmu? #Dialektika"
        )
    } else {
        format!(
            "{lead} — looking at {parent_ref} through the lens of {occupation} ({mbti}), the underlying tension is worth exploring further. #Dialectic"
        )
    }
}
