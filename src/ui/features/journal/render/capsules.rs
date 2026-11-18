//! The Capsules tab: the form while writing one (note, images, opening day), the opened
//! capsule being read, then every capsule: ready ones first, sealed ones with only their
//! day and how long is left (their contents stay hidden), opened ones last.

use super::super::panel::{JournalFields, JournalPanel};
use super::super::text::{capsule_state_label, day_label, tr};
use super::common::{area_field, block_title, caption, control_row, ghost_button, line_field, primary_button, surface};
use crate::domain::journal::capsule::notify::in_list_order;
use crate::domain::journal::capsule::{CapsuleState, SealProblem, TimeCapsule};
use crate::i18n::t_count;
use crate::ui::icons::LucideIcon;
use crate::ui::illustration::{EmptyKey, Illustration};
use gpui_kit::*;

const WRITE_ID: &str = "journal-capsule-write";
const ATTACH_ID: &str = "journal-capsule-attach";
const SEAL_ID: &str = "journal-capsule-seal";
const CANCEL_ID: &str = "journal-capsule-cancel";
const OPEN_ID: &str = "journal-capsule-open";
const DELETE_ID: &str = "journal-capsule-delete";

impl JournalPanel {
    pub(super) fn render_capsules(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &self.theme;
        let page = div().w_full().flex().flex_col().gap(theme.section_gap);
        let top = if self.capsule.writing { Some(self.capsule_form(cx)) } else { self.capsule.reading.clone().map(|id| self.render_opened(&id, cx)) };
        let write = (!self.capsule.writing).then(|| control_row(theme).child(primary_button(WRITE_ID, LucideIcon::Mail, tr("journal.capsule.write")).on_click(cx.listener(|this, _, window, cx| this.write_capsule(window, cx)))));
        if self.data.book.capsules.is_empty() && !self.capsule.writing {
            return page.child(self.capsules_empty(cx));
        }
        let cards: Vec<Div> = in_list_order(&self.data.book.capsules, self.data.today).into_iter().filter(|c| self.capsule.reading.as_ref() != Some(&c.id)).enumerate().map(|(i, c)| self.capsule_card(i, c, cx)).collect();
        page.children(write).children(top).children(cards)
    }

    fn capsule_form(&self, cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        let problem = self.capsule.problem.map(|p| match p {
            SealProblem::Empty => tr("journal.capsule.problem_empty"),
            SealProblem::OutOfRange => tr("journal.capsule.problem_day"),
        });
        let attached = (!self.capsule.attachments.is_empty()).then(|| caption(t_count("journal.capsule.images", self.capsule.attachments.len()), theme));
        let attach = ghost_button(ATTACH_ID, LucideIcon::Paperclip, tr("journal.capsule.attach")).on_click(cx.listener(|this, _, _, cx| this.attach_capsule_images(cx)));
        let seal = primary_button(SEAL_ID, LucideIcon::Lock, tr("journal.capsule.seal")).on_click(cx.listener(|this, _, window, cx| this.seal_capsule(window, cx)));
        let cancel = ghost_button(CANCEL_ID, LucideIcon::X, tr("journal.capsule.cancel")).on_click(cx.listener(|this, _, _, cx| this.cancel_capsule(cx)));
        let until = JournalFields::text(&self.fields.capsule_until, cx);
        let preview = crate::domain::journal::capsule::parse_until(&until).and_then(|u| crate::domain::journal::capsule::opening_day(u, self.data.today));
        let opens = preview.map(|day| caption(rust_i18n::t!("journal.capsule.opens_on", date = day_label(day)).to_string(), theme));
        surface(theme)
            .child(block_title(LucideIcon::Mail, theme.accent_dialectic, tr("journal.capsule.form_title"), theme))
            .child(caption(tr("journal.capsule.form_hint"), theme))
            .child(area_field(&self.fields.capsule_body, theme))
            .child(control_row(theme).child(div().w(theme.journal_row_label_width).child(line_field(&self.fields.capsule_until, theme))).child(caption(tr("journal.capsule.until_label"), theme)))
            .children(opens)
            .children(attached)
            .children(problem.map(|text| caption(text, theme).text_color(theme.accent_danger)))
            .child(control_row(theme).child(seal).child(attach).child(cancel))
    }

    fn capsule_card(&self, index: usize, capsule: &TimeCapsule, cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        let state = capsule.state(self.data.today);
        let (icon, colour) = match state {
            CapsuleState::Sealed { .. } => (LucideIcon::Lock, theme.text_muted),
            CapsuleState::Ready => (LucideIcon::LockOpen, theme.accent_dialectic),
            CapsuleState::Opened => (LucideIcon::MailOpen, theme.accent_success),
        };
        let title = rust_i18n::t!("journal.capsule.from", date = day_label(capsule.written_on)).to_string();
        let mut facts = vec![capsule_state_label(state, capsule.opens_on)];
        facts.extend((capsule.attachment_count() > 0).then(|| t_count("journal.capsule.images", capsule.attachment_count())));
        let id = capsule.id.clone();
        let open = (!matches!(state, CapsuleState::Sealed { .. })).then(|| {
            let label = if state == CapsuleState::Opened { tr("journal.capsule.read") } else { tr("journal.capsule.open") };
            primary_button((OPEN_ID, index), LucideIcon::MailOpen, label).on_click(cx.listener(move |this, _, window, cx| this.open_capsule(&id, window, cx)))
        });
        let delete_id = capsule.id.clone();
        let delete = ghost_button((DELETE_ID, index), LucideIcon::Trash, tr("journal.entry.delete")).on_click(cx.listener(move |this, _, _, cx| this.delete_capsule(delete_id.clone(), cx)));
        let art = matches!(state, CapsuleState::Sealed { .. }).then(|| Illustration::empty(EmptyKey::CapsuleSealed).width(theme.art_compact_width));
        let border = if state == CapsuleState::Ready { theme.accent_dialectic_border } else { theme.border_subtle };
        surface(theme)
            .border_color(border)
            .child(control_row(theme).children(art).child(div().flex_1().min_w_0().flex().flex_col().gap(theme.spacing_xs).child(block_title(icon, colour, title, theme)).child(caption(facts.join(crate::i18n::COUNT_SEPARATOR), theme))).children(open).child(delete))
    }
}
