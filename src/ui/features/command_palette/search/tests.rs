use super::{fuzzy_match, highlight_ranges};

fn score(query: &str, candidate: &str) -> i32 {
    fuzzy_match(query, candidate).unwrap_or_else(|| panic!("{query:?} should match {candidate:?}")).score
}

#[test]
fn missing_characters_do_not_match() {
    assert_eq!(fuzzy_match("xyz", "Mind Graph"), None);
    assert_eq!(fuzzy_match("gm", "Mind Graph"), None, "order matters");
}

#[test]
fn empty_query_matches_everything_with_zero_score() {
    let found = fuzzy_match("  ", "Timeline").expect("blank matches");
    assert_eq!((found.score, found.positions.len()), (0, 0));
}

#[test]
fn matching_ignores_case_and_query_whitespace() {
    let found = fuzzy_match("MIND gr", "mind graph").expect("matches");
    assert_eq!(found.positions, vec![0, 1, 2, 3, 5, 6]);
}

#[test]
fn match_tightens_onto_word_starts() {
    let found = fuzzy_match("mg", "Mind Graph").expect("matches");
    assert_eq!(found.positions, vec![0, 5]);
}

#[test]
fn backward_pass_prefers_the_tight_run() {
    // Leftmost greedy would take the first `t`; the tight match is "tim" in "Timeline".
    let found = fuzzy_match("tim", "a t Timeline").expect("matches");
    assert_eq!(found.positions, vec![4, 5, 6]);
}

#[test]
fn prefix_beats_middle() {
    assert!(score("set", "Settings") > score("set", "Reset view"));
}

#[test]
fn consecutive_beats_scattered() {
    assert!(score("graph", "Mind Graph") > score("graph", "go reach a path home"));
}

#[test]
fn word_starts_beat_inner_letters() {
    assert!(score("nt", "New thread") > score("nt", "Contents"));
}

#[test]
fn camel_case_humps_count_as_word_starts() {
    let found = fuzzy_match("mg", "openMindGraph").expect("matches");
    assert_eq!(found.positions, vec![4, 8]);
}

#[test]
fn exact_match_scores_highest() {
    assert!(score("vault", "Vault") > score("vault", "Vault settings"));
}

#[test]
fn shorter_gaps_score_higher() {
    assert!(score("ab", "a-b") > score("ab", "a----b"));
}

#[test]
fn positions_are_char_indices_for_non_ascii_text() {
    let found = fuzzy_match("ér", "Théorie").expect("matches");
    assert_eq!(found.positions, vec![2, 4]);
}

#[test]
fn highlight_merges_adjacent_characters_into_byte_ranges() {
    assert_eq!(highlight_ranges("Mind Graph", &[0, 1, 5]), vec![0..2, 5..6]);
}

#[test]
fn highlight_uses_byte_offsets_of_multibyte_characters() {
    // "é" is two bytes: char 2 spans bytes 2..4.
    assert_eq!(highlight_ranges("Théorie", &[2, 4]), vec![2..4, 5..6]);
}

#[test]
fn highlight_ignores_positions_past_the_end() {
    assert_eq!(highlight_ranges("ab", &[1, 9]), vec![1..2]);
    assert!(highlight_ranges("", &[0]).is_empty());
}
