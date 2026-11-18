//! Requests the Insights page sends the shell.

use super::state::PageState;
use crate::domain::graph::NodeRef;
use crate::domain::{PostId, ProfileId};

#[derive(Debug, Clone, PartialEq)]
pub enum InsightsEvent {
    /// The view, filter or a display choice changed: save it (and recompute when needed).
    Changed(PageState),
    /// Open a post's thread.
    OpenPost(PostId),
    /// Open a graph node where it lives (post, document, concept or profile).
    OpenNode(NodeRef),
    /// Open an author's profile.
    OpenAuthor(ProfileId),
}
