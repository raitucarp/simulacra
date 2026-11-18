//! Reading a model answer into display blocks: headings, paragraphs, list items, concept
//! relations ("A -> relation -> B") and confidence scores ("NN%"). Markdown noise is removed,
//! so the result reads cleanly as plain text too.

use super::kind::ActionKind;
use super::output::{ActionOutput, ConceptRelation, OutputBlock};

/// Longest answer kept, in characters (a runaway reply is cut).
pub const MAX_ANSWER_CHARS: usize = 12_000;
const HEADING_MARKS: &[char] = &['#'];
const BULLET_MARKS: &[&str] = &["- ", "* ", "• ", "– "];
const RELATION_ARROW: &str = "->";
const EMPHASIS: &[char] = &['*', '_', '`'];
const FENCE: &str = "```";
const PERCENT: char = '%';
const MAX_PERCENT: u8 = 100;

/// Parses `raw` (the model's text) for an action of `kind`.
pub fn parse_output(kind: ActionKind, raw: &str) -> ActionOutput {
    let text: String = raw.chars().take(MAX_ANSWER_CHARS).collect();
    let blocks = text
        .lines()
        .filter(|line| !line.trim_start().starts_with(FENCE))
        .map(|line| classify(kind, line.trim()))
        .fold(Vec::new(), join_paragraphs);
    ActionOutput { kind, blocks: blocks.into_iter().flatten().collect() }
}

/// Appends `next` (`None` is a blank line), joining consecutive prose lines into one paragraph.
fn join_paragraphs(mut blocks: Vec<Option<OutputBlock>>, next: Option<OutputBlock>) -> Vec<Option<OutputBlock>> {
    if let (Some(Some(OutputBlock::Paragraph(text))), Some(OutputBlock::Paragraph(more))) = (blocks.last_mut(), &next) {
        text.push(' ');
        text.push_str(more);
        return blocks;
    }
    blocks.push(next);
    blocks
}

/// The block one line opens (`None` for a blank line).
fn classify(kind: ActionKind, line: &str) -> Option<OutputBlock> {
    if line.is_empty() {
        return None;
    }
    if line.starts_with(HEADING_MARKS) {
        return Some(OutputBlock::Heading(clean(line.trim_start_matches(HEADING_MARKS))));
    }
    let Some(item) = list_item(line) else {
        return Some(paragraph_or_score(kind, line));
    };
    let block = match kind {
        ActionKind::MapConcepts => relation(&item).map(OutputBlock::Relation),
        ActionKind::ClaimConfidence => score(&item),
        _ => None,
    };
    Some(block.unwrap_or(OutputBlock::Item(item)))
}

/// A closing "Overall: NN%" line is a score; any other line is prose.
fn paragraph_or_score(kind: ActionKind, line: &str) -> OutputBlock {
    let cleaned = clean(line);
    match kind {
        ActionKind::ClaimConfidence => score(&cleaned).unwrap_or(OutputBlock::Paragraph(cleaned)),
        _ => OutputBlock::Paragraph(cleaned),
    }
}

/// The text of a bulleted or numbered line.
fn list_item(line: &str) -> Option<String> {
    if let Some(rest) = BULLET_MARKS.iter().find_map(|mark| line.strip_prefix(mark)) {
        return Some(clean(rest));
    }
    let digits = line.chars().take_while(char::is_ascii_digit).count();
    let rest = &line[digits..];
    let numbered = digits > 0 && (rest.starts_with(". ") || rest.starts_with(") "));
    numbered.then(|| clean(&rest[2..]))
}

/// Emphasis marks removed and whitespace collapsed.
fn clean(text: &str) -> String {
    let stripped: String = text.chars().filter(|c| !EMPHASIS.contains(c)).collect();
    stripped.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn relation(item: &str) -> Option<ConceptRelation> {
    let parts: Vec<&str> = item.split(RELATION_ARROW).map(str::trim).collect();
    match parts.as_slice() {
        [from, relation, to] if !from.is_empty() && !to.is_empty() => {
            Some(ConceptRelation { from: from.to_string(), relation: relation.to_string(), to: to.to_string() })
        }
        _ => None,
    }
}

/// "label — NN% — reason" or "Overall: NN%": the label (reason kept) and the percentage.
fn score(text: &str) -> Option<OutputBlock> {
    let at = text.find(PERCENT)?;
    let digits: String = text[..at].chars().rev().take_while(char::is_ascii_digit).collect::<Vec<_>>().into_iter().rev().collect();
    let percent: u8 = digits.parse().ok().filter(|value| *value <= MAX_PERCENT)?;
    let before = text[..at - digits.len()].trim_end_matches([' ', '—', '-', ':', '–']).trim();
    let after = text[at + PERCENT.len_utf8()..].trim_start_matches([' ', '—', '-', ':', '–']).trim();
    let label = [before, after].iter().filter(|part| !part.is_empty()).copied().collect::<Vec<_>>().join(" — ");
    Some(OutputBlock::Score { label, percent })
}
