//! What a post with symbols shows in place of its avatar (S4): one primary symbol and at most
//! [`MAX_BADGES`] signifier badges on its corner.
//!
//! The primary symbol is the entry in its state (a task's dot or cross, an event's ring, the
//! note's dash); without an entry, the first custom symbol; without either, the first
//! signifier. The badges are the signifiers not already shown as the primary symbol.

use super::art::SymbolArt;
use super::kinds::SymbolRef;
use super::PostSymbols;

/// Signifier badges drawn at most.
pub const MAX_BADGES: usize = 2;

/// One drawn symbol: core art, or a custom icon from the library.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SymbolGlyph {
    Core(SymbolArt),
    Custom(SymbolRef),
}

/// The primary symbol and its badges.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SymbolFace {
    pub primary: SymbolGlyph,
    pub badges: Vec<SymbolArt>,
}

impl PostSymbols {
    /// What replaces the avatar; `None` when the post has no symbols (it shows its avatar).
    pub fn face(&self) -> Option<SymbolFace> {
        let signifiers = self.signifiers.iter().copied().map(SymbolArt::Signifier);
        let (primary, skip) = match (self.entry, self.custom.first(), self.signifiers.first()) {
            (Some(entry), _, _) => (SymbolGlyph::Core(SymbolArt::Entry(entry, self.state)), 0),
            (None, Some(custom), _) => (SymbolGlyph::Custom(custom.clone()), 0),
            (None, None, Some(first)) => (SymbolGlyph::Core(SymbolArt::Signifier(*first)), 1),
            (None, None, None) => return None,
        };
        Some(SymbolFace { primary, badges: signifiers.skip(skip).take(MAX_BADGES).collect() })
    }
}
