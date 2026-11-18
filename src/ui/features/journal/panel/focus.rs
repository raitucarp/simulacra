//! The focus run: choosing a length and a link, the timer and its one-second ticker, pause,
//! resume and stop, distractions sent to quick capture, focus mode, and the reflection that
//! completes the stored session.

use super::{JournalFields, JournalPanel};
use crate::domain::journal::focus::timer::whole_minutes;
use crate::domain::journal::focus::{custom_minutes, FocusLink, FocusSession, FocusTimer, Outcome, PRESET_MINUTES};
use crate::domain::journal::{journal_day, RecordId};
use crate::ui::features::journal::events::JournalEvent;
use chrono::{DateTime, Local, Utc};
use gpui_kit::{Context, Window};
use std::time::Duration;

/// How often a running timer is read.
const TICK: Duration = Duration::from_millis(1_000);

/// The focus tab's state between sessions and while one runs.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct FocusRun {
    /// The chosen preset (`None`: the custom field).
    pub preset: Option<u32>,
    pub link: Option<FocusLink>,
    pub timer: Option<FocusTimer>,
    pub started_at: Option<DateTime<Utc>>,
    pub intention: String,
    pub distractions: u32,
    /// The finished session awaiting its reflection (already stored).
    pub finished: Option<FocusSession>,
    pub focus_mode: bool,
}

/// Wall-clock milliseconds now.
pub fn now_ms() -> i64 {
    Utc::now().timestamp_millis()
}

impl JournalPanel {
    pub(in crate::ui::features::journal) fn chosen_minutes(&self, cx: &gpui_kit::App) -> Option<u32> {
        match self.focus_run.preset {
            Some(minutes) => Some(minutes),
            None => custom_minutes(&JournalFields::text(&self.fields.minutes, cx)),
        }
    }

    pub(in crate::ui::features::journal) fn choose_preset(&mut self, preset: Option<u32>, cx: &mut Context<Self>) {
        self.focus_run.preset = preset;
        cx.notify();
    }

    pub(in crate::ui::features::journal) fn choose_link(&mut self, link: Option<FocusLink>, cx: &mut Context<Self>) {
        self.focus_run.link = link;
        cx.notify();
    }

    /// Starts a session with the intention, length and link chosen.
    pub(in crate::ui::features::journal) fn start_focus(&mut self, cx: &mut Context<Self>) {
        let Some(minutes) = self.chosen_minutes(cx) else { return };
        let goal = JournalFields::text(&self.fields.goal, cx);
        let link = match &self.focus_run.link {
            Some(FocusLink::Goal(_)) if goal.is_empty() => None,
            Some(FocusLink::Goal(_)) => Some(FocusLink::Goal(goal)),
            other => other.clone(),
        };
        let intention = JournalFields::text(&self.fields.intention, cx);
        self.focus_run = FocusRun { timer: Some(FocusTimer::start(minutes, now_ms())), started_at: Some(Utc::now()), intention, link, finished: None, distractions: 0, ..self.focus_run.clone() };
        self.start_ticker(cx);
        cx.notify();
    }

    fn start_ticker(&mut self, cx: &mut Context<Self>) {
        self.ticker = Some(cx.spawn(async move |this, cx| loop {
            cx.background_executor().timer(TICK).await;
            if this.update(cx, |panel, cx| panel.tick(cx)).is_err() {
                break;
            }
        }));
    }

    fn tick(&mut self, cx: &mut Context<Self>) {
        let Some(timer) = self.focus_run.timer.map(|t| t.tick(now_ms())) else { return };
        self.focus_run.timer = Some(timer);
        if timer.is_finished() {
            self.finish_focus(cx);
        }
        cx.notify();
    }

    pub(in crate::ui::features::journal) fn pause_focus(&mut self, cx: &mut Context<Self>) {
        self.focus_run.timer = self.focus_run.timer.map(|t| if t.is_running() { t.pause(now_ms()) } else { t.resume(now_ms()) });
        cx.notify();
    }

    pub(in crate::ui::features::journal) fn stop_focus(&mut self, cx: &mut Context<Self>) {
        self.focus_run.timer = self.focus_run.timer.map(|t| t.stop(now_ms()));
        self.finish_focus(cx);
        cx.notify();
    }

    /// Stores the finished session and asks for the reflection.
    fn finish_focus(&mut self, cx: &mut Context<Self>) {
        let (Some(timer), Some(started_at)) = (self.focus_run.timer.take(), self.focus_run.started_at.take()) else { return };
        self.ticker = None;
        let run = &self.focus_run;
        let session = FocusSession {
            id: RecordId::random(), day: journal_day(&Local::now()), started_at, ended_at: Utc::now(), intention: run.intention.clone(),
            planned_minutes: whole_minutes(timer.planned_ms), focused_minutes: whole_minutes(timer.elapsed(now_ms())), link: run.link.clone(),
            distractions: run.distractions, outcome: None, reflection: String::new(),
        };
        cx.emit(JournalEvent::SaveSession(session.clone()));
        self.focus_run.finished = Some(session);
    }

    /// Completes the finished session with its outcome and the one-line reflection.
    pub(in crate::ui::features::journal) fn reflect(&mut self, outcome: Option<Outcome>, window: &mut Window, cx: &mut Context<Self>) {
        let Some(session) = self.focus_run.finished.take() else { return };
        let reflection = JournalFields::text(&self.fields.reflection, cx);
        cx.emit(JournalEvent::SaveSession(FocusSession { outcome, reflection, ..session }));
        for field in [&self.fields.reflection, &self.fields.intention] {
            field.update(cx, |state, cx| state.set_value("", window, cx));
        }
        self.set_focus_mode(false, cx);
    }

    pub(in crate::ui::features::journal) fn capture_distraction(&mut self, cx: &mut Context<Self>) {
        self.focus_run.distractions += 1;
        cx.emit(JournalEvent::CaptureDistraction);
        cx.notify();
    }

    pub fn set_focus_mode(&mut self, on: bool, cx: &mut Context<Self>) {
        self.focus_run.focus_mode = on;
        cx.emit(JournalEvent::FocusMode(on));
        cx.notify();
    }

    /// The presets offered.
    pub(in crate::ui::features::journal) fn presets() -> [u32; 3] {
        PRESET_MINUTES
    }
}
