//! Thinking tools wiring of the shell (sb E1): what was loaded from the vault (argument maps
//! by thread root, credences by post, remembered tension pairs by content hash), the
//! inspector's Argument view, the credence popover's note field, the atomicity sheet, the
//! last tension ranking and the running AI requests (each with its cancel flag and ticket).

use super::view::AppView;
use crate::agent::post_actions::CancelToken;
use crate::domain::thinking::argument::ArgumentMap;
use crate::domain::thinking::credence::Credence;
use crate::domain::thinking::tensions::{Candidate, TensionRecord};
use crate::domain::PostId;
use crate::state::AppState;
use crate::ui::features::thinking::argument::ArgumentPanel;
use crate::ui::features::thinking::atomic::AtomicSheet;
use crate::ui::features::thinking::insights::TensionMode;
use gpui_kit::component::input::InputState;
use gpui_kit::*;
use rust_i18n::t;
use std::collections::HashMap;

/// A running AI request: its cancel flag and the ticket its result must still match.
pub(super) struct Running {
    pub cancel: CancelToken,
    pub ticket: u64,
}

/// The contradiction finder's latest ranking and what it is doing.
#[derive(Default)]
pub(super) struct TensionScan {
    pub candidates: Vec<Candidate>,
    /// A side key: the scan around one post ("Find tensions with this post").
    pub focus: Option<String>,
    pub mode: TensionMode,
    pub ranking: bool,
    /// The ticket the running ranking must still match.
    pub ranking_ticket: u64,
    pub classifying: Option<Running>,
    /// The batch being classified, and how many batches may follow it.
    pub batch: (Vec<Candidate>, usize),
    pub problem: Option<String>,
    pub scanned: bool,
    /// What the ranking was computed from (store revision, focus, mode).
    pub stamp: Option<(u64, Option<String>, TensionMode)>,
}

/// Everything the thinking tools keep in the shell.
pub struct ThinkingHost {
    pub maps: HashMap<PostId, ArgumentMap>,
    pub credences: HashMap<PostId, Credence>,
    pub tensions: HashMap<String, TensionRecord>,
    pub argument: Entity<ArgumentPanel>,
    /// The credence popover's resolution note.
    pub note: Entity<InputState>,
    /// The post whose credence popover is open.
    pub credence_open: Option<PostId>,
    pub atomic_sheet: Option<AtomicSheet>,
    /// The Calibration view lists only due predictions.
    pub due_only: bool,
    pub(super) extract: Option<Running>,
    /// The thread the running extraction maps.
    pub(super) extract_root: Option<PostId>,
    pub(super) scan: TensionScan,
    /// Bumped for every request started (tickets).
    pub(super) tickets: u64,
}

impl ThinkingHost {
    /// Loads the rows and creates the Argument view, returning its subscription.
    pub(super) fn new(state: &AppState, window: &mut Window, cx: &mut Context<AppView>) -> (Self, Subscription) {
        let rows = state.db.as_ref().and_then(|db| db.load_thinking().map_err(|err| eprintln!("simulacra: thinking rows not loaded: {err}")).ok()).unwrap_or_default();
        let argument = cx.new(|cx| ArgumentPanel::new(state.theme_tokens.clone(), window, cx));
        let placeholder = t!("thinking.credence.note_placeholder").to_string();
        let note = cx.new(|cx| InputState::new(window, cx).placeholder(placeholder));
        let subscription = cx.subscribe_in(&argument, window, AppView::handle_argument_event);
        let host = Self {
            maps: rows.maps.into_iter().map(|map| (map.root.clone(), map)).collect(),
            credences: rows.credences.into_iter().map(|c| (c.post_id.clone(), c)).collect(),
            tensions: rows.tensions.into_iter().map(|r| (r.pair.clone(), r)).collect(),
            argument,
            note,
            credence_open: None,
            atomic_sheet: None,
            due_only: false,
            extract: None,
            extract_root: None,
            scan: TensionScan::default(),
            tickets: 0,
        };
        (host, subscription)
    }

    /// A fresh ticket.
    pub(super) fn next_ticket(&mut self) -> u64 {
        self.tickets += 1;
        self.tickets
    }

    /// Stops a running request.
    pub(super) fn stop(running: &mut Option<Running>) {
        if let Some(running) = running.take() {
            running.cancel.cancel();
        }
    }
}
