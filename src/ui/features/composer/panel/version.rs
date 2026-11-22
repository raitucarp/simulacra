//! The composer's version row state (V3): editing a published post, the version it starts
//! from, the bump the user picked over the one suggested by the changes so far, and the
//! optional edit note. Saving sends them with the text as a [`PostEdit`].

use super::super::version_picker::VersionView;
use super::ComposerModalPanel;
use crate::domain::data::history::{Attachments, Bump, EditBase, PostRecord};
use crate::domain::media::ImageAttachment;
use crate::domain::AuthoredPost;
use crate::state::PostEdit;
use gpui_kit::component::input::InputState;
use gpui_kit::*;
use rust_i18n::t;
use std::rc::Rc;

/// The row's state; `base` is `None` unless a published post is being edited.
pub(super) struct VersionPicker {
    pub base: Option<EditBase>,
    pub chosen: Option<Bump>,
    pub note: Entity<InputState>,
}

impl VersionPicker {
    pub fn new(window: &mut Window, cx: &mut Context<ComposerModalPanel>) -> Self {
        let placeholder = t!("data.edit.note_placeholder").to_string();
        let note = cx.new(|cx| InputState::new(window, cx).placeholder(placeholder));
        Self { base: None, chosen: None, note }
    }
}

impl ComposerModalPanel {
    /// Opens the composer on `target` (a published post), the edit starting from `base`.
    pub fn open_edit_version(&mut self, target: AuthoredPost, base: EditBase, window: &mut Window, cx: &mut Context<Self>) {
        let (text, target_post) = (base.text.clone(), target.post.clone());
        self.open_edit(target, window, cx);
        let record = base.record.clone().unwrap_or_else(|| PostRecord::of(&target_post));
        (self.tint, self.symbols.chosen) = (record.tint, record.symbols); // V10: the version's own look
        self.prefill_attachments(record.attachments.unwrap_or_else(|| PostRecord::of(&target_post).attachments.unwrap_or_default()), window, cx); // V11
        self.version.base = Some(base);
        if let Some(input) = self.drafts.first().map(|slot| slot.input.clone()) {
            input.update(cx, |state, cx| state.set_value(text, window, cx));
        }
        cx.notify();
    }

    /// Picks the bump the edit makes over the suggested one.
    pub(super) fn pick_bump(&mut self, bump: Bump, cx: &mut Context<Self>) {
        self.version.chosen = Some(bump);
        cx.notify();
    }

    /// Forgets the base, the picked bump and the note (opening and clearing the composer).
    pub(super) fn reset_version(&mut self, window: &mut Window, cx: &mut App) {
        (self.version.base, self.version.chosen) = (None, None);
        self.version.note.update(cx, |state, cx| state.set_value("", window, cx));
    }

    fn first_text(&self, cx: &App) -> String {
        self.drafts.first().map(|slot| slot.input.read(cx).value().to_string()).unwrap_or_default()
    }

    /// The row as the dialog renders it, while a published post is being edited.
    pub(super) fn version_view(&self, cx: &mut Context<Self>) -> Option<VersionView> {
        let base = self.version.base.as_ref()?;
        let suggested = base.suggested(&self.first_text(cx));
        let chosen = self.version.chosen.unwrap_or(suggested);
        let entity = cx.entity();
        Some(VersionView {
            from: base.version,
            next: base.next(chosen),
            suggested,
            chosen,
            branch: base.revision.is_some(),
            note: self.version.note.clone(),
            on_pick: Rc::new(move |bump, _, cx| entity.update(cx, |this, cx| this.pick_bump(bump, cx))),
        })
    }

    /// The edit to save: the first draft's text with the picked bump, the note and the base.
    pub(super) fn version_edit(&self, cx: &App) -> Option<PostEdit> {
        let base = self.version.base.as_ref()?;
        let note = self.version.note.read(cx).value().trim().to_string();
        Some(PostEdit { content: self.first_text(cx), bump: self.version.chosen, note, base: base.revision.clone(), look: None })
    }
}

impl ComposerModalPanel {
    /// Fills the link field and the image tray with the attachments of the version edited
    /// (V11), each image with its description.
    fn prefill_attachments(&mut self, attachments: Attachments, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(link) = attachments.link_url.clone() {
            self.link_input.update(cx, |state, cx| state.set_value(link, window, cx));
            self.is_link_input_open = true;
        }
        let images = if attachments.images.is_empty() { attachments.image_path.map(ImageAttachment::at).into_iter().collect() } else { attachments.images };
        for attachment in images {
            let (placeholder, text) = (t!("composer.media_alt_placeholder").to_string(), attachment.alt.clone());
            let alt = cx.new(|cx| InputState::new(window, cx).placeholder(placeholder));
            alt.update(cx, |state, cx| state.set_value(text, window, cx));
            self.images.push(super::ComposerImage { attachment, alt });
        }
    }
}
