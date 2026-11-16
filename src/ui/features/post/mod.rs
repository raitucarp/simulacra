//! Threads-style single thought post item with contemporary design system foundations.
//! Features tokenized typography hierarchy, continuous threadline, dialectic callouts,
//! human-readable wikilink excerpts, and click-to-focus thread inspection.

mod actions;
mod attachments;
mod body;
mod consts;
mod excerpt;
mod header;
mod host;
mod item;
mod lookup;
mod quote_card;
mod threadline;

pub use actions::ActionClickCallback;
pub use body::render_post_body;
pub use excerpt::resolve_wikilink_excerpt;
pub use host::extract_host_domain;
pub use item::{render_post_item, PostSelectCallback};
pub use lookup::PostLookup;
pub use threadline::ThreadlineStyle;

#[cfg(test)]
mod tests;
