//! Post symbols in the store (S2): only the user's personas set them, on submit and on edit,
//! and an agent's post never carries any (the engine output mapping has no symbols field, and
//! a stored agent post loses whatever symbols it arrived with).

use super::{require_persona, AppState};
use crate::domain::symbols::PostSymbols;
use crate::domain::{Post, PostId, Profile};
use crate::error::{AppError, Result};

/// Checks that `author` may set `symbols`: nothing to check for an empty set, otherwise the
/// author must be a persona and the state must fit the entry.
pub(super) fn check_symbols(author: &Profile, symbols: &PostSymbols) -> Result<()> {
    if symbols.is_empty() {
        return Ok(());
    }
    require_persona(author)?;
    if symbols.is_consistent() { Ok(()) } else { Err(AppError::InvalidSymbols) }
}

impl AppState {
    /// Replaces the symbol set of the user's own post `id` (agents' posts are never edited).
    /// Symbols are not text, so no revision is kept. `Ok(false)` when nothing changed.
    pub fn set_post_symbols(&mut self, id: &PostId, symbols: PostSymbols) -> Result<bool> {
        let position = self.post_position(id).ok_or_else(|| AppError::DatabaseError(format!("post '{id}' not found")))?;
        let author = self.author_of(&self.posts[position]);
        require_persona(author)?;
        check_symbols(author, &symbols)?;
        if self.posts[position].extras.symbols == symbols {
            return Ok(false);
        }
        self.posts[position].extras.symbols = symbols;
        if let Some(ref db) = self.db {
            db.save_post(&self.posts[position])?;
        }
        Ok(true)
    }

    /// `post` without symbols unless one of the user's personas wrote it.
    pub(super) fn without_agent_symbols(&self, mut post: Post) -> Post {
        if !post.extras.symbols.is_empty() && require_persona(self.author_of(&post)).is_err() {
            post.extras.symbols = PostSymbols::default();
        }
        post
    }
}
