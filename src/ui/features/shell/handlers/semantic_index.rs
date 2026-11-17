//! Keeping the semantic index current (sb B3): on each render a cheap key says whether posts,
//! documents or the provider changed; if so the pure plan diffs them against the stored
//! vectors, trims what is gone and queues what is new or stale for the background pump.

use super::super::view::AppView;
use crate::agent::embedding::EmbeddingConfig;
use crate::domain::embedding::{content_hash, plan_index, ChunkSpec, IndexPlan, PlanSpec};
use crate::domain::preferences::ToastKind;
use crate::state::semantic::{index_sources, IndexKey};
use gpui_kit::component::notification::Notification;
use gpui_kit::component::WindowExt;
use gpui_kit::*;
use rust_i18n::t;

/// Joins the parts of the configuration identity.
const IDENTITY_SEPARATOR: &str = "|";

impl AppView {
    /// The embedding configuration the preferences and stored keys describe.
    pub(crate) fn embedding_config(&self) -> EmbeddingConfig {
        EmbeddingConfig::resolve(self.state.preferences(), &self.state.credentials, &self.state.llm_settings)
    }

    /// The chunk geometry of the Embeddings preferences.
    pub(crate) fn chunk_spec(&self) -> ChunkSpec {
        ChunkSpec::from_prefs(&self.state.preferences().embeddings)
    }

    /// Render hook: loads the stored vectors once, then plans and pumps when anything changed.
    pub(in crate::ui::features::shell) fn sync_semantic_index(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if !self.semantic.loaded {
            self.load_semantic_index();
        }
        let config = self.embedding_config();
        if !self.state.preferences().embeddings.semantic_search || config.readiness().is_err() {
            return;
        }
        let key = IndexKey::of(&self.state, &identity(&config));
        if self.semantic.watched.as_ref() == Some(&key) && !self.semantic.force_next_plan {
            return;
        }
        if self.semantic.watched.as_ref().is_some_and(|watched| !watched.same_config(&key)) {
            self.semantic.queue = std::mem::take(&mut self.semantic.queue).resumed();
            self.semantic.reported = None;
        }
        let spec = PlanSpec { chunk: self.chunk_spec(), model: config.model.clone(), signature: config.signature(), force: self.semantic.force_next_plan };
        let plan = plan_index(&index_sources(&self.state), &self.semantic.index, &spec);
        self.apply_plan(plan);
        self.semantic.watched = Some(key);
        self.semantic.force_next_plan = false;
        self.pump_semantic_index(window, cx);
    }

    fn load_semantic_index(&mut self) {
        self.semantic.loaded = true;
        let Some(vault) = self.state.db.as_ref() else { return };
        match vault.load_vector_index() {
            Ok(index) => self.semantic.index = index,
            Err(err) => eprintln!("simulacra: embeddings not loaded: {err}"),
        }
        self.semantic.usage = vault.load_embedding_usage();
    }

    /// Drops trimmed and removed rows (memory and vault), then queues the jobs.
    fn apply_plan(&mut self, plan: IndexPlan) {
        let IndexPlan { jobs, trims, removals } = plan;
        let cuts = trims.into_iter().chain(removals.into_iter().map(|owner| (owner, 0)));
        for (owner, keep) in cuts {
            self.semantic.index = std::mem::take(&mut self.semantic.index).truncated(&owner, keep);
            if let Some(vault) = self.state.db.as_ref() {
                let _ = vault.trim_embeddings(&owner, keep);
            }
        }
        self.semantic.queue = std::mem::take(&mut self.semantic.queue).with_plan(jobs);
    }

    /// Settings › Rebuild the index: re-embeds everything with the current model.
    pub(crate) fn rebuild_semantic_index(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let config = self.embedding_config();
        if let Err(kind) = config.readiness() {
            return self.report_semantic_failure(kind, "", window, cx);
        }
        self.semantic.queue = std::mem::take(&mut self.semantic.queue).cancelled().resumed();
        (self.semantic.force_next_plan, self.semantic.reported, self.semantic.badge_dismissed) = (true, None, false);
        let note = Notification::info(t!("semantic.indexing.rebuild_started", model = config.model.as_str()).to_string());
        self.toast(ToastKind::Confirmation, note, window, cx);
        self.sync_semantic_index(window, cx);
        cx.notify();
    }

    /// Stops the running indexing (the indicator's stop button, Settings › Stop).
    pub(crate) fn cancel_semantic_index(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let was_running = self.semantic.queue.progress().is_some();
        self.semantic.queue = std::mem::take(&mut self.semantic.queue).cancelled();
        self.semantic.badge_dismissed = true;
        let key = if was_running { "semantic.indexing.cancelled" } else { "semantic.indexing.nothing" };
        window.push_notification(Notification::info(t!(key).to_string()), cx);
        cx.notify();
    }
}

/// Model, signature, endpoint and a hash of the key: a change resumes a paused queue.
fn identity(config: &EmbeddingConfig) -> String {
    let key_hash = content_hash(&[config.api_key.trim()]);
    [config.model.as_str(), &config.signature(), &config.endpoint, &key_hash].join(IDENTITY_SEPARATOR)
}
