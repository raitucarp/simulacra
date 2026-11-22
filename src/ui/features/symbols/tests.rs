use super::*;
use crate::domain::symbols::{EntrySymbol, PostSymbols, Signifier, SymbolArt, SymbolGlyph, SymbolRef};
use crate::theme::ThemeTokens;

#[test]
fn every_core_symbol_has_a_translated_name() {
    for art in SymbolArt::all() {
        let label = art_label(art);
        assert!(!label.is_empty() && !label.contains("symbols."), "{art:?}: {label}");
    }
    assert_eq!(glyph_label(&SymbolGlyph::Custom(SymbolRef::new("fi", "zoom-out"))), "zoom out");
}

#[test]
fn a_face_renders_with_core_and_custom_symbols() {
    let theme = ThemeTokens::default();
    let mut set = PostSymbols::entry(EntrySymbol::Task);
    set.add_signifier(Signifier::Priority);
    let _task = render_symbol_face("p1", &set.face().unwrap(), theme.avatar_size_lg, &theme);
    let custom = PostSymbols { custom: vec![SymbolRef::new("fi", "zap")], ..PostSymbols::default() };
    let _custom = render_symbol_face("p2", &custom.face().unwrap(), theme.avatar_size_lg, &theme);
}

#[test]
fn a_custom_symbol_is_named_by_its_meaning_once_the_key_gives_one() {
    use crate::domain::symbols::SymbolKey;
    let flask = SymbolRef::new("fi", "flask-round");
    assert_eq!(glyph_label(&SymbolGlyph::Custom(flask.clone())), "flask round");
    let mut key = SymbolKey::default();
    key.set(flask.clone(), "an experiment");
    set_symbol_meanings(&key);
    assert_eq!(glyph_label(&SymbolGlyph::Custom(flask)), "an experiment");
    set_symbol_meanings(&SymbolKey::default());
}
