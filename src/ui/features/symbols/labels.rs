//! Translated names of the core symbols (`symbols.entry.*`, `symbols.state.*`,
//! `symbols.signifier.*`), keyed by their stable keys.

use crate::domain::symbols::{entry_key, signifier_key, state_key, EntryState, EntrySymbol, Signifier, SymbolArt, SymbolGlyph, SymbolKey};
use std::sync::RwLock;
use crate::ui::features::data::tr;
use rust_i18n::t;

pub fn entry_label(entry: EntrySymbol) -> String {
    tr(&format!("symbols.entry.{}", entry_key(entry)))
}

pub fn state_label(state: EntryState) -> String {
    tr(&format!("symbols.state.{}", state_key(state)))
}

pub fn signifier_label(signifier: Signifier) -> String {
    tr(&format!("symbols.signifier.{}", signifier_key(signifier)))
}

/// "Task · Done", "Note", "Priority".
pub fn art_label(art: SymbolArt) -> String {
    match art {
        SymbolArt::Entry(entry, Some(state)) => t!("symbols.face.entry_state", entry = entry_label(entry), state = state_label(state)).to_string(),
        SymbolArt::Entry(entry, None) => entry_label(entry),
        SymbolArt::Signifier(signifier) => signifier_label(signifier),
    }
}

/// What each custom symbol means, from the user's symbol key (S9); read by tooltips.
static MEANINGS: RwLock<Option<SymbolKey>> = RwLock::new(None);

/// Hands the symbol key to the tooltips (call again whenever it changes).
pub fn set_symbol_meanings(key: &SymbolKey) {
    if let Ok(mut meanings) = MEANINGS.write() {
        *meanings = Some(key.clone());
    }
}

/// A glyph's name: the core symbol's; for a library icon, its meaning in the symbol key,
/// else its own name.
pub fn glyph_label(glyph: &SymbolGlyph) -> String {
    match glyph {
        SymbolGlyph::Core(art) => art_label(*art),
        SymbolGlyph::Custom(symbol) => {
            let meaning = MEANINGS.read().ok().and_then(|key| key.as_ref()?.meaning_of(symbol).map(str::to_string));
            meaning.unwrap_or_else(|| symbol.name.replace('-', " "))
        }
    }
}
