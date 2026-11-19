//! The indexing pump (sb B3): one batch at a time goes to the provider on the background
//! executor; finished vectors land in the index and the vault, failures pause the queue
//! with a clear notice. The daily AI budget is checked before every request.

use super::super::view::AppView;
use crate::agent::embedding::{embed_blocking, EmbedInput, EmbeddingClient, EmbeddingError, TaskType};
use crate::domain::embedding::{estimate_tokens, Batch, BudgetLimits, FailureKind, StoredChunk};
use crate::domain::preferences::{Choice, ToastKind};
use gpui_kit::component::notification::Notification;
use gpui_kit::*;
use rust_i18n::t;

/// Chunks per request (well under every provider's batch limit, so progress moves often).
const BATCH_SIZE: usize = 32;
/// Local calendar day key of the budget.
const DAY_FORMAT: &str = "%Y-%m-%d";
/// Locale prefixes of the failure notices and provider names.
const ERROR_KEY_PREFIX: &str = "semantic.error";
const PROVIDER_KEY_PREFIX: &str = "semantic.provider";

impl AppView {
    /// Sends the next batch unless one is out, the queue is stopped or the budget is spent.
    pub(in crate::ui::features::shell) fn pump_semantic_index(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let (queue, batch) = std::mem::take(&mut self.semantic.queue).take_batch(BATCH_SIZE);
        self.semantic.queue = queue;
        let Some(batch) = batch else { return };
        let today = crate::time::local_now().strftime(DAY_FORMAT).to_string();
        let tokens = estimate_tokens(batch.jobs.iter().map(|job| job.text.as_str()));
        let limits = BudgetLimits::from_prefs(&self.state.preferences().ai);
        if !self.semantic.usage.allows(limits, &today, 1, tokens) {
            self.semantic.queue = std::mem::take(&mut self.semantic.queue).failed(batch.generation, FailureKind::Budget);
            return self.report_semantic_failure(FailureKind::Budget, "", window, cx);
        }
        self.semantic.usage = self.semantic.usage.recorded(&today, 1, tokens);
        if let Some(vault) = self.state.db.as_ref() {
            let _ = vault.save_embedding_usage(&self.semantic.usage);
        }
        let client = EmbeddingClient::new(self.embedding_config());
        let inputs: Vec<EmbedInput> = batch.jobs.iter().map(|job| EmbedInput { title: job.title.clone(), text: job.text.clone() }).collect();
        let task = cx.background_executor().spawn(async move { embed_blocking(&client, TaskType::RetrievalDocument, &inputs) });
        cx.spawn_in(window, async move |this, cx| {
            let outcome = task.await;
            let _ = this.update_in(cx, |this, window, cx| this.finish_semantic_batch(batch, outcome, window, cx));
        })
        .detach();
        cx.notify();
    }

    /// Stores a finished batch (or pauses on its failure) and sends the next one.
    fn finish_semantic_batch(&mut self, batch: Batch, outcome: Result<Vec<Vec<f32>>, EmbeddingError>, window: &mut Window, cx: &mut Context<Self>) {
        if !self.semantic.queue.is_current(batch.generation) {
            return;
        }
        match outcome {
            Ok(vectors) => {
                let model = self.embedding_config().model;
                let rows: Vec<_> = batch
                    .jobs
                    .into_iter()
                    .zip(vectors)
                    .map(|(job, vector)| (job.owner, StoredChunk { index: job.chunk_index, model: model.clone(), content_hash: job.content_hash, vector }))
                    .collect();
                if let Some(vault) = self.state.db.as_ref() {
                    let _ = vault.save_embeddings(rows.clone());
                }
                self.semantic.index = std::mem::take(&mut self.semantic.index).with_chunks(rows);
                self.semantic.queue = std::mem::take(&mut self.semantic.queue).completed(batch.generation);
                self.semantic.reported = None;
                self.pump_semantic_index(window, cx);
            }
            Err(err) => {
                self.semantic.queue = std::mem::take(&mut self.semantic.queue).failed(batch.generation, err.kind);
                self.semantic.badge_dismissed = false;
                self.report_semantic_failure(err.kind, &err.detail, window, cx);
            }
        }
        cx.notify();
    }

    /// Tells the user why embedding stopped (once per kind until it works again).
    pub(crate) fn report_semantic_failure(&mut self, kind: FailureKind, detail: &str, window: &mut Window, cx: &mut Context<Self>) {
        if self.semantic.reported == Some(kind) {
            return;
        }
        self.semantic.reported = Some(kind);
        if !detail.is_empty() {
            eprintln!("simulacra: embedding failed ({}): {detail}", kind.key());
        }
        let note = Notification::warning(self.semantic_failure_text(kind));
        self.toast(ToastKind::AiFailure, note, window, cx);
    }

    /// The localized notice of `kind` for the configured provider and model.
    pub(crate) fn semantic_failure_text(&self, kind: FailureKind) -> String {
        let config = self.embedding_config();
        let provider_key = format!("{PROVIDER_KEY_PREFIX}.{}", Choice::key(self.state.preferences().embeddings.provider));
        let provider = t!(provider_key.as_str()).to_string();
        let key = format!("{ERROR_KEY_PREFIX}.{}", kind.key());
        t!(key.as_str(), provider = provider, model = config.model.as_str()).to_string()
    }
}
