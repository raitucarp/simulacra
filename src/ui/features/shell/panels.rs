//! Cached shell sections.
//!
//! In this gpui, `cx.notify()` on any child entity (an input, the composer) marks every
//! ancestor view dirty, so the root `AppView` re-renders on each keystroke. The heavy
//! sections therefore live in their own [`ShellPanel`] views rendered through
//! `AnyView::cached`: gpui reuses their last frame unless the panel itself (or one of its own
//! descendants) was notified. `AppView` notifies every panel whenever it is notified itself
//! (see [`AppView::observe_own_changes`]), so state changes still reach them, while typing in
//! the composer only re-renders the center panel that contains it.

use super::binder::Binder;
use super::view::AppView;
use crate::ui::render_stats;
use crate::ui::responsive::LayoutClass;
use gpui_kit::*;

/// A section of the window rendered by its own cached view.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShellSection {
    LeftSidebar,
    Center,
    RightSidebar,
}

impl ShellSection {
    /// Name used by the dev render counter.
    pub const fn stats_name(self) -> &'static str {
        match self {
            Self::LeftSidebar => "LeftSidebar",
            Self::Center => "Center",
            Self::RightSidebar => "RightSidebar",
        }
    }
}

/// View rendering one shell section from the root view's state.
pub struct ShellPanel {
    app: WeakEntity<AppView>,
    section: ShellSection,
}

impl Render for ShellPanel {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        render_stats::record(self.section.stats_name());
        let Some(app) = self.app.upgrade() else {
            return div().into_any_element();
        };
        let bind = Binder::from_entity(app.clone());
        let view = app.read(cx);
        let class = LayoutClass::of_window(window, &view.theme);
        match self.section {
            ShellSection::LeftSidebar => view.left_sidebar(&bind).into_any_element(),
            ShellSection::Center => view.center_panel(&bind, class),
            ShellSection::RightSidebar => view.right_sidebar(&bind).into_any_element(),
        }
    }
}

/// The cached section views of the shell.
pub struct ShellPanels {
    left: Entity<ShellPanel>,
    center: Entity<ShellPanel>,
    right: Entity<ShellPanel>,
}

impl ShellPanels {
    pub(super) fn new(cx: &mut Context<AppView>) -> Self {
        let app = cx.entity().downgrade();
        let mut panel = |section| cx.new(|_| ShellPanel { app: app.clone(), section });
        Self {
            left: panel(ShellSection::LeftSidebar),
            center: panel(ShellSection::Center),
            right: panel(ShellSection::RightSidebar),
        }
    }

    /// Marks every section for re-render (the root state changed).
    pub(super) fn invalidate(&self, cx: &mut App) {
        for panel in [&self.left, &self.center, &self.right] {
            panel.update(cx, |_, cx| cx.notify());
        }
    }

    /// The section as a cached, full-size view element.
    pub(super) fn cached(&self, section: ShellSection) -> AnyElement {
        let panel = match section {
            ShellSection::LeftSidebar => &self.left,
            ShellSection::Center => &self.center,
            ShellSection::RightSidebar => &self.right,
        };
        AnyView::from(panel.clone()).cached(StyleRefinement::default().size_full()).into_any_element()
    }
}

impl AppView {
    /// Re-renders every cached section whenever the root view itself is notified.
    pub(super) fn observe_own_changes(cx: &mut Context<Self>) -> Subscription {
        cx.observe_self(|this, cx| this.panels.invalidate(cx))
    }
}
