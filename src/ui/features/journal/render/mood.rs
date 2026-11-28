//! The Mood tab: the check-in, then (once there are check-ins) the charts, the careful
//! associations with activity, and the recent check-ins.

use super::super::panel::JournalPanel;
use super::super::text::{region_label, tr};
use super::common::{block_title, caption, control_row, ghost_button, surface};
use super::confirm::DeleteKind;
use crate::domain::journal::mood::summary::newest_checkins;
use crate::domain::journal::MoodCheckin;
use crate::i18n::COUNT_SEPARATOR;
use crate::ui::icons::LucideIcon;
use crate::ui::responsive::LayoutClass;
use gpui_kit::*;
use rust_i18n::t;

const DELETE_ID: &str = "journal-checkin-delete";
/// Check-ins listed under the charts.
const RECENT_CHECKINS: usize = 12;

impl JournalPanel {
    pub(super) fn render_mood(&self, class: LayoutClass, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &self.theme;
        let page = div().w_full().flex().flex_col().gap(theme.section_gap).child(self.render_checkin(class, cx));
        if self.data.book.checkins.is_empty() {
            return page.child(self.mood_empty());
        }
        page.child(self.render_mood_charts(class)).child(self.render_associations()).child(self.recent_checkins(cx))
    }

    fn recent_checkins(&self, cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        let rows = newest_checkins(&self.data.book.checkins).into_iter().take(RECENT_CHECKINS).enumerate().map(|(i, c)| self.checkin_row(i, c, cx)).collect::<Vec<_>>();
        surface(theme).child(block_title(LucideIcon::List, theme.accent_brand, tr("journal.mood.recent"), theme)).children(rows)
    }

    fn checkin_row(&self, index: usize, checkin: &MoodCheckin, cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        let mut facts = vec![format!("{} {}", crate::ui::format::date(checkin.at.date()), crate::ui::format::time_of_day(&checkin.at)), t!("journal.mood.energy_value", value = checkin.energy).to_string()];
        facts.extend(checkin.sleep_hours.map(|h| t!("journal.mood.sleep_value", hours = format!("{h:.1}")).to_string()));
        facts.extend(checkin.tags.iter().map(|tag| format!("#{tag}")));
        let id = checkin.id.clone();
        let delete = ghost_button((DELETE_ID, index), LucideIcon::Trash, tr("journal.entry.delete")).on_click(self.confirm_delete(DeleteKind::Checkin, id, cx));
        let note = (!checkin.note.is_empty()).then(|| div().text_size(theme.font_size_sm).text_color(theme.text_primary).child(checkin.note.clone()));
        div()
            .w_full()
            .pt(theme.stack_gap)
            .border_t(theme.border_width_hairline)
            .border_color(theme.border_subtle)
            .flex()
            .flex_col()
            .gap(theme.spacing_xs)
            .child(control_row(theme).child(div().flex_1().text_size(theme.font_size_sm).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_primary).child(region_label(checkin.affect.region()))).child(delete))
            .child(caption(facts.join(COUNT_SEPARATOR), theme))
            .children(note)
            .children(checkin.wants_support.then(|| self.support_text()))
    }
}
