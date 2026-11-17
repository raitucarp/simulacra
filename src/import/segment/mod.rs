//! Pure segmentation of a parsed document into units: whole chapters, paragraphs (body
//! blocks) or sentences, each with its position in the source.

mod locale;
mod sentences;

#[cfg(test)]
mod tests;

pub use locale::SentenceLocale;
pub use sentences::split_sentences;

use super::document::ParsedDocument;
use crate::domain::preferences::ImportSegmentation;

/// The unit size a document is cut into.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Granularity {
    Chapter,
    #[default]
    Paragraph,
    Sentence,
}

impl Granularity {
    pub const ALL: [Self; 3] = [Self::Chapter, Self::Paragraph, Self::Sentence];

    /// Stable id (locale keys).
    pub const fn id(self) -> &'static str {
        match self {
            Self::Chapter => "chapter",
            Self::Paragraph => "paragraph",
            Self::Sentence => "sentence",
        }
    }
}

impl From<ImportSegmentation> for Granularity {
    fn from(choice: ImportSegmentation) -> Self {
        match choice {
            ImportSegmentation::Chapter => Self::Chapter,
            ImportSegmentation::Paragraph => Self::Paragraph,
            ImportSegmentation::Sentence => Self::Sentence,
        }
    }
}

/// Where a unit sits in the source (all zero-based).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct UnitPosition {
    pub chapter: usize,
    /// Body block within the chapter (`None` for a whole chapter).
    pub paragraph: Option<usize>,
    /// Sentence within the paragraph (`None` unless cut by sentence).
    pub sentence: Option<usize>,
}

/// One importable unit of text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Unit {
    pub position: UnitPosition,
    pub text: String,
}

/// Separator between the paragraphs of a whole-chapter unit.
const CHAPTER_JOINER: &str = "\n\n";

/// Cuts `doc` into units of `granularity`, in reading order. Headings are never units.
pub fn segment_document(doc: &ParsedDocument, granularity: Granularity) -> Vec<Unit> {
    let locale = SentenceLocale::from_language_tag(doc.metadata.language.as_deref());
    doc.sections
        .iter()
        .enumerate()
        .flat_map(|(chapter, section)| {
            let paragraphs = section.body().enumerate();
            let units: Vec<Unit> = match granularity {
                Granularity::Chapter => {
                    let text = section.body().map(|block| block.text.as_str()).collect::<Vec<_>>().join(CHAPTER_JOINER);
                    let position = UnitPosition { chapter, ..UnitPosition::default() };
                    (!text.is_empty()).then_some(Unit { position, text }).into_iter().collect()
                }
                Granularity::Paragraph => paragraphs
                    .map(|(paragraph, block)| Unit { position: at(chapter, paragraph, None), text: block.text.clone() })
                    .collect(),
                Granularity::Sentence => paragraphs
                    .flat_map(|(paragraph, block)| {
                        let sentences = split_sentences(&block.text, locale).into_iter().enumerate();
                        sentences.map(move |(sentence, text)| Unit { position: at(chapter, paragraph, Some(sentence)), text: text.to_string() })
                    })
                    .collect(),
            };
            units
        })
        .collect()
}

fn at(chapter: usize, paragraph: usize, sentence: Option<usize>) -> UnitPosition {
    UnitPosition { chapter, paragraph: Some(paragraph), sentence }
}
