//! Threads-style single thought post item with contemporary design system foundations.
//! Features tokenized typography hierarchy, continuous threadline, dialectic callouts,
//! human-readable wikilink excerpts, and click-to-focus thread inspection.

mod actions;
mod anchor_line; // N2
mod garden_badge; // N3
mod branch_chip; // N6
mod pinned_card; // N7
mod audience_badge; // N8
mod question_chip; // N9
mod series_strip; // N10
mod correction_banner; // N14
mod still_true_strip; // N13
mod distill_chip; // N15
mod voice_card; // N11
mod ink_card; // N12
mod media_lines; // N11, N12
mod idea_lines; // N3, N6–N10
mod thought_lines; mod lens_chip; mod term_chip; mod open_chip; // N16–N20
mod attachments;
mod body;
mod checklist; // D5
mod consts;
mod controls;
mod countdown; // Y7
mod excerpt;
mod extras;
mod extras_detail;
mod feeling_row; // M3
mod header;
mod host;
mod item;
mod locked; // X4
mod lookup;
mod quote_card;
mod surface;
mod threadline;

pub use actions::ActionClickCallback;
pub use body::{needs_confirm, render_body_tokens, render_post_body, run_line, BodyLinks, CancelRunAction, CodeActions, OpenTextAction, RunCodeAction, RunRequest};
pub(crate) use body::render_code_block;
pub use checklist::{render_post_content, render_task_row, todo_due_label, TodoToggle};
pub use controls::{post_permalink, share_text, LearnAction, LearnControls, LearnRequestKind, LearnStatus, OpenPostAction, PickPostAction, PostAction, PostCardHandlers};
pub use excerpt::resolve_wikilink_excerpt;
pub use extras::{mood_chip, source_chip, tool_chip, OpenSourceAction};
pub(crate) use extras_detail::render_extras_detail;
pub(crate) use attachments::missing_image;
pub use host::extract_host_domain;
pub use item::render_post_item;
pub use locked::{render_locked_body, render_locked_placeholder, LockControls, LockSource, OpenedText};
pub use lookup::PostLookup;
pub(crate) use surface::row_tint;
pub(crate) use feeling_row::feeling_line; // W7: the profile timeline
pub use threadline::ThreadlineStyle;

#[cfg(test)]
mod tests;
#[cfg(test)]
#[path = "tests/ideas.rs"]
mod ideas_tests; // N6–N10
#[cfg(test)]
#[path = "tests/media.rs"]
mod media_tests; // N11, N12
#[cfg(test)]
#[path = "tests/thoughts.rs"]
mod thoughts_tests; // N16–N20
