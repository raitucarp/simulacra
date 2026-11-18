//! The contradiction finder's work (sb E1): the network's posts and vault passages as
//! candidate inputs (with their embedding vectors when meaning search is available), the
//! ranking on the background executor, and the AI classification of unchecked candidates in
//! small batches through the post actions runner, cached by each pair's content hash.

use super::super::thinking_host::Running;
use super::super::view::AppView;
use super::thinking_ai::failure_message;
use crate::agent::post_actions::{ActionFailure, CancelToken};
use crate::domain::post_actions::ActionFailureKind;
use crate::domain::embedding::{chunk_text, owner_vectors, EmbeddingOwner, OwnerKind};
use crate::domain::thinking::tensions::{classification_prompt, parse_classifications, rank_candidates, Candidate, CandidateInput, RankOptions, TensionRecord, TensionSide, BATCH_SIZE, CLASSIFY_ANSWER_TOKENS};
use crate::ui::features::thinking::insights::TensionMode;
use gpui_kit::*;
use std::collections::HashMap;

/// Posts and passages considered at most (the most recent posts, the first passages).
const MAX_POSTS: usize = 1_500;
const MAX_PASSAGES: usize = 400;
/// Batches classified after one scan (the rest wait for the next scan).
pub(super) const BATCHES_PER_SCAN: usize = 3;

impl AppView {
    /// Every post and passage of the active network, with vectors when meaning is available.
    fn tension_inputs(&self) -> Vec<CandidateInput> {
        let model = self.embedding_config().model;
        let index = &self.semantic.index;
        let meaning = self.semantic_fallback().is_none();
        let vectors: HashMap<String, Vec<f32>> = if meaning { owner_vectors(index, OwnerKind::Post, &model).into_iter().map(|(owner, v)| (owner.id, v)).collect() } else { HashMap::new() };
        let posts = self.state.current_network_posts().filter(|post| !post.is_instant_repost() && !post.content.trim().is_empty()).take(MAX_POSTS).map(|post| CandidateInput {
            side: TensionSide::post(post.id.as_str(), self.state.author_of(post).name.clone(), post.content.clone()),
            vector: vectors.get(post.id.as_str()).cloned(),
        });
        let spec = self.chunk_spec();
        let documents = self.state.documents.iter().filter(|doc| doc.network_id == self.state.active_network_id);
        let passages = documents.flat_map(|doc| chunk_text(&doc.content, spec).into_iter().map(move |chunk| (doc, chunk))).take(MAX_PASSAGES).map(|(doc, chunk)| {
            let owner = EmbeddingOwner::document(doc.id.clone());
            let vector = index.chunk(&owner, chunk.index).filter(|c| meaning && c.model == model).map(|c| c.vector.clone());
            CandidateInput { side: TensionSide::passage(doc.id.clone(), chunk.index, doc.title.clone(), chunk.text), vector }
        });
        posts.chain(passages).collect()
    }

    /// Ranks candidates in the background, then classifies the unchecked ones.
    pub(super) fn start_tension_scan(&mut self, cx: &mut Context<Self>) {
        let mode = match (self.has_ai_key(), self.semantic_fallback().is_none()) {
            (false, _) => TensionMode::LexicalOnly,
            (true, false) => TensionMode::NoMeaning,
            (true, true) => TensionMode::Full,
        };
        let inputs = self.tension_inputs();
        let options = RankOptions { focus: self.thinking.scan.focus.clone(), ..RankOptions::default() };
        let ticket = self.thinking.next_ticket();
        let scan = &mut self.thinking.scan;
        (scan.mode, scan.ranking, scan.problem, scan.stamp) = (mode, true, None, Some((self.state.index.revision(), options.focus.clone(), mode)));
        Self::stop_classifying(&mut self.thinking.scan.classifying);
        self.thinking.scan.ranking_ticket = ticket;
        let job = cx.background_executor().spawn(async move { rank_candidates(&inputs, &options) });
        cx.spawn(async move |this, cx| {
            let ranked = job.await;
            let _ = this.update(cx, |this, cx| this.finish_tension_ranking(ticket, ranked, cx));
        })
        .detach();
        cx.notify();
    }

    fn stop_classifying(running: &mut Option<Running>) {
        if let Some(running) = running.take() {
            running.cancel.cancel();
        }
    }

    fn finish_tension_ranking(&mut self, ticket: u64, ranked: Vec<Candidate>, cx: &mut Context<Self>) {
        if ticket != self.thinking.scan.ranking_ticket {
            return;
        }
        let fresh: Vec<TensionRecord> = ranked.iter().filter(|c| !self.thinking.tensions.contains_key(&c.key())).map(|c| TensionRecord::new(&c.key(), &c.left.key(), &c.right.key())).collect();
        self.remember_tensions(fresh);
        let scan = &mut self.thinking.scan;
        (scan.candidates, scan.ranking, scan.scanned) = (ranked, false, true);
        if scan.mode != TensionMode::LexicalOnly {
            self.classify_next_batch(BATCHES_PER_SCAN, cx);
        }
        cx.notify();
    }

    /// Keeps `records` and queues them for the vault.
    pub(super) fn remember_tensions(&mut self, records: Vec<TensionRecord>) {
        if records.is_empty() {
            return;
        }
        if let Some(vault) = self.state.db.as_ref() {
            let _ = vault.save_tension_records(records.clone(), chrono::Utc::now().timestamp());
        }
        self.thinking.tensions.extend(records.into_iter().map(|record| (record.pair.clone(), record)));
    }

    /// Sends the next batch of unchecked open candidates; `left` batches may follow it.
    fn classify_next_batch(&mut self, left: usize, cx: &mut Context<Self>) {
        let unchecked = |c: &&Candidate| self.thinking.tensions.get(&c.key()).is_some_and(|r| r.verdict.is_none() && r.is_shown());
        let batch: Vec<Candidate> = self.thinking.scan.candidates.iter().filter(unchecked).take(BATCH_SIZE).cloned().collect();
        if left == 0 || batch.is_empty() {
            return;
        }
        let language = self.state.preferences().general.language.english_label().to_string();
        let cancel = CancelToken::new();
        match self.thinking_job(classification_prompt(&batch, &language), CLASSIFY_ANSWER_TOKENS, cancel.clone()) {
            Ok(job) => {
                let ticket = self.thinking.next_ticket();
                self.thinking.scan.classifying = Some(Running { cancel, ticket });
                self.thinking.scan.batch = (batch, left - 1);
                self.spawn_thinking_job(job, ticket, Self::finish_classification, cx);
            }
            // An automatic scan the user has not allowed to spend keeps its word-based results quietly.
            Err(failure) => self.thinking.scan.problem = (failure.kind != ActionFailureKind::Unapproved).then(|| failure_message(&failure)),
        }
    }

    fn finish_classification(&mut self, ticket: u64, outcome: Result<String, ActionFailure>, cx: &mut Context<Self>) {
        if self.thinking.scan.classifying.as_ref().map(|r| r.ticket) != Some(ticket) {
            return;
        }
        self.thinking.scan.classifying = None;
        let (batch, left) = std::mem::take(&mut self.thinking.scan.batch);
        match outcome {
            Ok(text) => {
                let verdicts = parse_classifications(&text, &batch);
                let records = verdicts.iter().filter_map(|v| self.thinking.tensions.get(&v.pair).map(|r| r.clone().classified(v))).collect();
                self.remember_tensions(records);
                self.classify_next_batch(left, cx);
            }
            Err(failure) => self.thinking.scan.problem = Some(failure_message(&failure)),
        }
        cx.notify();
    }
}
