//! Where the database lives: the per-user application-data directory, an explicit
//! `SIMULACRA_DB_PATH` override, and the one-time copy of a legacy `./simulacra.db`.
//!
//! Resolution is pure (the environment lookup and platform are injected) so it is unit
//! tested; [`prepare_db_path`] performs the directory creation and migration I/O.

use std::io;
use std::path::{Path, PathBuf};

/// Environment variable that, when set and non-blank, is used verbatim as the database path.
pub const DB_PATH_ENV: &str = "SIMULACRA_DB_PATH";
/// Application folder created inside the per-user data directory.
pub const APP_DIR_NAME: &str = "simulacra";
/// Windows per-user roaming application-data directory.
const WINDOWS_DATA_ENV: &str = "APPDATA";
/// XDG base directory for user data on Linux and other Unixes.
const XDG_DATA_ENV: &str = "XDG_DATA_HOME";
/// The user's home directory on macOS and Unix.
const HOME_ENV: &str = "HOME";
/// macOS data directory relative to `$HOME`.
const MACOS_DATA_SUBDIR: [&str; 2] = ["Library", "Application Support"];
/// XDG fallback data directory relative to `$HOME`.
const UNIX_DATA_SUBDIR: [&str; 2] = [".local", "share"];
/// SQLite write-ahead-log sidecar suffix (WAL mode keeps recent commits there).
const WAL_SUFFIX: &str = "-wal";

/// Operating-system family that decides the data-directory convention.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Platform {
    Windows,
    MacOs,
    Unix,
}

impl Platform {
    /// The platform this binary was compiled for.
    pub const fn current() -> Self {
        if cfg!(windows) {
            Self::Windows
        } else if cfg!(target_os = "macos") {
            Self::MacOs
        } else {
            Self::Unix
        }
    }
}

/// What to do about a legacy database before opening the resolved one.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Migration {
    /// The target exists, or there is no legacy file: open the target as-is.
    NotNeeded,
    /// Copy (never move) the legacy file to the new location first.
    CopyLegacy { from: PathBuf, to: PathBuf },
}

/// Non-blank value of the environment variable `key`.
fn env_value(env: &dyn Fn(&str) -> Option<String>, key: &str) -> Option<String> {
    env(key).filter(|value| !value.trim().is_empty())
}

/// The per-user data directory for `platform`, if its environment is available.
pub fn per_user_data_dir(env: &dyn Fn(&str) -> Option<String>, platform: Platform) -> Option<PathBuf> {
    let home_join = |parts: [&str; 2]| {
        env_value(env, HOME_ENV).map(|home| parts.iter().fold(PathBuf::from(home), |p, s| p.join(s)))
    };
    match platform {
        Platform::Windows => env_value(env, WINDOWS_DATA_ENV).map(PathBuf::from),
        Platform::MacOs => home_join(MACOS_DATA_SUBDIR),
        Platform::Unix => env_value(env, XDG_DATA_ENV)
            .map(PathBuf::from)
            .or_else(|| home_join(UNIX_DATA_SUBDIR)),
    }
}

/// The database path: the override, else `<data dir>/simulacra/<file_name>`, else `file_name`.
pub fn resolve_db_path(env: &dyn Fn(&str) -> Option<String>, platform: Platform, file_name: &str) -> PathBuf {
    if let Some(explicit) = env_value(env, DB_PATH_ENV) {
        return PathBuf::from(explicit);
    }
    per_user_data_dir(env, platform)
        .map(|dir| dir.join(APP_DIR_NAME).join(file_name))
        .unwrap_or_else(|| PathBuf::from(file_name))
}

/// Decides whether the `legacy` database must be copied to `target`.
pub fn plan_migration(target: &Path, legacy: &Path) -> Migration {
    if target.exists() || !legacy.is_file() {
        return Migration::NotNeeded;
    }
    Migration::CopyLegacy { from: legacy.to_path_buf(), to: target.to_path_buf() }
}

/// Copies the legacy database (and its WAL sidecar, if present) to the new location.
pub fn apply_migration(migration: &Migration) -> io::Result<()> {
    let Migration::CopyLegacy { from, to } = migration else {
        return Ok(());
    };
    if let Some(parent) = to.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::copy(from, to)?;
    let wal_from = with_suffix(from, WAL_SUFFIX);
    if wal_from.is_file() {
        std::fs::copy(&wal_from, with_suffix(to, WAL_SUFFIX))?;
    }
    Ok(())
}

/// `path` with `suffix` appended to its file name (`a.db` → `a.db-wal`).
fn with_suffix(path: &Path, suffix: &str) -> PathBuf {
    let mut raw = path.as_os_str().to_os_string();
    raw.push(suffix);
    PathBuf::from(raw)
}

/// Resolves the database path, creates its folder and copies `legacy` there once.
/// Failures are logged and leave the resolved path in place (SQLite reports the rest).
pub fn prepare_db_path(env: &dyn Fn(&str) -> Option<String>, platform: Platform, legacy: &Path) -> PathBuf {
    let file_name = legacy.file_name().and_then(|n| n.to_str()).unwrap_or_default();
    let target = resolve_db_path(env, platform, file_name);
    if env_value(env, DB_PATH_ENV).is_some() || target == legacy {
        return target;
    }
    if let Some(parent) = target.parent() {
        if let Err(e) = std::fs::create_dir_all(parent) {
            eprintln!("simulacra: cannot create data directory {}: {e}", parent.display());
        }
    }
    let migration = plan_migration(&target, legacy);
    match apply_migration(&migration) {
        Ok(()) if migration != Migration::NotNeeded => {
            eprintln!("simulacra: copied legacy database {} -> {}", legacy.display(), target.display());
        }
        Ok(()) => {}
        Err(e) => eprintln!("simulacra: legacy database copy failed: {e}"),
    }
    target
}
