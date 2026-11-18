//! The atomic-note check in the shell (sb E1): the "Check atomicity" sheet on a post, its
//! split (the composer opens on one post per claim, quoting the original), and the
//! composer hint's setting (Settings › Reading & Writing) and sentence rules.

use super::super::binder::Binder;
use super::super::view::AppView;
use crate::domain::thinking::atomic::{check_atomicity, split_proposal};
use crate::domain::PostId;
use crate::import::segment::SentenceLocale;
use crate::state::semantic::hits::shortened;
use crate::ui::features::composer::ComposerModalMode;
use crate::ui::features::thinking::atomic::{render_atomic_sheet, AtomicSheet, AtomicSheetEvent};
use gpui_kit::*;
use std::rc::Rc;

/// Characters of the post shown in the sheet's header.
const SOURCE_CHARS: usize = 80;

impl AppView {
    /// The sentence rules of the reading language.
    pub(in crate::ui::features::shell) fn reading_locale(&self) -> SentenceLocale {
        SentenceLocale::from_language_tag(Some(self.state.preferences().general.language.code()))
    }

    /// Render hook: the composer's hint follows its setting and the reading language.
    pub(super) fn sync_atomic_hints(&mut self, cx: &mut Context<Self>) {
        let (enabled, locale) = (self.state.preferences().writing.atomic_hints, self.reading_locale());
        self.composer_modal.update(cx, |panel, cx| panel.set_atomic_hints(enabled, locale, cx));
    }

    /// "Check atomicity": the sheet with the post's score, findings and proposed split.
    pub(super) fn check_atomicity_of(&mut self, post_id: PostId, _: &mut Window, cx: &mut Context<Self>) {
        let Some(post) = self.state.get_post(&post_id) else { return };
        let locale = self.reading_locale();
        let report = check_atomicity(&post.content, locale);
        let proposal = split_proposal(&post.content, locale);
        self.thinking.atomic_sheet = Some(AtomicSheet { post_id, source: shortened(&post.content, SOURCE_CHARS), report, proposal });
        cx.notify();
    }

    fn on_atomic_sheet_event(&mut self, event: AtomicSheetEvent, window: &mut Window, cx: &mut Context<Self>) {
        let Some(sheet) = self.thinking.atomic_sheet.take() else { return };
        if event == AtomicSheetEvent::Split {
            if let Some(original) = self.state.get_post(&sheet.post_id).map(|post| self.state.authored(post)) {
                let mode = ComposerModalMode::Quote(Box::new(original));
                self.composer_modal.update(cx, |panel, cx| panel.open_with_drafts(mode, sheet.proposal, window, cx));
            }
        }
        cx.notify();
    }

    /// The sheet overlay, while open.
    pub(in crate::ui::features::shell) fn atomic_sheet_overlay(&self, bind: &Binder) -> Option<Stateful<Div>> {
        let sheet = self.thinking.atomic_sheet.as_ref()?;
        Some(render_atomic_sheet(sheet, Rc::new(bind.with(Self::on_atomic_sheet_event)), &self.theme))
    }
}
