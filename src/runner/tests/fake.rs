//! A fake runtime (P2): this test binary itself, started by the runner with only its own
//! ignored test selected and a mode variable. It needs no runtime installed.

use crate::runner::{run_code, Captured, Limits, RunSpec};
use std::io::Write;
use std::sync::atomic::AtomicBool;
use std::time::Duration;

/// The variable naming what the fake runtime does.
pub(super) const MODE: &str = "SIMULACRA_FAKE_RUNTIME";
/// The fake runtime's test, as libtest names it.
const FAKE_TEST: &str = "runner::tests::fake::fake_runtime";
/// The file the code is written to.
pub(super) const CODE_FILE: &str = "main.txt";
/// How long the sleeping modes sleep (far past every limit used).
const LONG_SLEEP: Duration = Duration::from_secs(60);

/// The arguments that run only [`fake_runtime`] in a copy of this binary.
fn fake_args() -> Vec<String> {
    ["--exact", FAKE_TEST, "--ignored", "--nocapture", "--test-threads=1", "--quiet"].map(String::from).to_vec()
}

/// A spec running the fake runtime in `mode` on `code`.
pub(super) fn spec(mode: &str, code: &str, limits: Limits) -> RunSpec {
    let program = std::env::current_exe().expect("test binary");
    RunSpec { program, args: fake_args(), file_name: CODE_FILE.into(), code: code.into(), env: vec![(MODE.into(), mode.into())], cache: None, limits }
}

/// Runs the fake runtime.
pub(super) fn run_fake(mode: &str, code: &str, limits: Limits) -> Captured {
    run_code(&spec(mode, code, limits), &AtomicBool::new(false)).expect("ran")
}

#[test]
#[ignore = "the fake runtime the runner tests start; it does nothing on its own"]
fn fake_runtime() {
    let Ok(mode) = std::env::var(MODE) else { return };
    let mut out = std::io::stdout();
    match mode.as_str() {
        "echo" => {
            let code = std::fs::read_to_string(CODE_FILE).unwrap_or_default();
            let dir = std::env::current_dir().map(|dir| dir.display().to_string()).unwrap_or_default();
            let names: Vec<String> = std::env::vars_os().map(|(name, _)| name.to_string_lossy().into_owned()).collect();
            let _ = writeln!(out, "code={code}\ndir={dir}\nvars={}", names.join(","));
        }
        "exit3" => std::process::exit(3),
        "flood" => loop {
            if out.write_all(&[b'x'; 4096]).is_err() {
                return;
            }
        },
        "spawn" => {
            let child = std::process::Command::new(std::env::current_exe().expect("binary")).args(fake_args()).env(MODE, "sleep").spawn();
            let _ = writeln!(out, "spawned={}", child.map(|c| c.id()).unwrap_or_default());
            let _ = out.flush();
            std::thread::sleep(LONG_SLEEP);
        }
        _ => std::thread::sleep(LONG_SLEEP),
    }
}
