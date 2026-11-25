//! The runner (P2) against the fake runtime: a fresh folder, a cleared environment, a time
//! limit, an output cap, cancel, and the whole process tree ended.

use super::fake::{run_fake, spec, CODE_FILE};
use crate::runner::folder::is_plain_name;
use crate::runner::process::READ_GRACE;
use crate::runner::{run_code, End, Limits};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

const ROOMY: Limits = Limits { timeout: Duration::from_secs(60), output_cap: 64 * 1024 };
const SHORT: Duration = Duration::from_millis(1_500);
/// Variables the fake runtime's own system libraries set as it starts (none come from us):
/// on macOS, the system framework every app links writes its text encoding at load.
#[cfg(target_os = "macos")]
const SET_BY_THE_CHILD_ITSELF: &[&str] = &["__CF_USER_TEXT_ENCODING"];
#[cfg(not(target_os = "macos"))]
const SET_BY_THE_CHILD_ITSELF: &[&str] = &[];

fn line<'a>(output: &'a str, key: &str) -> &'a str {
    output.lines().find_map(|line| line.strip_prefix(key)).unwrap_or_default()
}

#[test]
fn code_runs_in_a_fresh_folder_that_is_removed_afterwards() {
    let first = run_fake("echo", "print(1)", ROOMY);
    let second = run_fake("echo", "print(2)", ROOMY);
    assert_eq!((first.end, first.exit_code), (End::Exited, Some(0)), "{}", first.output);
    assert_eq!(line(&first.output, "code="), "print(1)", "the code is the file {CODE_FILE}");
    let (dir_a, dir_b) = (line(&first.output, "dir="), line(&second.output, "dir="));
    assert!(!dir_a.is_empty() && dir_a != dir_b, "a new folder each run");
    assert!(!std::path::Path::new(dir_a).exists() && !std::path::Path::new(dir_b).exists(), "removed after the run");
}

#[test]
fn the_environment_holds_only_what_a_runtime_needs() {
    let found = run_fake("echo", "", ROOMY);
    let vars: Vec<&str> = line(&found.output, "vars=").split(',').collect();
    let allowed = ["PATH", "HOME", "TMPDIR", "USERPROFILE", "TEMP", "TMP", "SystemRoot", "SystemDrive", "windir", super::fake::MODE];
    let set_by_the_child = SET_BY_THE_CHILD_ITSELF;
    assert!(vars.iter().all(|var| allowed.iter().chain(set_by_the_child).any(|ok| ok.eq_ignore_ascii_case(var))), "{vars:?}");
    assert!(!vars.iter().any(|var| var.starts_with("CARGO")), "nothing of this process leaks in");
}

#[test]
fn the_exit_code_is_kept() {
    let found = run_fake("exit3", "", ROOMY);
    assert_eq!((found.end, found.exit_code), (End::Exited, Some(3)));
}

#[test]
fn a_run_past_its_time_is_stopped_with_everything_it_started() {
    let started = Instant::now();
    let found = run_fake("spawn", "", Limits { timeout: SHORT, ..ROOMY });
    assert_eq!((found.end, found.exit_code), (End::TimedOut, None), "{}", found.output);
    assert!(line(&found.output, "spawned=").parse::<u32>().is_ok_and(|pid| pid > 0), "a grandchild started: {}", found.output);
    // The grandchild shares the output pipe: had it survived, reading would wait out READ_GRACE.
    assert!(started.elapsed() < SHORT + READ_GRACE, "the grandchild was ended too ({:?})", started.elapsed());
}

#[test]
fn an_os_tool_that_never_ends_is_stopped_at_its_limit() {
    let mut tool = std::process::Command::new(std::env::current_exe().expect("test binary"));
    tool.args(super::fake::fake_args()).env(super::fake::MODE, "sleep");
    let started = Instant::now();
    let limits = Limits { timeout: SHORT, ..crate::media::files::poster::QUICK_LOOK_LIMITS };
    assert!(!crate::media::files::poster::ran_to_success(tool, limits), "a tool stopped at its limit did not succeed");
    assert!(started.elapsed() < SHORT + READ_GRACE, "stopped, not waited for ({:?})", started.elapsed());
}

#[test]
fn output_past_the_cap_stops_the_run() {
    let cap = 16 * 1024;
    let found = run_fake("flood", "", Limits { output_cap: cap, ..ROOMY });
    assert_eq!(found.end, End::OutputCapped);
    assert_eq!(found.output.len(), cap);
}

#[test]
fn cancel_stops_a_run() {
    let cancel = Arc::new(AtomicBool::new(false));
    let flag = cancel.clone();
    std::thread::spawn(move || {
        std::thread::sleep(Duration::from_millis(300));
        flag.store(true, Ordering::Relaxed);
    });
    let found = run_code(&spec("sleep", "", ROOMY), &cancel).expect("ran");
    assert_eq!(found.end, End::Cancelled);
    assert!(found.duration < Duration::from_secs(10));
}

#[test]
fn code_files_are_plain_names() {
    assert!(is_plain_name("main.py"));
    for bad in ["", ".", "..", "../x.py", "a/b.py", "a\\b.py", "C:x.py"] {
        assert!(!is_plain_name(bad), "{bad}");
        let mut wrong = spec("echo", "", ROOMY);
        wrong.file_name = bad.into();
        assert!(run_code(&wrong, &AtomicBool::new(false)).is_err(), "{bad}");
    }
}
