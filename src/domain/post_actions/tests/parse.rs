use super::*;
use crate::domain::post_actions::parse::MAX_ANSWER_CHARS;

#[test]
fn headings_paragraphs_and_items_are_read_and_markdown_is_removed() {
    let raw = "```\n## **Strongest version**\nFreedom is not the absence\nof causes.\n\n## Why it holds\n- one *reason*\n2. two\n3) three\n```";
    let output = parse_output(ActionKind::Steelman, raw);
    assert_eq!(
        output.blocks,
        vec![
            OutputBlock::Heading("Strongest version".into()),
            OutputBlock::Paragraph("Freedom is not the absence of causes.".into()),
            OutputBlock::Heading("Why it holds".into()),
            OutputBlock::Item("one reason".into()),
            OutputBlock::Item("two".into()),
            OutputBlock::Item("three".into()),
        ]
    );
}

#[test]
fn blank_lines_separate_paragraphs() {
    let output = parse_output(ActionKind::ContinueThought, "First part\ncontinues.\n\nSecond part.");
    assert_eq!(output.blocks, vec![OutputBlock::Paragraph("First part continues.".into()), OutputBlock::Paragraph("Second part.".into())]);
}

#[test]
fn concept_maps_read_relations_and_keep_other_items() {
    let raw = "## Concepts\n- Freedom: acting from one's nature\n## Relations\n- Freedom -> requires -> Understanding\n- Necessity->grounds->Freedom\n- broken -> line";
    let output = parse_output(ActionKind::MapConcepts, raw);
    let relation = |from: &str, relation: &str, to: &str| OutputBlock::Relation(ConceptRelation { from: from.into(), relation: relation.into(), to: to.into() });
    assert!(output.blocks.contains(&relation("Freedom", "requires", "Understanding")));
    assert!(output.blocks.contains(&relation("Necessity", "grounds", "Freedom")));
    assert!(output.blocks.contains(&OutputBlock::Item("broken -> line".into())));
    assert!(output.plain_text().contains("• Freedom → requires → Understanding"));
}

#[test]
fn confidence_lines_become_scores() {
    let raw = "- Every event has a cause — 80% — physics agrees\n- Praise is useless — 120% — impossible\nOverall: 55%";
    let output = parse_output(ActionKind::ClaimConfidence, raw);
    assert_eq!(output.blocks[0], OutputBlock::Score { label: "Every event has a cause — physics agrees".into(), percent: 80 });
    assert_eq!(output.blocks[1], OutputBlock::Item("Praise is useless — 120% — impossible".into()), "out of range stays text");
    assert_eq!(output.blocks[2], OutputBlock::Score { label: "Overall".into(), percent: 55 });
}

#[test]
fn other_actions_never_read_scores_or_relations() {
    let output = parse_output(ActionKind::Socratic, "- Is it 50% true?\n- A -> B -> C?");
    assert!(output.blocks.iter().all(|block| matches!(block, OutputBlock::Item(_))));
}

#[test]
fn empty_and_runaway_answers_are_handled() {
    assert!(parse_output(ActionKind::Crux, "  \n```\n```\n").is_empty());
    let long = "word ".repeat(MAX_ANSWER_CHARS);
    let output = parse_output(ActionKind::Crux, &long);
    assert!(output.plain_text().chars().count() <= MAX_ANSWER_CHARS);
}

#[test]
fn plain_text_round_trips_the_reading_order() {
    let output = parse_output(ActionKind::Steelman, "## A\nprose\n- item");
    assert_eq!(output.plain_text(), "A\nprose\n\n• item");
}
