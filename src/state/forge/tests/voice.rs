use super::*;
use crate::domain::agent_config::AgentConfig;
use crate::domain::{
    CapitalizationStyle as Caps, DictionRegister as Register, EmojiUsage as Emoji, GrammarStyle as Grammar,
    PunctuationHabit as Punct, SentenceLengthPreference as Length,
};

const BASELINE: &str = "I suspect our debate about ethics rests on an unexamined premise. We should first ask what the term actually picks out. Consider this.";

fn base() -> AgentConfig {
    AgentConfig {
        diction_register: Register::AcademicPhilosophical,
        grammar_style: Grammar::StandardFormal,
        sentence_length: Length::BalancedCadence,
        capitalization: Caps::StandardSentence,
        punctuation_habit: Punct::CrispMinimal,
        emoji_usage: Emoji::Never,
        preferred_topics: "#Ethics #Logic".into(),
        signature_phrases: "Consider this, Another one".into(),
        ..AgentConfig::default()
    }
}

fn with(change: impl FnOnce(&mut AgentConfig)) -> String {
    let mut cfg = base();
    change(&mut cfg);
    voice_sample_for(&cfg)
}

#[test]
fn baseline_sample_is_deterministic() {
    assert_eq!(voice_sample_for(&base()), BASELINE);
    assert_eq!(voice_sample_for(&base()), voice_sample_for(&base()));
    let mut draft = agent();
    draft.edit(DraftEdit::Trait(TraitValue::EmojiUsage(Emoji::Never)));
    assert_eq!(voice_sample(&draft), voice_sample_for(&draft.profile().config));
}

#[test]
fn every_register_has_its_own_words() {
    let samples: std::collections::HashSet<String> =
        Register::ALL.iter().map(|r| with(|c| c.diction_register = *r)).collect();
    assert_eq!(samples.len(), Register::ALL.len());
    assert!(with(|c| c.diction_register = Register::LiteraryPoetic).starts_with("Ethics arrives like late light"));
    assert!(with(|c| c.diction_register = Register::SlangColloquial).contains("lowkey"));
}

#[test]
fn sentence_length_shapes_the_post() {
    assert_eq!(with(|c| c.sentence_length = Length::UltraConciseAphorism), "I suspect our debate about ethics rests on an unexamined premise. Consider this.");
    assert!(with(|c| c.sentence_length = Length::ShortPunchy).contains("picks out. Only then"));
    assert!(with(|c| c.sentence_length = Length::LongPeriodic).contains("premise, we should first ask what the term actually picks out, and only then"));
}

#[test]
fn grammar_style_restyles_the_sentences() {
    assert!(with(|c| c.grammar_style = Grammar::RelaxedConversational).starts_with("Honestly, I suspect"));
    let relaxed = with(|c| { c.grammar_style = Grammar::RelaxedConversational; c.diction_register = Register::EverydayNatural; });
    assert!(relaxed.contains("It's simpler"), "{relaxed}");
    assert!(with(|c| c.grammar_style = Grammar::FragmentedStream).starts_with("Suspect our debate"));
    assert!(with(|c| c.grammar_style = Grammar::PoeticClassical).contains("Thus, we should"));
    let minimal = with(|c| c.grammar_style = Grammar::MinimalistLowercase);
    assert_eq!(minimal, minimal.to_lowercase());
    assert!(with(|c| c.grammar_style = Grammar::ColloquialExpressive).contains("Like, we should"));
}

#[test]
fn punctuation_habit_joins_and_ends_sentences() {
    assert_eq!(with(|c| c.punctuation_habit = Punct::EmDashAndSemicolon), "I suspect our debate about ethics rests on an unexamined premise; we should first ask what the term actually picks out — consider this.");
    let ellipsis = with(|c| c.punctuation_habit = Punct::EllipsisContemplative);
    assert!(ellipsis.contains("premise... We") && ellipsis.ends_with("this..."));
    assert!(with(|c| c.punctuation_habit = Punct::QuestionDriven).ends_with("Consider this?"));
    let bare = with(|c| c.punctuation_habit = Punct::ZeroTrailingPeriod);
    assert!(bare.ends_with("Consider this") && bare.contains("premise. We"));
}

#[test]
fn capitalization_style_is_applied() {
    let lower = with(|c| c.capitalization = Caps::AllLowercase);
    assert_eq!(lower, BASELINE.to_lowercase());
    assert!(with(|c| c.capitalization = Caps::ExpressiveEmphasis).contains("unexamined PREMISE"));
}

#[test]
fn emoji_usage_adds_real_emoji() {
    assert!(BASELINE.is_ascii());
    assert!(with(|c| c.emoji_usage = Emoji::RareSubtle).ends_with("this. 🌿"));
    assert!(with(|c| c.emoji_usage = Emoji::ModerateExpressive).ends_with("🤔✨"));
    assert!(with(|c| c.emoji_usage = Emoji::KaomojiTextOnly).ends_with("(・_・;)"));
}

#[test]
fn signature_phrase_and_topic_are_woven_in() {
    assert!(!with(|c| c.signature_phrases.clear()).contains("Consider"));
    assert!(with(|c| c.signature_phrases = "Consider whether...".into()).ends_with("Consider whether."));
    assert!(with(|c| c.preferred_topics = "#PhilosophyOfScience".into()).contains("about philosophy of science rests"));
    assert!(with(|c| c.preferred_topics.clear()).contains("about this idea rests"));
}
