//! The post lifecycle heartbeat (Y2): on every tick of the scheduler heartbeat, scheduled
//! posts whose moment has come are published, posts due to be archived are archived (Y6) and
//! posts due to delete themselves are deleted (Y7).
//! The clock is injected so ticks are testable.

use super::AppState;
use crate::agent::schedule::Clock;
use crate::domain::post::{PostLifecycle, PostStatus};
use crate::domain::{Post, PostId};
use crate::error::Result;
use jiff::Timestamp;

/// What one heartbeat changed.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct LifecycleReport {
    /// Scheduled posts that went out, in publishing order.
    pub published: Vec<PostId>,
    /// Posts that moved to the archive (Y6).
    pub archived: Vec<PostId>,
    /// Posts that deleted themselves (Y7).
    pub deleted: Vec<PostId>,
}

impl LifecycleReport {
    /// Whether the tick changed nothing.
    pub fn is_empty(&self) -> bool {
        self.published.is_empty() && self.archived.is_empty() && self.deleted.is_empty()
    }
}

impl AppState {
    /// One heartbeat at `clock`'s current time.
    pub fn lifecycle_heartbeat(&mut self, clock: &dyn Clock) -> Result<LifecycleReport> {
        let now = clock.now().timestamp();
        let published = self.publish_due(now)?;
        let archived = self.archive_due(now)?;
        Ok(LifecycleReport { published, archived, deleted: self.delete_due(now)? })
    }

    /// Publishes every scheduled post due at `now`, in publishing order.
    pub fn publish_due(&mut self, now: Timestamp) -> Result<Vec<PostId>> {
        let (due, waiting): (Vec<Post>, Vec<Post>) = std::mem::take(&mut self.scheduled).into_iter().partition(|p| p.lifecycle.is_due_to_publish(now));
        self.scheduled = waiting;
        due.into_iter().map(|post| self.release_scheduled(post, now)).collect()
    }

    /// Puts a scheduled post on the timeline as written at `now` (its reply counts the parent;
    /// the default auto-archive moment counts from then).
    pub(super) fn release_scheduled(&mut self, post: Post, now: Timestamp) -> Result<PostId> {
        let lifecycle = PostLifecycle { status: PostStatus::Published, publish_at: None, ..post.lifecycle };
        let id = post.id.clone();
        self.commit_post(Post { created_at: now, lifecycle, ..post })?;
        Ok(id)
    }
}
