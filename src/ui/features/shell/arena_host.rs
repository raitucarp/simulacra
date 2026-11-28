//! Arena wiring of the shell (sb F1): the saved debates and experiments and every agent's
//! beliefs (loaded once from the vault), the run in progress, the Arena page and the
//! profile's Beliefs tab, and their data kept in step with the records, the network's
//! agents, today's AI budget and the open thread.

use super::view::AppView;
use crate::agent::post_actions::CancelToken;
use crate::domain::arena::debate::{Debate, Verdict};
use crate::domain::arena::lab::Experiment;
use crate::domain::beliefs::{BeliefBook, BeliefSettings, BeliefSource};
use crate::domain::ProfileId;
use crate::state::AppState;
use crate::ui::features::arena::{ArenaData, ArenaPanel, ArenaTab, RunState};
use crate::ui::features::beliefs::BeliefsPanel;
use crate::ui::features::roster::ProfileTab;
use gpui_kit::*;
use std::collections::HashMap;

/// What the views' data was last built from: records version, network, profiles, key,
/// today's usage, the open thread, the selected agent.
pub type Stamp = (u64, String, usize, bool, crate::domain::embedding::DailyUsage, Option<String>, Option<ProfileId>);

/// What the running request will do when it lands.
#[derive(Debug, Clone, PartialEq)]
pub enum RunStep {
    /// The next debate turn.
    Turn,
    /// The judge's verdict.
    Judge,
    /// The argument map, the verdict waiting for it.
    Map(Verdict),
    /// One experiment answer.
    Cell(ProfileId, usize),
}

/// The Arena request in flight (one at a time).
pub struct ArenaRun {
    pub ticket: u64,
    pub cancel: CancelToken,
    pub state: RunState,
    pub step: RunStep,
}

/// A belief update in flight.
pub struct PendingUpdate {
    pub agent: ProfileId,
    pub source: BeliefSource,
}

/// The records, the run and the two views.
pub struct ArenaHost {
    pub panel: Entity<ArenaPanel>,
    pub beliefs_panel: Entity<BeliefsPanel>,
    pub debates: Vec<Debate>,
    pub experiments: Vec<Experiment>,
    pub book: BeliefBook,
    pub settings: HashMap<ProfileId, BeliefSettings>,
    pub run: Option<ArenaRun>,
    /// Bumped for every request, so a stale reply is dropped.
    pub ticket: u64,
    /// Why the last request failed, and the tab whose run it was (shown only there).
    pub notice: Option<(ArenaTab, String)>,
    pub updates: HashMap<u64, PendingUpdate>,
    pub belief_notice: Option<String>,
    pub profile_tab: ProfileTab,
    /// Bumped on every change of the records.
    pub(super) version: u64,
    pub(super) stamp: Option<Stamp>,
}

fn loaded<T>(result: crate::error::Result<T>, what: &str) -> Option<T> {
    result.map_err(|err| eprintln!("simulacra: {what} not loaded: {err}")).ok()
}

impl ArenaHost {
    pub(super) fn new(state: &AppState, window: &mut Window, cx: &mut Context<AppView>) -> (Self, Vec<Subscription>) {
        let db = state.db.as_ref();
        let debates = db.and_then(|db| loaded(db.load_arena_records(), "debates")).unwrap_or_default();
        let experiments = db.and_then(|db| loaded(db.load_arena_records(), "experiments")).unwrap_or_default();
        let book = db.and_then(|db| loaded(db.load_belief_book(), "beliefs")).unwrap_or_default();
        let settings = db.and_then(|db| loaded(db.load_belief_settings(), "belief settings")).unwrap_or_default();
        let theme = state.theme_tokens.clone();
        let panel = cx.new(|cx| ArenaPanel::new(ArenaData::default(), theme.clone(), window, cx));
        let beliefs_panel = cx.new(|cx| BeliefsPanel::new(theme, window, cx));
        let subscriptions = vec![cx.subscribe_in(&panel, window, AppView::handle_arena_event), cx.subscribe_in(&beliefs_panel, window, AppView::handle_beliefs_event)];
        let host = Self {
            panel, beliefs_panel, debates, experiments, book, settings, run: None, ticket: 0, notice: None,
            updates: HashMap::new(), belief_notice: None, profile_tab: ProfileTab::default(), version: 0, stamp: None,
        };
        (host, subscriptions)
    }

    /// Stops the run in progress (its reply, if it still lands, is dropped).
    pub fn stop(&mut self) {
        if let Some(run) = self.run.take() {
            run.cancel.cancel();
        }
        self.ticket += 1;
    }

    /// A fresh ticket.
    pub fn next_ticket(&mut self) -> u64 {
        self.ticket += 1;
        self.ticket
    }

    /// The settings of `agent` (on by default).
    pub fn settings_of(&self, agent: &ProfileId) -> BeliefSettings {
        self.settings.get(agent).cloned().unwrap_or_default()
    }
}
