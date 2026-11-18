//! The Tensions view's requests in the shell (sb E1): "Find tensions with this post", scans
//! (again, or once the stream changed), reconciling a pair in the composer (quoting one side
//! and linking the other), marking pairs resolved or dismissed, and the view's rows.

use super::super::view::AppView;
use crate::domain::thinking::tensions::{Candidate, LexicalSignals, SideKind, TensionSide, TensionStatus};
use crate::domain::PostId;
use crate::state::semantic::hits::shortened;
use crate::ui::features::command_palette::CommandId;
use crate::ui::features::composer::ComposerModalMode;
use crate::ui::features::thinking::insights::{TensionRow, TensionsData};
use gpui_kit::*;
use rust_i18n::t;

/// Characters of the focused post named in the view.
const FOCUS_CHARS: usize = 60;

/// The signals of a pair, worded for the card.
fn signal_words(lexical: &LexicalSignals) -> Vec<String> {
    let negation = lexical.negation.then(|| t!("thinking.tensions.signal_negation").to_string());
    let pairs = lexical.antonyms.iter().chain(&lexical.quantifiers).map(|(a, b)| format!("{a} / {b}"));
    negation.into_iter().chain(pairs).collect()
}

/// How the composer refers to a side: a wikilink to the post, or the document's title.
fn reference(side: &TensionSide) -> String {
    match side.kind {
        SideKind::Post => format!("[[{}]]", side.id),
        SideKind::Passage => format!("[[{}]]", side.label),
    }
}

impl AppView {
    /// "Find tensions with this post": the Tensions view, scanning around the post.
    pub(super) fn find_tensions_with(&mut self, post_id: PostId, window: &mut Window, cx: &mut Context<Self>) {
        self.thinking.scan.focus = Some(TensionSide::post(post_id.as_str(), "", "").key());
        self.open_insights(CommandId::InsightsTensions, window, cx);
        self.rescan_tensions(cx);
    }

    pub(super) fn rescan_tensions(&mut self, cx: &mut Context<Self>) {
        self.thinking.scan.stamp = None;
        self.start_tension_scan(cx);
    }

    /// Scans when the Tensions view is open and the stream changed since the last scan.
    pub(super) fn ensure_tension_scan(&mut self, cx: &mut Context<Self>) {
        let current = (self.state.index.revision(), self.thinking.scan.focus.clone());
        let stale = self.thinking.scan.stamp.as_ref().map(|(revision, focus, _)| (*revision, focus.clone())) != Some(current);
        if stale && !self.thinking.scan.ranking {
            self.start_tension_scan(cx);
        }
    }

    fn candidate(&self, pair: &str) -> Option<Candidate> {
        self.thinking.scan.candidates.iter().find(|c| c.key() == pair).cloned()
    }

    /// Opens the composer quoting one side (a post) and linking the other.
    pub(super) fn reconcile_tension(&mut self, pair: &str, window: &mut Window, cx: &mut Context<Self>) {
        let Some(candidate) = self.candidate(pair) else { return };
        let (quoted, other) = match (candidate.left.kind, candidate.right.kind) {
            (SideKind::Post, _) => (Some(&candidate.left), &candidate.right),
            (_, SideKind::Post) => (Some(&candidate.right), &candidate.left),
            _ => (None, &candidate.right),
        };
        let quote = quoted.and_then(|side| self.state.get_post(&PostId::from(side.id.as_str()))).map(|post| self.state.authored(post));
        let draft = match quote {
            Some(_) => t!("thinking.tensions.reconcile_draft", other = reference(other)).to_string(),
            None => t!("thinking.tensions.reconcile_both", left = reference(&candidate.left), right = reference(other)).to_string(),
        };
        let mode = quote.map_or(ComposerModalMode::NewThread, |post| ComposerModalMode::Quote(Box::new(post)));
        self.composer_modal.update(cx, |panel, cx| panel.open_with_drafts(mode, vec![draft], window, cx));
        cx.notify();
    }

    pub(super) fn set_tension_status(&mut self, pair: &str, status: TensionStatus, cx: &mut Context<Self>) {
        if let Some(record) = self.thinking.tensions.get(pair).cloned() {
            self.remember_tensions(vec![record.with_status(status)]);
            cx.notify();
        }
    }

    /// The Tensions view's snapshot: candidates still shown, with their verdicts.
    pub(super) fn tensions_data(&self) -> TensionsData {
        let scan = &self.thinking.scan;
        let rows = scan.candidates.iter().filter_map(|candidate| {
            let record = self.thinking.tensions.get(&candidate.key()).filter(|r| r.is_shown())?;
            Some(TensionRow { pair: record.pair.clone(), left: candidate.left.clone(), right: candidate.right.clone(), verdict: record.verdict, reason: record.reason.clone(), signals: signal_words(&candidate.lexical) })
        });
        let focused = |post: &&crate::domain::Post| scan.focus.as_deref() == Some(TensionSide::post(post.id.as_str(), "", "").key().as_str());
        let focus = scan.focus.as_ref().and_then(|_| self.state.posts.iter().find(focused)).map(|post| shortened(&post.content, FOCUS_CHARS));
        TensionsData { rows: rows.collect(), mode: scan.mode, running: scan.ranking || scan.classifying.is_some(), focus, problem: scan.problem.clone(), scanned: scan.scanned }
    }
}
