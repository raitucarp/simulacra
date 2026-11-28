//! The run report: one row per case in a process-wide collector, rewritten to
//! `target/live-report.md` after every row (so the file is complete when the last case ends,
//! whatever filter was used). With `--test-threads=1` the rows come in test-name order.

use super::harness::{Live, AUDIO_VAR, TIERS_VAR};
use std::fmt::Write as _;
use std::path::PathBuf;
use std::sync::Mutex;

/// The report file, under the target directory.
const REPORT_FILE: &str = "live-report.md";
/// Folder (under the target directory) where generated media is kept for a look.
const MEDIA_FOLDER: &str = "live-media";
/// Characters of a note kept in the table.
const MAX_NOTE_CHARS: usize = 220;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    Pass,
    Fail,
    Skip,
}

impl Status {
    const fn label(self) -> &'static str {
        match self {
            Self::Pass => "pass",
            Self::Fail => "FAIL",
            Self::Skip => "skip",
        }
    }
}

/// One case. `note` is already redacted.
#[derive(Debug, Clone)]
pub struct Row {
    pub tier: char,
    pub name: String,
    pub status: Status,
    pub latency_ms: u128,
    pub model: String,
    pub note: String,
}

static ROWS: Mutex<Vec<Row>> = Mutex::new(Vec::new());

/// `CARGO_TARGET_DIR` when set, else the package's `target` folder.
fn target_dir() -> PathBuf {
    std::env::var_os("CARGO_TARGET_DIR").map(PathBuf::from).unwrap_or_else(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target"))
}

pub fn report_path() -> PathBuf {
    target_dir().join(REPORT_FILE)
}

/// A fresh folder for one case's generated media.
pub fn media_dir() -> PathBuf {
    target_dir().join(MEDIA_FOLDER).join(uuid::Uuid::new_v4().simple().to_string())
}

/// Adds `row`, prints it and rewrites the report.
pub fn record(row: Row) {
    println!("[live] {} {:<4} {:>6} ms  {}  ({}): {}", row.tier, row.status.label(), row.latency_ms, row.name, row.model, row.note);
    let mut rows = ROWS.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    rows.push(row);
    let text = render(&rows);
    if let Err(error) = std::fs::create_dir_all(target_dir()).and_then(|()| std::fs::write(report_path(), text)) {
        eprintln!("[live] could not write {}: {error}", report_path().display());
    }
}

/// One table cell: a single line with pipes escaped, shortened.
fn cell(text: &str) -> String {
    let line = text.split_whitespace().collect::<Vec<_>>().join(" ").replace('|', "\\|");
    let mut short: String = line.chars().take(MAX_NOTE_CHARS).collect();
    if line.chars().count() > MAX_NOTE_CHARS {
        short.push('…');
    }
    short
}

fn render(rows: &[Row]) -> String {
    let live = Live::from_env();
    let count = |status| rows.iter().filter(|row| row.status == status).count();
    let mut out = String::from("# Live model report\n\n");
    let _ = writeln!(out, "- Run: {}", jiff::Timestamp::now());
    let _ = writeln!(out, "- Text model: `{}`; image: `{}`; embeddings: `{}`", live.model, live.image_model, live.embed_model);
    let _ = writeln!(out, "- Tiers (`{TIERS_VAR}`): `{}`; key set: {}; `{AUDIO_VAR}` set: {}", live.tiers(), live.has_key(), std::env::var_os(AUDIO_VAR).is_some());
    let _ = writeln!(out, "- Result: {} passed, {} failed, {} skipped\n", count(Status::Pass), count(Status::Fail), count(Status::Skip));
    out.push_str("| Tier | Case | Status | Latency (ms) | Model | Note |\n|---|---|---|---:|---|---|\n");
    for row in rows {
        let _ = writeln!(out, "| {} | {} | {} | {} | `{}` | {} |", row.tier.to_ascii_uppercase(), cell(&row.name), row.status.label(), row.latency_ms, cell(&row.model), cell(&row.note));
    }
    out
}
