//! What the shell keeps for post media (sb C3): the open lightbox and its focus, and what the
//! background pump last looked at (so it only re-plans when the stream, the cache or the
//! privacy preference changed).

use crate::domain::PostId;
use crate::ui::features::media::GalleryOpen;
use gpui_kit::{App, FocusHandle};

/// The inputs of the last media plan.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct MediaWatch {
    /// Identity of the stream listing (its shared positions) and how much of it is shown.
    pub(super) rows: usize,
    pub(super) shown: usize,
    pub(super) revision: u64,
    pub(super) previews_enabled: bool,
    pub(super) selected: Option<PostId>,
}

/// Post media state of the window.
pub struct MediaHost {
    /// The open lightbox: a post's images and the one shown.
    pub lightbox: Option<GalleryOpen>,
    pub lightbox_focus: FocusHandle,
    /// The cache was read from the vault.
    pub(super) loaded: bool,
    pub(super) watched: Option<MediaWatch>,
}

impl MediaHost {
    pub(super) fn new(cx: &mut App) -> Self {
        Self { lightbox: None, lightbox_focus: cx.focus_handle(), loaded: false, watched: None }
    }
}
