//! Capsules: writing one (with images copied into the media folder), sealing it, opening it
//! on its day, replying, turning it into a post and deleting it.

use super::{JournalFields, JournalPanel};
use crate::domain::journal::capsule::{parse_until, seal, SealProblem};
use crate::domain::journal::records::find;
use crate::domain::journal::RecordId;
use crate::domain::media::ImageAttachment;
use crate::media::store_attachment_file;
use crate::ui::components::pick_image_paths;
use crate::ui::features::journal::events::JournalEvent;
use crate::ui::features::journal::text::tr;
use chrono::Utc;
use gpui_kit::{Context, Window};

/// The capsule being written, and the one being read.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct CapsuleForm {
    pub writing: bool,
    pub attachments: Vec<ImageAttachment>,
    pub problem: Option<SealProblem>,
    /// The opened capsule shown with its reflection.
    pub reading: Option<RecordId>,
}

impl JournalPanel {
    pub fn write_capsule(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.tab = crate::ui::features::journal::tab::JournalTab::Capsules;
        self.capsule = CapsuleForm { writing: true, ..CapsuleForm::default() };
        self.fields.capsule_body.update(cx, |state, cx| state.focus(window, cx));
        cx.notify();
    }

    pub(in crate::ui::features::journal) fn cancel_capsule(&mut self, cx: &mut Context<Self>) {
        self.capsule = CapsuleForm::default();
        cx.notify();
    }

    /// Picks images and copies them into the media folder, off the UI thread.
    pub(in crate::ui::features::journal) fn attach_capsule_images(&mut self, cx: &mut Context<Self>) {
        let media_dir = self.data.media_dir.clone();
        let title = tr("journal.capsule.pick_images");
        cx.spawn(async move |this, cx| {
            let paths = pick_image_paths(&title).await;
            let stored = cx.background_executor().spawn(async move { paths.iter().filter_map(|p| store_attachment_file(p, &media_dir).ok()).collect::<Vec<_>>() }).await;
            let _ = this.update(cx, |panel, cx| {
                panel.capsule.attachments.extend(stored);
                cx.notify();
            });
        })
        .detach();
    }

    /// Seals the note until the typed day or number of days.
    pub(in crate::ui::features::journal) fn seal_capsule(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let body = JournalFields::area_text(&self.fields.capsule_body, cx);
        let until = parse_until(&JournalFields::text(&self.fields.capsule_until, cx));
        let sealed = until.ok_or(SealProblem::OutOfRange).and_then(|until| seal(RecordId::random(), &body, self.capsule.attachments.clone(), until, Utc::now(), self.data.today));
        match sealed {
            Ok(capsule) => {
                cx.emit(JournalEvent::SaveCapsule(capsule));
                self.capsule = CapsuleForm::default();
                self.fields.capsule_body.update(cx, |state, cx| state.set_value("", window, cx));
                self.fields.capsule_until.update(cx, |state, cx| state.set_value("", window, cx));
            }
            Err(problem) => self.capsule.problem = Some(problem),
        }
        cx.notify();
    }

    /// Opens a capsule whose day has come, and shows it.
    pub(in crate::ui::features::journal) fn open_capsule(&mut self, id: &RecordId, window: &mut Window, cx: &mut Context<Self>) {
        let Some(capsule) = find(&self.data.book.capsules, id) else { return };
        let Some(opened) = capsule.opened(Utc::now(), self.data.today) else { return };
        let reply = opened.reply.clone().unwrap_or_default();
        self.fields.reply.update(cx, |state, cx| state.set_value(reply, window, cx));
        if opened != *capsule {
            cx.emit(JournalEvent::SaveCapsule(opened));
        }
        self.capsule.reading = Some(id.clone());
        cx.notify();
    }

    pub(in crate::ui::features::journal) fn close_capsule(&mut self, cx: &mut Context<Self>) {
        self.capsule.reading = None;
        cx.notify();
    }

    pub(in crate::ui::features::journal) fn save_reply(&mut self, id: &RecordId, cx: &mut Context<Self>) {
        let reply = JournalFields::area_text(&self.fields.reply, cx);
        if let Some(replied) = find(&self.data.book.capsules, id).and_then(|c| c.replied(&reply)) {
            cx.emit(JournalEvent::SaveCapsule(replied));
        }
    }

    pub(in crate::ui::features::journal) fn capsule_into_post(&mut self, id: &RecordId, cx: &mut Context<Self>) {
        if let Some(body) = find(&self.data.book.capsules, id).and_then(|c| c.body()) {
            cx.emit(JournalEvent::TurnIntoPost(body.to_string()));
        }
    }

    pub(in crate::ui::features::journal) fn delete_capsule(&mut self, id: RecordId, cx: &mut Context<Self>) {
        if self.capsule.reading.as_ref() == Some(&id) {
            self.capsule.reading = None;
        }
        cx.emit(JournalEvent::DeleteCapsule(id));
        cx.notify();
    }
}
