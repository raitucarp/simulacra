use super::super::terms::{content_terms, jaccard, words};
use super::super::*;
use super::close;

fn pair(left: &str, right: &str) -> Vec<(String, String)> {
    vec![(left.to_string(), right.to_string())]
}

#[test]
fn one_sided_negation_on_a_shared_topic_is_a_signal() {
    let signals = lexical_signals("Free will is an illusion.", "Free will is not an illusion.");
    assert!(signals.negation);
    assert!(signals.shared_terms.contains(&"illusion".to_string()), "{:?}", signals.shared_terms);
    assert!(close(signals.strength(), 0.4));
    assert!(!signals.is_empty());
}

#[test]
fn negation_needs_a_shared_topic_and_reads_parity() {
    assert!(!lexical_signals("Cats are not dogs.", "Markets clear quickly.").negation);
    assert!(!lexical_signals("No, memory is not reliable.", "Memory is reliable.").negation, "two negations affirm");
    assert!(lexical_signals("Habits don’t matter much.", "Habits matter much.").negation, "typographic apostrophe");
    assert!(lexical_signals("Habits don't matter much.", "Habits matter much.").negation);
}

#[test]
fn antonyms_match_after_a_plural_trim_with_the_left_word_first() {
    let signals = lexical_signals("Sugar increases focus.", "Sugar decreases focus.");
    assert_eq!(signals.antonyms, pair("increase", "decrease"));
    let flipped = lexical_signals("Sugar decreases focus.", "Sugar increases focus.");
    assert_eq!(flipped.antonyms, pair("decrease", "increase"));
    assert!(lexical_signals("Sugar is good and bad.", "Sugar is bad.").antonyms.is_empty(), "a side holding both poles takes neither");
}

#[test]
fn opposite_quantifiers_include_phrases() {
    let everyone = lexical_signals("Everyone can learn calculus.", "No one can learn calculus.");
    assert!(everyone.quantifiers.contains(&("everyone".to_string(), "no one".to_string())), "{:?}", everyone.quantifiers);
    let must = lexical_signals("You must rest after training.", "You must not rest after training.");
    assert_eq!(must.quantifiers, pair("must", "must not"));
    assert!(must.negation);
}

#[test]
fn other_languages_have_their_own_poles() {
    let german = lexical_signals("Arbeit macht immer glücklich.", "Arbeit macht nie glücklich.");
    assert_eq!(german.quantifiers, pair("immer", "nie"));
    assert!(german.negation);
    assert!(close(german.strength(), 0.75));
    let indonesian = lexical_signals("Kopi itu baik untuk tidur.", "Kopi itu buruk untuk tidur.");
    assert_eq!(indonesian.antonyms, pair("baik", "buruk"));
    let spanish = lexical_signals("El café siempre ayuda.", "El café nunca ayuda.");
    assert_eq!(spanish.quantifiers, pair("siempre", "nunca"));
}

#[test]
fn shared_terms_alone_are_a_topic_not_a_signal() {
    let signals = lexical_signals("Habits shape identity.", "Identity shapes habits.");
    assert!(signals.is_empty());
    assert!(!signals.shared_terms.is_empty());
    assert!(close(signals.strength(), 0.0));
}

#[test]
fn strength_is_capped_at_one() {
    let signals = lexical_signals("All good things always increase.", "No good things never... none are bad, they decrease.");
    assert!(signals.strength() <= 1.0);
    let many = LexicalSignals { negation: true, antonyms: [pair("a", "b"), pair("e", "f"), pair("g", "h")].concat(), quantifiers: pair("c", "d"), shared_terms: Vec::new() };
    assert!(close(many.strength(), 1.0));
}

#[test]
fn content_terms_skip_stopwords_short_words_and_possessives() {
    let terms = content_terms("Spinoza's ethics is about the freedom that we have.");
    assert!(terms.contains("spinoza") && terms.contains("freedom"), "{terms:?}");
    assert!(!terms.contains("about") && !terms.contains("that") && !terms.contains("is"));
    assert_eq!(words("Don’t STOP"), vec!["don't".to_string(), "stop".to_string()]);
    assert!(content_terms("an ox ate it").is_empty(), "short words are not terms");
    assert!(close(jaccard(&content_terms("an ox"), &content_terms("a cat")), 0.0), "no terms, no overlap");
    assert!(close(jaccard(&content_terms("rivers flood towns"), &content_terms("rivers flood farms")), 0.5));
}
