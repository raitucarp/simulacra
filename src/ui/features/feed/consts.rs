//! Named constants for the main feed: windowing, layout sizes, element ids, and labels.

/// Default initial window size for the main timeline stream.
pub const DEFAULT_FEED_WINDOW_SIZE: usize = 8;
/// Number of additional thoughts to load when expanding the timeline stream.
pub const FEED_WINDOW_INCREMENT: usize = 12;
/// Maximum number of direct/indirect replies rendered in the focus thread inspector.
pub const MAX_INSPECTOR_REPLIES: usize = 4;

/// Default width of the stream pane in the resizable split.
pub(crate) const STREAM_PANE_DEFAULT_WIDTH: f32 = 520.0;
/// Minimum width of the stream pane.
pub(crate) const STREAM_PANE_MIN_WIDTH: f32 = 380.0;
/// Maximum width of the stream pane.
pub(crate) const STREAM_PANE_MAX_WIDTH: f32 = 720.0;
/// Width of the active "For you" tab underline indicator.
pub(crate) const ACTIVE_TAB_INDICATOR_WIDTH: f32 = 48.0;

/// Element id of the stream / inspector resizable group.
pub(crate) const TIMELINE_RESIZABLE_ID: &str = "timeline-stream-inspector-resizable";
/// Element id of the scrollable main stream.
pub(crate) const STREAM_SCROLL_ID: &str = "general-feed-scroll-stream";
/// Element id of the scrollable focus thread body.
pub(crate) const INSPECTOR_SCROLL_ID: &str = "focus-thread-scroll-stream";
/// Element id of the load-more container.
pub(crate) const LOAD_MORE_CONTAINER_ID: &str = "feed-load-more-container";
/// Element id of the load-more button.
pub(crate) const LOAD_MORE_BUTTON_ID: &str = "btn-load-more-posts";
/// Element id of the inspector quick reply button.
pub(crate) const QUICK_REPLY_BUTTON_ID: &str = "btn-quick-reply-thread";

/// Words of the load-more window summary ("Showing <n> of <total> thoughts").
pub(crate) const SUMMARY_SHOWING: &str = "Showing";
pub(crate) const SUMMARY_OF: &str = "of";
pub(crate) const SUMMARY_THOUGHTS: &str = "thoughts";
/// Lead text of the inspector quick reply bar ("Reply to <handle>").
pub(crate) const REPLY_TO_LABEL: &str = "Reply to";
