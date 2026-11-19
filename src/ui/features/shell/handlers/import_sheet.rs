//! The import sheet's controls and its two endings: posts (with the vault document) or the
//! vault document alone.

use super::super::import_state::ImportUi;
use super::super::view::AppView;
use super::import_flow::{report_timing, CREATOR_SEPARATOR};
use crate::domain::preferences::{PostIdentity, ToastKind};
use crate::import::apply::{vault_entry, ImportRequest, OriginalFile};
use crate::import::plan::AuthorChoice;
use crate::import::{DocumentMetadata, Granularity, LoadedFile};
use crate::state::{FeedTab, NavigationTab};
use crate::ui::features::import::ImportSession;
use gpui_kit::component::notification::Notification;
use gpui_kit::component::WindowExt;
use gpui_kit::*;
use rust_i18n::t;
use std::time::Instant;

/// `Some(text)` unless blank.
fn filled(text: String) -> Option<String> {
    (!text.is_empty()).then_some(text)
}

impl AppView {
    fn edit_import(&mut self, edit: impl FnOnce(&mut ImportSession), cx: &mut Context<Self>) {
        if let Some(session) = self.import.session.as_mut() {
            edit(session);
            cx.notify();
        }
    }

    pub(crate) fn select_import_granularity(&mut self, granularity: Granularity, _: &mut Window, cx: &mut Context<Self>) {
        self.edit_import(|session| session.set_granularity(granularity), cx);
    }

    pub(crate) fn toggle_import_unit(&mut self, index: usize, _: &mut Window, cx: &mut Context<Self>) {
        self.edit_import(|session| session.toggle(index), cx);
    }

    pub(crate) fn select_all_import_units(&mut self, on: bool, _: &mut Window, cx: &mut Context<Self>) {
        self.edit_import(|session| session.select_all(on), cx);
    }

    pub(crate) fn select_import_chapter(&mut self, chapter: usize, _: &mut Window, cx: &mut Context<Self>) {
        self.edit_import(|session| session.select_chapter(chapter), cx);
    }

    pub(crate) fn select_import_identity(&mut self, identity: PostIdentity, window: &mut Window, cx: &mut Context<Self>) {
        self.edit_import(|session| session.identity = identity, cx);
        if identity == PostIdentity::Pseudonym {
            self.import.inputs.pseudonym.update(cx, |state, cx| state.focus(window, cx));
        }
    }

    /// The request for the open sheet, from its session and edited inputs.
    fn import_request(&self, session: &ImportSession, file: &LoadedFile, create_posts: bool, cx: &App) -> ImportRequest {
        let inputs = &self.import.inputs;
        let text = |input| ImportUi::text(input, cx);
        let title = filled(text(&inputs.title)).or_else(|| file.doc.metadata.title.clone()).unwrap_or_else(|| file.name.clone());
        let creators = text(&inputs.creators).split(CREATOR_SEPARATOR.trim()).map(str::trim).filter(|c| !c.is_empty()).map(str::to_string).collect();
        let metadata = DocumentMetadata { title: Some(title.clone()), creators, language: filled(text(&inputs.language)), date: filled(text(&inputs.date)), ..file.doc.metadata.clone() };
        let original = OriginalFile { name: file.name.clone(), extension: file.extension(), sha256: file.doc.facts.sha256.clone(), bytes: file.bytes.clone() };
        ImportRequest {
            entry: vault_entry(&file.doc, &title, metadata),
            original: Some(original),
            units: if create_posts { session.selected_units() } else { Vec::new() },
            chapter_titles: session.chapter_titles(),
            author: AuthorChoice::from_identity(session.identity, &text(&inputs.pseudonym)),
            create_posts,
        }
    }

    /// Applies the sheet: the vault document, the kept original and (when asked) the posts.
    pub(crate) fn apply_import_sheet(&mut self, create_posts: bool, window: &mut Window, cx: &mut Context<Self>) {
        let Some(session) = self.import.session.clone() else { return };
        let Some(file) = session.loaded.clone() else { return };
        let request = self.import_request(&session, &file, create_posts, cx);
        let started = Instant::now();
        let outcome = self.state.apply_import(request);
        report_timing("apply", started);
        match outcome {
            Ok(done) => {
                let message = match done.posts {
                    0 => t!("import.done_vault", title = done.title),
                    posts => t!("import.done_posts", title = done.title, posts = posts),
                };
                self.toast(ToastKind::Confirmation, Notification::success(message.to_string()), window, cx);
                if let Some(reason) = done.original_error {
                    window.push_notification(Notification::warning(t!("import.original_not_kept", reason = reason).to_string()), cx);
                }
                if done.posts > 0 {
                    self.feed.feed_tab = FeedTab::ForYou;
                    self.select_nav(NavigationTab::Timeline, window, cx);
                }
            }
            Err(err) => window.push_notification(Notification::error(t!("import.apply_failed", reason = err.to_string()).to_string()), cx),
        }
        self.close_import(window, cx);
    }
}
