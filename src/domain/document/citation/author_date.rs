//! Author–date style: `Family, I. (Year). Title (I. Family, Trans.). Publisher. Link`

use super::names::{initials_first, inverted_initials};
use super::{closed, Citation, CitationSource};
use crate::domain::document::record::CreatorRole;

/// Written in place of a missing year.
const NO_DATE: &str = "n.d.";
const TRANSLATOR_NOTE: &str = "Trans.";
const EDITOR_NOTE: &str = "Ed.";

/// "A, B., & C, D." (the ampersand also joins two names).
fn author_list(names: Vec<String>) -> String {
    match names.as_slice() {
        [] => String::new(),
        [only] => only.clone(),
        [init @ .., last] => format!("{}, & {last}", init.join(", ")),
    }
}

/// " (I. Family, Trans.)" for the translators, else the editors, else nothing.
fn contributor_note(source: &CitationSource<'_>) -> String {
    let translators = source.creators_in(CreatorRole::Translator);
    let editors = source.creators_in(CreatorRole::Editor);
    let (names, note) = match (translators.is_empty(), editors.is_empty()) {
        (false, _) => (translators, TRANSLATOR_NOTE),
        (true, false) => (editors, EDITOR_NOTE),
        (true, true) => return String::new(),
    };
    format!(" ({}, {note})", initials_first(&names).join(", "))
}

pub fn cite(source: CitationSource<'_>) -> Citation {
    let authors: Vec<&str> = source.record.authors().iter().map(|c| c.name.as_str()).collect();
    let authors = author_list(inverted_initials(&authors));
    let year = source.year().map_or_else(|| NO_DATE.to_string(), |year| year.to_string());
    let title = source.full_title();
    let lead = if authors.is_empty() {
        Citation::default()
    } else {
        Citation::default().plain(format!("{authors} ({year}). "))
    };
    let titled = if source.is_whole_work() { lead.emphasised(title) } else { lead.plain(title) };
    let dated = if authors.is_empty() { format!(" ({year})") } else { String::new() };
    let mut citation = titled.plain(format!("{}{dated}.", contributor_note(&source)));
    if let Some(publisher) = source.publisher() {
        citation = citation.plain(format!(" {}", closed(publisher)));
    }
    match source.link() {
        Some(link) => citation.plain(format!(" {link}")),
        None => citation,
    }
}
