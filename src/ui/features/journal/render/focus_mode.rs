//! The running timer (intention, clock, progress, pause or resume, stop, a distraction to the
//! Inbox, focus mode), the reflection after a session, and focus mode itself: the timer alone
//! in the middle of the window, fading in unless motion is reduced.

use super::super::panel::{focus_now_ms, JournalPanel};
use super::super::text::tr;
use super::common::{caption, control_row, entrance, ghost_button, line_field, primary_button, progress_bar, surface};
use crate::domain::journal::focus::timer::clock_text;
use crate::domain::journal::focus::{Outcome, Phase};
use crate::ui::icons::LucideIcon;
use crate::ui::responsive::LayoutClass;
use gpui_kit::*;
use rust_i18n::t;

const PAUSE_ID: &str = "journal-focus-pause";
const STOP_ID: &str = "journal-focus-stop";
const DISTRACTION_ID: &str = "journal-focus-distraction";
const MODE_ID: &str = "journal-focus-mode";
const OUTCOME_ID: &str = "journal-focus-outcome";
const SKIP_ID: &str = "journal-focus-skip";
const FOCUS_MODE_ID: &str = "journal-focus-mode-view";

impl JournalPanel {
    pub(super) fn render_running(&self, class: LayoutClass, cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        let Some(timer) = self.focus_run.timer else { return div() };
        let now = focus_now_ms();
        let large = self.focus_run.focus_mode && !class.is_compact();
        let size = if large { theme.journal_timer_text_large } else { theme.journal_timer_text };
        let state = match timer.phase {
            Phase::Paused { suspended: true } => tr("journal.focus.slept"),
            Phase::Paused { .. } => tr("journal.focus.paused"),
            _ => t!("journal.focus.of_planned", minutes = timer.planned_ms / crate::domain::journal::focus::timer::MS_PER_MINUTE).to_string(),
        };
        let intention = if self.focus_run.intention.is_empty() { tr("journal.focus.no_intention") } else { self.focus_run.intention.clone() };
        let clock = div().font_family(theme.font_family_mono.clone()).text_size(size).line_height(size).font_weight(FontWeight::BOLD).text_color(theme.text_primary).child(clock_text(timer.remaining(now)));
        surface(theme)
            .items_center()
            .child(div().text_size(theme.font_size_md).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_primary).text_center().child(intention))
            .child(clock)
            .child(progress_bar(timer.progress(now), theme.accent_success, theme))
            .child(caption(state, theme))
            .child(self.running_buttons(timer.is_running(), cx))
    }

    fn running_buttons(&self, running: bool, cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        let (icon, label) = if running { (LucideIcon::Pause, tr("journal.focus.pause")) } else { (LucideIcon::Play, tr("journal.focus.resume")) };
        let pause = primary_button(PAUSE_ID, icon, label).on_click(cx.listener(|this, _, _, cx| this.pause_focus(cx)));
        let stop = ghost_button(STOP_ID, LucideIcon::CircleStop, tr("journal.focus.stop")).on_click(cx.listener(|this, _, _, cx| this.stop_focus(cx)));
        let captured = t!("journal.focus.distraction", count = self.focus_run.distractions).to_string();
        let distraction = ghost_button(DISTRACTION_ID, LucideIcon::Inbox, captured).on_click(cx.listener(|this, _, _, cx| this.capture_distraction(cx)));
        let on = self.focus_run.focus_mode;
        let (mode_icon, mode_label) = if on { (LucideIcon::Minimize2, tr("journal.focus.leave_mode")) } else { (LucideIcon::Maximize2, tr("journal.focus.enter_mode")) };
        let mode = ghost_button(MODE_ID, mode_icon, mode_label).on_click(cx.listener(move |this, _, _, cx| this.set_focus_mode(!on, cx)));
        control_row(theme).justify_center().child(pause).child(stop).child(distraction).child(mode)
    }

    /// How did it go: done, partly or not, and one line.
    pub(super) fn render_reflection(&self, cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        let Some(session) = &self.focus_run.finished else { return div() };
        let summary = t!("journal.focus.ended", minutes = session.focused_minutes, planned = session.planned_minutes).to_string();
        let outcomes = Outcome::ALL.into_iter().enumerate().map(|(i, outcome)| {
            primary_button((OUTCOME_ID, i), LucideIcon::Check, tr(&format!("journal.outcome.{}", outcome.key()))).on_click(cx.listener(move |this, _, window, cx| this.reflect(Some(outcome), window, cx)))
        });
        let skip = ghost_button(SKIP_ID, LucideIcon::X, tr("journal.focus.skip")).on_click(cx.listener(|this, _, window, cx| this.reflect(None, window, cx)));
        surface(theme)
            .child(div().text_size(theme.font_size_md).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_primary).child(tr("journal.focus.how_did_it_go")))
            .child(caption(summary, theme))
            .child(line_field(&self.fields.reflection, theme))
            .child(control_row(theme).children(outcomes).child(skip))
    }

    /// Focus mode: nothing but the timer (or the reflection) in the middle of the window.
    pub(super) fn render_focus_mode(&self, class: LayoutClass, cx: &mut Context<Self>) -> AnyElement {
        let theme = &self.theme;
        let body = if self.focus_run.timer.is_some() { self.render_running(class, cx) } else { self.render_reflection(cx) };
        let column = div().w_full().max_w(theme.journal_focus_mode_width).px(theme.panel_padding_compact).child(body);
        let view = div().size_full().flex().items_center().justify_center().child(column);
        entrance(view, FOCUS_MODE_ID, self.data.reduced_motion, theme)
    }
}
