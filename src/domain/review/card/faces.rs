//! The prompt and the answer side of a card as styled parts, so a cloze blank can be drawn
//! as a pill and the revealed answer highlighted.

use super::cloze::{parse_cloze, ClozePart};
use super::{CardKind, ConceptDirection};

/// One run of a card face.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FacePart {
    Text(String),
    /// A hidden deletion, with its hint if it has one.
    Blank(Option<String>),
    /// The answer, emphasised.
    Answer(String),
}

/// Both sides of a card.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CardFaces {
    pub prompt: Vec<FacePart>,
    /// Shown under the prompt once revealed.
    pub answer: Vec<FacePart>,
}

/// The faces of a cloze card: the target deletions blanked (prompt) or highlighted (answer).
fn cloze_faces(text: &str, ordinal: u32) -> CardFaces {
    let parts = parse_cloze(text);
    let render = |reveal: bool| -> Vec<FacePart> {
        parts
            .iter()
            .map(|part| match part {
                ClozePart::Plain(text) => FacePart::Text(text.clone()),
                ClozePart::Deletion { ordinal: n, answer, hint } if *n == ordinal => {
                    if reveal { FacePart::Answer(answer.clone()) } else { FacePart::Blank(hint.clone()) }
                }
                ClozePart::Deletion { answer, .. } => FacePart::Text(answer.clone()),
            })
            .collect()
    };
    CardFaces { prompt: render(false), answer: render(true) }
}

/// The faces of `kind`.
pub fn card_faces(kind: &CardKind) -> CardFaces {
    let side = |text: &str| vec![FacePart::Text(text.trim().to_string())];
    let answer = |text: &str| vec![FacePart::Answer(text.trim().to_string())];
    match kind {
        CardKind::Basic { front, back } => CardFaces { prompt: side(front), answer: answer(back) },
        CardKind::Cloze { text, ordinal } => cloze_faces(text, *ordinal),
        CardKind::Concept { term, definition, direction: ConceptDirection::TermToDefinition } => {
            CardFaces { prompt: side(term), answer: answer(definition) }
        }
        CardKind::Concept { term, definition, direction: ConceptDirection::DefinitionToTerm } => {
            CardFaces { prompt: side(definition), answer: answer(term) }
        }
    }
}

impl CardFaces {
    /// The prompt as plain text (a blank becomes an ellipsis).
    pub fn prompt_text(&self) -> String {
        self.prompt
            .iter()
            .map(|part| match part {
                FacePart::Text(text) | FacePart::Answer(text) => text.as_str(),
                FacePart::Blank(_) => "…",
            })
            .collect()
    }
}
