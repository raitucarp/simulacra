//! Finding the installed runtimes (P1): each runtime's executables are looked up on the
//! search path, and the first found is asked its version with its version flag and nothing
//! else, in a cleared environment, with a short time limit and a small output cap.

use super::env::program_env;
use super::process::{run, Captured, End, Limits};
use super::stop_child;
use crate::domain::code::{parse_version, DetectedRuntime, Runtime, RuntimeCache};
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::AtomicBool;
use std::time::Duration;

/// How long a version probe may take.
const PROBE_TIMEOUT: Duration = Duration::from_secs(10);
/// Output a version probe may print.
const PROBE_OUTPUT: usize = 4 * 1024;
/// The only executables taken on Windows (never a `.bat` or `.cmd`, which run in a shell).
#[cfg(windows)]
const EXECUTABLE_EXTENSION: &str = "exe";

/// `name` as an executable file in one of the folders of `path_var`, the first found.
pub fn find_on_path(name: &str, path_var: &OsStr) -> Option<PathBuf> {
    std::env::split_paths(path_var).filter(|dir| dir.is_absolute()).map(|dir| candidate(&dir, name)).find(|file| is_executable(file))
}

#[cfg(windows)]
fn candidate(dir: &Path, name: &str) -> PathBuf {
    dir.join(format!("{name}.{EXECUTABLE_EXTENSION}"))
}

#[cfg(not(windows))]
fn candidate(dir: &Path, name: &str) -> PathBuf {
    dir.join(name)
}

#[cfg(windows)]
fn is_executable(file: &Path) -> bool {
    file.is_file()
}

#[cfg(not(windows))]
fn is_executable(file: &Path) -> bool {
    use std::os::unix::fs::PermissionsExt;
    std::fs::metadata(file).is_ok_and(|meta| meta.is_file() && meta.permissions().mode() & 0o111 != 0)
}

/// The version `program` reports for `args`, if it reports one in time.
pub fn version_of(program: &Path, args: &[&str]) -> Option<String> {
    let home = std::env::temp_dir();
    let mut command = Command::new(program);
    command.args(args).env_clear().envs(program_env(program, &home)).current_dir(&home);
    super::hide_window(&mut command);
    let limits = Limits { timeout: PROBE_TIMEOUT, output_cap: PROBE_OUTPUT };
    let Captured { output, end, .. } = run(command, limits, &AtomicBool::new(false), stop_child).ok()?;
    (end == End::Exited).then(|| parse_version(&output)).flatten()
}

/// `runtime` as found on `path_var`: its first executable that reports a version.
pub fn detect(runtime: Runtime, path_var: &OsStr) -> Option<DetectedRuntime> {
    let probe = runtime.probe();
    probe.executables.iter().filter_map(|name| find_on_path(name, path_var)).find_map(|path| {
        let version = version_of(&path, probe.args)?;
        Some(DetectedRuntime { runtime, path: path.to_string_lossy().into_owned(), version })
    })
}

/// Every runtime found on this process's search path, now (blocking: run it off the UI
/// thread).
pub fn detect_all() -> RuntimeCache {
    let path_var = std::env::var_os("PATH").unwrap_or_default();
    let found = Runtime::ALL.iter().filter_map(|runtime| detect(*runtime, &path_var)).collect();
    RuntimeCache { found, probed_at: Some(jiff::Timestamp::now()) }
}
