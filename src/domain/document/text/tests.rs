use super::{blocks, outline, reading_minutes, BlockKind};

const SAMPLE: &str = "# Part One\nThe world is\neverything that is the case.\n\n> A quoted\n> line\n\n## 1.1 Facts\nFacts in logical space.\n#nothashtag heading\n\n\n### \n";

#[test]
fn text_splits_into_headings_paragraphs_and_quotes() {
    let found = blocks(SAMPLE);
    let kinds: Vec<BlockKind> = found.iter().map(|b| b.kind).collect();
    assert_eq!(
        kinds,
        [BlockKind::Heading(1), BlockKind::Paragraph, BlockKind::Quote, BlockKind::Heading(2), BlockKind::Paragraph, BlockKind::Paragraph]
    );
    assert_eq!(found[1].text, "The world is everything that is the case.");
    assert_eq!(found[2].text, "A quoted line");
    assert_eq!(found[4].text, "Facts in logical space. #nothashtag heading");
    assert_eq!(found[5].text, "###");
}

#[test]
fn the_outline_lists_headings_with_their_block() {
    let entries = outline(&blocks(SAMPLE));
    let summary: Vec<(&str, u8, usize)> = entries.iter().map(|e| (e.title.as_str(), e.level, e.block)).collect();
    assert_eq!(summary, [("Part One", 1, 0), ("1.1 Facts", 2, 3)]);
}

#[test]
fn reading_time_rounds_up() {
    assert_eq!((reading_minutes(0), reading_minutes(1), reading_minutes(231)), (0, 1, 2));
}
