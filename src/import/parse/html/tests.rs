use super::*;
use crate::import::document::BlockKind;

const CHAPTER: &str = r#"<?xml version="1.0" encoding="utf-8"?>
<!DOCTYPE html>
<html xmlns="http://www.w3.org/1999/xhtml" xml:lang="en-GB">
<head><title>Loomings</title><style>p { margin: 0 }</style></head>
<body>
  <section epub:type="chapter">
    <h2 epub:type="title">Chapter 1. <span>Loomings</span></h2>
    <p>Call me Ishmael. Some years
       ago&#8212;never mind how long.</p>
    <blockquote><p>Quoted &amp; indented.</p></blockquote>
    <ul><li>One</li><li>Two</li></ul>
    <p>Line one<br/>Line two</p>
    <pre>  keep   spacing
  here</pre>
    <svg><text>not prose</text></svg>
  </section>
</body></html>"#;

#[test]
fn keeps_headings_paragraphs_quotes_lists_and_line_breaks() {
    let html = html_blocks(CHAPTER);
    assert_eq!(html.title.as_deref(), Some("Loomings"));
    assert_eq!(html.language.as_deref(), Some("en-GB"));
    let kinds: Vec<(BlockKind, &str)> = html.blocks.iter().map(|b| (b.kind, b.text.as_str())).collect();
    assert_eq!(
        kinds,
        vec![
            (BlockKind::Heading(2), "Chapter 1. Loomings"),
            (BlockKind::Paragraph, "Call me Ishmael. Some years ago—never mind how long."),
            (BlockKind::Quote, "Quoted & indented."),
            (BlockKind::ListItem, "One"),
            (BlockKind::ListItem, "Two"),
            (BlockKind::Paragraph, "Line one\nLine two"),
            (BlockKind::Preformatted, "  keep   spacing\n  here"),
        ]
    );
}

#[test]
fn standalone_pages_split_into_sections_at_top_headings() {
    let src = "<html><body><p>Intro</p><h1>One</h1><p>a</p><h2>Sub</h2><p>b</p><h1>Two</h1><p>c</p></body></html>";
    let content = parse(src);
    let titles: Vec<Option<&str>> = content.sections.iter().map(|s| s.title.as_deref()).collect();
    assert_eq!(titles, [None, Some("One"), Some("Two")]);
    assert_eq!(content.sections[1].blocks.len(), 4);
}

#[test]
fn malformed_markup_still_yields_text() {
    let html = html_blocks("<p>Unclosed <b>bold<p>Next & last <3");
    let texts: Vec<&str> = html.blocks.iter().map(|b| b.text.as_str()).collect();
    assert_eq!(texts, ["Unclosed bold", "Next & last <3"]);
}
