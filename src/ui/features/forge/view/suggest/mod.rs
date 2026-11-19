//! Forge state of the suggestions (sb D1): the AI request in flight, the values it proposed
//! per parameter, and the status line of the current step's suggestion bar. The request is
//! sent by the host (it owns the shared daily AI budget) through [`ForgeEvent::Suggest`].
//!
//! [`ForgeEvent::Suggest`]: super::ForgeEvent::Suggest

mod accept;
mod run;

use crate::agent::post_actions::CancelToken;
use crate::domain::post_actions::ActionPrompt;
use crate::state::forge::suggest::ai::{SuggestScope, SuggestedValue};
use crate::state::forge::suggest::Param;

/// What the host is asked to send.
#[derive(Debug, Clone)]
pub struct SuggestJob {
    /// Matches the answer to the request (a newer request makes older answers stale).
    pub generation: u64,
    pub prompt: ActionPrompt,
    /// The least answer budget the request needs.
    pub answer_tokens: u32,
    pub cancel: CancelToken,
}

/// The request in flight.
#[derive(Debug, Clone)]
pub struct SuggestRun {
    pub scope: SuggestScope,
    pub targets: Vec<Param>,
    pub cancel: CancelToken,
}

/// The step bar's status line.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum SuggestStatus {
    #[default]
    Idle,
    /// AI values wait under their fields (`discarded` unusable ones were dropped).
    Ready { count: usize, discarded: usize },
    /// "Fill the rest" changed this many fields.
    Filled(usize),
    /// An informational line (a locale key).
    Note(&'static str),
    /// A display-safe failure message.
    Failed(String),
}

/// Everything the forge remembers about suggestions on the current step.
#[derive(Debug, Clone, Default)]
pub struct SuggestUi {
    pub generation: u64,
    pub run: Option<SuggestRun>,
    pub results: Vec<(Param, Vec<SuggestedValue>)>,
    pub status: SuggestStatus,
}

impl SuggestUi {
    /// The AI values proposed for `param`.
    pub fn values(&self, param: Param) -> &[SuggestedValue] {
        self.results.iter().find(|(candidate, _)| *candidate == param).map_or(&[], |(_, values)| values.as_slice())
    }

    /// Whether a request for `scope` is in flight.
    pub fn is_running(&self, scope: SuggestScope) -> bool {
        self.run.as_ref().is_some_and(|run| run.scope == scope)
    }

    /// Stops any request and forgets every result (a new step starts clean).
    pub fn reset(&mut self) {
        if let Some(run) = self.run.take() {
            run.cancel.cancel();
        }
        self.generation = self.generation.wrapping_add(1);
        self.results.clear();
        self.status = SuggestStatus::Idle;
    }
}
