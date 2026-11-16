//! Architecture guard: enforces the "extreme modularity" size budget on every source file.
//!
//! - Production files: at most `MAX_FILE_LINES` lines.
//! - Test files (`tests.rs`, or anything under a `tests/` directory): at most `MAX_TEST_FILE_LINES`.
//! - Every `fn` body: at most `MAX_FN_LINES` lines (`MAX_TEST_FN_LINES` inside test files).
//!
//! Set `ARCH_SCOPE=src/domain` (comma-separated prefixes) to check only part of the tree
//! while a refactor is in progress.

use std::fs;
use std::path::{Path, PathBuf};

const MAX_FILE_LINES: usize = 150;
const MAX_TEST_FILE_LINES: usize = 300;
const MAX_FN_LINES: usize = 40;
const MAX_TEST_FN_LINES: usize = 80;
const SOURCE_ROOT: &str = "src";
const SCOPE_ENV: &str = "ARCH_SCOPE";

fn collect_rust_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(dir) else { return };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_rust_files(&path, out);
        } else if path.extension().is_some_and(|ext| ext == "rs") {
            out.push(path);
        }
    }
}

fn normalized(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

fn in_scope(path: &str) -> bool {
    match std::env::var(SCOPE_ENV) {
        Ok(scope) if !scope.trim().is_empty() => {
            scope.split(',').any(|prefix| path.starts_with(prefix.trim()))
        }
        _ => true,
    }
}

fn is_test_file(path: &str) -> bool {
    path.ends_with("/tests.rs") || path.contains("/tests/")
}

fn is_fn_signature(line: &str) -> bool {
    let trimmed = line.trim_start();
    let without_vis = trimmed
        .strip_prefix("pub(crate) ")
        .or_else(|| trimmed.strip_prefix("pub(super) "))
        .or_else(|| trimmed.strip_prefix("pub "))
        .unwrap_or(trimmed);
    let without_mods = without_vis
        .trim_start_matches("const ")
        .trim_start_matches("async ")
        .trim_start_matches("unsafe ");
    without_mods.starts_with("fn ")
}

/// Brace delta of a line, ignoring braces inside string and char literals and `//` comments.
fn brace_delta(line: &str) -> i64 {
    let mut delta = 0;
    let mut in_str = false;
    let mut prev = '\0';
    let chars: Vec<char> = line.chars().collect();
    for (i, &c) in chars.iter().enumerate() {
        if !in_str && c == '/' && chars.get(i + 1) == Some(&'/') {
            break;
        }
        if c == '"' && prev != '\\' {
            in_str = !in_str;
        } else if !in_str && c == '\'' && chars.get(i + 2) == Some(&'\'') {
            prev = c;
            continue;
        } else if !in_str && (c == '{' || c == '}') {
            let quoted = i > 0 && chars[i - 1] == '\'' && chars.get(i + 1) == Some(&'\'');
            if !quoted {
                delta += if c == '{' { 1 } else { -1 };
            }
        }
        prev = c;
    }
    delta
}

/// Returns (start line, body length) of every function whose body exceeds `limit`.
fn long_functions(source: &str, limit: usize) -> Vec<(usize, usize)> {
    let lines: Vec<&str> = source.lines().collect();
    let mut violations = Vec::new();
    for (start, line) in lines.iter().enumerate() {
        if !is_fn_signature(line) {
            continue;
        }
        let mut depth = 0i64;
        let mut opened = false;
        for (offset, body_line) in lines[start..].iter().enumerate() {
            if !opened && body_line.trim_end().ends_with(';') && !body_line.contains('{') {
                break; // trait method declaration without body
            }
            depth += brace_delta(body_line);
            opened |= body_line.contains('{');
            if opened && depth <= 0 {
                if offset + 1 > limit {
                    violations.push((start + 1, offset + 1));
                }
                break;
            }
        }
    }
    violations
}

#[test]
fn source_files_respect_modularity_budget() {
    let mut files = Vec::new();
    collect_rust_files(Path::new(SOURCE_ROOT), &mut files);
    files.sort();

    let mut report = Vec::new();
    for file in &files {
        let path = normalized(file);
        if !in_scope(&path) {
            continue;
        }
        let source = fs::read_to_string(file).expect("readable source file");
        let test_file = is_test_file(&path);
        let (file_limit, fn_limit) = if test_file {
            (MAX_TEST_FILE_LINES, MAX_TEST_FN_LINES)
        } else {
            (MAX_FILE_LINES, MAX_FN_LINES)
        };
        let line_count = source.lines().count();
        if line_count > file_limit {
            report.push(format!("{path}: {line_count} lines (max {file_limit})"));
        }
        for (line, len) in long_functions(&source, fn_limit) {
            report.push(format!("{path}:{line}: fn body {len} lines (max {fn_limit})"));
        }
    }

    assert!(
        report.is_empty(),
        "{} modularity violations:\n{}",
        report.len(),
        report.join("\n")
    );
}
