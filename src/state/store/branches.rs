//! N6: what-if branches in the store. Forking a post publishes the first reply of a branch
//! (with its [`BranchPoint`]); the lines at a post are its main line (the replies without a
//! pointer) and each branch, every one followed down its thread, oldest first. A reply that
//! starts a branch of its own leaves its line. A locked post is never forked, and a branch
//! whose first post is locked is labelled with the placeholder only.

use super::AppState;
use crate::domain::post::branch::BranchPoint;
use crate::domain::post::lock::LOCKED_PLACEHOLDER;
use crate::domain::{Post, PostId};
use crate::error::{AppError, Result};

/// How deep a reply chain is followed up to its branch.
const MAX_THREAD_DEPTH: usize = 256;

/// One line of thought under a fork point.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BranchLine {
    /// What the branch supposes (`None`: the main line; the placeholder when its first post is locked).
    pub label: Option<String>,
    /// The posts of the line in reading order, its first post first.
    pub posts: Vec<PostId>,
}

impl AppState {
    /// Publishes `text` as the current profile's reply to `at` starting the branch `label`.
    pub fn fork_branch(&mut self, at: &PostId, label: &str, text: &str) -> Result<PostId> {
        let parent = self.get_post(at).ok_or_else(|| AppError::DatabaseError(format!("post '{at}' not found")))?;
        if parent.is_locked() {
            return Err(AppError::PostLocked); // X2: a sealed post is never forked
        }
        let point = BranchPoint::named(label).ok_or(AppError::EmptyContent)?;
        let author = self.current_profile_or_err()?;
        let mut reply = Post::create_reply(Self::new_post_id(), self.active_network_id.clone(), author, text, at.clone(), None, None)?;
        reply.extras.branch = Some(point);
        let id = reply.id.clone();
        self.commit_post(reply)?;
        Ok(id)
    }

    /// The lines at `at`: none until one of its replies starts a branch; then its main line (when
    /// it has plain replies) and each branch, oldest first.
    pub fn branch_lines(&self, at: &PostId) -> Vec<BranchLine> {
        let replies = self.oldest_first(at);
        if replies.iter().all(|reply| reply.extras.branch.is_none()) {
            return Vec::new();
        }
        let main: Vec<PostId> = replies.iter().filter(|reply| reply.extras.branch.is_none()).flat_map(|reply| self.line_from(reply)).collect();
        let heads = replies.iter().filter(|reply| reply.extras.branch.is_some());
        let branches = heads.map(|head| BranchLine { label: Some(branch_label(head)), posts: self.line_from(head) });
        let main = (!main.is_empty()).then_some(BranchLine { label: None, posts: main });
        main.into_iter().chain(branches).collect()
    }

    /// The branch `post` follows: the nearest first post of a branch at or above it.
    pub fn branch_of<'a>(&'a self, post: &'a Post) -> Option<(&'a Post, String)> {
        let mut at = post;
        for _ in 0..MAX_THREAD_DEPTH {
            if at.extras.branch.is_some() {
                return Some((at, branch_label(at)));
            }
            at = self.get_post(at.reply_to_id.as_ref()?)?;
        }
        None
    }

    /// `first` and every reply under it that stays on its line, in reading order.
    fn line_from(&self, first: &Post) -> Vec<PostId> {
        let mut out = vec![first.id.clone()];
        let mut stack: Vec<&Post> = self.oldest_first(&first.id).into_iter().rev().collect();
        while let Some(post) = stack.pop() {
            if post.extras.branch.is_some() || out.len() > MAX_THREAD_DEPTH {
                continue;
            }
            out.push(post.id.clone());
            stack.extend(self.oldest_first(&post.id).into_iter().rev());
        }
        out
    }

    fn oldest_first(&self, id: &PostId) -> Vec<&Post> {
        let mut replies = self.replies_for_post(id);
        replies.sort_by_key(|reply| (reply.created_at, std::cmp::Reverse(self.post_position(&reply.id).unwrap_or_default())));
        replies
    }
}

/// The label of the branch `head` starts (the placeholder when it is locked).
pub fn branch_label(head: &Post) -> String {
    match (head.is_locked(), &head.extras.branch) {
        (false, Some(point)) => point.label.clone(),
        _ => LOCKED_PLACEHOLDER.to_string(),
    }
}
