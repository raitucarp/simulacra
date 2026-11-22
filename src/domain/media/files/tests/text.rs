//! L9: the first lines of a text file and the language that colours them.

use super::super::{text_preview, FilePreview, TEXT_PREVIEW_LINES};

fn preview(head: &[u8], total: u64, extension: Option<&str>) -> (String, Option<String>, bool) {
    match text_preview(head, total, extension) {
        FilePreview::Text { lines, lang, more } => (lines, lang, more),
        other => panic!("not a text preview: {other:?}"),
    }
}

#[test]
fn a_short_file_shows_whole_with_its_language() {
    let code = b"fn main() {\r\n\tprintln!(\"hi\");\r\n}\r\n";
    assert_eq!(preview(code, code.len() as u64, Some("RS")), ("fn main() {\n    println!(\"hi\");\n}".to_string(), Some("rs".to_string()), false));
    assert_eq!(preview(b"# Notes", 7, Some("md")).1, None, "prose has no grammar");
}

#[test]
fn long_files_stop_after_the_first_lines() {
    let text: String = (1..=20).map(|n| format!("line {n}\n")).collect();
    let (lines, _, more) = preview(text.as_bytes(), text.len() as u64, Some("txt"));
    assert_eq!(lines.lines().count(), TEXT_PREVIEW_LINES);
    assert!(lines.starts_with("line 1\n") && more);
    let (_, _, more) = preview(b"one line", 9_000, None);
    assert!(more, "the file goes on past what was read");
}

#[test]
fn a_cut_character_and_a_byte_order_mark_are_dropped() {
    let mut head = "\u{feff}héllo wö".as_bytes().to_vec();
    head.pop();
    let (lines, _, _) = preview(&head, 100, None);
    assert_eq!(lines, "héllo w");
    let wide = "x".repeat(500);
    assert_eq!(preview(wide.as_bytes(), 500, None).0.chars().count(), 160);
}
