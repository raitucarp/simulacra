//! Bullet-journal shorthand at the start of a post (S6).

use crate::domain::symbols::*;

fn parsed(text: &str) -> Option<(PostSymbols, String)> {
    parse_shorthand(text).map(|short| (short.symbols, short.text))
}

#[test]
fn each_marker_gives_its_symbol() {
    let cases = [
        (". Call the bakery", PostSymbols::entry(EntrySymbol::Task)),
        ("o Dentist at nine", PostSymbols::entry(EntrySymbol::Event)),
        ("- Bread rises faster when warm", PostSymbols::entry(EntrySymbol::Note)),
        ("* Pay rent", PostSymbols { signifiers: vec![Signifier::Priority], ..PostSymbols::default() }),
        ("! A museum of smells", PostSymbols { signifiers: vec![Signifier::Inspiration], ..PostSymbols::default() }),
    ];
    for (text, symbols) in cases {
        let (got, rest) = parsed(text).unwrap_or_else(|| panic!("{text}"));
        assert_eq!(got, symbols, "{text}");
        assert_eq!(rest, text[2..], "{text}");
    }
}

#[test]
fn markers_stack_and_only_one_entry_counts() {
    let (symbols, text) = parsed("* ! . Ship the release").unwrap();
    assert_eq!((symbols.entry, symbols.state), (Some(EntrySymbol::Task), Some(EntryState::Open)));
    assert_eq!(symbols.signifiers, [Signifier::Priority, Signifier::Inspiration]);
    assert_eq!(text, "Ship the release");
    let (symbols, text) = parsed(". - not a note").unwrap();
    assert_eq!(symbols.entry, Some(EntrySymbol::Task));
    assert_eq!(text, "- not a note", "a second entry marker is text");
}

#[test]
fn text_without_a_marker_and_a_space_is_left_alone() {
    for text in ["Plain thought", ".no space", "o", "ok then", "- ", "*bold*", "", "  . leading spaces"] {
        assert_eq!(parse_shorthand(text), None, "{text:?}");
    }
}
