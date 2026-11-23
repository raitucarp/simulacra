//! A saved run of a code block (P6): the runtime and its version, how it ended, the exit code,
//! how long it took, when, and what it printed (shortened). Kept on the post as `extras.code_runs`,
//! the latest run of each block; saving one never makes a new version of the post.

use crate::domain::document::keyed::keyed_enum;
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

/// Output characters a saved run keeps; the rest is cut (the panel says so).
pub const SAVED_OUTPUT_CHARS: usize = 8 * 1024;

keyed_enum! {
    /// How a run ended. Keys are stored: never rename one.
    RunEnd {
        Exited => "exited",
        TimedOut => "timed_out",
        Cancelled => "cancelled",
        OutputCapped => "output_capped",
        /// The runtime did not start.
        Failed => "failed",
    }
}

/// One finished run of one code block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct CodeRun {
    /// The block's place among the post's code blocks (from 0).
    pub block: usize,
    /// The runtime's key (`python`).
    pub runtime: String,
    pub version: String,
    /// Unknown ends (from a newer version) read as failed.
    #[serde(deserialize_with = "lenient_end")]
    pub end: RunEnd,
    pub exit_code: Option<i32>,
    pub duration_ms: u64,
    pub ran_at: Option<Timestamp>,
    pub output: String,
    /// The output was cut to [`SAVED_OUTPUT_CHARS`].
    pub truncated: bool,
}

impl Default for CodeRun {
    fn default() -> Self {
        Self { block: 0, runtime: String::new(), version: String::new(), end: RunEnd::Failed, exit_code: None, duration_ms: 0, ran_at: None, output: String::new(), truncated: false }
    }
}

fn lenient_end<'de, D: serde::Deserializer<'de>>(deserializer: D) -> Result<RunEnd, D::Error> {
    let value = Option::<String>::deserialize(deserializer)?;
    Ok(value.as_deref().and_then(RunEnd::from_key).unwrap_or(RunEnd::Failed))
}

/// `output` cut to [`SAVED_OUTPUT_CHARS`] characters, and whether it was cut.
pub fn kept_output(output: &str) -> (String, bool) {
    match output.char_indices().nth(SAVED_OUTPUT_CHARS) {
        Some((cut, _)) => (output[..cut].to_string(), true),
        None => (output.to_string(), false),
    }
}

/// `runs` with `run` as its block's latest, in block order.
pub fn with_run(mut runs: Vec<CodeRun>, run: CodeRun) -> Vec<CodeRun> {
    runs.retain(|kept| kept.block != run.block);
    runs.push(run);
    runs.sort_by_key(|kept| kept.block);
    runs
}
