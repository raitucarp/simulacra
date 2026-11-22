//! Bullet-journal shorthand at the start of a post (S6), read only while symbol shorthand is
//! on in Settings › Writing: `. ` a task, `o ` an event, `- ` a note, `* ` priority and `! `
//! inspiration. Markers may be stacked (`* . Call the bakery`: a priority task); at most one
//! entry marker counts, and the first character that is not a marker ends them.

use super::kinds::{EntrySymbol, Signifier};
use super::PostSymbols;

/// What a marker stands for.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Marker {
    Entry(EntrySymbol),
    Signifier(Signifier),
}

/// Every marker character, in the order the help lists them.
const MARKERS: [(char, Marker); 5] = [
    ('.', Marker::Entry(EntrySymbol::Task)),
    ('o', Marker::Entry(EntrySymbol::Event)),
    ('-', Marker::Entry(EntrySymbol::Note)),
    ('*', Marker::Signifier(Signifier::Priority)),
    ('!', Marker::Signifier(Signifier::Inspiration)),
];

/// A post's text with its leading markers read off.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Shorthand {
    pub symbols: PostSymbols,
    /// The text after the markers.
    pub text: String,
}

/// The markers at the start of `text` and the text after them; `None` when it starts with
/// none, or when nothing follows them.
pub fn parse_shorthand(text: &str) -> Option<Shorthand> {
    let mut symbols = PostSymbols::default();
    let mut rest = text;
    while let Some(marker) = leading_marker(rest) {
        match marker {
            Marker::Entry(_) if symbols.entry.is_some() => break,
            Marker::Entry(entry) => symbols = PostSymbols { signifiers: symbols.signifiers, ..PostSymbols::entry(entry) },
            Marker::Signifier(signifier) => symbols.add_signifier(signifier),
        }
        rest = rest[1..].trim_start_matches(' ');
    }
    let rest = rest.trim();
    (!symbols.is_empty() && !rest.is_empty()).then(|| Shorthand { symbols, text: rest.to_string() })
}

/// The marker `text` starts with, when a space follows it.
fn leading_marker(text: &str) -> Option<Marker> {
    let mut chars = text.chars();
    let (first, second) = (chars.next()?, chars.next()?);
    let marker = MARKERS.iter().find(|(c, _)| *c == first).map(|(_, marker)| *marker)?;
    (second == ' ').then_some(marker)
}
