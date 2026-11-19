//! Responsive layout classes: a pure mapping from the window width to compact, medium or wide
//! layouts, with the breakpoints taken from the theme's width tokens.
//!
//! - **Compact:** the right sidebar becomes a drawer over the content and the thread
//!   inspector stacks below the stream.
//! - **Medium:** the right sidebar narrows and the inspector still stacks below the stream.
//! - **Wide:** every column at its full size.

use crate::theme::ThemeTokens;
use gpui_kit::{Pixels, Window};

/// How much horizontal room the window gives the shell.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LayoutClass {
    Compact,
    Medium,
    Wide,
}

impl LayoutClass {
    /// The class of a window `width` wide (`layout_medium_min_width` and
    /// `layout_wide_min_width` start the medium and wide classes).
    pub fn for_width(width: Pixels, theme: &ThemeTokens) -> Self {
        if width >= theme.layout_wide_min_width {
            Self::Wide
        } else if width >= theme.layout_medium_min_width {
            Self::Medium
        } else {
            Self::Compact
        }
    }

    /// The class of `window`'s current width.
    pub fn of_window(window: &Window, theme: &ThemeTokens) -> Self {
        Self::for_width(window.viewport_size().width, theme)
    }

    /// True for the compact class.
    pub const fn is_compact(self) -> bool {
        matches!(self, Self::Compact)
    }

    /// Whether the thread inspector stacks under the stream (the center is too narrow for
    /// both side by side below the wide class).
    pub const fn stacks_inspector(self) -> bool {
        !matches!(self, Self::Wide)
    }

    /// Default width and resize range of the right sidebar column (the compact class shows it
    /// as a drawer of `right_drawer_width` instead).
    pub fn right_sidebar_size(self, theme: &ThemeTokens) -> (Pixels, Pixels, Pixels) {
        match self {
            Self::Wide => (theme.right_sidebar_width, theme.right_sidebar_min, theme.right_sidebar_max),
            Self::Medium | Self::Compact => {
                (theme.right_sidebar_medium_width, theme.right_sidebar_min, theme.right_sidebar_medium_max)
            }
        }
    }
}

#[cfg(test)]
mod tests;
