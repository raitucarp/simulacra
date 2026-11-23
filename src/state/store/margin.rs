//! N2: margin replies: a reply to one span of a post, and the margin notes a post shows (its
//! direct replies that answer a span, in the order of their spans). A locked post is never
//! quoted: a margin reply to one is refused, and its notes show only the placeholder.

use super::AppState;
use crate::domain::post::anchor::{AnchorPlace, SpanAnchor};
use crate::domain::post::lock::LOCKED_PLACEHOLDER;
use crate::domain::{Post, PostId};
use crate::error::{AppError, Result};
use std::ops::Range;

/// One margin note of a post.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MarginNote {
    pub reply: PostId,
    /// Where its span is in the post now (always gone on a locked post).
    pub place: AnchorPlace,
    /// The words it answers (the placeholder on a locked post).
    pub quote: String,
}

fn missing(id: &PostId) -> AppError {
    AppError::DatabaseError(format!("post '{id}' not found"))
}

impl AppState {
    /// Publishes `text` as the current profile's reply to the span `range` (characters) of the
    /// post `parent`, quoted from the version it shows.
    pub fn submit_margin_reply(&mut self, parent: &PostId, range: Range<usize>, text: &str) -> Result<PostId> {
        let target = self.get_post(parent).ok_or_else(|| missing(parent))?;
        if target.is_locked() {
            return Err(AppError::PostLocked); // X2: sealed text is never quoted
        }
        let version = Some(target.extras.head.as_ref().map_or_else(Default::default, |head| head.version));
        let anchor = SpanAnchor::of(&target.content, range, version).ok_or(AppError::EmptyContent)?;
        let author = self.current_profile_or_err()?;
        let mut reply = Post::create_reply(Self::new_post_id(), self.active_network_id.clone(), author, text, parent.clone(), None, None)?;
        reply.extras.anchor = Some(anchor);
        let id = reply.id.clone();
        self.commit_post(reply)?;
        Ok(id)
    }

    /// The margin notes of `post`: its direct replies answering a span, ordered by where the
    /// span is now (gone ones last), then oldest first.
    pub fn margin_notes(&self, post: &PostId) -> Vec<MarginNote> {
        let Some(parent) = self.get_post(post) else { return Vec::new() };
        let mut notes: Vec<(usize, jiff::Timestamp, MarginNote)> = self
            .index
            .reply_positions(post)
            .filter_map(|at| self.posts.get(at))
            .filter_map(|reply| Some((reply, reply.extras.anchor.as_ref()?)))
            .map(|(reply, anchor)| {
                let (place, quote) = match parent.is_locked() {
                    true => (AnchorPlace::Gone, LOCKED_PLACEHOLDER.to_string()),
                    false => (anchor.place(&parent.content), anchor.quote.clone()),
                };
                let at = match &place {
                    AnchorPlace::Exact(range) | AnchorPlace::Moved(range) => range.start,
                    AnchorPlace::Gone => usize::MAX,
                };
                (at, reply.created_at, MarginNote { reply: reply.id.clone(), place, quote })
            })
            .collect();
        notes.sort_by_key(|note| (note.0, note.1));
        notes.into_iter().map(|(_, _, note)| note).collect()
    }
}
