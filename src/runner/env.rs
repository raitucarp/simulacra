//! The environment a program starts with: cleared, then only what a runtime needs to start.
//! Nothing of the user's own environment (keys, tokens, proxies, home) is passed on.
//!
//! - `PATH`: the program's own folder only (its libraries and helpers live there).
//! - Home and temp variables: `home`, a folder of the run's own.
//! - Windows: `SystemRoot`, `SystemDrive` and `windir`, copied, since runtimes fail to start
//!   without them (sockets, random numbers, code pages).

use std::ffi::OsString;
use std::path::Path;

/// Variables copied as they are, so the system's own libraries load.
#[cfg(windows)]
const SYSTEM_VARIABLES: &[&str] = &["SystemRoot", "SystemDrive", "windir"];
#[cfg(not(windows))]
const SYSTEM_VARIABLES: &[&str] = &[];

/// Variables pointed at the run's own folder.
#[cfg(windows)]
const HOME_VARIABLES: &[&str] = &["USERPROFILE", "HOME", "TEMP", "TMP"];
#[cfg(not(windows))]
const HOME_VARIABLES: &[&str] = &["HOME", "TMPDIR"];

/// The whole environment of `program`, run with `home` as its home and temp folder.
pub fn program_env(program: &Path, home: &Path) -> Vec<(OsString, OsString)> {
    let path = program.parent().map(|dir| dir.as_os_str().to_os_string()).unwrap_or_default();
    let system = SYSTEM_VARIABLES.iter().filter_map(|name| std::env::var_os(name).map(|value| (OsString::from(name), value)));
    let homes = HOME_VARIABLES.iter().map(|name| (OsString::from(name), home.as_os_str().to_os_string()));
    std::iter::once((OsString::from("PATH"), path)).chain(system).chain(homes).collect()
}
