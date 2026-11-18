//! Insights wiring of the shell (sb D2): the page entity, its remembered state, and the
//! report. Reports are computed on the background executor from an owned snapshot and cached
//! by a fingerprint of everything they depend on (store revision, network, documents,
//! profiles, vectors, the filter and the network scope), so returning to an earlier filter is
//! instant and nothing is computed while the page is closed.

use super::view::AppView;
use crate::domain::document::record::fnv1a;
use crate::domain::embedding::{owner_vectors, OwnerKind};
use crate::domain::insights::{InsightsInput, InsightsReport};
use crate::state::{AppState, NavigationTab};
use crate::storage::SettingKey;
use crate::ui::features::insights::{InsightsPanel, PageState};
use chrono::{Local, Utc};
use gpui_kit::*;
use std::collections::VecDeque;
use std::rc::Rc;

/// Reports kept by fingerprint.
const CACHED_REPORTS: usize = 4;

/// The page, the fingerprint of the newest requested report and the cached reports.
pub struct InsightsHost {
    pub panel: Entity<InsightsPanel>,
    requested: Option<u64>,
    cache: VecDeque<(u64, Rc<InsightsReport>)>,
    /// Fingerprint the author list was last built for.
    pub(super) authors_for: Option<u64>,
}

impl InsightsHost {
    pub(super) fn new(state: &AppState, window: &mut Window, cx: &mut Context<AppView>) -> (Self, Subscription) {
        let saved = state.db.as_ref().and_then(|db| db.get_setting(SettingKey::InsightsState).ok().flatten());
        let page = PageState::from_json(saved.as_deref());
        let theme = state.theme_tokens.clone();
        let panel = cx.new(|cx| InsightsPanel::new(page, theme, cx));
        let subscription = cx.subscribe_in(&panel, window, AppView::handle_insights_event);
        (Self { panel, requested: None, cache: VecDeque::new(), authors_for: None }, subscription)
    }

    fn cached(&self, fingerprint: u64) -> Option<Rc<InsightsReport>> {
        self.cache.iter().find(|(f, _)| *f == fingerprint).map(|(_, r)| Rc::clone(r))
    }

    fn remember(&mut self, fingerprint: u64, report: Rc<InsightsReport>) {
        self.cache.retain(|(f, _)| *f != fingerprint);
        self.cache.push_front((fingerprint, report));
        self.cache.truncate(CACHED_REPORTS);
    }
}

impl AppView {
    /// What the store side of a report depends on.
    pub(super) fn insights_data_fingerprint(&self) -> u64 {
        let state = &self.state;
        let key = format!("{}|{}|{}|{}|{}|{}", state.active_network_id.as_str(), state.index.revision(), state.documents.len(), state.user_profiles.len(), self.semantic.index.len(), Local::now().date_naive());
        fnv1a(key.as_bytes())
    }

    /// The data fingerprint combined with what the page asks for.
    fn insights_fingerprint(&self, page: &PageState) -> u64 {
        let inputs = serde_json::to_string(&page.report_inputs()).unwrap_or_default();
        fnv1a(format!("{}|{inputs}", self.insights_data_fingerprint()).as_bytes())
    }

    /// The owned snapshot a report is computed from.
    fn insights_input(&self, page: &PageState) -> InsightsInput {
        let state = &self.state;
        let posts = state.posts.iter().filter(|p| page.filter.all_networks || p.network_id == state.active_network_id).cloned().collect();
        let model = self.embedding_config().model;
        let vectors = owner_vectors(&self.semantic.index, OwnerKind::Post, &model).into_iter().map(|(owner, v)| (owner.id, v)).collect();
        InsightsInput {
            network: state.active_network_id.clone(),
            posts,
            documents: state.documents.clone(),
            profiles: state.user_profiles.clone(),
            subjects: state.library.subjects.iter().map(|s| (s.id.clone(), s.label.clone())).collect(),
            vectors,
            now: Utc::now(),
            offset: Some(*Local::now().offset()),
        }
    }

    /// Render hook: theme, author list, and (while the page is open) a fresh report when
    /// anything it depends on changed.
    pub(super) fn sync_insights(&mut self, cx: &mut Context<Self>) {
        let theme = self.theme.clone();
        self.insights.panel.update(cx, |panel, cx| panel.set_theme(&theme, cx));
        if self.state.active_tab != NavigationTab::Insights {
            return;
        }
        self.sync_insights_authors(cx);
        let page = self.insights.panel.read(cx).state().clone();
        let fingerprint = self.insights_fingerprint(&page);
        if self.insights.requested == Some(fingerprint) {
            return;
        }
        self.insights.requested = Some(fingerprint);
        if let Some(report) = self.insights.cached(fingerprint) {
            self.insights.panel.update(cx, |panel, cx| panel.set_report(report, cx));
            return;
        }
        self.insights.panel.update(cx, |panel, cx| panel.set_pending(true, cx));
        let input = self.insights_input(&page);
        let job = cx.background_executor().spawn(async move { InsightsReport::compute(&input, &page.filter, page.scope) });
        cx.spawn(async move |this, cx| {
            let report = Rc::new(job.await);
            let _ = this.update(cx, |this, cx| {
                this.insights.remember(fingerprint, Rc::clone(&report));
                if this.insights.requested == Some(fingerprint) {
                    this.insights.panel.update(cx, |panel, cx| panel.set_report(report, cx));
                }
            });
        })
        .detach();
    }
}
