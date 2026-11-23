//! N15: distilling a thread into the vault. The thread of a post (its root and every reply
//! under it, in reading order) becomes a Markdown document with a link back to each post; the
//! root remembers it. A locked post shows only its placeholder, and so does a post kept from
//! some readers (N8): every agent reads the vault, so nothing goes there that one may not read.

use super::AppState;
use crate::domain::post::distill::{post_link, thread_markdown, title_from, DistillPost, DistillWords, Distilled};
use crate::domain::{Post, PostId};
use crate::error::{AppError, Result};
use jiff::Timestamp;

/// Longest reply chain followed up to a root (corrupt data must not loop forever).
const MAX_CHAIN: usize = 512;

impl AppState {
    /// The root of `id`'s thread.
    fn thread_root_of(&self, id: &PostId) -> Option<&Post> {
        let mut current = self.get_post(id)?;
        for _ in 0..MAX_CHAIN {
            match current.reply_to_id.as_ref().filter(|parent| **parent != current.id).and_then(|parent| self.get_post(parent)) {
                Some(parent) => current = parent,
                None => break,
            }
        }
        Some(current)
    }

    /// Every post of `id`'s thread, root first, each followed by its replies (oldest first).
    pub fn thread_of(&self, id: &PostId) -> Vec<&Post> {
        let Some(root) = self.thread_root_of(id) else { return Vec::new() };
        let mut out: Vec<&Post> = Vec::new();
        let mut stack = vec![root];
        while let Some(post) = stack.pop() {
            if out.iter().any(|seen| seen.id == post.id) {
                continue;
            }
            out.push(post);
            let mut replies = self.replies_for_post(&post.id);
            replies.sort_by_key(|reply| std::cmp::Reverse(reply.created_at));
            stack.extend(replies);
        }
        out
    }

    /// What `post` puts in the document.
    fn distill_post(&self, post: &Post, words: &DistillWords, date: &impl Fn(Timestamp) -> String) -> DistillPost {
        let text = match (post.is_locked(), post.extras.audience.is_some()) {
            (true, _) => words.locked.clone(),
            (false, true) => words.restricted.clone(),
            (false, false) => post.distill_text(),
        };
        let correction = post.extras.correction.as_deref().filter(|_| !post.is_locked()).cloned();
        DistillPost { link: post_link(post.network_id.as_str(), post.id.as_str()), author: self.author_of(post).name.clone(), date: date(post.created_at), text, correction }
    }

    /// Distills `id`'s thread into a new vault document (`words` for a thread of so many
    /// posts, `summary` on top when given) and records it on the root.
    pub fn distill_thread(&mut self, id: &PostId, words: impl Fn(usize) -> DistillWords, summary: Option<&str>, date: impl Fn(Timestamp) -> String) -> Result<Distilled> {
        let posts = self.thread_of(id);
        let root = posts.first().copied().ok_or_else(|| AppError::DatabaseError(format!("post '{id}' not found")))?;
        let words = words(posts.len());
        let readable = !root.is_locked() && root.extras.audience.is_none();
        let title = Some(title_from(&root.content)).filter(|title| readable && !title.is_empty()).unwrap_or_else(|| words.untitled.clone());
        let parts: Vec<DistillPost> = posts.iter().map(|post| self.distill_post(post, &words, &date)).collect();
        let root_id = root.id.clone();
        let markdown = thread_markdown(&title, &parts, summary, &words);
        let doc_id = self.add_document(&title, &markdown)?;
        let distilled = Distilled { doc_id, title };
        if let Some(position) = self.post_position(&root_id) {
            self.posts[position].extras.distilled = Some(Box::new(distilled.clone()));
            if let Some(db) = &self.db {
                db.save_post(&self.posts[position])?;
            }
        }
        Ok(distilled)
    }
}
