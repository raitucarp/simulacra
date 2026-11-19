//! The session log: totals (sessions, time, how often the intention was met), focus minutes
//! per day over the last two weeks, and the latest sessions with their reflection and link.

use super::super::events::JournalEvent;
use super::super::panel::JournalPanel;
use super::super::text::{day_label, minutes_label, tr};
use super::common::{block_title, caption, control_row, count_tile, ghost_button, surface};
use crate::domain::journal::focus::stats::{minutes_by_day, newest_sessions, totals};
use crate::domain::journal::focus::{FocusLink, FocusSession};
use crate::i18n::COUNT_SEPARATOR;
use crate::ui::features::insights::chart::columns::{Column, ColumnChart};
use crate::ui::icons::LucideIcon;
use gpui_kit::*;
use rust_i18n::t;

const CHART_ID: &str = "journal-focus-days";
const LINK_ID: &str = "journal-focus-open-link";
const CHART_DAYS: i64 = 14;
const LOGGED_SESSIONS: usize = 12;
const PERCENT: f32 = 100.0;

impl JournalPanel {
    pub(super) fn render_focus_log(&self, cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        let sessions = &self.data.book.sessions;
        let sum = totals(sessions);
        let done = sum.done_share.map(|s| t!("journal.percent", value = (s * PERCENT).round() as u32).to_string()).unwrap_or_else(|| tr("journal.none_yet"));
        let tiles = div()
            .flex()
            .flex_wrap()
            .gap(theme.control_gap)
            .child(count_tile(sum.sessions.to_string(), tr("journal.focus.sessions"), theme.accent_brand, theme))
            .child(count_tile(minutes_label(sum.minutes), tr("journal.focus.total"), theme.accent_success, theme))
            .child(count_tile(done, tr("journal.focus.done_share"), theme.accent_dialectic, theme));
        let rows: Vec<Div> = newest_sessions(sessions).into_iter().take(LOGGED_SESSIONS).enumerate().map(|(i, s)| self.session_row(i, s, cx)).collect();
        surface(theme)
            .child(block_title(LucideIcon::ChartColumn, theme.accent_success, tr("journal.focus.log"), theme))
            .child(tiles)
            .child(self.minutes_chart())
            .children(rows)
    }

    fn minutes_chart(&self) -> ColumnChart {
        let theme = &self.theme;
        let by_day = minutes_by_day(&self.data.book.sessions);
        let days: Vec<_> = (0..CHART_DAYS).rev().map(|back| self.data.today - crate::time::days(back)).collect();
        let columns = days.iter().map(|day| {
            let minutes = by_day.get(day).copied().unwrap_or(0);
            Column { segments: vec![(f64::from(minutes), theme.accent_success)], tip: format!("{}{COUNT_SEPARATOR}{}", day_label(*day), minutes_label(minutes)) }
        });
        let labels = days.iter().map(|day| day.strftime("%d").to_string()).collect();
        ColumnChart { id: CHART_ID.into(), columns: columns.collect(), labels, height: theme.journal_chart_height, on_click: None, theme: theme.clone() }
    }

    fn session_row(&self, index: usize, session: &FocusSession, cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        let outcome = session.outcome.map(|o| tr(&format!("journal.outcome.{}", o.key()))).unwrap_or_else(|| tr("journal.outcome.unreflected"));
        let facts = [day_label(session.day), t!("journal.focus.minutes_of", minutes = session.focused_minutes, planned = session.planned_minutes).to_string(), outcome, t!("journal.focus.distraction", count = session.distractions).to_string()];
        let title = if session.intention.is_empty() { tr("journal.focus.no_intention") } else { session.intention.clone() };
        let link = session.link.clone().filter(|l| !matches!(l, FocusLink::Goal(_))).map(|link| {
            ghost_button((LINK_ID, index), LucideIcon::ExternalLink, tr("journal.focus.open_link")).on_click(cx.listener(move |_, _, _, cx| cx.emit(JournalEvent::OpenLink(link.clone()))))
        });
        let goal = match &session.link {
            Some(FocusLink::Goal(goal)) => Some(caption(t!("journal.focus.goal_value", goal = goal).to_string(), theme)),
            _ => None,
        };
        let reflection = (!session.reflection.is_empty()).then(|| div().text_size(theme.font_size_sm).text_color(theme.text_secondary).child(session.reflection.clone()));
        div()
            .w_full()
            .pt(theme.stack_gap)
            .border_t(theme.border_width_hairline)
            .border_color(theme.border_subtle)
            .flex()
            .flex_col()
            .gap(theme.spacing_xs)
            .child(control_row(theme).child(div().flex_1().min_w_0().text_size(theme.font_size_sm).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_primary).child(title)).children(link))
            .child(caption(facts.join(COUNT_SEPARATOR), theme))
            .children(goal)
            .children(reflection)
    }
}
