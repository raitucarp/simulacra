//! The checks before anything starts (P7): only a runtime's own executable runs, and a cache
//! kept between runs must be the user's own.

use super::Scratch;
use crate::domain::code::{DetectedRuntime, Runtime};
use crate::runner::{is_runtime_executable, private_cache, spec_for, Limits};
use std::path::Path;
use std::time::Duration;

fn found(runtime: Runtime, path: &Path) -> DetectedRuntime {
    DetectedRuntime { runtime, path: path.to_string_lossy().into_owned(), version: "1.0.0".into() }
}

#[test]
fn only_an_executable_named_like_its_runtime_runs() {
    let dir = Scratch::new("trust");
    let python = super::executable(&dir.0, "python3");
    let lua = super::executable(&dir.0, "lua5.4");
    let other = super::executable(&dir.0, "calc");
    assert!(is_runtime_executable(&found(Runtime::Python, &python)));
    assert!(is_runtime_executable(&found(Runtime::Lua, &lua)));
    assert!(!is_runtime_executable(&found(Runtime::Python, &other)), "a cache edited to name another program");
    assert!(!is_runtime_executable(&found(Runtime::Node, &python)), "another runtime's executable");
    assert!(!is_runtime_executable(&found(Runtime::Python, Path::new("python3"))), "a relative path");
    assert!(!is_runtime_executable(&found(Runtime::Python, &dir.0.join("python"))), "a file that is gone");
    let script = dir.0.join("python.cmd");
    std::fs::write(&script, b"@echo off").expect("script");
    assert!(!is_runtime_executable(&found(Runtime::Python, &script)));
    let limits = Limits { timeout: Duration::from_secs(1), output_cap: 1024 };
    assert!(spec_for(&found(Runtime::Python, &other), "print(1)", limits).is_err(), "refused before anything starts");
    assert!(spec_for(&found(Runtime::Python, &python), "print(1)", limits).is_ok());
}

#[test]
fn a_cache_folder_is_made_and_kept_private() {
    let dir = Scratch::new("cache");
    let cache = dir.0.join("deno");
    assert_eq!(private_cache(&cache), Some(cache.clone()));
    assert!(cache.is_dir());
    let file = dir.0.join("not-a-folder");
    std::fs::write(&file, b"x").expect("file");
    assert_eq!(private_cache(&file), None);
}
