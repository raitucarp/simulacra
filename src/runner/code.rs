//! One run of a code block (P2): a fresh folder holding the code as a file, the program
//! started there with its arguments as a list (never a shell), a cleared environment, a time
//! limit, an output cap, and its whole process tree ended afterwards.

use super::env::program_env;
use super::folder::RunFolder;
use super::process::{run, Captured, Limits};
use super::trust::{is_runtime_executable, private_cache};
use crate::domain::code::{DetectedRuntime, Runtime};
use std::ffi::OsString;
use std::path::PathBuf;
use std::process::Command;
use std::sync::atomic::AtomicBool;

/// In an extra variable's value, the run's folder (`GOCACHE={dir}/cache`).
pub const DIR_PLACEHOLDER: &str = "{dir}";
/// In an extra variable's value, the runtime's cache kept between runs (P3: compiled
/// packages, so a Go run need not build its standard library each time).
pub const CACHE_PLACEHOLDER: &str = "{cache}";
/// The folder, under the system temp folder, that holds every runtime's cache.
const CACHE_ROOT: &str = "simulacra-runtime-cache";

/// What to run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunSpec {
    /// The runtime's executable, by full path.
    pub program: PathBuf,
    /// Its arguments; the code file is named by `file_name`, relative to the run's folder.
    pub args: Vec<String>,
    /// The file the code is written to (a plain name).
    pub file_name: String,
    pub code: String,
    /// Variables the runtime needs on top of the cleared environment.
    pub env: Vec<(String, String)>,
    /// The runtime's cache folder, for [`CACHE_PLACEHOLDER`] (`None`: the run's own folder).
    pub cache: Option<PathBuf>,
    pub limits: Limits,
}

/// The cache folder of `runtime`.
pub fn runtime_cache_dir(runtime: Runtime) -> PathBuf {
    std::env::temp_dir().join(CACHE_ROOT).join(runtime.key())
}

/// The run of `code` in the runtime `found`, with its fixed command line (P3). Refused when
/// `found` no longer names an executable of its runtime (P7); the cache is used only when it
/// is the user's own.
pub fn spec_for(found: &DetectedRuntime, code: &str, limits: Limits) -> std::io::Result<RunSpec> {
    if !is_runtime_executable(found) {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, format!("{} is not a {} executable", found.path, found.runtime.label())));
    }
    let invocation = found.runtime.invocation();
    let cache = private_cache(&runtime_cache_dir(found.runtime));
    Ok(RunSpec { program: PathBuf::from(&found.path), args: invocation.args, file_name: invocation.file_name.into(), code: code.into(), env: invocation.env, cache, limits })
}

/// Runs `spec` to its end (blocking: call it off the UI thread); `cancel` stops it early.
pub fn run_code(spec: &RunSpec, cancel: &AtomicBool) -> std::io::Result<Captured> {
    let folder = RunFolder::create()?;
    folder.write(&spec.file_name, &spec.code)?;
    let dir = folder.path().to_string_lossy().into_owned();
    let cache = spec.cache.as_ref().map(|cache| cache.to_string_lossy().into_owned()).unwrap_or_else(|| dir.clone());
    let fill = |value: &str| value.replace(DIR_PLACEHOLDER, &dir).replace(CACHE_PLACEHOLDER, &cache);
    let extra = spec.env.iter().map(|(name, value)| (OsString::from(name), OsString::from(fill(value))));
    let mut command = Command::new(&spec.program);
    command.args(&spec.args).env_clear().envs(program_env(&spec.program, folder.path())).envs(extra).current_dir(folder.path());
    super::hide_window(&mut command);
    run(command, spec.limits, cancel)
}
