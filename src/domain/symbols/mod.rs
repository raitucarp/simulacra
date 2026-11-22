//! Post symbols: what kind of entry a post is (task, event, note), the state it is in, the
//! signifiers that mark it out, and custom icons from the icon library. A post with symbols
//! shows the symbol in place of the avatar. Stored as `extras.symbols` on the post.
//!
//! - [`kinds`]: the value types ([`EntrySymbol`], [`EntryState`], [`Signifier`], [`SymbolRef`]).
//! - [`art`]: the in-house drawing of each core symbol (S3).
//! - [`face`]: what replaces the avatar: the primary symbol and its badges (S4).
//! - [`picker`]: what the composer's symbol picker offers: core set, recent, search (S5).
//! - [`shorthand`]: `. ` task, `o ` event, `- ` note, `* ` priority, `! ` inspiration (S6).
//! - [`transitions`]: which states an entry may carry and how it may move between them.

pub mod art;
pub mod face;
pub mod kinds;
pub mod picker;
pub mod shorthand;
pub mod transitions;
#[cfg(test)]
mod tests;

pub use art::{entry_key, signifier_key, state_key, SymbolArt, SYMBOL_ART_DIR};
pub use face::{SymbolFace, SymbolGlyph, MAX_BADGES};
pub use picker::{core_choices, search_symbols, words_match, SymbolChoice, SymbolMemory, RECENT_SYMBOLS, SYMBOL_RESULTS};
pub use shorthand::{parse_shorthand, Shorthand};
pub use kinds::{EntryState, EntrySymbol, Signifier, SymbolRef, SYMBOL_ASSET_SCHEME};
pub use transitions::{allowed_states, can_transition, initial_state, state_allowed};

use serde::{Deserialize, Serialize};

/// The full symbol set of one post. Empty (the default) means the post shows its avatar.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PostSymbols {
    /// The entry kind; `None` when the post only carries signifiers or custom symbols.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry: Option<EntrySymbol>,
    /// The entry state; always `None` for notes and for posts without an entry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<EntryState>,
    /// Signifiers in the order they were added, without duplicates.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub signifiers: Vec<Signifier>,
    /// Custom symbols from the icon library, in display order.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub custom: Vec<SymbolRef>,
}

impl PostSymbols {
    /// A symbol set for `entry` in its initial state (open for tasks and events).
    pub fn entry(entry: EntrySymbol) -> Self {
        Self { entry: Some(entry), state: initial_state(entry), ..Self::default() }
    }

    /// Whether nothing is set (the post then shows its avatar and nothing is stored).
    pub fn is_empty(&self) -> bool {
        self == &Self::default()
    }

    /// Whether the state fits the entry: notes and entry-less sets carry no state, tasks and
    /// events carry one of their [`allowed_states`].
    pub fn is_consistent(&self) -> bool {
        match (self.entry, self.state) {
            (None, state) => state.is_none(),
            (Some(entry), None) => initial_state(entry).is_none(),
            (Some(entry), Some(state)) => state_allowed(entry, state),
        }
    }

    /// Moves the entry to `to` when [`can_transition`] allows it; returns whether it moved.
    pub fn set_state(&mut self, to: EntryState) -> bool {
        let Some(entry) = self.entry else { return false };
        let Some(from) = self.state else { return false };
        if !can_transition(entry, from, to) {
            return false;
        }
        self.state = Some(to);
        true
    }

    /// Adds `signifier` unless it is already set.
    pub fn add_signifier(&mut self, signifier: Signifier) {
        if !self.signifiers.contains(&signifier) {
            self.signifiers.push(signifier);
        }
    }

    /// Removes `signifier` if set.
    pub fn remove_signifier(&mut self, signifier: Signifier) {
        self.signifiers.retain(|s| *s != signifier);
    }

    /// Adds a custom symbol unless the same one is already listed.
    pub fn add_custom(&mut self, symbol: SymbolRef) {
        if !self.custom.contains(&symbol) {
            self.custom.push(symbol);
        }
    }
}
