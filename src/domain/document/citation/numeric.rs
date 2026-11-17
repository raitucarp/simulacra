//! Numeric style: `[n] I. Family and I. Family, Title, Publisher, Year. Link`

use super::names::{initials_first, join};
use super::{Citation, CitationSource};

const CONJUNCTION: &str = "and";
const OPEN_QUOTE: &str = "\u{201c}";
const CLOSE_QUOTE: &str = "\u{201d}";

pub fn cite(source: CitationSource<'_>, number: usize) -> Citation {
    let authors: Vec<&str> = source.record.authors().iter().map(|c| c.name.as_str()).collect();
    let authors = join(&initials_first(&authors), CONJUNCTION, true);
    let lead = Citation::default().plain(format!("[{number}] "));
    let lead = if authors.is_empty() { lead } else { lead.plain(format!("{authors}, ")) };
    let title = source.full_title();
    let tail: Vec<String> = [source.publisher().map(str::to_string), source.year().map(|y| y.to_string())]
        .into_iter()
        .flatten()
        .collect();
    let tail = tail.join(", ");
    let citation = match (source.is_whole_work(), tail.is_empty()) {
        (true, true) => lead.emphasised(title).plain("."),
        (true, false) => lead.emphasised(title).plain(format!(", {tail}.")),
        (false, true) => lead.plain(format!("{OPEN_QUOTE}{title}.{CLOSE_QUOTE}")),
        (false, false) => lead.plain(format!("{OPEN_QUOTE}{title},{CLOSE_QUOTE} {tail}.")),
    };
    match source.link() {
        Some(link) => citation.plain(format!(" {link}")),
        None => citation,
    }
}
