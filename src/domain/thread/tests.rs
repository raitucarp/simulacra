use super::*;

fn lens(chunks: &[String]) -> Vec<usize> {
    chunks.iter().map(|c| c.chars().count()).collect()
}

#[test]
fn short_text_is_one_trimmed_chunk() {
    assert_eq!(split_into_thread("  hello world  ", 280), vec!["hello world"]);
}

#[test]
fn blank_text_yields_no_chunks() {
    assert!(split_into_thread("   \n ", 280).is_empty());
}

#[test]
fn splits_on_sentence_boundaries_first() {
    let text = "One two three. Four five six! Seven eight nine?";
    let chunks = split_into_thread(text, 30);
    assert_eq!(chunks, vec!["One two three. Four five six!", "Seven eight nine?"]);
}

#[test]
fn falls_back_to_word_boundaries_for_long_sentences() {
    let text = "alpha beta gamma delta epsilon zeta eta theta";
    let chunks = split_into_thread(text, 12);
    assert_eq!(chunks, vec!["alpha beta", "gamma delta", "epsilon zeta", "eta theta"]);
    assert!(lens(&chunks).iter().all(|&n| n <= 12));
}

#[test]
fn hard_splits_words_longer_than_the_limit() {
    let chunks = split_into_thread("abcdefghij xy", 4);
    assert_eq!(chunks, vec!["abcd", "efgh", "ij", "xy"]);
}

#[test]
fn keeps_line_breaks_between_paragraphs() {
    let text = "First paragraph.\nSecond one.\nThird paragraph here.";
    let chunks = split_into_thread(text, 30);
    assert_eq!(chunks, vec!["First paragraph.\nSecond one.", "Third paragraph here."]);
}

#[test]
fn packs_sentences_and_keeps_decimals_intact() {
    let text = "Wait... pi is 3.14 roughly. Next sentence goes here.";
    let chunks = split_into_thread(text, 30);
    assert_eq!(chunks, vec!["Wait... pi is 3.14 roughly.", "Next sentence goes here."]);
}

#[test]
fn every_chunk_fits_the_post_limit_and_no_words_are_lost() {
    let sentence = "Socrates asks what justice is, and Thrasymachus answers with power. ";
    let text = sentence.repeat(12);
    let chunks = split_into_thread(&text, MAX_POST_CHARS);
    assert!(chunks.len() > 1);
    assert!(lens(&chunks).iter().all(|&n| n <= MAX_POST_CHARS));
    let rejoined: Vec<&str> = chunks.iter().flat_map(|c| c.split_whitespace()).collect();
    assert_eq!(rejoined, text.split_whitespace().collect::<Vec<_>>());
}

#[test]
fn counts_characters_not_bytes() {
    let text = "ééééé ééééé";
    assert_eq!(split_into_thread(text, 5), vec!["ééééé", "ééééé"]);
}

#[test]
fn draft_issue_flags_empty_and_too_long() {
    assert_eq!(draft_issue("   ", false), Some(DraftIssue::Empty));
    assert_eq!(draft_issue("   ", true), None);
    assert_eq!(draft_issue("ok", false), None);
    let long = "a".repeat(MAX_POST_CHARS + 3);
    let issue = draft_issue(&long, false).expect("too long");
    assert_eq!(issue, DraftIssue::TooLong { actual: MAX_POST_CHARS + 3, max: MAX_POST_CHARS });
    assert_eq!(issue.overflow(), 3);
}

#[test]
fn draft_length_counts_visible_text_not_markup() {
    let fits = format!("**{}**", "a".repeat(MAX_POST_CHARS));
    assert_eq!(draft_issue(&fits, false), None);
    let over = format!("*{}*", "a".repeat(MAX_POST_CHARS + 2));
    assert_eq!(draft_issue(&over, false), Some(DraftIssue::TooLong { actual: MAX_POST_CHARS + 2, max: MAX_POST_CHARS }));
}

#[test]
fn first_thread_problem_reports_the_first_blocking_draft() {
    let long = "b".repeat(MAX_POST_CHARS + 1);
    let drafts = ["fine", long.as_str(), ""];
    let problem = first_thread_problem(&drafts, false).expect("problem");
    assert_eq!(problem.index, 1);
    assert!(matches!(problem.issue, DraftIssue::TooLong { .. }));
    assert_eq!(first_thread_problem(&["a", "b"], false), None);
}

#[test]
fn only_the_first_draft_may_rely_on_media() {
    assert_eq!(first_thread_problem(&["", "text"], true), None);
    let problem = first_thread_problem(&["text", ""], true).expect("second empty");
    assert_eq!(problem, ThreadProblem { index: 1, issue: DraftIssue::Empty });
    let empty: [&str; 0] = [];
    assert_eq!(first_thread_problem(&empty, true).map(|p| p.issue), Some(DraftIssue::Empty));
}
