//! Semantic search overlays (sb B3): the "find similar" sheet and the indexing indicator.

use super::super::binder::Binder;
use super::super::view::AppView;
use crate::domain::embedding::QueueStatus;
use crate::ui::features::semantic::{render_indexing_indicator, render_similar_sheet, IndexingBadge, SheetActions};
use crate::ui::features::settings::SettingsTab;
use gpui_kit::*;
use std::rc::Rc;

impl AppView {
    /// The "find similar" sheet, while open.
    pub(super) fn similar_sheet(&self, bind: &Binder) -> Option<Stateful<Div>> {
        let view = self.semantic.similar.as_ref()?;
        let actions = SheetActions {
            on_open: Rc::new(bind.with(Self::open_similar_hit)),
            on_again: Rc::new(bind.with(|this, target: crate::state::semantic::HitTarget, window, cx| this.find_similar(target.owner(), window, cx))),
            on_close: Rc::new(bind.act(Self::close_similar_sheet)),
        };
        Some(render_similar_sheet(view, actions, &self.semantic.sheet_focus, &self.theme))
    }

    /// The indexing pill: progress while running, the reason while paused (until closed).
    pub(super) fn indexing_indicator(&self, bind: &Binder) -> Option<Stateful<Div>> {
        let badge = match self.semantic.queue.status() {
            QueueStatus::Running => {
                let progress = self.semantic.queue.progress()?;
                IndexingBadge::Progress { done: progress.done, total: progress.total }
            }
            QueueStatus::Paused(kind) if !self.semantic.badge_dismissed => IndexingBadge::Paused { reason: self.semantic_failure_text(kind) },
            QueueStatus::Paused(_) | QueueStatus::Idle | QueueStatus::Cancelled => return None,
        };
        let on_stop = match badge {
            IndexingBadge::Progress { .. } => bind.boxed_act(Self::cancel_semantic_index),
            IndexingBadge::Paused { .. } => bind.boxed_act(|this, _, cx| {
                this.semantic.badge_dismissed = true;
                cx.notify();
            }),
        };
        let on_open = bind.boxed_act(|this, window, cx| this.open_settings_tab(SettingsTab::Embeddings, window, cx));
        Some(render_indexing_indicator(&badge, on_stop, on_open, &self.theme))
    }
}
