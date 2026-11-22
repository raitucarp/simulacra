//! The Search page in the shell (Q4): its field, the query read from it, the rows found and
//! the selection. Made the first time the page opens.

use super::binder::Binder;
use super::view::AppView;
use crate::domain::search_query::{PostQuery, SavedSearches};
use crate::ui::features::search::{render_search_page, SearchRow, SearchView};
use gpui_kit::component::input::{InputEvent, InputState};
use gpui_kit::*;
use rust_i18n::t;
use std::rc::Rc;

/// What the Search page keeps for the session.
pub struct SearchHost {
    pub input: Entity<InputState>,
    pub query: PostQuery,
    pub rows: Vec<SearchRow>,
    pub selected: Option<usize>,
    /// Why meaning did not help the last search.
    pub note: Option<String>,
    /// Bumped on every change of the query; an answer for an older query is dropped.
    pub generation: u64,
    /// Q5: the saved searches.
    pub saved: SavedSearches,
    /// Q6: earlier versions are searched too.
    pub include_versions: bool,
    _subscription: Subscription,
}

impl SearchHost {
    fn new(saved: SavedSearches, window: &mut Window, cx: &mut Context<AppView>) -> Self {
        let placeholder = t!("search.page.placeholder").to_string();
        let input = cx.new(|cx| InputState::new(window, cx).placeholder(placeholder));
        let subscription = cx.subscribe_in(&input, window, |this, _, event: &InputEvent, window, cx| {
            if matches!(event, InputEvent::Change) {
                this.on_search_changed(window, cx);
            }
        });
        Self { input, query: PostQuery::default(), rows: Vec::new(), selected: None, note: None, generation: 0, saved, include_versions: false, _subscription: subscription }
    }
}

impl AppView {
    /// Makes the page on its first opening and puts the cursor in its field.
    pub(super) fn open_search_page(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.search.is_none() {
            self.search = Some(SearchHost::new(self.state.saved_searches(), window, cx));
        }
        let Some(host) = self.search.as_ref() else { return };
        host.input.update(cx, |input, cx| input.focus(window, cx));
    }

    /// The Search page as it stands.
    pub(super) fn search_page(&self, bind: &Binder) -> AnyElement {
        let Some(host) = &self.search else { return div().into_any_element() };
        let on_event = Rc::new(bind.with(Self::on_search_event));
        let view = SearchView { input: &host.input, query: &host.query, rows: &host.rows, selected: host.selected, note: host.note.as_deref(), saved: &host.saved, include_versions: host.include_versions, on_event };
        render_search_page(view, &self.theme)
    }
}
