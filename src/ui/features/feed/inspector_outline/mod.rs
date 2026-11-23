//! The inspector's outline view (sb C2): the whole thread as nested, text-only bullets that
//! fold and unfold by click or keyboard (arrows move, left and right fold, Enter opens).
//!
//! - [`tree`]: the thread tree and its visible rows (pure).
//! - [`memory`]: folded branches and the cursor per thread, and every command (pure).
//! - [`mode`]: the view the feed renders (from the registry) and the commands it sends.
//! - `render` / `row`: the pane, its toolbar and one bullet.

pub mod keys;
pub mod memory;
pub mod mode;
mod render;
mod row;
pub mod tree;

#[cfg(test)]
mod tests;

pub use keys::init_key_bindings;
pub use memory::{apply_command, OutlineCommand, OutlineKey, OutlineMemory, OutlineThread};
pub use mode::{InspectorCallback, InspectorCommand, InspectorView};
pub(crate) use render::{render_outline_pane, thread_outline};
pub use tree::{build_outline, thread_root, visible_rows, OutlineNode, OutlineRow};
