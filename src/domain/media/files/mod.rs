//! Local files attached to a post (epic L). A file is linked, not copied: the post keeps
//! where it was and what it was when attached ([`FileRef`], in `extras.files`), so cards can
//! open it, show it in its folder and notice when it has moved or changed.

mod file_ref;

pub use file_ref::{FileRef, MAX_FILES_PER_POST};

#[cfg(test)]
mod tests;
