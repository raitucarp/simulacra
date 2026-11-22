//! Running code on this computer (epic P). Off unless the user turns it on (Settings › Code).
//!
//! - [`probe`]: finding the installed runtimes by asking each its version (P1).
//! - [`process`]: one program run to its end, output captured, bounded in time and size.
//! - [`env`]: the cleared environment programs start with.
//!
//! Programs are always started with their arguments as a list, never through a shell, and
//! with no window of their own.

pub mod env;
pub mod probe;
pub mod process;
#[cfg(test)]
mod tests;

pub use probe::{detect, detect_all, find_on_path, version_of};
pub use process::{Captured, End, Limits};

use std::process::{Child, Command};

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

/// Ends `child`.
fn stop_child(child: &mut Child) {
    let _ = child.kill();
}
