//! Running code blocks in the store: the cached detection (P1) and saved runs (P6).

use crate::domain::code::{CodeRun, DetectedRuntime, RunEnd, Runtime, RuntimeCache};
use crate::domain::crypto::KdfParams;
use crate::domain::post::lock::LockPassword;
use crate::error::AppError;
use crate::state::{AppState, ThreadAnchor, ThreadDraft};

fn run(block: usize, output: &str) -> CodeRun {
    CodeRun { block, runtime: "python".into(), version: "3.12.2".into(), end: RunEnd::Exited, exit_code: Some(0), duration_ms: 120, ran_at: Some(jiff::Timestamp::now()), output: output.into(), truncated: false }
}

#[test]
fn the_detection_is_cached_in_the_vault() {
    let state = AppState::new_in_memory();
    assert_eq!(state.runtime_cache(), RuntimeCache::default(), "never detected");
    let cache = RuntimeCache { found: vec![DetectedRuntime { runtime: Runtime::Node, path: "node".into(), version: "20.1.0".into() }], probed_at: Some(jiff::Timestamp::now()) };
    state.store_runtime_cache(&cache);
    assert_eq!(state.runtime_cache(), cache);
}

#[test]
fn a_run_is_kept_per_block_and_survives_a_reload_without_a_new_version() {
    let mut state = AppState::new_in_memory();
    let vault = state.db.clone().expect("vault");
    let id = state.submit_post("```py\nprint(1)\n```\n```py\nprint(2)\n```").expect("posted");
    state.save_code_run(&id, run(1, "2\n")).expect("saved");
    state.save_code_run(&id, run(0, "first\n")).expect("saved");
    state.save_code_run(&id, run(0, "1\n")).expect("saved again");
    let runs = &state.get_post(&id).expect("post").extras.code_runs;
    assert_eq!(runs.iter().map(|r| (r.block, r.output.as_str())).collect::<Vec<_>>(), vec![(0, "1\n"), (1, "2\n")], "the latest of each block");
    assert!(state.post_revisions(&id).is_empty(), "a run is no edit");
    let reloaded = AppState::load_from_db(&vault).expect("reload");
    assert_eq!(&reloaded.get_post(&id).expect("post").extras.code_runs, runs);
}

#[test]
fn a_locked_post_never_keeps_a_run() {
    let mut state = AppState::new_in_memory();
    let fast = KdfParams { memory_kib: 8, iterations: 1, lanes: 1 };
    let id = state.submit_locked_thread(&[ThreadDraft::text("```py\nprint('secret')\n```")], &ThreadAnchor::default(), &LockPassword::new("seven tidy lanterns"), fast).expect("locked").remove(0);
    assert_eq!(state.save_code_run(&id, run(0, "secret")), Err(AppError::PostLocked));
    assert!(state.get_post(&id).expect("post").extras.code_runs.is_empty());
}
