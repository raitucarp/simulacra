//! Semantic search from the palette (sb B3): the query is debounced, embedded on the
//! background executor and ranked against the index; when meaning is unavailable (search
//! off, no key, nothing indexed, or a failed request) word matches answer instead, with a
//! subtle note saying so.

use super::super::view::AppView;
use crate::agent::embedding::{embed_blocking, EmbedInput, EmbeddingClient, TaskType};
use crate::domain::embedding::{top_k, FailureKind};
use crate::state::semantic::{hits_for, lexical_hits, HitTarget, SemanticHit};
use crate::ui::features::command_palette::model::{PaletteGroup, PaletteItem, PaletteTarget, SemanticPhase, SemanticRow, SemanticSource};
use crate::ui::features::semantic::hit_icon;
use gpui_kit::*;
use rust_i18n::t;
use std::time::Duration;

/// Results shown for a query.
pub(super) const RESULT_LIMIT: usize = 12;
/// Quiet time after the last keystroke before a query is embedded.
const QUERY_DEBOUNCE: Duration = Duration::from_millis(300);

/// Why word matches answer instead of meaning.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum Fallback {
    Off,
    NoKey,
    NotIndexed,
    Failed(String),
}

impl Fallback {
    pub(super) fn note(&self) -> String {
        match self {
            Self::Off => t!("semantic.fallback.off").to_string(),
            Self::NoKey => t!("semantic.fallback.no_key").to_string(),
            Self::NotIndexed => t!("semantic.fallback.not_indexed").to_string(),
            Self::Failed(reason) => t!("semantic.fallback.failed", reason = reason.as_str()).to_string(),
        }
    }
}

impl AppView {
    /// A palette keystroke in semantic mode: runs `text` once typing pauses.
    pub(in crate::ui::features::shell) fn on_semantic_query(&mut self, text: String, window: &mut Window, cx: &mut Context<Self>) {
        self.semantic.query_generation += 1;
        let generation = self.semantic.query_generation;
        cx.spawn_in(window, async move |this, cx| {
            cx.background_executor().timer(QUERY_DEBOUNCE).await;
            let _ = this.update_in(cx, |this, window, cx| {
                if this.semantic.query_generation == generation {
                    this.run_semantic_query(text, window, cx);
                }
            });
        })
        .detach();
    }

    /// Why meaning cannot answer right now, if it cannot.
    pub(super) fn semantic_fallback(&self) -> Option<Fallback> {
        let config = self.embedding_config();
        if !self.state.preferences().embeddings.semantic_search {
            return Some(Fallback::Off);
        }
        if config.readiness().is_err() {
            return Some(Fallback::NoKey);
        }
        (self.semantic.index.count_for_model(&config.model) == 0).then_some(Fallback::NotIndexed)
    }

    fn run_semantic_query(&mut self, text: String, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(fallback) = self.semantic_fallback() {
            return self.show_word_matches(text, fallback, cx);
        }
        let client = EmbeddingClient::new(self.embedding_config());
        let input = [EmbedInput::text(text.clone())];
        let task = cx.background_executor().spawn(async move { embed_blocking(&client, TaskType::RetrievalQuery, &input) });
        cx.spawn_in(window, async move |this, cx| {
            let outcome = task.await;
            let _ = this.update_in(cx, |this, window, cx| match outcome.map(|mut vectors| vectors.pop()) {
                Ok(Some(query)) => this.show_meaning_matches(text, &query, cx),
                Ok(None) => this.show_word_matches(text, Fallback::Failed(this.semantic_failure_text(FailureKind::InvalidResponse)), cx),
                Err(err) => {
                    let reason = this.semantic_failure_text(err.kind);
                    if matches!(err.kind, FailureKind::BadKey | FailureKind::ModelNotFound) {
                        this.report_semantic_failure(err.kind, &err.detail, window, cx);
                    }
                    this.show_word_matches(text, Fallback::Failed(reason), cx);
                }
            });
        })
        .detach();
    }

    fn show_meaning_matches(&mut self, query: String, vector: &[f32], cx: &mut Context<Self>) {
        let model = self.embedding_config().model;
        let ranked = top_k(&self.semantic.index, vector, &model, RESULT_LIMIT, |owner| self.state.is_in_active_network(owner));
        let rows = hits_for(&self.state, &ranked, self.chunk_spec()).into_iter().map(|hit| palette_row(hit, PaletteGroup::Meaning)).collect();
        self.push_semantic_phase(SemanticPhase::Ready { query, source: SemanticSource::Meaning, rows }, cx);
    }

    fn show_word_matches(&mut self, query: String, fallback: Fallback, cx: &mut Context<Self>) {
        let hits = lexical_hits(&self.state, &query, RESULT_LIMIT, self.chunk_spec());
        let rows = hits.into_iter().map(|hit| palette_row(hit, PaletteGroup::Words)).collect();
        let source = SemanticSource::Words { note: fallback.note() };
        self.push_semantic_phase(SemanticPhase::Ready { query, source, rows }, cx);
    }

    fn push_semantic_phase(&mut self, phase: SemanticPhase, cx: &mut Context<Self>) {
        if let Some(palette) = self.palette.palette.as_ref() {
            palette.update(cx, |palette, cx| palette.show_semantic_results(phase, cx));
        }
    }
}

/// A hit as a palette row.
fn palette_row(hit: SemanticHit, group: PaletteGroup) -> SemanticRow {
    let target = match &hit.target {
        HitTarget::Post(id) => PaletteTarget::Post(id.clone()),
        HitTarget::Document(id) => PaletteTarget::Document(id.clone()),
    };
    let item = PaletteItem { target, group, title: hit.title, detail: hit.detail, keywords: String::new(), icon: hit_icon(&hit.target), shortcut: None };
    SemanticRow { item, excerpt: hit.excerpt, score: hit.score }
}
