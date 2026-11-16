//! Named constants for the main feed: windowing, layout sizes, element ids, and labels.

/// Default initial window size for the main timeline stream.
pub const DEFAULT_FEED_WINDOW_SIZE: usize = 8;
/// Number of additional thoughts to load when expanding the timeline stream.
pub const FEED_WINDOW_INCREMENT: usize = 12;
/// Direct replies shown in the focus thread inspector before "show more".
pub const MAX_INSPECTOR_REPLIES: usize = 4;
/// Replies shown under each direct reply (one nesting level) before "show more".
pub const MAX_INSPECTOR_NESTED_REPLIES: usize = 1;

/// Element id of the stream / inspector resizable group.
pub(crate) const TIMELINE_RESIZABLE_ID: &str = "timeline-stream-inspector-resizable";
/// Element id of the scrollable focus thread body.
pub(crate) const INSPECTOR_SCROLL_ID: &str = "focus-thread-scroll-stream";
/// Element id of the load-more container.
pub(crate) const LOAD_MORE_CONTAINER_ID: &str = "feed-load-more-container";
/// Element id of the load-more button.
pub(crate) const LOAD_MORE_BUTTON_ID: &str = "btn-load-more-posts";
/// Element id of the inspector "show more / fewer replies" button.
pub(crate) const TOGGLE_REPLIES_BUTTON_ID: &str = "btn-toggle-inspector-replies";
/// Element id prefix of the stream tabs (suffixed with the tab position).
pub(crate) const STREAM_TAB_ID: &str = "stream-tab";
/// Element id of the inspector quick reply button.
pub(crate) const QUICK_REPLY_BUTTON_ID: &str = "btn-quick-reply-thread";
