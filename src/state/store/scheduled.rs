//! Scheduled posts (Y1): held in `AppState::scheduled`, outside the timeline, until they are
//! due. Everything that reads the timeline (the feed, its search, the post index, the
//! meaning index, insights and the agents' context) therefore never sees them.

use super::{require_persona, AppState, ThreadAnchor, ThreadDraft};
use crate::domain::post::PostLifecycle;
use crate::domain::{Post, PostId};
use crate::error::{AppError, Result};
use jiff::Timestamp;
use std::collections::HashMap;

/// How many of `post`'s ancestors are among `parents` (a thread's later posts come after
/// its earlier ones even when they were written in the same instant).
fn depth(post: &Post, parents: &HashMap<&PostId, Option<&PostId>>) -> usize {
    let mut at = post.reply_to_id.as_ref();
    let mut depth = 0;
    while let Some(parent) = at.filter(|_| depth < parents.len()) {
        let Some(next) = parents.get(parent) else { break };
        depth += 1;
        at = *next;
    }
    depth
}

/// `posts` in publishing order: soonest due first, a thread's posts in thread order.
pub(super) fn soonest_first(mut posts: Vec<Post>) -> Vec<Post> {
    let depths: Vec<usize> = {
        let parents: HashMap<&PostId, Option<&PostId>> = posts.iter().map(|p| (&p.id, p.reply_to_id.as_ref())).collect();
        posts.iter().map(|post| depth(post, &parents)).collect()
    };
    let mut keyed: Vec<(usize, Post)> = depths.into_iter().zip(posts.drain(..)).collect();
    keyed.sort_by_key(|(depth, post)| (post.lifecycle.publish_at, *depth, post.created_at));
    keyed.into_iter().map(|(_, post)| post).collect()
}

impl AppState {
    /// The scheduled posts, soonest due first.
    pub fn scheduled_posts(&self) -> &[Post] {
        &self.scheduled
    }

    /// Schedules `drafts` as a thread (validated as [`AppState::submit_thread`] does) to be
    /// published at `publish_at`, which must be after `now`. Returns the post ids in thread order.
    pub fn schedule_thread(&mut self, drafts: &[ThreadDraft], anchor: &ThreadAnchor, publish_at: Timestamp, now: Timestamp) -> Result<Vec<PostId>> {
        if publish_at <= now {
            return Err(AppError::ScheduleNotInFuture);
        }
        require_persona(&self.current_profile_or_err()?)?;
        let posts = self.build_thread(drafts, anchor)?;
        let ids = posts.iter().map(|p| p.id.clone()).collect();
        for post in posts {
            self.hold_scheduled(Post { lifecycle: PostLifecycle::scheduled(publish_at), ..post })?;
        }
        self.draft_text.clear();
        Ok(ids)
    }

    /// Persists a scheduled post and queues it in publishing order.
    pub(super) fn hold_scheduled(&mut self, post: Post) -> Result<()> {
        if let Some(ref db) = self.db {
            db.save_post(&post)?;
        }
        self.scheduled.push(post);
        self.scheduled = soonest_first(std::mem::take(&mut self.scheduled));
        Ok(())
    }
}
