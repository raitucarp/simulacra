//! L6 on macOS: Quick Look's thumbnail through `qlmanage`, the tool every macOS ships. It
//! writes `<file name>.png` into the folder it is given, scaled to the size asked.

use std::ffi::OsString;
use std::path::{Path, PathBuf};

const QUICK_LOOK_TOOL: &str = "qlmanage";
const PNG_EXTENSION: &str = "png";

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

/// Quick Look's thumbnail of `path`, made in `scratch` (emptied afterwards).
#[cfg_attr(not(target_os = "macos"), allow(dead_code))]
pub(super) fn quick_look(path: &Path, edge: u32, scratch: &Path) -> Option<image::DynamicImage> {
    std::fs::create_dir_all(scratch).ok()?;
    let ran = std::process::Command::new(QUICK_LOOK_TOOL)
        .args(quick_look_args(path, edge, scratch))
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .is_ok_and(|status| status.success());
    let output = quick_look_output(path, scratch)?;
    let poster = ran.then(|| image::open(&output).ok()).flatten();
    let _ = std::fs::remove_file(&output);
    poster
}
