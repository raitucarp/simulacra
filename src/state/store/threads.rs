//! Publishing multi-post threads: every draft is validated before any post is stored.

use super::AppState;
use crate::domain::{Post, PostId};
use crate::error::{AppError, Result};

/// One post of a thread being published.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ThreadDraft {
    pub content: String,
    pub link: Option<String>,
    pub image: Option<String>,
    /// Images attached in the composer with their alt text (sb C3); `image` is the first.
    pub images: Vec<crate::domain::media::ImageAttachment>,
    /// The container tint picked in the composer (sb C3).
    pub tint: Option<crate::domain::post::Tint>,
    /// The symbol set picked in the composer (S2: personas only).
    pub symbols: crate::domain::symbols::PostSymbols,
}

impl ThreadDraft {
    /// A text-only draft.
    pub fn text(content: impl Into<String>) -> Self {
        Self { content: content.into(), ..Self::default() }
    }

    /// The draft with its bullet-journal shorthand read off its start (S6): the markers'
    /// symbols, joined with those already picked (a picked entry wins), and the text after
    /// them. Unchanged when it starts with no marker.
    #[must_use]
    pub fn with_shorthand(self) -> Self {
        let Some(short) = crate::domain::symbols::parse_shorthand(&self.content) else { return self };
        let mut symbols = short.symbols;
        if self.symbols.entry.is_some() {
            (symbols.entry, symbols.state) = (self.symbols.entry, self.symbols.state);
        }
        self.symbols.signifiers.iter().for_each(|signifier| symbols.add_signifier(*signifier));
        self.symbols.custom.iter().cloned().for_each(|custom| symbols.add_custom(custom));
        Self { content: short.text, symbols, ..self }
    }

}

/// Where a published thread attaches: a reply target for its first post and/or a quoted post.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ThreadAnchor {
    pub reply_to: Option<PostId>,
    pub quote_of: Option<PostId>,
}

fn missing(kind: &str, id: &PostId) -> AppError {
    AppError::DatabaseError(format!("{kind} post '{id}' not found"))
}

impl AppState {
    /// Publishes `drafts` as a thread: the first post replies to `anchor.reply_to` (or is a new
    /// root) and quotes `anchor.quote_of`; each later post replies to the one before it.
    ///
    /// All drafts are validated up front, so an invalid draft stores nothing. Returns the new
    /// post ids in thread order.
    pub fn submit_thread(&mut self, drafts: &[ThreadDraft], anchor: &ThreadAnchor) -> Result<Vec<PostId>> {
        let posts = self.build_thread(drafts, anchor)?;
        let ids = posts.iter().map(|p| p.id.clone()).collect();
        for post in posts {
            self.commit_post(post)?;
        }
        self.draft_text.clear();
        Ok(ids)
    }

    /// Validates the anchor and every draft, returning the chained (not yet stored) posts.
    pub(super) fn build_thread(&self, drafts: &[ThreadDraft], anchor: &ThreadAnchor) -> Result<Vec<Post>> {
        if drafts.is_empty() {
            return Err(AppError::EmptyContent);
        }
        let author = self.current_profile_or_err()?;
        for draft in drafts {
            super::symbols::check_symbols(&author, &draft.symbols)?;
        }
        for (kind, id) in [("Parent", &anchor.reply_to), ("Target", &anchor.quote_of)] {
            if let Some(id) = id.as_ref().filter(|id| !self.post_exists(id)) {
                return Err(missing(kind, id));
            }
        }
        let mut parent = anchor.reply_to.clone();
        let mut posts = Vec::with_capacity(drafts.len());
        for (index, draft) in drafts.iter().enumerate() {
            let mut post = Post::create_with_attachments(
                Self::new_post_id(),
                self.active_network_id.clone(),
                author.clone(),
                &draft.content,
                draft.link.clone(),
                draft.image.clone(),
                parent.take(),
                if index == 0 { anchor.quote_of.clone() } else { None },
                None,
            )?;
            post.extras.images = draft.images.clone();
            post.extras.tint = draft.tint;
            post.extras.symbols = draft.symbols.clone();
            parent = Some(post.id.clone());
            posts.push(post);
        }
        Ok(posts)
    }
}
