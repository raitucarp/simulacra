//! Design-token guard: UI code takes every visual size and color from `ThemeTokens`
//! (AGENTS.md rule 1), never from raw literals or Tailwind-style rem utilities.
//!
//! Scans `src/ui/**` (test files excluded) and fails on:
//! - `px(` / `rems(` / `relative(` called with a numeric literal;
//! - raw color literals: `rgb(0x..)`, `rgba(0x..)`, `hsla(<number>..)`;
//! - Tailwind-style size utilities such as `.text_xs()`, `.p_2()`, `.gap_1p5()`,
//!   `.rounded_md()`, `.w_4()`, `.size_3()` or `.border_1()` (zero steps like `.min_w_0()` and
//!   layout utilities like `.w_full()` are fine).
//!
//! Computed geometry (`px(x * zoom)`) and `Pixels::ZERO` are allowed.

use std::fs;
use std::path::{Path, PathBuf};

const UI_ROOT: &str = "src/ui";
/// Trees still being cleaned on another branch (none left).
const PENDING_CLEANUP: &[&str] = &[];
/// Calls that must never receive a numeric literal.
const LITERAL_CALLS: &[&str] = &["px(", "rems(", "relative(", "hsla("];
/// Calls that must never receive a hex color literal.
const HEX_CALLS: &[&str] = &["rgb(0x", "rgba(0x"];
/// Tailwind text sizes, corner sizes and corner sides.
const TEXT_SIZES: &[&str] = &["xs", "sm", "base", "lg", "xl", "2xl", "3xl"];
const CORNER_SIZES: &[&str] = &["xs", "sm", "md", "lg", "xl", "2xl", "3xl", "full"];
const CORNER_SIDES: &[&str] = &["t", "b", "l", "r", "tl", "tr", "bl", "br"];
/// Utility prefixes followed by a rem step (`_1`, `_2p5`, ...).
const STEP_PREFIXES: &[&str] = &[
    "p", "px", "py", "pt", "pb", "pl", "pr", "m", "mx", "my", "mt", "mb", "ml", "mr", "gap",
    "gap_x", "gap_y", "w", "h", "size", "min_w", "min_h", "max_w", "max_h", "border", "border_t",
    "border_b", "border_l", "border_r", "border_x", "border_y", "top", "left", "right", "bottom",
    "inset",
];

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

fn is_checked(path: &str) -> bool {
    let test_file = path.ends_with("/tests.rs") || path.contains("/tests/");
    !test_file && !PENDING_CLEANUP.iter().any(|prefix| path.starts_with(prefix))
}

/// `2`, `1p5`, `0p5` and `12` are rem steps; a bare `0` is not a magic number.
fn is_nonzero_step(step: &str) -> bool {
    let digits = step.strip_suffix("p5").unwrap_or(step);
    !digits.is_empty() && digits.bytes().all(|b| b.is_ascii_digit()) && (digits != "0" || step != digits)
}

/// Whether a zero-argument method name is a Tailwind-style size utility.
fn is_size_utility(name: &str) -> bool {
    if let Some(size) = name.strip_prefix("text_") {
        return TEXT_SIZES.contains(&size);
    }
    if let Some(rest) = name.strip_prefix("rounded_") {
        let size = match rest.split_once('_') {
            Some((side, size)) if CORNER_SIDES.contains(&side) => size,
            _ => rest,
        };
        return CORNER_SIZES.contains(&size);
    }
    name.rsplit_once('_')
        .is_some_and(|(prefix, step)| STEP_PREFIXES.contains(&prefix) && is_nonzero_step(step))
}

/// Code part of a line: everything before a `//` comment.
fn code_of(line: &str) -> &str {
    line.find("//").map_or(line, |at| &line[..at])
}

fn starts_with_number(rest: &str) -> bool {
    let rest = rest.trim_start().trim_start_matches('-');
    rest.starts_with(|c: char| c.is_ascii_digit()) || rest.starts_with('.')
}

/// Every offending snippet in one line of code.
fn line_violations(code: &str) -> Vec<String> {
    let mut found = Vec::new();
    for call in LITERAL_CALLS {
        for (at, _) in code.match_indices(call) {
            let ident_before = code[..at].ends_with(|c: char| c.is_alphanumeric() || c == '_');
            if !ident_before && starts_with_number(&code[at + call.len()..]) {
                found.push(format!("{call}<literal>"));
            }
        }
    }
    found.extend(HEX_CALLS.iter().filter(|call| code.contains(*call)).map(|call| call.to_string()));
    for (at, _) in code.match_indices("()") {
        let head = &code[..at];
        let start = head.rfind(|c: char| !(c.is_alphanumeric() || c == '_')).map_or(0, |i| i + 1);
        if head[..start].ends_with('.') && is_size_utility(&head[start..]) {
            found.push(format!(".{}()", &head[start..]));
        }
    }
    found
}

#[test]
fn ui_uses_theme_tokens_not_literals() {
    let mut files = Vec::new();
    collect_rust_files(Path::new(UI_ROOT), &mut files);
    files.sort();
    let mut report = Vec::new();
    for file in &files {
        let path = file.to_string_lossy().replace('\\', "/");
        if !is_checked(&path) {
            continue;
        }
        let source = fs::read_to_string(file).expect("readable source file");
        for (index, line) in source.lines().enumerate() {
            for snippet in line_violations(code_of(line)) {
                report.push(format!("{path}:{}: {snippet}", index + 1));
            }
        }
    }
    assert!(
        report.is_empty(),
        "{} design-token violations (use `theme.<token>` instead):\n{}",
        report.len(),
        report.join("\n")
    );
}

#[test]
fn guard_flags_literals_and_utilities() {
    let bad = [
        ".w(px(48.0))", ".h(px( 3.))", "px(-1.0)", ".line_height(rems(1.2))", "rgb(0x101010)",
        "rgba(0xffffff0a)", "hsla(0.5, 0.2, 0.1, 1.0)", ".text_xs()", ".p_2()", ".px_2p5()",
        ".gap_0p5()", ".rounded_md()", ".rounded_t_lg()", ".w_4()", ".size_3()", ".border_1()",
        ".border_b_1()", ".min_w_6()", ".gap_x_2()",
    ];
    for code in bad {
        assert!(!line_violations(code).is_empty(), "missed {code}");
    }
    let good = [
        ".w(theme.forge_rail_width)", "px(x * zoom)", "px(r)", ".min_w_0()", ".inset_0()",
        ".w_full()", ".size_full()", ".flex_1()", ".px(theme.spacing_sm)", ".text_size(theme.font_size_xs)",
        ".rounded(theme.radius_full)", "Pixels::ZERO", ".border(theme.border_width_hairline)",
    ];
    for code in good {
        assert!(line_violations(code).is_empty(), "false positive on {code}: {:?}", line_violations(code));
    }
}
