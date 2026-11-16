//! Conversation view of a selected post: its ancestor chain, the post, and its replies.

use crate::domain::{Post, PostId};
use crate::state::AppState;
use std::cmp::Reverse;
use std::collections::HashSet;

/// How many replies a conversation shows before "show more".
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReplyLimits {
    /// Direct replies of the focused post.
    pub direct: usize,
    /// Replies shown under each direct reply (one nesting level).
    pub nested: usize,
}

impl ReplyLimits {
    /// Shows every direct reply and every nested reply.
    pub const UNLIMITED: Self = Self { direct: usize::MAX, nested: usize::MAX };
}

/// A direct reply of the focused post with its own (one level) replies.
#[derive(Debug, Clone, PartialEq)]
pub struct ReplyBranch<'a> {
    pub reply: &'a Post,
    pub nested: Vec<&'a Post>,
}

/// Ancestors (root first) → focused post → direct replies (oldest first).
#[derive(Debug, Clone, PartialEq)]
pub struct Conversation<'a> {
    pub ancestors: Vec<&'a Post>,
    pub focused: &'a Post,
    pub branches: Vec<ReplyBranch<'a>>,
    /// Number of direct replies, shown or not.
    pub direct_total: usize,
    /// Direct and nested replies left out by `limits`.
    pub hidden: usize,
}

/// Builds the conversation around `focused`, capping replies to `limits`.
pub fn build_conversation<'a>(state: &'a AppState, focused: &'a Post, limits: ReplyLimits) -> Conversation<'a> {
    let direct = chronological_replies(state, &focused.id);
    let direct_total = direct.len();
    let mut hidden = direct_total.saturating_sub(limits.direct);
    let branches = direct
        .into_iter()
        .take(limits.direct)
        .map(|reply| {
            let mut nested = chronological_replies(state, &reply.id);
            hidden += nested.len().saturating_sub(limits.nested);
            nested.truncate(limits.nested);
            ReplyBranch { reply, nested }
        })
        .collect();
    Conversation { ancestors: ancestors(state, focused), focused, branches, direct_total, hidden }
}

/// The reply chain above `focused`, root first (stops at missing parents and cycles).
fn ancestors<'a>(state: &'a AppState, focused: &'a Post) -> Vec<&'a Post> {
    let mut seen: HashSet<&PostId> = HashSet::from([&focused.id]);
    let mut chain: Vec<&'a Post> = Vec::new();
    let mut parent_id = focused.reply_to_id.as_ref();
    while let Some(parent) = parent_id.and_then(|id| state.get_post(id)) {
        if !seen.insert(&parent.id) {
            break;
        }
        chain.push(parent);
        parent_id = parent.reply_to_id.as_ref();
    }
    chain.reverse();
    chain
}

/// Direct replies of `id`, oldest first (ties: earlier insertion first).
fn chronological_replies<'a>(state: &'a AppState, id: &PostId) -> Vec<&'a Post> {
    let mut replies = state.replies_for_post(id);
    replies.sort_by_key(|post| {
        let store_idx = state.post_position(&post.id).unwrap_or_default();
        (post.created_at, Reverse(store_idx))
    });
    replies
}
