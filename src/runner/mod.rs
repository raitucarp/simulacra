//! Running code on this computer (epic P). Off unless the user turns it on (Settings › Code).
//!
//! - [`probe`]: finding the installed runtimes by asking each its version (P1).
//! - [`code`]: one run of a code block in a fresh folder (P2).
//! - [`process`]: one program run to its end, output captured, bounded in time and size.
//! - [`tree`]: the program and everything it starts, ended together (job object, process group).
//! - [`env`]: the cleared environment programs start with.
//! - [`folder`]: the fresh folder a run works in.
//! - [`trust`]: the checks before anything starts (P7 security review).
//!
//! Programs are always started with their arguments as a list, never through a shell, and
//! with no window of their own.

pub mod code;
pub mod env;
pub mod folder;
pub mod probe;
pub mod process;
pub mod tree;
pub mod trust; // P7
#[cfg(test)]
mod tests;

pub use code::{run_code, runtime_cache_dir, spec_for, RunSpec, CACHE_PLACEHOLDER, DIR_PLACEHOLDER};
pub use probe::{detect, detect_all, find_on_path, version_of};
pub use process::{Captured, End, Limits};
pub use trust::{is_runtime_executable, private_cache};

use std::process::Command;

/// Windows: no console window for the program (the app has none to share).
#[cfg(windows)]
fn hide_window(command: &mut Command) {
    use std::os::windows::process::CommandExt;
    /// `CREATE_NO_WINDOW` (winbase.h).
    const CREATE_NO_WINDOW: u32 = 0x0800_0000;
    command.creation_flags(CREATE_NO_WINDOW);
}

#[cfg(not(windows))]
fn hide_window(_command: &mut Command) {}
