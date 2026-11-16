//! Center timeline feed featuring:
//! 1. Threads-style Main Dialectic Stream ("For you" / "Dialectic")
//! 2. Focus Thread & Epistemic Inspector (replacing redundant second timeline)
//! 3. Resizable split panels using gpui-kit h_resizable
//! 4. 100% tokenized via ThemeTokens (colors, typography, spacing, radii)

mod callbacks;
mod consts;
mod inspector;
mod inspector_header;
mod inspector_replies;
mod inputs;
mod load_more;
mod params;
mod render;
mod stream;
mod stream_header;
mod threadline;
mod timeline;

pub use consts::{
    DEFAULT_FEED_WINDOW_SIZE, FEED_WINDOW_INCREMENT, MAX_INSPECTOR_NESTED_REPLIES,
    MAX_INSPECTOR_REPLIES,
};
pub use params::{
    AddDocCallback, CycleProfileCallback, DeleteDocCallback, DeletePostCallback, FeedParams,
    ImportDocCallback, LikePostCallback, LoadMoreCallback, QuotePostCallback, ReplyPostCallback, RepostPostCallback,
    SelectFeedTabCallback, SelectPostCallback, ToggleRepliesCallback,
};
pub use render::render_main_feed;
pub use stream::{FeedReader, StreamInputs, StreamRows};
pub use threadline::compute_stream_threadline_style;
