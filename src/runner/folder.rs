//! The folder a run works in: made fresh for each run (never reused, private to the user on
//! Unix), the code written into it as one file, and removed with everything in it afterwards.

use std::io;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

/// Folder name prefix, under the system temp folder.
const PREFIX: &str = "simulacra-run";
/// Tries to remove the folder (Windows may still hold a file of a process just ended).
const REMOVE_TRIES: u32 = 5;
const REMOVE_PAUSE: Duration = Duration::from_millis(100);

/// Characters a code file's name may not hold: folder separators, a drive, a NUL.
const FORBIDDEN: [char; 4] = ['/', '\\', ':', '\0'];

/// Runs made by this process, for unique names.
static RUNS: AtomicU64 = AtomicU64::new(0);

/// A run's own folder, removed when dropped.
#[derive(Debug)]
pub struct RunFolder(PathBuf);

impl RunFolder {
    /// A new, empty folder; never one that already exists.
    pub fn create() -> io::Result<Self> {
        let run = RUNS.fetch_add(1, Ordering::Relaxed);
        let name = format!("{PREFIX}-{}-{}-{run}", std::process::id(), jiff::Timestamp::now().as_nanosecond());
        let dir = std::env::temp_dir().join(name);
        make_private_dir(&dir)?;
        Ok(Self(dir))
    }

    pub fn path(&self) -> &Path {
        &self.0
    }

    /// Writes `code` to the file `name` inside the folder: a plain name, no folders.
    pub fn write(&self, name: &str, code: &str) -> io::Result<PathBuf> {
        if !is_plain_name(name) {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "file name"));
        }
        let file = self.0.join(name);
        std::fs::write(&file, code)?;
        Ok(file)
    }
}

/// A file name with no folder in it (`main.py`, never `../x` or `a/b`).
pub fn is_plain_name(name: &str) -> bool {
    !name.is_empty() && name != "." && name != ".." && !name.contains(FORBIDDEN)
}

#[cfg(unix)]
fn make_private_dir(dir: &Path) -> io::Result<()> {
    use std::os::unix::fs::DirBuilderExt;
    std::fs::DirBuilder::new().mode(0o700).create(dir)
}

#[cfg(not(unix))]
fn make_private_dir(dir: &Path) -> io::Result<()> {
    std::fs::create_dir(dir)
}

impl Drop for RunFolder {
    fn drop(&mut self) {
        for _ in 0..REMOVE_TRIES {
            if std::fs::remove_dir_all(&self.0).is_ok() || !self.0.exists() {
                return;
            }
            std::thread::sleep(REMOVE_PAUSE);
        }
    }
}
