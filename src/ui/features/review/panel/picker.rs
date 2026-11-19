//! The post picker: the stream's posts, filtered as the learner types, to learn one (write
//! its question or answer) or to gather several into a connection map.

use super::{PanelFields, ReviewPanel, Screen};
use crate::domain::review::connections::MAX_POINTS;
use crate::domain::PostId;
use crate::ui::features::review::data::PostView;
use crate::ui::features::review::events::{PickerMode, ReviewEvent};
use gpui_kit::{App, Context, Window};

/// Most posts listed at once.
pub const PICKER_ROWS: usize = 80;

/// The picker's state.
#[derive(Debug, Clone)]
pub struct Picker {
    pub mode: PickerMode,
    /// The stream's posts, newest first.
    pub posts: Vec<PostView>,
    /// Posts gathered for a map, in the order picked.
    pub picked: Vec<PostId>,
}

/// Whether `view` matches every word of `query` (in its text, author name or handle).
pub fn matches_query(view: &PostView, query: &str) -> bool {
    let haystack = format!("{} {} {}", view.post.content, view.author.name, view.author.handle).to_lowercase();
    query.to_lowercase().split_whitespace().all(|word| haystack.contains(word))
}

impl Picker {
    /// The posts matching `query`, at most [`PICKER_ROWS`].
    pub fn visible(&self, query: &str) -> Vec<&PostView> {
        self.posts.iter().filter(|view| matches_query(view, query)).take(PICKER_ROWS).collect()
    }
}

impl ReviewPanel {
    /// Opens the picker over `posts` (newest first).
    pub fn open_picker(&mut self, mode: PickerMode, posts: Vec<PostView>, window: &mut Window, cx: &mut Context<Self>) {
        self.picker = Some(Picker { mode, posts, picked: Vec::new() });
        self.fields.search.update(cx, |input, cx| {
            input.set_value("", window, cx);
            input.focus(window, cx);
        });
        self.screen = Screen::Picker;
        cx.notify();
    }

    pub(in crate::ui::features::review) fn picker_query(&self, cx: &App) -> String {
        PanelFields::text(&self.fields.search, cx)
    }

    /// Adds or removes `id` from the posts gathered for a map.
    pub(in crate::ui::features::review) fn toggle_picked(&mut self, id: PostId, cx: &mut Context<Self>) {
        let Some(picker) = self.picker.as_mut() else { return };
        match picker.picked.iter().position(|p| *p == id) {
            Some(at) => {
                picker.picked.remove(at);
            }
            None if picker.picked.len() < MAX_POINTS => picker.picked.push(id),
            None => {}
        }
        cx.notify();
    }

    /// Seeds a map from the gathered posts.
    pub(in crate::ui::features::review) fn map_from_picked(&mut self, cx: &mut Context<Self>) {
        let Some(picked) = self.picker.as_ref().map(|p| p.picked.clone()).filter(|p| !p.is_empty()) else { return };
        cx.emit(ReviewEvent::MapFromPosts(picked));
    }
}
