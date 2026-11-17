use super::*;
use crate::import::document::BlockKind;
use crate::import::parse::Content;

fn texts(content: &Content) -> Vec<String> {
    content.sections.iter().flat_map(|s| s.blocks.iter()).map(|b| b.text.clone()).collect()
}

#[test]
fn decodes_boms_utf8_and_windows_1252() {
    assert_eq!(decode_text(b"\xEF\xBB\xBFhello"), "hello");
    assert_eq!(decode_text(b"\xFF\xFEh\0i\0"), "hi");
    assert_eq!(decode_text(b"\xFE\xFF\0h\0i"), "hi");
    assert_eq!(decode_text(b"caf\xE9 \x93quoted\x94"), "café “quoted”");
    assert_eq!(decode_text("naïve".as_bytes()), "naïve");
}

#[test]
fn plain_text_joins_wrapped_prose_and_keeps_verse() {
    let text = "It is a truth universally acknowledged, that a single man in possession\nof a good fortune, must be in want of a wife.\n\n\nRoses are red,\nViolets are blue.\n";
    let content = parse_plain(text);
    assert_eq!(
        texts(&content),
        [
            "It is a truth universally acknowledged, that a single man in possession of a good fortune, must be in want of a wife.",
            "Roses are red,\nViolets are blue.",
        ]
    );
}

#[test]
fn markdown_blocks_and_inline_markup() {
    let md = "---\ntitle: x\n---\n# Notes\n\nSome **bold** and [a link](https://example.org) here\ncontinued.\n\n> A quote\n\n- item one\n2. item two\n\n```\nlet x = 1;\n```\n## Next\nEnd.";
    let content = parse_markdown(md);
    assert_eq!(content.metadata.title.as_deref(), Some("Notes"));
    let blocks: Vec<(BlockKind, String)> = content.sections.iter().flat_map(|s| s.blocks.iter()).map(|b| (b.kind, b.text.clone())).collect();
    assert_eq!(
        blocks,
        vec![
            (BlockKind::Heading(1), "Notes".into()),
            (BlockKind::Paragraph, "Some bold and a link here continued.".into()),
            (BlockKind::Quote, "A quote".into()),
            (BlockKind::ListItem, "item one".into()),
            (BlockKind::ListItem, "item two".into()),
            (BlockKind::Preformatted, "let x = 1;".into()),
            (BlockKind::Heading(2), "Next".into()),
            (BlockKind::Paragraph, "End.".into()),
        ]
    );
}

#[test]
fn rtf_text_skips_tables_and_reads_info() {
    let rtf = r"{\rtf1\ansi{\fonttbl{\f0 Times;}}{\colortbl;\red0\green0\blue0;}{\info{\title Letters}{\author Jane Austen}}
\pard Dear Cassandra,\par I have \'e9t\'e9 here \u8212? truly\emdash yours.\par{\*\generator Writer;}\line}";
    let content = parse_rtf(rtf);
    assert_eq!(content.metadata.title.as_deref(), Some("Letters"));
    assert_eq!(content.metadata.creators, ["Jane Austen"]);
    assert_eq!(texts(&content), ["Dear Cassandra,", "I have été here — truly—yours."]);
}

#[test]
fn csv_rows_become_column_value_blocks() {
    let csv = "name,quote\n\"Seneca\",\"Luck is what happens when preparation meets opportunity\"\nEpictetus,\"He said \"\"no\"\"\"\n,\n";
    let content = parse_delimited(csv, ',');
    assert_eq!(
        texts(&content),
        ["name: Seneca · quote: Luck is what happens when preparation meets opportunity", "name: Epictetus · quote: He said \"no\""]
    );
    assert_eq!(texts(&parse_delimited("a\tb\n1\t2", '\t')), ["a: 1 · b: 2"]);
}

#[test]
fn json_arrays_and_objects_become_blocks() {
    let content = parse_json(br#"[{"text": "Know thyself", "by": "Delphi"}, "plain", null]"#).expect("json");
    assert_eq!(texts(&content), ["text: Know thyself · by: Delphi", "plain"], "keys keep their order");
    let object = parse_json(br#"{"title": "Ideas", "items": ["a", "b"]}"#).expect("json");
    assert_eq!(object.metadata.title.as_deref(), Some("Ideas"));
    assert_eq!(texts(&object), ["title: Ideas", "items: a, b"]);
    assert!(parse_json(b"{oops").is_err());
}
