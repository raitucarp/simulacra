//! The user's symbol key (S9).

use crate::domain::symbols::*;

#[test]
fn a_meaning_is_set_changed_and_cleared() {
    let flask = SymbolRef::new("fi", "flask");
    let mut key = SymbolKey::default();
    assert!(key.set(flask.clone(), "  an   experiment "));
    assert_eq!(key.meaning_of(&flask), Some("an experiment"));
    assert!(!key.set(flask.clone(), "an experiment"), "unchanged");
    assert!(key.set(flask.clone(), "a trial"));
    assert!(key.set(flask.clone(), "   "), "an empty meaning takes it off");
    assert_eq!(key.meaning_of(&flask), None);
    assert_eq!(clean_meaning(&"x".repeat(MAX_MEANING_CHARS + 9)).chars().count(), MAX_MEANING_CHARS);
}

#[test]
fn the_key_reads_back_clean() {
    let flask = SymbolRef::new("fi", "flask");
    let raw = SymbolKey { entries: vec![KeyEntry { symbol: flask.clone(), meaning: " try ".into() }, KeyEntry { symbol: flask.clone(), meaning: "again".into() }, KeyEntry { symbol: SymbolRef::new("fi", "x"), meaning: " ".into() }] };
    let key = raw.validated();
    assert_eq!(key.entries, [KeyEntry { symbol: flask, meaning: "try".into() }]);
    let json = serde_json::to_string(&key).unwrap();
    assert_eq!(serde_json::from_str::<SymbolKey>(&json).unwrap(), key);
    assert_eq!(serde_json::from_str::<SymbolKey>("{}").unwrap(), SymbolKey::default());
}
