//! The Insights page's thinking views (sb E1): Calibration (how well stated credences
//! matched what happened, and the predictions still open) and Tensions (pairs of posts or
//! vault passages that may contradict). The shell hands the page a [`ThinkingInsights`]
//! snapshot; the views answer with [`ThinkingRequest`]s.

pub mod calibration;
mod panel;
pub mod reliability;
pub mod tensions;

use crate::domain::thinking::credence::{CalibrationSummary, Outcome};
use crate::domain::thinking::tensions::{TensionSide, Verdict};
use crate::domain::PostId;
use chrono::NaiveDate;
use gpui_kit::{App, Window};
use std::rc::Rc;

/// One open prediction in the Calibration view's list.
#[derive(Debug, Clone, PartialEq)]
pub struct PredictionRow {
    pub post_id: PostId,
    pub excerpt: String,
    pub percent: u8,
    pub due: Option<NaiveDate>,
    pub is_due: bool,
}

/// What the Calibration view shows.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct CalibrationData {
    pub summary: Option<CalibrationSummary>,
    /// Open predictions by due date.
    pub open: Vec<PredictionRow>,
    /// Only the predictions that are due are listed ("Resolve due predictions").
    pub due_only: bool,
}

/// One pair in the Tensions view.
#[derive(Debug, Clone, PartialEq)]
pub struct TensionRow {
    pub pair: String,
    pub left: TensionSide,
    pub right: TensionSide,
    pub verdict: Option<Verdict>,
    pub reason: String,
    /// The lexical signals, already worded ("negation", "always / never").
    pub signals: Vec<String>,
}

/// How the candidates were found.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TensionMode {
    /// Meaning and the AI's verdicts.
    #[default]
    Full,
    /// No embeddings: candidates found by shared words, still checked by the AI.
    NoMeaning,
    /// No AI key: word-based candidates only, unchecked.
    LexicalOnly,
}

/// What the Tensions view shows.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct TensionsData {
    pub rows: Vec<TensionRow>,
    pub mode: TensionMode,
    /// Candidates are being ranked or classified.
    pub running: bool,
    /// The opening words of the post the search is focused on.
    pub focus: Option<String>,
    /// A failure of the last classification (already localized).
    pub problem: Option<String>,
    /// A scan has finished at least once.
    pub scanned: bool,
}

/// Everything the two views show.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ThinkingInsights {
    pub calibration: CalibrationData,
    pub tensions: TensionsData,
}

/// Requests the views send the shell.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ThinkingRequest {
    OpenPost(PostId),
    /// Open a vault document.
    OpenDocument(String),
    ResolvePrediction(PostId, Outcome),
    ToggleDueOnly,
    /// Rank candidates again (and classify the new ones).
    ScanTensions,
    /// Search the whole stream instead of around one post.
    ClearFocus,
    /// Reply to reconcile the pair (the composer quotes both).
    Reconcile(String),
    MarkResolved(String),
    Dismiss(String),
}

/// Sends a request to the shell.
pub type RequestAction = Rc<dyn Fn(ThinkingRequest, &mut Window, &mut App) + 'static>;
