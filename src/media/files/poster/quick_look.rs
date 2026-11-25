//! L6 on macOS: Quick Look's thumbnail through `qlmanage`, the tool every macOS ships. It
//! writes `<file name>.png` into the folder it is given, scaled to the size asked.

use crate::runner::process::{End, Limits};
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::AtomicBool;
use std::time::Duration;

const QUICK_LOOK_TOOL: &str = "qlmanage";
const PNG_EXTENSION: &str = "png";
/// How long the tool may take, and how much it may print (it says little; the file is the result).
pub const QUICK_LOOK_LIMITS: Limits = Limits { timeout: Duration::from_secs(10), output_cap: 64 * 1024 };

/// `qlmanage -t -s <edge> -o <out> <file>`.
pub fn quick_look_args(path: &Path, edge: u32, out: &Path) -> Vec<OsString> {
    vec!["-t".into(), "-s".into(), edge.to_string().into(), "-o".into(), out.as_os_str().to_owned(), path.as_os_str().to_owned()]
}

/// Where the tool writes the thumbnail of `path` inside `out`.
pub fn quick_look_output(path: &Path, out: &Path) -> Option<PathBuf> {
    let mut name = path.file_name()?.to_owned();
    name.push(".");
    name.push(PNG_EXTENSION);
    Some(out.join(name))
}

/// Quick Look's thumbnail of `path`, made in `scratch` (emptied afterwards). The tool can wait
/// for ever when no window server answers (a headless session), so it is given
/// [`QUICK_LOOK_LIMITS`] and ended, with everything it started, once they pass.
#[cfg_attr(not(target_os = "macos"), allow(dead_code))]
pub(super) fn quick_look(path: &Path, edge: u32, scratch: &Path) -> Option<image::DynamicImage> {
    std::fs::create_dir_all(scratch).ok()?;
    let mut command = std::process::Command::new(QUICK_LOOK_TOOL);
    command.args(quick_look_args(path, edge, scratch));
    let ran = ran_to_success(command, QUICK_LOOK_LIMITS);
    let output = quick_look_output(path, scratch)?;
    let poster = ran.then(|| image::open(&output).ok()).flatten();
    let _ = std::fs::remove_file(&output);
    poster
}

/// Whether `command` exited by itself with code 0 within `limits`; a run that is timed out or
/// cannot start counts as a failure, and nothing it started outlives it.
pub fn ran_to_success(command: Command, limits: Limits) -> bool {
    crate::runner::process::run(command, limits, &AtomicBool::new(false))
        .is_ok_and(|run| run.end == End::Exited && run.exit_code == Some(0))
}
