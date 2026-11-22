//! Searching from the Search page (Q4): every change runs the words and filters at once;
//! once typing pauses, the words are embedded and the search runs again with their meaning
//! (when search by meaning is on and ready), so one list holds both.

use super::super::view::AppView;
use super::semantic_search::Fallback;
use crate::agent::embedding::{embed_blocking, EmbedInput, EmbeddingClient, TaskType};
use crate::domain::embedding::{top_k, ScoredOwner};
use crate::domain::search_query::PostQuery;
use crate::state::search::{SearchHit, SearchRequest};
use crate::state::{AppState, NavigationTab};
use crate::ui::features::search::{snippet, stepped, SearchEvent, SearchRow};
use gpui_kit::*;
use jiff::tz::TimeZone;
use std::time::Duration;

/// Results the page lists.
const PAGE_RESULTS: usize = 100;
/// Posts asked of the meaning index.
const MEANING_CANDIDATES: usize = 50;
/// Quiet time after the last keystroke before the words are embedded.
const MEANING_DEBOUNCE: Duration = Duration::from_millis(300);

impl AppView {
    /// The field changed: runs the query now, and by meaning once typing pauses.
    pub(in crate::ui::features::shell) fn on_search_changed(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let fallback = self.semantic_fallback();
        let Some(host) = self.search.as_mut() else { return };
        host.query = PostQuery::parse(&host.input.read(cx).value());
        host.generation += 1;
        let (generation, words) = (host.generation, host.query.text());
        let wants_meaning = !words.trim().is_empty();
        self.show_search_results(&[], fallback.as_ref().filter(|_| wants_meaning).map(Fallback::note), cx);
        if wants_meaning && fallback.is_none() {
            self.search_by_meaning(words, generation, window, cx);
        }
    }

    fn search_by_meaning(&mut self, words: String, generation: u64, window: &mut Window, cx: &mut Context<Self>) {
        let client = EmbeddingClient::new(self.embedding_config());
        cx.spawn_in(window, async move |this, cx| {
            cx.background_executor().timer(MEANING_DEBOUNCE).await;
            let current = |this: &AppView| this.search.as_ref().is_some_and(|host| host.generation == generation);
            if !this.update(cx, |this, _| current(this)).unwrap_or(false) {
                return;
            }
            let input = [EmbedInput::text(words)];
            let outcome = cx.background_executor().spawn(async move { embed_blocking(&client, TaskType::RetrievalQuery, &input) }).await;
            let _ = this.update(cx, |this, cx| {
                if !current(this) {
                    return;
                }
                match outcome.map(|mut vectors| vectors.pop()) {
                    Ok(Some(vector)) => {
                        let model = this.embedding_config().model;
                        let ranked: Vec<ScoredOwner> = top_k(&this.semantic.index, &vector, &model, MEANING_CANDIDATES, |owner| this.state.is_in_active_network(owner));
                        this.show_search_results(&ranked, None, cx);
                    }
                    Ok(None) => this.show_search_results(&[], None, cx),
                    Err(err) => this.show_search_results(&[], Some(Fallback::Failed(this.semantic_failure_text(err.kind)).note()), cx),
                }
            });
        })
        .detach();
    }

    fn show_search_results(&mut self, meaning: &[ScoredOwner], note: Option<String>, cx: &mut Context<Self>) {
        let Some(host) = self.search.as_ref() else { return };
        let request = SearchRequest { query: &host.query, meaning, limit: PAGE_RESULTS, today: crate::time::local_now().date(), tz: TimeZone::system() };
        let rows: Vec<SearchRow> = self.state.search(&request).iter().filter_map(|hit| search_row(&self.state, hit, &host.query)).collect();
        let host = self.search.as_mut().expect("checked above");
        host.selected = (!rows.is_empty()).then_some(0);
        (host.rows, host.note) = (rows, note);
        cx.notify();
    }

    pub(in crate::ui::features::shell) fn on_search_event(&mut self, event: SearchEvent, window: &mut Window, cx: &mut Context<Self>) {
        let Some(host) = self.search.as_mut() else { return };
        let rewrite = match event {
            SearchEvent::RemoveFilter(filter, negated) => Some(host.query.without_filter(&filter, negated)),
            SearchEvent::ToggleFilter(filter) => Some(host.query.toggled_filter(filter)),
            SearchEvent::Step(forward) => {
                host.selected = stepped(host.selected, host.rows.len(), forward);
                None
            }
            SearchEvent::Confirm => {
                if let Some(id) = host.selected.and_then(|at| host.rows.get(at)).map(|row| row.post_id.clone()) {
                    self.open_search_hit(id, window, cx);
                }
                return;
            }
            SearchEvent::Open(id) => return self.open_search_hit(id, window, cx),
        };
        if let Some(query) = rewrite {
            host.input.update(cx, |input, cx| input.set_value(query.to_text(), window, cx));
            self.on_search_changed(window, cx);
        }
        cx.notify();
    }

    fn open_search_hit(&mut self, id: crate::domain::PostId, window: &mut Window, cx: &mut Context<Self>) {
        self.select_nav(NavigationTab::Timeline, window, cx);
        self.select_post(id, window, cx);
    }
}

/// A hit as the page shows it.
fn search_row(state: &AppState, hit: &SearchHit, query: &PostQuery) -> Option<SearchRow> {
    let post = state.get_post(&hit.post_id)?;
    let author = state.author_of(post);
    let day = crate::time::local_date(post.created_at).to_string();
    let by_meaning = hit.by_meaning && !hit.by_text;
    Some(SearchRow { post_id: post.id.clone(), author: author.name.clone(), handle: author.display_handle(), day, snippet: snippet(&post.content, query), by_meaning })
}
