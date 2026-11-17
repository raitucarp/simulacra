//! Semantic mode results (sb B3): the shell embeds the query and hands back scored rows; the
//! palette shows them as one group with a similarity bar, or as word matches with a note.

use super::items::{PaletteGroup, PaletteItem};
use super::rank::{RankedItem, ResultGroup};

/// One scored result.
#[derive(Debug, Clone, PartialEq)]
pub struct SemanticRow {
    pub item: PaletteItem,
    /// The best matching passage.
    pub excerpt: String,
    /// Similarity in `[0, 1]`.
    pub score: f32,
}

/// How the rows were found.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SemanticSource {
    /// By meaning (embeddings).
    Meaning,
    /// By words, with the localized reason meaning was unavailable.
    Words { note: String },
}

/// What the palette shows in semantic mode.
#[derive(Debug, Clone, PartialEq)]
pub enum SemanticPhase {
    /// No query yet.
    Prompt,
    /// The query was sent (`query` is the text being searched).
    Searching { query: String },
    /// Results for `query`.
    Ready { query: String, source: SemanticSource, rows: Vec<SemanticRow> },
}

impl SemanticPhase {
    /// The scored rows (none unless ready).
    pub fn rows(&self) -> &[SemanticRow] {
        match self {
            Self::Ready { rows, .. } => rows,
            Self::Prompt | Self::Searching { .. } => &[],
        }
    }

    /// The query these results answer.
    pub fn query(&self) -> Option<&str> {
        match self {
            Self::Ready { query, .. } | Self::Searching { query } => Some(query),
            Self::Prompt => None,
        }
    }

    /// The result group over [`Self::rows`] (indices into the rows).
    pub fn groups(&self) -> Vec<ResultGroup> {
        let Self::Ready { source, rows, .. } = self else { return Vec::new() };
        if rows.is_empty() {
            return Vec::new();
        }
        let group = match source {
            SemanticSource::Meaning => PaletteGroup::Meaning,
            SemanticSource::Words { .. } => PaletteGroup::Words,
        };
        let items = (0..rows.len()).map(|index| RankedItem { index, positions: Vec::new() }).collect();
        vec![ResultGroup { group, items }]
    }
}
