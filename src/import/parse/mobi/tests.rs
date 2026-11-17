use super::*;
use crate::import::fixtures::{mobi_book, palmdoc_compress, MobiOptions};

fn long_book() -> String {
    let chapter = |n: usize| {
        let paragraphs: String = (0..40)
            .map(|p| format!("<p>Chapter {n}, paragraph {p}: the monster spoke — «über» naïve words.</p>"))
            .collect();
        format!("<h2>Chapter {n}</h2>{paragraphs}<mbp:pagebreak/>")
    };
    format!("<html><head><guide></guide></head><body>{}</body></html>", (1..=4).map(chapter).collect::<String>())
}

#[test]
fn palmdoc_round_trips_every_code() {
    let samples = ["", "aaaaaaaaaaaaaaaaaaaaaaaa", "the cat sat on the mat, the cat sat", "\u{1}\u{8} bytes ünïcödé 中文 text"];
    for sample in samples {
        let compressed = palmdoc_compress(sample.as_bytes());
        assert_eq!(palmdoc::decompress(&compressed), sample.as_bytes(), "sample {sample:?}");
    }
    assert!(palmdoc_compress(b"abcabcabcabcabcabc").len() < 18);
}

#[test]
fn malformed_back_references_do_not_panic() {
    assert_eq!(palmdoc::decompress(&[0x80, 0xFF]), Vec::<u8>::new());
    assert_eq!(palmdoc::decompress(&[b'a', 0x80]), b"a");
    assert_eq!(palmdoc::decompress(&[0x08, b'x']), b"x");
}

#[test]
fn trailing_entries_are_stripped() {
    let record = [b'h', b'i', 0xEE, 0x01, b'T', b'B', 0x83];
    assert_eq!(palmdoc::strip_trailing_entries(&record, 0b11), b"hi");
    assert_eq!(palmdoc::strip_trailing_entries(&record, 0), &record[..]);
}

#[test]
fn reads_a_compressed_book_across_records() {
    let html = long_book();
    let content = parse(&mobi_book(&html, &MobiOptions::default())).expect("mobi parses");
    assert_eq!(content.metadata.title.as_deref(), Some("Synthetic Mobi"));
    assert_eq!(content.metadata.creators, ["Mary Shelley"]);
    assert_eq!(content.metadata.language.as_deref(), Some("en"));
    assert_eq!(content.sections.len(), 4);
    assert_eq!(content.sections[3].title.as_deref(), Some("Chapter 4"));
    let last = content.sections[3].body().last().expect("paragraphs");
    assert_eq!(last.text, "Chapter 4, paragraph 39: the monster spoke — «über» naïve words.");
    assert_eq!(content.sections.iter().map(|s| s.body().count()).sum::<usize>(), 160);
}

#[test]
fn uncompressed_books_without_trailing_entries_read_too() {
    let options = MobiOptions { compression: 1, trailing_entries: false, ..MobiOptions::default() };
    let content = parse(&mobi_book("<p>Plain record.</p>", &options)).expect("mobi parses");
    assert_eq!(content.sections[0].blocks[0].text, "Plain record.");
}

#[test]
fn huffman_books_are_reported_unsupported() {
    let options = MobiOptions { compression: 17480, ..MobiOptions::default() };
    assert_eq!(parse(&mobi_book("<p>x</p>", &options)), Err(ParseError::Unsupported("HUFF/CDIC")));
    assert!(parse(b"too short").is_err());
}
