use super::*;
use crate::domain::post::PostExtras;

const DOC: &str = "Memory is reconstructive. Each recall rewrites the trace a little.";

#[test]
fn a_quote_keeps_the_words_where_they_were_and_the_document_hash() {
    let quote = PinnedQuote::of("doc1", "On memory", DOC, 25..66).expect("quote");
    assert_eq!((quote.passage.as_str(), quote.start), ("Each recall rewrites the trace a little.", 26));
    assert_eq!(quote.source_hash, source_hash(DOC));
    assert_eq!(PinnedQuote::of("doc1", "On memory", DOC, 60..400), None, "out of the text");
    assert_eq!(PinnedQuote::of("doc1", "On memory", "   ", 0..3), None, "nothing to quote");
    let long = "x".repeat(MAX_QUOTE_CHARS + 1);
    assert_eq!(PinnedQuote::of("doc1", "Long", &long, 0..MAX_QUOTE_CHARS + 1), None);
}

#[test]
fn drift_tells_an_unchanged_source_from_a_changed_one_and_a_lost_passage() {
    let quote = PinnedQuote::of("doc1", "On memory", DOC, 25..66).expect("quote");
    assert_eq!(quote.drift(Some(DOC)), QuoteDrift::Intact);
    assert!(!QuoteDrift::Intact.flagged());
    let edited = format!("Preface. {DOC}");
    assert_eq!(quote.drift(Some(&edited)), QuoteDrift::SourceChanged);
    assert_eq!(quote.drift(Some("Memory is a recording.")), QuoteDrift::PassageGone);
    assert_eq!(quote.drift(None), QuoteDrift::SourceMissing);
    assert!([QuoteDrift::SourceChanged, QuoteDrift::PassageGone, QuoteDrift::SourceMissing].iter().all(|drift| drift.flagged()));
}

#[test]
fn repinning_follows_the_words_and_takes_the_new_hash() {
    let quote = PinnedQuote::of("doc1", "On memory", DOC, 25..66).expect("quote");
    let edited = format!("Preface. {DOC}");
    let again = quote.repinned(&edited).expect("still there");
    assert_eq!((again.start, again.drift(Some(&edited))), (35, QuoteDrift::Intact));
    assert_eq!(quote.repinned("Memory is a recording."), None);
}

#[test]
fn the_quote_round_trips_in_the_extras_and_older_rows_have_none() {
    let extras = PostExtras { pinned: PinnedQuote::of("doc1", "On memory", DOC, 0..25).map(Box::new), ..Default::default() };
    let json = serde_json::to_string(&extras).expect("json");
    assert!(json.starts_with(r#"{"pinned":{"doc_id":"doc1""#), "{json}");
    assert_eq!(serde_json::from_str::<PostExtras>(&json).expect("back"), extras);
    assert_eq!(serde_json::from_str::<PostExtras>("{}").expect("old").pinned, None);
    assert!(!extras.has_details());
}
