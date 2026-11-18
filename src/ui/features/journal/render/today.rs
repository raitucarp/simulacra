//! The Today tab: the day's prompt (write about it, another one, or start from a template),
//! today's entries, a quick check-in, the focus timer, and capsules whose day has come.

use super::super::panel::JournalPanel;
use super::super::tab::JournalTab;
use super::super::text::{day_label, tr};
use super::common::{block_title, caption, control_row, ghost_button, primary_button, surface};
use crate::domain::journal::capsule::CapsuleState;
use crate::domain::journal::entry::entries_on;
use crate::domain::journal::templates::Template;
use crate::ui::icons::LucideIcon;
use crate::ui::responsive::LayoutClass;
use gpui_kit::*;
use rust_i18n::t;

const WRITE_ID: &str = "journal-today-write";
const ANOTHER_ID: &str = "journal-today-another";
const TEMPLATE_ID: &str = "journal-today-template";
const CONTINUE_ID: &str = "journal-today-continue";
const FOCUS_ID: &str = "journal-today-focus";
const OPEN_CAPSULE_ID: &str = "journal-today-open-capsule";

impl JournalPanel {
    pub(super) fn render_today(&self, class: LayoutClass, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &self.theme;
        div()
            .w_full()
            .flex()
            .flex_col()
            .gap(theme.section_gap)
            .children(self.ready_capsules(cx))
            .child(self.prompt_card(cx))
            .children(self.todays_entries(cx))
            .child(self.render_checkin(class, cx))
            .child(self.focus_teaser(cx))
    }

    /// The prompt as a callout, with its buttons.
    pub(super) fn prompt_card(&self, cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        let prompt = self.current_prompt();
        let callout = div()
            .pl(theme.card_padding)
            .py(theme.spacing_xs)
            .border_l(theme.border_width_threadline)
            .border_color(theme.accent_dialectic)
            .bg(theme.accent_dialectic_subtle)
            .rounded_r(theme.radius_md)
            .flex()
            .flex_col()
            .gap(theme.spacing_3xs)
            .child(div().text_size(theme.font_size_2xs).font_weight(FontWeight::SEMIBOLD).text_color(theme.accent_dialectic).child(tr(&prompt.kind_key())))
            .child(div().font_family(theme.font_family_body.clone()).text_size(theme.font_size_md).text_color(theme.text_primary).child(tr(&prompt.text_key())));
        let write = primary_button(WRITE_ID, LucideIcon::Feather, tr("journal.today.write")).on_click(cx.listener(|this, _, window, cx| this.new_entry(None, true, window, cx)));
        let another = ghost_button(ANOTHER_ID, LucideIcon::Shuffle, tr("journal.today.another")).on_click(cx.listener(|this, _, _, cx| this.another_prompt(cx)));
        let templates = Template::ALL.into_iter().map(|template| {
            let icon = match template { Template::Morning => LucideIcon::Sun, Template::Evening => LucideIcon::Moon, Template::Weekly => LucideIcon::Calendar };
            ghost_button((TEMPLATE_ID, template as usize), icon, tr(&format!("journal.template.{}", template.key())))
                .on_click(cx.listener(move |this, _, window, cx| this.new_entry(Some(template), false, window, cx)))
        });
        surface(theme)
            .child(block_title(LucideIcon::Lightbulb, theme.accent_dialectic, tr("journal.today.prompt_title"), theme))
            .child(callout)
            .child(control_row(theme).child(write).child(another).children(templates))
    }

    fn todays_entries(&self, cx: &mut Context<Self>) -> Option<Div> {
        let theme = &self.theme;
        let entries = entries_on(&self.data.book.entries, self.data.today);
        if entries.is_empty() {
            return None;
        }
        let rows = entries.into_iter().enumerate().map(|(i, entry)| {
            let id = entry.id.clone();
            let open = ghost_button((CONTINUE_ID, i), LucideIcon::NotebookPen, tr("journal.today.continue")).on_click(cx.listener(move |this, _, window, cx| this.edit_entry(&id, window, cx)));
            let words = t!("journal.entry.words", count = entry.word_count()).to_string();
            control_row(theme).child(div().flex_1().min_w_0().text_size(theme.font_size_sm).text_color(theme.text_primary).child(entry.title())).child(caption(words, theme)).child(open)
        });
        Some(surface(theme).child(block_title(LucideIcon::NotebookPen, theme.accent_brand, tr("journal.today.written"), theme)).children(rows))
    }

    fn focus_teaser(&self, cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        let label = if self.focus_run.timer.is_some() { tr("journal.today.focus_running") } else { tr("journal.today.focus_start") };
        let go = ghost_button(FOCUS_ID, LucideIcon::Timer, label).on_click(cx.listener(|this, _, _, cx| this.show(JournalTab::Focus, cx)));
        surface(theme).child(block_title(LucideIcon::Target, theme.accent_success, tr("journal.today.focus_title"), theme)).child(caption(tr("journal.today.focus_hint"), theme)).child(control_row(theme).child(go))
    }

    /// One line per capsule whose day has come, with an Open button.
    fn ready_capsules(&self, cx: &mut Context<Self>) -> Vec<Div> {
        let theme = &self.theme;
        let ready = self.data.book.capsules.iter().filter(|c| c.state(self.data.today) == CapsuleState::Ready);
        ready
            .enumerate()
            .map(|(i, capsule)| {
                let id = capsule.id.clone();
                let open = primary_button((OPEN_CAPSULE_ID, i), LucideIcon::MailOpen, tr("journal.capsule.open")).on_click(cx.listener(move |this, _, window, cx| {
                    this.show(JournalTab::Capsules, cx);
                    this.open_capsule(&id, window, cx);
                }));
                let text = t!("journal.today.capsule_ready", date = day_label(capsule.written_on)).to_string();
                surface(theme).border_color(theme.accent_dialectic_border).child(control_row(theme).child(block_title(LucideIcon::Mail, theme.accent_dialectic, text, theme)).child(open))
            })
            .collect()
    }
}
