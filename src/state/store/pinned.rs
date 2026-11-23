//! N7: pinned quotes in the store. The current profile posts a comment on a passage of a vault
//! document, quoted verbatim with the document's content hash; each quote's drift is checked
//! against the document as it is now, and an author can re-pin a quote whose words are still
//! there. A locked post shows no quote and has no drift: its card shows only the placeholder.

use super::{require_persona, AppState};
use crate::domain::post::pinned::{PinnedQuote, QuoteDrift};
use crate::domain::{Post, PostId};
use crate::error::{AppError, Result};
use std::ops::Range;

fn missing(what: &str, id: &str) -> AppError {
    AppError::DatabaseError(format!("{what} '{id}' not found"))
}

impl AppState {
    /// Publishes `text` as the current profile's post quoting the passage `range` (characters)
    /// of the vault document `doc_id`, pinned to the document as it is now.
    pub fn pin_quote(&mut self, doc_id: &str, range: Range<usize>, text: &str) -> Result<PostId> {
        let doc = self.documents.iter().find(|doc| doc.id == doc_id).ok_or_else(|| missing("document", doc_id))?;
        let quote = PinnedQuote::of(&doc.id, &doc.title, &doc.content, range).ok_or(AppError::EmptyContent)?;
        let author = self.current_profile_or_err()?;
        let mut post = Post::create(Self::new_post_id(), self.active_network_id.clone(), author, text)?;
        post.extras.pinned = Some(Box::new(quote));
        let id = post.id.clone();
        self.commit_post(post)?;
        Ok(id)
    }

    /// How `post`'s pinned quote stands against its document now (`None`: no quote, or the
    /// post is locked).
    pub fn quote_drift(&self, post: &Post) -> Option<QuoteDrift> {
        let quote = post.extras.pinned.as_ref().filter(|_| !post.is_locked())?;
        let text = self.documents.iter().find(|doc| doc.id == quote.doc_id).map(|doc| doc.content.as_str());
        Some(quote.drift(text))
    }

    /// Re-pins the user's own post `id` to its document as it is now. `Ok(false)` when the
    /// quote had not drifted; an error when its words are gone, its document is or it is locked.
    pub fn repin_quote(&mut self, id: &PostId) -> Result<bool> {
        let position = self.post_position(id).ok_or_else(|| missing("post", id.as_str()))?;
        require_persona(self.author_of(&self.posts[position]))?;
        let post = &self.posts[position];
        if post.is_locked() {
            return Err(AppError::PostLocked);
        }
        let quote = post.extras.pinned.clone().ok_or(AppError::EmptyContent)?;
        let doc = self.documents.iter().find(|doc| doc.id == quote.doc_id).ok_or_else(|| missing("document", &quote.doc_id))?;
        let again = quote.repinned(&doc.content).ok_or(AppError::EmptyContent)?;
        if again == *quote {
            return Ok(false);
        }
        self.posts[position].extras.pinned = Some(Box::new(again));
        if let Some(ref db) = self.db {
            db.save_post(&self.posts[position])?;
        }
        Ok(true)
    }

    /// The posts whose pinned quote drifted, newest first (never a locked one).
    pub fn drifted_quotes(&self) -> Vec<(PostId, QuoteDrift)> {
        self.current_network_posts().filter_map(|post| Some((post.id.clone(), self.quote_drift(post).filter(|drift| drift.flagged())?))).collect()
    }
}
