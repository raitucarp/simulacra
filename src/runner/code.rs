//! One run of a code block (P2): a fresh folder holding the code as a file, the program
//! started there with its arguments as a list (never a shell), a cleared environment, a time
//! limit, an output cap, and its whole process tree ended afterwards.

use super::env::program_env;
use super::folder::RunFolder;
use super::process::{run, Captured, Limits};
use std::ffi::OsString;
use std::path::PathBuf;
use std::process::Command;
use std::sync::atomic::AtomicBool;

/// In an extra variable's value, the run's folder (`GOCACHE={dir}/cache`).
pub const DIR_PLACEHOLDER: &str = "{dir}";

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
    pub limits: Limits,
}

/// Runs `spec` to its end (blocking: call it off the UI thread); `cancel` stops it early.
pub fn run_code(spec: &RunSpec, cancel: &AtomicBool) -> std::io::Result<Captured> {
    let folder = RunFolder::create()?;
    folder.write(&spec.file_name, &spec.code)?;
    let dir = folder.path().to_string_lossy().into_owned();
    let extra = spec.env.iter().map(|(name, value)| (OsString::from(name), OsString::from(value.replace(DIR_PLACEHOLDER, &dir))));
    let mut command = Command::new(&spec.program);
    command.args(&spec.args).env_clear().envs(program_env(&spec.program, folder.path())).envs(extra).current_dir(folder.path());
    super::hide_window(&mut command);
    run(command, spec.limits, cancel)
}
