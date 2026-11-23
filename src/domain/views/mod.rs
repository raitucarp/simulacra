//! Views of a list of posts (epic W): the ways a thread (in the inspector) or a profile's
//! posts (on its page) can be shown, and where each one is offered.
//!
//! The registry is [`PostView::ALL`] in display order; [`PostView::scopes`] says where a view
//! is offered, so the thread inspector and the profile page build their switchers from the
//! same list.

pub mod board; // W6
pub mod calendar; // W4
pub mod future; // W5
pub mod garden; // N3
pub mod memory; // W8
pub mod rapid_log; // W2
pub mod raw; // W3
#[cfg(test)]
mod tests;

use crate::domain::document::keyed::keyed_enum;

keyed_enum! {
    /// One way of showing posts. The key is stored (the inspector's view setting): never
    /// rename one.
    PostView {
        /// Post cards with avatars, media and actions.
        Cards => "cards",
        /// Nested, text-only bullets (sb C2).
        Outline => "outline",
        /// The thread as an argument map (sb E1).
        Argument => "argument",
        /// W2: bullets grouped by day, symbols as bullets.
        RapidLog => "rapid_log",
        /// W3: the text as written, with "copy all".
        Raw => "raw",
        /// W4: a month of days, each with its posts.
        Calendar => "calendar",
        /// W5: scheduled posts and dated to-dos ahead, by month.
        Future => "future",
        /// W6: to-dos and task posts in columns by state.
        Board => "board",
        /// N3: garden notes in beds by maturity.
        Garden => "garden",
    }
}

impl Default for PostView {
    fn default() -> Self {
        Self::Cards
    }
}

/// Where a view switcher is shown.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ViewScope {
    /// The thread inspector: one conversation.
    Thread,
    /// A persona's or agent's page: everything it wrote.
    Profile,
}

const EVERYWHERE: &[ViewScope] = &[ViewScope::Thread, ViewScope::Profile];
const THREAD_ONLY: &[ViewScope] = &[ViewScope::Thread];

impl PostView {
    /// Where this view is offered.
    pub const fn scopes(self) -> &'static [ViewScope] {
        match self {
            Self::Cards | Self::RapidLog | Self::Raw | Self::Calendar | Self::Future | Self::Board | Self::Garden => EVERYWHERE,
            Self::Outline | Self::Argument => THREAD_ONLY,
        }
    }

    /// Whether `scope` offers this view.
    pub fn offered_in(self, scope: ViewScope) -> bool {
        self.scopes().contains(&scope)
    }

    /// The views `scope` offers, in display order.
    pub fn in_scope(scope: ViewScope) -> Vec<Self> {
        Self::ALL.iter().copied().filter(|view| view.offered_in(scope)).collect()
    }

    /// This view where `scope` offers it, else cards.
    pub fn within(self, scope: ViewScope) -> Self {
        if self.offered_in(scope) { self } else { Self::Cards }
    }

    /// Cards and outline swap; every other view goes back to cards (the palette's toggle).
    pub fn toggled(self) -> Self {
        match self {
            Self::Cards => Self::Outline,
            _ => Self::Cards,
        }
    }
}
