//! Views of a list of posts in the interface (epic W): the registry's icons and names, the
//! switcher the thread inspector and the profile page share, and the list views themselves.
//!
//! - [`registry`]: the icon and translated name of every [`PostView`].
//! - [`switcher`]: the row of view buttons for one scope.
//! - [`body`]: the list views over a [`ViewInput`] (rapid log W2, raw W3, calendar W4, future W5, board W6).

pub mod body;
pub mod common;
pub mod input;
mod board;
pub(crate) mod calendar;
mod future;
mod garden; // N3
pub(crate) mod branches; // N6
mod series; // N10
mod still_true; // N13
mod rapid_log;
mod raw;
pub mod registry;
pub mod switcher;
#[cfg(test)]
mod tests;

pub use crate::domain::views::{PostView, ViewScope};
pub use body::render_list_view;
pub use garden::{maturity_art, maturity_label};
pub use input::{OpenPost, PickMonth, ViewInput};
pub use registry::{view_icon, view_label, view_label_key};
pub use switcher::{render_view_switcher, switcher_button_id, PickView};
