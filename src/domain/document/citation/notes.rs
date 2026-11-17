//! Notes-and-bibliography style (bibliography entry):
//! `Family, Given, and Given Family. Title. Translated by Given Family. Publisher, Year. Link.`

use super::names::{first_inverted, join, natural};
use super::{closed, Citation, CitationSource};
use crate::domain::document::record::CreatorRole;

const CONJUNCTION: &str = "and";
const TRANSLATED_BY: &str = "Translated by";
const EDITED_BY: &str = "Edited by";
/// Quotation marks around the title of a part (an article, a page).
const OPEN_QUOTE: &str = "\u{201c}";
const CLOSE_QUOTE: &str = "\u{201d}";

fn contributor_sentence(source: &CitationSource<'_>, role: CreatorRole, label: &str) -> String {
    let names = source.creators_in(role);
    if names.is_empty() {
        String::new()
    } else {
        format!(" {label} {}.", join(&natural(&names), CONJUNCTION, true))
    }
}

/// " Publisher, Year." / " Publisher." / " Year." / "".
fn imprint(source: &CitationSource<'_>) -> String {
    match (source.publisher(), source.year()) {
        (Some(publisher), Some(year)) => format!(" {publisher}, {year}."),
        (Some(publisher), None) => format!(" {}", closed(publisher)),
        (None, Some(year)) => format!(" {year}."),
        (None, None) => String::new(),
    }
}

pub fn cite(source: CitationSource<'_>) -> Citation {
    let authors: Vec<&str> = source.record.authors().iter().map(|c| c.name.as_str()).collect();
    let authors = join(&first_inverted(&authors), CONJUNCTION, true);
    let lead = if authors.is_empty() {
        Citation::default()
    } else {
        Citation::default().plain(format!("{} ", closed(&authors)))
    };
    let title = source.full_title();
    let titled = if source.is_whole_work() {
        lead.emphasised(title).plain(".")
    } else {
        lead.plain(format!("{OPEN_QUOTE}{}{CLOSE_QUOTE}", closed(&title)))
    };
    let contributors = contributor_sentence(&source, CreatorRole::Translator, TRANSLATED_BY)
        + &contributor_sentence(&source, CreatorRole::Editor, EDITED_BY);
    let citation = titled.plain(contributors).plain(imprint(&source));
    match source.link() {
        Some(link) => citation.plain(format!(" {link}.")),
        None => citation,
    }
}
