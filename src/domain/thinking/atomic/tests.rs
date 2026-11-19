//! The atomic-note linter: pieces and claims, topics, context checks, findings and score,
//! and the split proposal.

use super::clauses::{clauses_of, list_item};
use super::context::{dangling_pronoun, has_link, has_source, sounds_empirical};
use super::findings::score_of;
use super::topics::topics_of;
use super::*;
use crate::import::segment::SentenceLocale;

const EN: SentenceLocale = SentenceLocale::English;

fn claims(text: &str) -> Vec<String> {
    claims_of(text, EN)
}

fn keys(text: &str) -> Vec<&'static str> {
    check_atomicity(text, EN).findings.iter().map(Finding::key).collect()
}

#[test]
fn sentences_and_independent_clauses_are_claims() {
    assert_eq!(claims("Cats sleep most of the day. Dogs need long walks every day.").len(), 2);
    assert_eq!(claims("Cats sleep most of the day, but dogs need long walks."), ["Cats sleep most of the day", "dogs need long walks."]);
    assert_eq!(claims("I like tea, and coffee.").len(), 1);
    assert_eq!(claims("Memory is reconstructive; recall changes the memory itself.").len(), 2);
    assert_eq!(clauses_of("a b c, and d e f, but g h i"), ["a b c", "d e f", "g h i"]);
}

#[test]
fn questions_fragments_and_repeats_are_not_extra_claims() {
    assert_eq!(claims("Is memory reliable? Memory is reconstructive by nature.").len(), 1);
    assert_eq!(claims("Yes. Memory is reconstructive by nature.").len(), 1);
    assert_eq!(claims("Memory is reconstructive. Memory is reconstructive!").len(), 1);
    assert!(claims("").is_empty());
}

#[test]
fn list_items_are_claims_of_their_own() {
    let text = "Three points:\n- Sleep consolidates memory traces\n- Spacing beats cramming for retention\n1. Testing strengthens recall";
    assert_eq!(claims(text), ["Sleep consolidates memory traces", "Spacing beats cramming for retention", "Testing strengthens recall"]);
    assert_eq!(list_item("1. First"), Some("First"));
    assert_eq!(list_item("  * Second "), Some("Second"));
    assert_eq!(list_item("-5 degrees outside"), None);
    assert_eq!(list_item("2025 was long"), None);
}

#[test]
fn other_languages_split_too() {
    let german = claims_of("Katzen schlafen sehr viel, aber Hunde brauchen lange Spaziergänge.", SentenceLocale::German);
    assert_eq!(german.len(), 2);
    let chinese = claims_of("记忆是可以重建的。睡眠巩固记忆。", SentenceLocale::Other);
    assert_eq!(chinese.len(), 2);
    assert_eq!(claims_of("好。", SentenceLocale::Other).len(), 0);
}

#[test]
fn an_atomic_note_scores_full_marks() {
    let report = check_atomicity("Spaced retrieval beats rereading for long-term retention, see [[Testing effect]].", EN);
    assert_eq!(report.findings, Vec::<Finding>::new());
    assert_eq!(report.score, 100);
    assert_eq!(report.claims.len(), 1);
    assert!(report.is_atomic());
    let blank = check_atomicity("   ", EN);
    assert_eq!((blank.score, blank.chars, blank.findings.len()), (100, 0, 0));
}

#[test]
fn length_bands_are_flagged() {
    assert_eq!(keys("Sleep matters."), ["too_short"]);
    let long = "memory ".repeat(LONG_CHARS / 6);
    let report = check_atomicity(&long, EN);
    assert!(report.findings.contains(&Finding::TooLong { chars: report.chars }));
    assert!(report.findings.contains(&Finding::NoLink));
}

#[test]
fn missing_context_is_flagged() {
    assert_eq!(keys("This changes how we should think about memory and learning."), ["dangling_pronoun"]);
    assert_eq!(keys("Over 70% of people forget a lecture within a week."), ["no_source"]);
    assert!(keys("Over 70% of people forget a lecture within a week (Ebbinghaus, 1885).").is_empty());
    let unlinked = "Forgetting is not a failure of memory but a feature that keeps what matters within reach.";
    assert_eq!(keys(unlinked), ["no_link"]);
    assert!(keys(&format!("{unlinked} #memory")).is_empty());
}

#[test]
fn context_checks_read_links_sources_and_openings() {
    assert!(has_link("see [[Note]]") && has_link("about #memory") && has_link("https://example.org"));
    assert!(!has_link("# heading only") && !has_link("[[unclosed"));
    assert!(has_source("(Smith, 2019)") && has_source("Studies show it, via the archive") && has_source("Rao et al. found"));
    assert!(!has_source("(see 12 items)"));
    assert!(sounds_empirical("Research suggests so") && sounds_empirical("It rose 12 percent"));
    assert!(!sounds_empirical("Builds on [[post-123]] and #2024 nicely"));
    assert_eq!(dangling_pronoun("“They said it would rain.”").as_deref(), Some("They"));
    assert_eq!(dangling_pronoun("Itu benar sekali.").as_deref(), Some("Itu"));
    assert_eq!(dangling_pronoun("Thistles grow here."), None);
    assert_eq!(dangling_pronoun(""), None);
}

#[test]
fn topics_are_concepts_that_meet_in_a_claim() {
    let apart = "[[Spinoza]] wrote about the passions of the mind. [[Photosynthesis]] converts light into chemical energy.";
    let found = topics_of(apart, &claims(apart));
    assert_eq!(found, ["Spinoza", "Photosynthesis"]);
    assert!(keys(apart).contains(&"multiple_topics"));
    let together = "[[Spinoza]] wrote about the passions. Spinoza thought the passions were confused ideas.";
    assert_eq!(topics_of(together, &claims(together)).len(), 1);
    assert_eq!(keys(together), ["multiple_claims"]);
}

#[test]
fn penalties_grow_with_claims_and_the_score_never_goes_negative() {
    assert_eq!(Finding::MultipleClaims { count: 1 }.penalty(), 0);
    assert_eq!(Finding::MultipleClaims { count: 3 }.penalty(), 30);
    assert_eq!(Finding::MultipleClaims { count: 40 }.penalty(), 60);
    assert_eq!(score_of(&[]), 100);
    let many = vec![Finding::MultipleClaims { count: 9 }, Finding::NoSource, Finding::NoLink, Finding::TooLong { chars: 900 }, Finding::DanglingPronoun { word: "It".into() }];
    assert_eq!(score_of(&many), 0);
    let report = check_atomicity("Cats sleep most of the day. Dogs need long walks every day. Birds sing at dawn in spring.", EN);
    assert!(!report.is_atomic());
    assert_eq!(report.findings[0], Finding::MultipleClaims { count: 3 });
}

#[test]
fn a_split_proposes_one_post_per_claim() {
    assert_eq!(split_proposal("Cats sleep most of the day, but dogs need long walks.", EN), ["Cats sleep most of the day.", "Dogs need long walks."]);
    assert_eq!(split_proposal("  One claim only, said plainly here.  ", EN), ["One claim only, said plainly here."]);
    assert!(split_proposal("   ", EN).is_empty());
    let asked = "Why do we forget? Memory traces decay over time. Retrieval practice slows that decay.";
    assert_eq!(split_proposal(asked, EN), ["Why do we forget? Memory traces decay over time.", "Retrieval practice slows that decay."]);
}

#[test]
fn a_split_keeps_links_and_caps_the_post_count() {
    let linked = split_proposal("Sleep consolidates memory traces [[Sleep]]. Spacing beats cramming for retention #learning.", EN);
    assert_eq!(linked.len(), 2);
    assert!(linked[0].contains("[[Sleep]]") && linked[1].contains("#learning"));
    let tagged = split_proposal("Cars pollute city air. Shops do better without them #urbanism", EN);
    assert_eq!(tagged[1], "Shops do better without them #urbanism", "no full stop after a trailing tag");
    let numbers = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten"];
    let text = numbers.iter().map(|n| format!("Claim number {n} is here.")).collect::<Vec<_>>().join(" ");
    let posts = split_proposal(&text, EN);
    assert_eq!(posts.len(), MAX_SPLIT);
    assert_eq!(posts[MAX_SPLIT - 1], "Claim number eight is here. Claim number nine is here. Claim number ten is here.");
}
