//! Where a post stands in its life: published, waiting to be published, or archived, with the
//! moments it is due to be published, archived or deleted. Persisted in the `posts.status`,
//! `publish_at`, `archive_at` and `delete_at` columns; posts stored before these existed read
//! back as published with nothing due.
//!
//! A scheduled post stays out of the timeline (the store keeps it aside) until it is due.

use crate::domain::document::keyed::keyed_enum;
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

keyed_enum! {
    /// A post's publication state. Keys are stored in `posts.status`; never rename one.
    PostStatus {
        /// Visible on the timeline.
        Published => "published",
        /// Waiting for its `publish_at` moment.
        Scheduled => "scheduled",
        /// Kept, but out of the timeline.
        Archived => "archived",
    }
}

impl Default for PostStatus {
    fn default() -> Self {
        Self::Published
    }
}

impl PostStatus {
    /// The status stored under `key` (`Published` for anything unknown).
    pub fn from_stored(key: &str) -> Self {
        Self::from_key(key).unwrap_or_default()
    }
}

/// A post's status and the moments it is due to change.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct PostLifecycle {
    pub status: PostStatus,
    /// When a scheduled post is due to be published.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish_at: Option<Timestamp>,
    /// When the post is due to be archived.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archive_at: Option<Timestamp>,
    /// When the post is due to be deleted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_at: Option<Timestamp>,
}

impl PostLifecycle {
    /// Published, with nothing due (serde skips the field then).
    pub fn is_default(&self) -> bool {
        *self == Self::default()
    }

    /// Waiting to be published at `at`.
    pub fn scheduled(at: Timestamp) -> Self {
        Self { status: PostStatus::Scheduled, publish_at: Some(at), ..Self::default() }
    }

    /// Whether the post waits to be published.
    pub fn is_scheduled(&self) -> bool {
        self.status == PostStatus::Scheduled
    }

    /// Whether a scheduled post is due at `now` (one without a moment is due at once).
    pub fn is_due_to_publish(&self, now: Timestamp) -> bool {
        self.is_scheduled() && self.publish_at.is_none_or(|at| at <= now)
    }
}
