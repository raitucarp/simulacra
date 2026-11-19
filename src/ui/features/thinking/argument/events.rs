//! What the Argument view shows and what it asks of the shell.

use crate::domain::thinking::argument::ArgumentMap;
use crate::domain::PostId;

/// The thread on screen: its root, the posts a statement may cite, and its map.
#[derive(Debug, Clone, PartialEq)]
pub struct ArgumentThread {
    pub root: PostId,
    /// The thread's posts, root first, with their authors' names.
    pub posts: Vec<(PostId, String)>,
    pub map: ArgumentMap,
}

/// Where the AI extraction stands.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ExtractStatus {
    #[default]
    Idle,
    Running,
    /// Why there is no new map (already localized).
    Failed(String),
    /// A map arrived; this many statements or links were dropped as invalid.
    Done { dropped: usize },
}

/// Requests the view sends the shell.
#[derive(Debug, Clone, PartialEq)]
pub enum ArgumentEvent {
    /// Map the thread rooted here with the AI (the result replaces the map).
    Extract(PostId),
    /// Stop a running extraction.
    Cancel,
    /// A manual edit changed the map: save it.
    Changed(ArgumentMap),
    /// Open a cited post.
    OpenPost(PostId),
}
