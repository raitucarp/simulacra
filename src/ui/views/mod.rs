//! Full application window views.

/// Compatibility path: the root view now lives in `crate::ui::features::shell`.
pub mod app_view {
    pub use crate::ui::features::shell::AppView;
}

pub use app_view::AppView;
