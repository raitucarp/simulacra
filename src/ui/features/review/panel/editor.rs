//! The connection map editor: the seeded map on its ring, where clicking two points joins
//! or separates them; connections can be turned one-way and named, points removed and
//! concepts added. Saving schedules the map as one review item.

use super::{PanelFields, ReviewPanel, Screen};
use crate::domain::review::connections::edit::{cycled_direction, map_problem, toggled_edge, with_concept, without_edge, without_point, MapProblem};
use crate::domain::review::connections::layout::slots;
use crate::domain::review::connections::ConnectionMap;
use crate::domain::review::ItemId;
use crate::ui::features::review::data::PostViews;
use crate::ui::features::review::events::ReviewEvent;
use gpui_kit::{Context, Window};
use rust_i18n::t;

/// A map being made or edited.
#[derive(Debug, Clone)]
pub struct MapEditor {
    /// The item being edited (none for a new map).
    pub existing: Option<ItemId>,
    pub map: ConnectionMap,
    /// Posts of the points (a seeded map may refer to posts no item uses yet).
    pub posts: PostViews,
    /// The first point of a connection being drawn.
    pub pending: Option<usize>,
    /// New connections run one way, from the first point clicked.
    pub directed: bool,
    pub problem: Option<MapProblem>,
}

impl MapEditor {
    /// Ring positions of the points (in order, so the structure reads clearly).
    pub fn positions(&self) -> Vec<(f32, f32)> {
        slots(self.map.points.len())
    }
}

impl ReviewPanel {
    /// Opens the editor on `map` (seeded by the shell, or an existing item's).
    pub fn open_map_editor(&mut self, map: ConnectionMap, existing: Option<ItemId>, posts: PostViews, window: &mut Window, cx: &mut Context<Self>) {
        let title = map.title.clone();
        self.fields.title.update(cx, |input, cx| input.set_value(title, window, cx));
        self.editor = Some(MapEditor { existing, map, posts, pending: None, directed: false, problem: None });
        self.screen = Screen::MapEditor;
        cx.notify();
    }

    /// Opens the editor on the map item `id`.
    pub(in crate::ui::features::review) fn edit_map(&mut self, id: &ItemId, window: &mut Window, cx: &mut Context<Self>) {
        let Some(map) = self.data.item(id).and_then(|item| item.subject.map()).cloned() else { return };
        let posts = self.data.posts.clone();
        self.open_map_editor(map, Some(id.clone()), posts, window, cx);
    }

    fn edit(&mut self, change: impl FnOnce(&mut MapEditor), cx: &mut Context<Self>) {
        if let Some(editor) = self.editor.as_mut() {
            change(editor);
            editor.problem = None;
            cx.notify();
        }
    }

    fn change_map(&mut self, change: impl FnOnce(ConnectionMap) -> ConnectionMap, cx: &mut Context<Self>) {
        self.edit(|editor| editor.map = change(std::mem::take(&mut editor.map)), cx);
    }

    /// A click on point `point`: the first of a pair, or the second (joining or separating).
    pub(in crate::ui::features::review) fn click_editor_point(&mut self, point: usize, cx: &mut Context<Self>) {
        let relation = PanelFields::text(&self.fields.relation, cx);
        self.edit(
            |editor| match editor.pending.take() {
                None => editor.pending = Some(point),
                Some(first) if first == point => {}
                Some(first) => {
                    let label = Some(relation).filter(|r| !r.is_empty());
                    editor.map = toggled_edge(std::mem::take(&mut editor.map), first, point, editor.directed, label);
                }
            },
            cx,
        );
    }

    pub(in crate::ui::features::review) fn toggle_directed(&mut self, cx: &mut Context<Self>) {
        self.edit(|editor| editor.directed = !editor.directed, cx);
    }

    pub(in crate::ui::features::review) fn cycle_edge(&mut self, index: usize, cx: &mut Context<Self>) {
        self.change_map(|map| cycled_direction(map, index), cx);
    }

    pub(in crate::ui::features::review) fn remove_edge(&mut self, index: usize, cx: &mut Context<Self>) {
        self.change_map(|map| without_edge(map, index), cx);
    }

    pub(in crate::ui::features::review) fn remove_point(&mut self, index: usize, cx: &mut Context<Self>) {
        self.edit(|editor| editor.pending = None, cx);
        self.change_map(|map| without_point(map, index), cx);
    }

    /// Adds the concept typed in its field.
    pub(in crate::ui::features::review) fn add_concept(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let label = PanelFields::text(&self.fields.concept, cx);
        if self.editor.is_none() || label.is_empty() {
            return;
        }
        self.change_map(|map| with_concept(map, &label), cx);
        self.fields.concept.update(cx, |input, cx| input.set_value("", window, cx));
    }

    /// Saves the map (or says what it still needs) and returns to the items.
    pub(in crate::ui::features::review) fn save_map(&mut self, cx: &mut Context<Self>) {
        let title = PanelFields::text(&self.fields.title, cx);
        let Some(editor) = self.editor.as_mut() else { return };
        editor.map.title = if title.is_empty() { t!("review.map.untitled").to_string() } else { title };
        editor.problem = map_problem(&editor.map);
        if editor.problem.is_none() {
            cx.emit(ReviewEvent::SaveMap { id: editor.existing.clone(), map: editor.map.clone() });
            self.editor = None;
            self.screen = Screen::Items;
        }
        cx.notify();
    }

    pub(in crate::ui::features::review) fn cancel_map(&mut self, cx: &mut Context<Self>) {
        self.editor = None;
        self.show(Screen::Items, cx);
    }
}
