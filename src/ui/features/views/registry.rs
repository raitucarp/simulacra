//! What each view looks like in a switcher: its icon and its translated name.

use crate::domain::views::PostView;
use crate::ui::icons::LucideIcon;
use rust_i18n::t;

/// The icon of `view` in a switcher.
pub fn view_icon(view: PostView) -> LucideIcon {
    match view {
        PostView::Cards => LucideIcon::LayoutGrid,
        PostView::Outline => LucideIcon::ListTree,
        PostView::Argument => LucideIcon::Waypoints,
        PostView::RapidLog => LucideIcon::NotebookText,
        PostView::Raw => LucideIcon::FileText,
    }
}

/// The locale key of `view`'s name.
pub fn view_label_key(view: PostView) -> &'static str {
    match view {
        PostView::Cards => "feed.outline.show_cards",
        PostView::Outline => "feed.outline.show_outline",
        PostView::Argument => "thinking.argument.show",
        PostView::RapidLog => "views.rapid_log.show",
        PostView::Raw => "views.raw.show",
    }
}

/// The translated name of `view` (its switcher tooltip).
pub fn view_label(view: PostView) -> String {
    t!(view_label_key(view)).to_string()
}
