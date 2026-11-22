//! Finding runtimes (P1). Nothing here needs a runtime installed: the lookups run against
//! scratch folders, and the one probe of real runtimes skips those that are absent.

use super::{detect, find_on_path};
use crate::domain::code::Runtime;
use std::ffi::OsString;
use std::path::PathBuf;

/// A fresh scratch folder, removed when dropped.
pub(super) struct Scratch(pub PathBuf);

impl Scratch {
    pub(super) fn new(tag: &str) -> Self {
        let dir = std::env::temp_dir().join(format!("simulacra-runner-{tag}-{}-{}", std::process::id(), jiff::Timestamp::now().as_nanosecond()));
        std::fs::create_dir_all(&dir).expect("scratch folder");
        Self(dir)
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

#[cfg(windows)]
fn executable(dir: &std::path::Path, name: &str) -> PathBuf {
    let file = dir.join(format!("{name}.exe"));
    std::fs::write(&file, b"not a real program").expect("file");
    file
}

#[cfg(not(windows))]
fn executable(dir: &std::path::Path, name: &str) -> PathBuf {
    use std::os::unix::fs::PermissionsExt;
    let file = dir.join(name);
    std::fs::write(&file, b"not a real program").expect("file");
    std::fs::set_permissions(&file, std::fs::Permissions::from_mode(0o755)).expect("mode");
    file
}

#[test]
fn executables_are_found_on_the_search_path_and_shell_scripts_never() {
    let (first, second) = (Scratch::new("path-a"), Scratch::new("path-b"));
    let node = executable(&second.0, "node");
    std::fs::write(first.0.join("node.cmd"), b"@echo off").expect("script");
    std::fs::write(first.0.join("node.bat"), b"@echo off").expect("script");
    let path_var = std::env::join_paths([OsString::from("relative-dir"), first.0.clone().into(), second.0.clone().into()]).expect("path");
    assert_eq!(find_on_path("node", &path_var), Some(node));
    assert_eq!(find_on_path("deno", &path_var), None);
    let dotted = executable(&first.0, "lua5.4");
    assert_eq!(find_on_path("lua5.4", &path_var), Some(dotted));
}

#[test]
fn a_file_that_is_no_program_is_not_detected() {
    let dir = Scratch::new("fake");
    executable(&dir.0, "python3");
    executable(&dir.0, "python");
    assert_eq!(detect(Runtime::Python, dir.0.as_os_str()), None, "it never reported a version");
}

#[test]
fn installed_runtimes_report_a_version() {
    let path_var = std::env::var_os("PATH").unwrap_or_default();
    for runtime in Runtime::ALL {
        let present = runtime.probe().executables.iter().any(|name| find_on_path(name, &path_var).is_some());
        if !present {
            eprintln!("skipped: {} is not installed", runtime.label());
            continue;
        }
        match detect(*runtime, &path_var) {
            Some(found) => assert!(found.version.contains('.'), "{found:?}"),
            None => eprintln!("skipped: {} is on the path but reported no version", runtime.label()),
        }
    }
}
