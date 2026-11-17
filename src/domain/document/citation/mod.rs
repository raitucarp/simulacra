//! Citation formatting in three generic styles, as pure functions of a catalogue record.
//!
//! - **Author–date:** name and year lead ("Spinoza, B. (1677). *Ethics*.").
//! - **Notes and bibliography:** full names, the year at the end ("Spinoza, Baruch. *Ethics*. 1677.").
//! - **Numeric:** a bracketed number and initials first ("[1] B. Spinoza, *Ethics*, 1677.").
//!
//! A citation is a list of parts; emphasised parts (whole-work titles) render in italics and
//! are copied as plain text.

mod author_date;
mod names;
mod notes;
mod numeric;
#[cfg(test)]
mod tests;

use super::keyed::keyed_enum;
use super::record::{CatalogueRecord, CreatorRole, IdentifierScheme};

keyed_enum! {
    /// A citation style.
    CitationStyle {
        AuthorDate => "author_date",
        NotesBibliography => "notes_bibliography",
        Numeric => "numeric",
    }
}

/// One run of citation text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CitationPart {
    pub text: String,
    /// Rendered in italics (titles of whole works).
    pub emphasis: bool,
}

/// A formatted citation.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Citation {
    pub parts: Vec<CitationPart>,
}

impl Citation {
    fn push(mut self, text: impl Into<String>, emphasis: bool) -> Self {
        let text = text.into();
        if !text.is_empty() {
            self.parts.push(CitationPart { text, emphasis });
        }
        self
    }

    fn plain(self, text: impl Into<String>) -> Self {
        self.push(text, false)
    }

    fn emphasised(self, text: impl Into<String>) -> Self {
        self.push(text, true)
    }

    /// The citation as plain text (for copying).
    pub fn text(&self) -> String {
        self.parts.iter().map(|part| part.text.as_str()).collect()
    }
}

/// What a citation is formatted from.
#[derive(Debug, Clone, Copy)]
pub struct CitationSource<'a> {
    pub title: &'a str,
    pub record: &'a CatalogueRecord,
}

/// Formats `source` in `style`; `number` is the reference number of the numeric style.
pub fn cite(source: CitationSource<'_>, style: CitationStyle, number: usize) -> Citation {
    match style {
        CitationStyle::AuthorDate => author_date::cite(source),
        CitationStyle::NotesBibliography => notes::cite(source),
        CitationStyle::Numeric => numeric::cite(source, number),
    }
}

/// Resolver prefix of a DOI link.
const DOI_RESOLVER: &str = "https://doi.org/";

impl CitationSource<'_> {
    /// "Title: Subtitle", trimmed of a closing full stop.
    fn full_title(&self) -> String {
        let title = self.title.trim().trim_end_matches('.');
        match self.record.subtitle.trim() {
            "" => title.to_string(),
            subtitle => format!("{title}: {subtitle}"),
        }
    }

    fn year(&self) -> Option<i32> {
        self.record.published_year()
    }

    fn publisher(&self) -> Option<&str> {
        Some(self.record.publisher.trim()).filter(|p| !p.is_empty())
    }

    fn creators_in(&self, role: CreatorRole) -> Vec<&str> {
        self.record.creators.iter().filter(|c| c.role == role).map(|c| c.name.as_str()).collect()
    }

    /// A DOI as a resolver link, or the URL; ISBNs are not linked.
    fn link(&self) -> Option<String> {
        let id = &self.record.identifier;
        match id.scheme.filter(|_| !id.is_empty())? {
            IdentifierScheme::Doi => Some(format!("{DOI_RESOLVER}{}", id.value.trim_start_matches(DOI_RESOLVER))),
            IdentifierScheme::Url => Some(id.value.clone()),
            IdentifierScheme::Isbn => None,
        }
    }

    fn is_whole_work(&self) -> bool {
        self.record.kind.is_whole_work()
    }
}

/// Appends `.` unless `text` already ends in terminal punctuation.
fn closed(text: &str) -> String {
    if text.ends_with(['.', '?', '!']) {
        text.to_string()
    } else {
        format!("{text}.")
    }
}
