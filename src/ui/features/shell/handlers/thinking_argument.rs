//! Argument maps in the shell (sb E1): the inspected thread's root and posts, keeping the
//! Argument view on it, saving manual edits per thread root, and the AI extraction ("Map the
//! argument") through the post actions runner, validated against the thread's posts.

use super::super::thinking_host::{Running, ThinkingHost};
use super::super::view::AppView;
use super::thinking_ai::failure_message;
use crate::agent::post_actions::{ActionFailure, CancelToken};
use crate::domain::thinking::argument::{extraction_prompt, parse_extraction, ArgumentMap, ThreadPost, EXTRACT_ANSWER_TOKENS};
use crate::domain::spend::SpendEstimate;
use crate::domain::{Post, PostId};
use crate::state::{build_conversation, ReplyLimits};
use crate::ui::features::feed::inspector_outline::{thread_root, InspectorMode};
use crate::ui::features::thinking::argument::{ArgumentEvent, ArgumentPanel, ArgumentThread, ExtractStatus};
use gpui_kit::*;
use rust_i18n::t;

impl AppView {
    /// The root of the thread around `post_id`.
    pub(super) fn root_of(&self, post_id: &PostId) -> Option<PostId> {
        let post = self.state.get_post(&self.state.action_target(post_id))?;
        Some(thread_root(post, |id| self.state.get_post(id)).id.clone())
    }

    /// The thread's posts, root first, then every reply in reading order.
    pub(super) fn thread_posts(&self, root: &PostId) -> Vec<ThreadPost> {
        let Some(post) = self.state.get_post(root) else { return Vec::new() };
        let conversation = build_conversation(&self.state, post, ReplyLimits::UNLIMITED);
        let line = |post: &Post| ThreadPost { id: post.id.clone(), author: self.state.author_of(post).name.clone(), text: post.content.clone() };
        let replies = conversation.branches.iter().flat_map(|branch| std::iter::once(branch.reply).chain(branch.nested.iter().copied()));
        std::iter::once(post).chain(replies).map(line).collect()
    }

    /// Render hook: the Argument view shows the inspected thread and its map.
    pub(super) fn sync_argument_panel(&mut self, cx: &mut Context<Self>) {
        if self.inspector.mode != InspectorMode::Argument {
            return;
        }
        let inspected = self.feed.selected_thread_post_id.clone().or_else(|| self.feed.stream.rows().post(&self.state, 0).map(|post| post.id.clone()));
        let root = inspected.and_then(|id| self.root_of(&id));
        let thread = root.map(|root| {
            let posts = self.thread_posts(&root).into_iter().map(|post| (post.id, post.author)).collect();
            let map = self.thinking.maps.get(&root).cloned().unwrap_or_else(|| ArgumentMap::empty(root.clone()));
            ArgumentThread { root, posts, map }
        });
        self.thinking.argument.update(cx, |panel, cx| panel.set_thread(thread, cx));
    }

    pub(in crate::ui::features::shell) fn handle_argument_event(&mut self, _: &Entity<ArgumentPanel>, event: &ArgumentEvent, window: &mut Window, cx: &mut Context<Self>) {
        match event.clone() {
            ArgumentEvent::Extract(root) => self.extract_argument(root, cx),
            ArgumentEvent::Cancel => {
                ThinkingHost::stop(&mut self.thinking.extract);
                let status = ExtractStatus::Failed(t!("post_actions.failure.cancelled").to_string());
                self.thinking.argument.update(cx, |panel, cx| panel.set_status(status, cx));
            }
            ArgumentEvent::Changed(map) => self.save_argument_map(map),
            ArgumentEvent::OpenPost(id) => self.reveal_post(id, window, cx),
        }
    }

    /// Keeps `map` for its thread and queues it for the vault.
    fn save_argument_map(&mut self, map: ArgumentMap) {
        if let Some(vault) = self.state.db.as_ref() {
            let _ = vault.save_argument_map(map.clone(), jiff::Timestamp::now().as_second());
        }
        self.thinking.maps.insert(map.root.clone(), map);
    }

    /// "Map the argument": opens the post's thread in the Argument view, extracting a map
    /// with the AI when the thread has none yet.
    pub(super) fn map_argument_of(&mut self, post_id: PostId, window: &mut Window, cx: &mut Context<Self>) {
        self.reveal_post(post_id.clone(), window, cx);
        self.set_inspector_mode(InspectorMode::Argument, window, cx);
        self.sync_argument_panel(cx);
        if let Some(root) = self.root_of(&post_id).filter(|root| self.thinking.maps.get(root).map_or(true, ArgumentMap::is_empty)) {
            self.extract_argument(root, cx);
        }
    }

    fn extract_argument(&mut self, root: PostId, cx: &mut Context<Self>) {
        let again = root.clone();
        if self.has_ai_key() && self.confirm_spend(SpendEstimate::one(), Box::new(move |this, _, cx| this.extract_argument(again, cx)), cx) {
            return;
        }
        ThinkingHost::stop(&mut self.thinking.extract);
        let posts = self.thread_posts(&root);
        let language = self.state.preferences().general.language.english_label().to_string();
        let cancel = CancelToken::new();
        let status = match self.thinking_job(extraction_prompt(&posts, &language), EXTRACT_ANSWER_TOKENS, cancel.clone()) {
            Ok(job) => {
                let ticket = self.thinking.next_ticket();
                self.thinking.extract = Some(Running { cancel, ticket });
                self.thinking.extract_root = Some(root);
                self.spawn_thinking_job(job, ticket, Self::finish_extraction, cx);
                ExtractStatus::Running
            }
            Err(failure) => ExtractStatus::Failed(failure_message(&failure)),
        };
        self.thinking.argument.update(cx, |panel, cx| panel.set_status(status, cx));
    }

    fn finish_extraction(&mut self, ticket: u64, outcome: Result<String, ActionFailure>, cx: &mut Context<Self>) {
        let Some(root) = self.thinking.extract.take().filter(|running| running.ticket == ticket).and_then(|_| self.thinking.extract_root.take()) else { return };
        let posts = self.thread_posts(&root);
        let status = match outcome.map(|text| parse_extraction(&text, &posts)) {
            Ok(Some(extraction)) => {
                self.save_argument_map(ArgumentMap { root: root.clone(), ..extraction.map });
                ExtractStatus::Done { dropped: extraction.dropped_nodes + extraction.dropped_edges }
            }
            Ok(None) => ExtractStatus::Failed(t!("thinking.argument.unreadable").to_string()),
            Err(failure) => ExtractStatus::Failed(failure_message(&failure)),
        };
        self.thinking.argument.update(cx, |panel, cx| panel.set_status(status, cx));
        self.sync_argument_panel(cx);
        cx.notify();
    }
}
