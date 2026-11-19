//! A parsed answer: display blocks (headings, paragraphs, list items, concept relations and
//! confidence scores) and their plain-text form.

use super::kind::ActionKind;

/// A concept map edge.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConceptRelation {
    pub from: String,
    pub relation: String,
    pub to: String,
}

/// One display block of an answer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OutputBlock {
    Heading(String),
    Paragraph(String),
    Item(String),
    Relation(ConceptRelation),
    /// A rated claim (the line without its score, and the score in percent).
    Score { label: String, percent: u8 },
}

/// A parsed answer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActionOutput {
    pub kind: ActionKind,
    pub blocks: Vec<OutputBlock>,
}

impl ActionOutput {
    /// Whether the answer said nothing readable.
    pub fn is_empty(&self) -> bool {
        self.blocks.is_empty()
    }

    /// The answer as plain text (copying, replying and saving use this).
    pub fn plain_text(&self) -> String {
        let lines: Vec<String> = self.blocks.iter().map(block_text).collect();
        lines.join("\n").trim().to_string()
    }
}

fn block_text(block: &OutputBlock) -> String {
    match block {
        OutputBlock::Heading(text) => format!("\n{text}"),
        OutputBlock::Paragraph(text) => format!("{text}\n"),
        OutputBlock::Item(text) => format!("• {text}"),
        OutputBlock::Relation(r) => format!("• {} → {} → {}", r.from, r.relation, r.to),
        OutputBlock::Score { label, percent } => format!("• {label} — {percent}%"),
    }
}
