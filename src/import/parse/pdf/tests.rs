use super::*;
use crate::import::fixtures::tiny_pdf;

fn lines(text: &[&str]) -> Vec<String> {
    text.iter().map(|line| line.to_string()).collect()
}

#[test]
fn reads_the_text_layer_page_by_page_with_metadata() {
    let pdf = tiny_pdf(&[
        &["The first page has a line of text that is long enough", "and a second line that ends the paragraph."],
        &["Second page."],
    ]);
    let content = parse(&pdf).expect("pdf parses");
    assert_eq!(content.metadata.title.as_deref(), Some("A Tiny PDF"));
    assert_eq!(content.metadata.creators, ["Grace Hopper"]);
    assert_eq!(content.metadata.date.as_deref(), Some("1952-01-01"));
    assert_eq!(content.sections.len(), 2);
    let first: Vec<&str> = content.sections[0].blocks.iter().map(|b| b.text.as_str()).collect();
    assert_eq!(first, ["The first page has a line of text that is long enough and a second line that ends the paragraph."]);
    assert_eq!(content.sections[1].blocks[0].text, "Second page.");
}

#[test]
fn pages_without_text_are_noted() {
    let content = parse(&tiny_pdf(&[&["Only page with text."], &[]])).expect("pdf parses");
    assert_eq!(content.sections.len(), 1);
    assert_eq!(content.notes, [ParseNote::PagesWithoutText(1)]);
}

#[test]
fn not_a_pdf_is_unreadable() {
    assert!(matches!(parse(b"%PDF-1.4 broken"), Err(ParseError::Unreadable(_))));
}

#[test]
fn paragraphs_join_lines_and_split_at_short_sentence_ends() {
    let page = lines(&[
        "Chapter Two",
        "It was the best of times, it was the worst of times, it was",
        "the age of wisdom, it was the age of foolishness, it was the epo-",
        "ch of belief. It was the season of Light, it was the season of",
        "Darkness.",
        "There were a king with a large jaw and a queen with a plain face",
        "on the throne of England; there were a king with a large jaw.",
        "• A bulleted item that runs long enough to be a full line of text",
    ]);
    let paragraphs = rebuild_paragraphs(&page);
    assert_eq!(paragraphs[0], "Chapter Two");
    assert!(paragraphs[1].contains("the epoch of belief."), "{}", paragraphs[1]);
    assert!(paragraphs[1].ends_with("season of Darkness."));
    assert!(paragraphs[2].starts_with("There were a king"));
    assert!(paragraphs[3].starts_with("• A bulleted item"));
    assert_eq!(paragraphs.len(), 4);
}

#[test]
fn creation_dates_are_shortened() {
    assert_eq!(pdf_date("D:20240131120000Z"), "2024-01-31");
    assert_eq!(pdf_date("D:1999"), "1999");
    assert_eq!(pdf_date("yesterday"), "yesterday");
}

#[test]
fn blank_lines_always_break_paragraphs() {
    let page = lines(&["A full line of body text that runs on and on without", "", "ending, then a new paragraph begins here and keeps going on"]);
    assert_eq!(rebuild_paragraphs(&page).len(), 2);
}
