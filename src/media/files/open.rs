//! Handing files and folders to the system file manager: it opens a folder, opens a file in
//! the app the system uses for its type (L4 "Open", a video or audio file in the system
//! player) and shows a file selected in its folder ("Show in folder").

use std::path::Path;
use std::process::Command;

/// The system file manager command, which also opens a file in its default app.
#[cfg(target_os = "windows")]
pub const FILE_MANAGER: &str = "explorer";
#[cfg(target_os = "macos")]
pub const FILE_MANAGER: &str = "open";
#[cfg(not(any(target_os = "windows", target_os = "macos")))]
pub const FILE_MANAGER: &str = "xdg-open";

/// Opens `folder` (created when missing) in the file manager.
pub fn open_folder(folder: &Path) -> Result<(), String> {
    std::fs::create_dir_all(folder).map_err(|err| err.to_string())?;
    spawn(Command::new(FILE_MANAGER).arg(folder))
}

/// Opens the file at `path` in the app the system uses for its type.
pub fn open_file(path: &Path) -> Result<(), String> {
    exists(path)?;
    spawn(Command::new(FILE_MANAGER).arg(path))
}

/// Shows the file at `path` in its folder, selected where the file manager can do that.
pub fn show_in_folder(path: &Path) -> Result<(), String> {
    exists(path)?;
    let mut command = Command::new(FILE_MANAGER);
    reveal(&mut command, path);
    spawn(&mut command)
}

/// `explorer /select,"<path>"` (written raw: the file manager reads its own quoting).
#[cfg(target_os = "windows")]
fn reveal(command: &mut Command, path: &Path) {
    use std::os::windows::process::CommandExt;
    command.raw_arg(format!("/select,\"{}\"", path.display()));
}

/// `open -R <path>`.
#[cfg(target_os = "macos")]
fn reveal(command: &mut Command, path: &Path) {
    command.arg("-R").arg(path);
}

/// The opener cannot select a file, so it opens the folder.
#[cfg(not(any(target_os = "windows", target_os = "macos")))]
fn reveal(command: &mut Command, path: &Path) {
    command.arg(path.parent().unwrap_or(path));
}

fn exists(path: &Path) -> Result<(), String> {
    match path.exists() {
        true => Ok(()),
        false => Err(format!("{} is not there", path.display())),
    }
}

fn spawn(command: &mut Command) -> Result<(), String> {
    command.spawn().map(|_| ()).map_err(|err| err.to_string())
}
