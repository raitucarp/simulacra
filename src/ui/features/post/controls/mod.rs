//! Interactive post controls: the "…" overflow menu, the references dropdown and sharing.

mod actions_menu;
mod handlers;
mod learn;
mod menu;
mod refs;
mod share;

pub use handlers::{OpenPostAction, PickPostAction, PostAction, PostCardHandlers};
pub use learn::{LearnAction, LearnControls, LearnRequestKind, LearnStatus};
pub(crate) use learn::render_learn_badge;
pub(crate) use menu::render_overflow_menu;
pub(crate) use refs::{collect_references, render_refs_button, PostReference};
#[cfg(test)]
use refs::ReferenceKind;
pub use share::{post_permalink, share_text};
pub(crate) use share::{copy_with_toast, render_share_button};

#[cfg(test)]
mod tests;
