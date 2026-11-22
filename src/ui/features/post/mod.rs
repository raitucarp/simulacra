//! Threads-style single thought post item with contemporary design system foundations.
//! Features tokenized typography hierarchy, continuous threadline, dialectic callouts,
//! human-readable wikilink excerpts, and click-to-focus thread inspection.

mod actions;
mod attachments;
mod body;
mod checklist; // D5
mod consts;
mod controls;
mod countdown; // Y7
mod excerpt;
mod extras;
mod extras_detail;
mod header;
mod host;
mod item;
mod lookup;
mod quote_card;
mod surface;
mod threadline;

pub use actions::ActionClickCallback;
pub use body::{render_body_tokens, render_post_body, BodyLinks, OpenTextAction};
pub use checklist::{render_post_content, render_task_row, todo_due_label, TodoToggle};
pub use controls::{post_permalink, share_text, LearnAction, LearnControls, LearnRequestKind, LearnStatus, OpenPostAction, PickPostAction, PostAction, PostCardHandlers};
pub use excerpt::resolve_wikilink_excerpt;
pub use extras::{mood_chip, source_chip, tool_chip, OpenSourceAction};
pub(crate) use extras_detail::render_extras_detail;
pub(crate) use attachments::missing_image;
pub use host::extract_host_domain;
pub use item::render_post_item;
pub use lookup::PostLookup;
pub(crate) use surface::row_tint;
pub use threadline::ThreadlineStyle;

#[cfg(test)]
mod tests;
