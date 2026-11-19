//! Lens wiring of the shell (sb F3): the Concept timeline, Viewpoint diversity and Readability
//! views get their report from an owned snapshot of posts, vault documents, review items,
//! profiles and post vectors, computed on the background executor only while one of them is
//! open, and cached by a fingerprint of the store, the filter, the deck and the reading
//! language. The composer's readability meter follows its setting from here too.
//!
//! Private records (journal entries, mood check-ins, focus sessions, time capsules) are not
//! part of the snapshot.

use super::view::AppView;
use crate::domain::document::record::fnv1a;
use crate::domain::embedding::{owner_vectors, OwnerKind};
use crate::domain::insights::lenses::{review_source, LensInput, LensReport};
use crate::domain::readability::ReadingLanguage;
use crate::state::NavigationTab;
use crate::ui::features::insights::PageState;
use jiff::Timestamp;
use gpui_kit::*;
use std::collections::VecDeque;
use std::rc::Rc;

/// Lens reports kept by fingerprint.
const CACHED_REPORTS: usize = 4;

/// The newest requested fingerprint and the cached reports.
#[derive(Default)]
pub struct LensHost {
    requested: Option<u64>,
    cache: VecDeque<(u64, Rc<LensReport>)>,
}

impl LensHost {
    fn cached(&self, fingerprint: u64) -> Option<Rc<LensReport>> {
        self.cache.iter().find(|(f, _)| *f == fingerprint).map(|(_, r)| Rc::clone(r))
    }

    fn remember(&mut self, fingerprint: u64, report: Rc<LensReport>) {
        self.cache.retain(|(f, _)| *f != fingerprint);
        self.cache.push_front((fingerprint, report));
        self.cache.truncate(CACHED_REPORTS);
    }
}

impl AppView {
    /// The reading language the formulas follow (the interface language).
    fn reading_language(&self) -> ReadingLanguage {
        ReadingLanguage::from_code(self.state.preferences().general.language.code())
    }

    fn lens_fingerprint(&self, page: &PageState) -> u64 {
        let filter = serde_json::to_string(&page.filter).unwrap_or_default();
        let key = format!("{}|{filter}|{}|{:?}", self.insights_data_fingerprint(), self.review.version, self.reading_language());
        fnv1a(key.as_bytes())
    }

    /// The owned snapshot the lenses are computed from.
    fn lens_input(&self) -> LensInput {
        let state = &self.state;
        let model = self.embedding_config().model;
        let vectors = owner_vectors(&self.semantic.index, OwnerKind::Post, &model).into_iter().map(|(owner, v)| (owner.id, v)).collect();
        LensInput {
            posts: state.posts.clone(),
            documents: state.documents.clone(),
            profiles: state.user_profiles.clone(),
            subjects: state.library.subjects.iter().map(|s| (s.id.clone(), s.label.clone())).collect(),
            review: self.review.deck.items.iter().filter(|item| !item.retired).map(review_source).collect(),
            vectors,
            language: self.reading_language(),
            offset: Some(crate::time::local_now().offset()),
            ..LensInput::empty(state.active_network_id.clone(), Timestamp::now())
        }
    }

    /// Render hook: the composer meter's setting, and (while a lens view is open) a fresh
    /// lens report when anything it depends on changed.
    pub(super) fn sync_lenses(&mut self, cx: &mut Context<Self>) {
        let (enabled, language) = (self.state.preferences().writing.readability_meter, self.reading_language());
        self.composer_modal.update(cx, |panel, cx| panel.set_readability_meter(enabled, language, cx));
        let page = self.insights.panel.read(cx).state().clone();
        if self.state.active_tab != NavigationTab::Insights || !page.view.is_lens() {
            return;
        }
        let fingerprint = self.lens_fingerprint(&page);
        if self.insights.lenses.requested == Some(fingerprint) {
            return;
        }
        self.insights.lenses.requested = Some(fingerprint);
        if let Some(report) = self.insights.lenses.cached(fingerprint) {
            self.insights.panel.update(cx, |panel, cx| panel.set_lenses(report, cx));
            return;
        }
        self.insights.panel.update(cx, |panel, cx| panel.set_lens_pending(true, cx));
        let input = self.lens_input();
        let job = cx.background_executor().spawn(async move { LensReport::compute(&input, &page.filter) });
        cx.spawn(async move |this, cx| {
            let report = Rc::new(job.await);
            let _ = this.update(cx, |this, cx| {
                this.insights.lenses.remember(fingerprint, Rc::clone(&report));
                if this.insights.lenses.requested == Some(fingerprint) {
                    this.insights.panel.update(cx, |panel, cx| panel.set_lenses(report, cx));
                }
            });
        })
        .detach();
    }
}
