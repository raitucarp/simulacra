use super::*;

#[test]
fn passages_are_sentences_and_lines_trimmed() {
    let text = "First idea. Second one!\nA line without end\n\n  Last?  ";
    let spans: Vec<String> = passages(text).into_iter().map(|range| span_text(text, range)).collect();
    assert_eq!(spans, vec!["First idea.", "Second one!", "A line without end", "Last?"]);
    let cjk = "実体は一つ。無限である。";
    assert_eq!(passages(cjk).into_iter().map(|range| span_text(cjk, range)).collect::<Vec<_>>(), vec!["実体は一つ。", "無限である。"]);
    assert_eq!(passages("v1.2 is out"), vec![0..11], "a dot inside a word ends nothing");
    assert!(passages("   ").is_empty());
}

#[test]
fn an_anchor_quotes_a_trimmed_span_and_refuses_empty_or_long_ones() {
    let text = "Alpha beta. Gamma delta.";
    let anchor = SpanAnchor::of(text, 11..24, Some(SemVer::ORIGINAL)).expect("anchor");
    assert_eq!((anchor.quote.as_str(), anchor.start), ("Gamma delta.", 12));
    assert!(SpanAnchor::of(text, 5..6, None).is_none(), "only a space");
    assert!(SpanAnchor::of(text, 20..99, None).is_none(), "past the end");
    let long = "x".repeat(MAX_ANCHOR_CHARS + 1);
    assert!(SpanAnchor::of(&long, 0..long.chars().count(), None).is_none());
}

#[test]
fn an_anchor_is_found_where_it_is_where_it_moved_or_reported_gone() {
    let anchor = SpanAnchor::of("Alpha beta. Gamma delta.", 12..24, None).expect("anchor");
    assert_eq!(anchor.place("Alpha beta. Gamma delta."), AnchorPlace::Exact(12..24));
    assert_eq!(anchor.place("New start. Alpha beta. Gamma delta."), AnchorPlace::Moved(23..35));
    assert_eq!(anchor.place("Gamma delta. x Gamma delta."), AnchorPlace::Moved(15..27), "the occurrence nearest the old place");
    assert_eq!(anchor.place("Entirely rewritten."), AnchorPlace::Gone);
    assert_eq!(anchor.place(""), AnchorPlace::Gone);
}

#[test]
fn an_anchor_round_trips_through_json_and_old_replies_have_none() {
    let anchor = SpanAnchor { quote: "Gamma".into(), start: 3, version: Some(SemVer::new(1, 2, 0)) };
    let json = serde_json::to_string(&anchor).expect("json");
    assert_eq!(serde_json::from_str::<SpanAnchor>(&json).expect("back"), anchor);
    let extras: crate::domain::post::PostExtras = serde_json::from_str("{}").expect("old extras");
    assert!(extras.anchor.is_none());
}
