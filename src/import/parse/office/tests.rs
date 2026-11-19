use super::*;
use crate::import::document::BlockKind;
use crate::import::fixtures;

fn kinds(content: &Content) -> Vec<(BlockKind, String)> {
    content.sections.iter().flat_map(|s| s.blocks.iter()).map(|b| (b.kind, b.text.clone())).collect()
}

#[test]
fn docx_paragraph_styles_become_block_kinds() {
    let content = parse_docx(&fixtures::docx()).expect("docx parses");
    assert_eq!(content.metadata.title.as_deref(), Some("Field Notes"));
    assert_eq!(content.metadata.creators, ["Rosalind Franklin"]);
    assert_eq!(content.metadata.language.as_deref(), Some("en-US"));
    assert_eq!(content.metadata.date.as_deref(), Some("2024-05-01T10:00:00Z"));
    let titles: Vec<Option<&str>> = content.sections.iter().map(|s| s.title.as_deref()).collect();
    assert_eq!(titles, [Some("Field Notes"), Some("Monday"), Some("Tuesday")]);
    let blocks = kinds(&content);
    assert!(blocks.contains(&(BlockKind::Paragraph, "Rain all day. Wrote & read.".into())));
    assert!(blocks.contains(&(BlockKind::ListItem, "Buy ink".into())));
    assert!(blocks.contains(&(BlockKind::Quote, "Line one\nLine two".into())));
    assert!(blocks.contains(&(BlockKind::Paragraph, "Sun. Walked.".into())));
}

#[test]
fn docx_style_names_map_to_levels() {
    assert_eq!(docx::style_kind("heading 3", false), BlockKind::Heading(3));
    assert_eq!(docx::style_kind("Heading9", false), BlockKind::Heading(6));
    assert_eq!(docx::style_kind("IntenseQuote", false), BlockKind::Quote);
    assert_eq!(docx::style_kind("Normal", true), BlockKind::ListItem);
}

#[test]
fn odt_headings_paragraphs_and_lists() {
    let content = parse_odt(&fixtures::odt()).expect("odt parses");
    assert_eq!(content.metadata.title.as_deref(), Some("Seasons"));
    assert_eq!(content.metadata.creators, ["Gregor Mendel"]);
    assert_eq!(content.metadata.language.as_deref(), Some("de"));
    let blocks = kinds(&content);
    assert_eq!(
        blocks,
        vec![
            (BlockKind::Heading(1), "Spring".into()),
            (BlockKind::Paragraph, "Blossoms everywhere.\nBees too.".into()),
            (BlockKind::ListItem, "Plant beans".into()),
            (BlockKind::Heading(1), "Summer".into()),
            (BlockKind::Paragraph, "Heat.".into()),
        ]
    );
    assert_eq!(content.sections.len(), 2);
}

#[test]
fn packages_without_their_main_part_are_unreadable() {
    let empty = fixtures::zip_of(&[("other.xml", "<x/>")]);
    assert!(matches!(parse_docx(&empty), Err(ParseError::Unreadable(_))));
    assert!(matches!(parse_odt(&empty), Err(ParseError::Unreadable(_))));
}
