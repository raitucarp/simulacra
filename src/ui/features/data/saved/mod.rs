//! The Knowledge page's "Saved" view (chosen over a Timeline filter because collections are
//! the vault's own shelves): chips for Bookmarks and every collection, the shelf's items in
//! order (posts and documents together, each with its note), drag a row by its grip to move
//! it, and "Learn this collection" for the Review page. The Library / Saved switch sits above
//! the vault and this view.

mod render;
mod rows;

pub use render::render_saved_page;

use super::events::{DataAction, DataEvent, SavedTarget};
use super::text::tr;
use crate::domain::data::saved::ItemRef;
use crate::theme::ThemeTokens;
use crate::ui::components::{segment_pill, segmented_track, Segment};
use crate::ui::icons::LucideIcon;
use gpui_kit::component::input::InputState;
use gpui_kit::*;

const SWITCH_ID_PREFIX: &str = "knowledge-switch-";

/// One chip of the shelf row.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShelfChip {
    pub target: SavedTarget,
    pub label: String,
    pub count: usize,
}

/// One row of the shelf.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SavedItemView {
    pub item: ItemRef,
    pub title: String,
    /// The author and time of a post, or the kind and creators of a document.
    pub subtitle: String,
    pub note: String,
}

/// Everything the Saved view shows.
#[derive(Clone)]
pub struct SavedView {
    pub target: SavedTarget,
    pub chips: Vec<ShelfChip>,
    pub items: Vec<SavedItemView>,
    /// Collections are ordered by hand; bookmarks are newest first.
    pub reorderable: bool,
    pub new_collection: Entity<InputState>,
    pub note: Entity<InputState>,
    pub editing_note: Option<ItemRef>,
}

/// The "Library | Saved" switch above the Knowledge page.
pub fn render_knowledge_switch(saved: bool, on_event: DataAction, theme: &ThemeTokens) -> Div {
    let pill = |key: &str, icon: LucideIcon, active: bool, event: DataEvent| {
        let on_event = on_event.clone();
        let segment = Segment { id: SharedString::from(format!("{SWITCH_ID_PREFIX}{key}")).into(), icon, label: tr(&format!("data.saved.switch.{key}")), active, active_icon_color: theme.accent_brand };
        segment_pill(segment, theme, move |window, cx| on_event(event.clone(), window, cx))
    };
    let track = segmented_track(theme).child(pill("library", LucideIcon::Library, !saved, DataEvent::ShowLibrary)).child(pill("saved", LucideIcon::Bookmark, saved, DataEvent::ShowSaved));
    div()
        .w_full()
        .flex_none()
        .px(theme.panel_padding_compact)
        .py(theme.spacing_xs)
        .bg(theme.bg_app)
        .border_b(theme.border_width_hairline)
        .border_color(theme.border_subtle)
        .child(div().w(theme.knowledge_switch_width).max_w_full().child(track))
}
