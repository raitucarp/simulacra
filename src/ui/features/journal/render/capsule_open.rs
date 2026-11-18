//! An opened capsule: when it was written and how long it waited, the note and its images,
//! a reflection prompt comparing then and now, and the reply (stored with the capsule), with
//! "Turn into a post" for the note.

use super::super::panel::JournalPanel;
use super::super::text::{day_label, tr};
use super::common::{area_field, block_title, caption, control_row, ghost_button, primary_button, prose, surface};
use crate::domain::journal::records::find;
use crate::domain::journal::RecordId;
use crate::i18n::t_count;
use crate::ui::icons::LucideIcon;
use gpui_kit::*;
use std::path::PathBuf;

const REPLY_ID: &str = "journal-capsule-reply";
const POST_ID: &str = "journal-capsule-post";
const CLOSE_ID: &str = "journal-capsule-close";
const THUMB_ID: &str = "journal-capsule-thumb";

impl JournalPanel {
    pub(super) fn render_opened(&self, id: &RecordId, cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        let Some(capsule) = find(&self.data.book.capsules, id) else { return div() };
        let (Some(body), Some(images)) = (capsule.body(), capsule.attachments()) else { return div() };
        let waited = t_count("journal.capsule.waited", capsule.days_sealed() as usize);
        let written = rust_i18n::t!("journal.capsule.from", date = day_label(capsule.written_on)).to_string();
        let thumbs = images.iter().enumerate().map(|(i, image)| {
            div().id((THUMB_ID, i)).size(theme.journal_thumb_size).rounded(theme.radius_md).overflow_hidden().bg(theme.bg_elevated).child(img(PathBuf::from(&image.path)).size_full().object_fit(ObjectFit::Cover))
        });
        let (reply_id, post_id) = (id.clone(), id.clone());
        let save = primary_button(REPLY_ID, LucideIcon::MessageSquareReply, tr("journal.capsule.save_reply")).on_click(cx.listener(move |this, _, _, cx| this.save_reply(&reply_id, cx)));
        let post = ghost_button(POST_ID, LucideIcon::Send, tr("journal.editor.into_post")).on_click(cx.listener(move |this, _, _, cx| this.capsule_into_post(&post_id, cx)));
        let close = ghost_button(CLOSE_ID, LucideIcon::X, tr("journal.editor.close")).on_click(cx.listener(|this, _, _, cx| this.close_capsule(cx)));
        let callout = div().pl(theme.card_padding).py(theme.spacing_xs).border_l(theme.border_width_threadline).border_color(theme.accent_dialectic).bg(theme.accent_dialectic_subtle).rounded_r(theme.radius_md).child(prose(body.to_string(), theme));
        surface(theme)
            .border_color(theme.accent_dialectic_border)
            .child(block_title(LucideIcon::MailOpen, theme.accent_dialectic, written, theme))
            .child(caption(waited, theme))
            .child(callout)
            .child(div().flex().flex_wrap().gap(theme.control_gap).children(thumbs))
            .child(div().text_size(theme.font_size_sm).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_primary).child(tr("journal.capsule.then_and_now")))
            .child(caption(tr("journal.capsule.then_and_now_hint"), theme))
            .child(area_field(&self.fields.reply, theme))
            .child(control_row(theme).child(save).child(post).child(close))
    }
}
