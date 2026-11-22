use super::*;
use crate::import::detect::detect_format;
use crate::import::fixtures;

fn parse_named(name: &str, bytes: &[u8]) -> ParsedDocument {
    parse_document(detect_format(name, None, bytes), bytes, name)
}

#[test]
fn every_synthetic_format_detects_and_parses() {
    let cases: Vec<(&str, Vec<u8>, Format)> = vec![
        ("book.epub", fixtures::epub(), Format::Epub),
        ("book.azw3", fixtures::mobi_book("<p>Hi.</p>", &fixtures::MobiOptions::default()), Format::Mobi),
        ("paper.pdf", fixtures::tiny_pdf(&[&["One line."]]), Format::Pdf),
        ("notes.docx", fixtures::docx(), Format::Docx),
        ("notes.odt", fixtures::odt(), Format::Odt),
        ("page.html", b"<html><body><p>Hi.</p></body></html>".to_vec(), Format::Html),
        ("idea.md", b"# Idea\n\nBody.".to_vec(), Format::Markdown),
        ("idea.txt", b"Body.".to_vec(), Format::PlainText),
    ];
    for (name, bytes, format) in cases {
        let doc = parse_named(name, &bytes);
        assert_eq!(doc.format, format, "{name}");
        assert!(doc.paragraph_count() > 0, "{name} has no text");
        assert_eq!(doc.facts.size_bytes, bytes.len() as u64);
        assert_eq!(doc.facts.sha256.len(), 64);
        assert!(doc.notes.is_empty(), "{name}: {:?}", doc.notes);
    }
}

#[test]
fn unreadable_files_fall_back_to_binary_with_a_note() {
    let broken = b"PK\x03\x04 not really a zip, but printable text inside it".to_vec();
    let doc = parse_named("broken.epub", &broken);
    assert_eq!(doc.format, Format::Unknown);
    assert!(matches!(doc.notes.first(), Some(ParseNote::Unreadable(_))));
    assert_eq!(doc.metadata.title.as_deref(), Some("broken"));
    assert!(doc.paragraph_count() > 0);
}

#[test]
fn random_bytes_still_yield_facts_and_a_title() {
    let noise: Vec<u8> = (0..4096u32).map(|i| (i.wrapping_mul(2654435761) >> 11) as u8).collect();
    let doc = parse_named("random.bin", &noise);
    assert_eq!(doc.format, Format::Unknown);
    assert_eq!(doc.facts.mime, "application/octet-stream");
    assert_eq!(doc.metadata.title.as_deref(), Some("random"));
    assert!(matches!(doc.notes.as_slice(), [ParseNote::ExtractedStrings(_)]));
}

#[test]
fn titles_from_file_names_are_readable() {
    assert_eq!(title_from_file_name("C:\\x\\moby_dick-whale.epub"), "moby dick whale");
    assert_eq!(title_from_file_name("README"), "README");
    assert_eq!(title_from_file_name(".bashrc"), ".bashrc");
}

#[test]
fn outlines_read_metadata_and_extent_without_the_text() {
    use crate::import::fixtures::{epub, tiny_pdf};
    let pdf = document_outline(Format::Pdf, &tiny_pdf(&[&["one"], &["two"], &[]])).expect("pdf outline");
    assert_eq!((pdf.metadata.title.as_deref(), pdf.pages, pdf.chapters), (Some("A Tiny PDF"), Some(3), None));
    assert_eq!(pdf.metadata.creators, ["Grace Hopper"]);
    let book = document_outline(Format::Epub, &epub()).expect("epub outline");
    assert_eq!((book.metadata.title.as_deref(), book.pages, book.chapters), (Some("A Tiny Book"), None, Some(2)));
    assert_eq!(book.metadata.creators, ["Ada Lovelace", "Charles Babbage"]);
    assert_eq!(document_outline(Format::Pdf, b"%PDF-broken"), None);
    assert_eq!(document_outline(Format::Docx, &epub()), None);
}
