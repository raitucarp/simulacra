//! The Search page's pure parts (Q4): passages and their marks, chips, keys and selection.

use super::render::key_event_for_tests as key_event;
use super::{active_chips, snippet, stepped, SearchEvent, QUICK_FILTERS};
use crate::domain::post::lock::LOCKED_PLACEHOLDER;
use crate::domain::search_query::{HasFilter, PostQuery, QueryFilter};

fn marked(content: &str, query: &str) -> Vec<String> {
    let found = snippet(content, &PostQuery::parse(query));
    found.highlights.iter().map(|range| found.text[range.clone()].to_string()).collect()
}

#[test]
fn words_are_marked_whole_and_phrases_term_by_term() {
    assert_eq!(marked("Spinoza wrote the Ethics; spinozism followed.", "spino"), vec!["Spinoza", "spinozism"]);
    assert_eq!(marked("my sourdough starter recipe", "\"starter recipe\""), vec!["starter", "recipe"]);
    assert_eq!(marked("Ünïcode café notes", "café"), vec!["café"]);
    assert!(marked("nothing here", "has:image").is_empty(), "filters mark nothing");
}

#[test]
fn a_long_post_opens_near_its_first_match_on_a_word() {
    let content = format!("{} the needle sits here {}", "lorem ipsum dolor ".repeat(20), "sit amet ".repeat(40));
    let found = snippet(&content, &PostQuery::parse("needle"));
    assert!(found.text.starts_with('…') && found.text.ends_with('…'));
    assert!(found.text.contains("needle"));
    assert!(found.text.chars().count() <= super::snippet::SNIPPET_CHARS + 2);
    assert!(found.text.trim_start_matches('…').starts_with(|c: char| c.is_alphanumeric()), "cut at a word");
    let short = snippet("line one\n\nline   two", &PostQuery::parse("two"));
    assert_eq!(short.text, "line one line two");
}

#[test]
fn a_locked_post_shows_only_its_placeholder() {
    let found = snippet(LOCKED_PLACEHOLDER, &PostQuery::parse("orchard"));
    assert_eq!(found.text, LOCKED_PLACEHOLDER);
    assert!(found.highlights.is_empty());
}

#[test]
fn chips_read_as_the_query_syntax() {
    let chips = active_chips(&PostQuery::parse("rye has:image -from:\"Sam Lee\""));
    let labels: Vec<&str> = chips.iter().map(|chip| chip.label.as_str()).collect();
    assert_eq!(labels, vec!["has:image", "-from:\"Sam Lee\""]);
    assert!(chips[1].negated && !chips[0].negated);
    assert!(QUICK_FILTERS.contains(&QueryFilter::Has(HasFilter::Image)));
}

#[test]
fn arrows_move_the_selection_round_and_enter_confirms() {
    assert_eq!(stepped(None, 3, true), Some(0));
    assert_eq!(stepped(None, 3, false), Some(2));
    assert_eq!(stepped(Some(2), 3, true), Some(0));
    assert_eq!(stepped(Some(0), 3, false), Some(2));
    assert_eq!(stepped(Some(1), 0, true), None);
    assert_eq!(key_event("down"), Some(SearchEvent::Step(true)));
    assert_eq!(key_event("up"), Some(SearchEvent::Step(false)));
    assert_eq!(key_event("enter"), Some(SearchEvent::Confirm));
    assert_eq!(key_event("a"), None);
}
