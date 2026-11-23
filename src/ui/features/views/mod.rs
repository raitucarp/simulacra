//! Views of a list of posts in the interface (epic W): the registry's icons and names, and
//! the switcher the thread inspector and the profile page share.
//!
//! - [`registry`]: the icon and translated name of every [`PostView`].
//! - [`switcher`]: the row of view buttons for one scope.

pub mod registry;
pub mod switcher;
#[cfg(test)]
mod tests;

pub use crate::domain::views::{PostView, ViewScope};
pub use registry::{view_icon, view_label, view_label_key};
pub use switcher::{render_view_switcher, switcher_button_id, PickView};
