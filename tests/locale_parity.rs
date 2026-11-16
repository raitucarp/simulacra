//! Locale parity guard: every shipped locale has exactly the keys of `en.yml`, and every
//! translation keeps the same `%{placeholder}` and `{{placeholder}}` names as English.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

const LOCALE_DIR: &str = "locales";
const SOURCE_LOCALE: &str = "en";
const LOCALES: &[&str] = &["en", "id", "de", "fr", "es", "pt", "ru", "ar", "ja", "zh-CN"];
const INDENT: usize = 2;
const COMMENT: char = '#';
const PLACEHOLDER_MARKERS: &[(&str, &str)] = &[("%{", "}"), ("{{", "}}")];

/// Dotted path -> raw value of every leaf in a (simple, two-space indented) locale file.
fn leaves(yaml: &str) -> BTreeMap<String, String> {
    let mut stack: Vec<String> = Vec::new();
    let mut out = BTreeMap::new();
    for line in yaml.lines() {
        let trimmed = line.trim_start();
        if trimmed.is_empty() || trimmed.starts_with(COMMENT) {
            continue;
        }
        let depth = (line.len() - trimmed.len()) / INDENT;
        let Some((key, value)) = trimmed.split_once(':') else { continue };
        stack.truncate(depth);
        stack.push(key.trim().to_string());
        if !value.trim().is_empty() {
            out.insert(stack.join("."), value.trim().to_string());
        }
    }
    out
}

fn placeholders(value: &str) -> BTreeSet<String> {
    let mut found = BTreeSet::new();
    for (open, close) in PLACEHOLDER_MARKERS {
        let mut rest = value;
        while let Some(start) = rest.find(open) {
            let after = &rest[start + open.len()..];
            let Some(end) = after.find(close) else { break };
            found.insert(format!("{open}{}{close}", &after[..end]));
            rest = &after[end + close.len()..];
        }
    }
    found
}

fn load(locale: &str) -> BTreeMap<String, String> {
    let path = Path::new(LOCALE_DIR).join(format!("{locale}.yml"));
    leaves(&fs::read_to_string(&path).unwrap_or_else(|_| panic!("missing {}", path.display())))
}

fn compare(locale: &str, source: &BTreeMap<String, String>, report: &mut Vec<String>) {
    let target = load(locale);
    for key in source.keys().filter(|k| !target.contains_key(*k)) {
        report.push(format!("{locale}: missing {key}"));
    }
    for key in target.keys().filter(|k| !source.contains_key(*k)) {
        report.push(format!("{locale}: extra {key}"));
    }
    for (key, value) in &target {
        let Some(english) = source.get(key) else { continue };
        if placeholders(english) != placeholders(value) {
            report.push(format!("{locale}: placeholders differ in {key}"));
        }
    }
}

#[test]
fn every_locale_matches_english_keys_and_placeholders() {
    let source = load(SOURCE_LOCALE);
    let mut report = Vec::new();
    for locale in LOCALES.iter().filter(|l| **l != SOURCE_LOCALE) {
        compare(locale, &source, &mut report);
    }
    assert!(report.is_empty(), "{} locale problems:\n{}", report.len(), report.join("\n"));
}

#[test]
fn every_locale_file_is_listed() {
    let shipped: BTreeSet<String> = fs::read_dir(LOCALE_DIR)
        .expect("locale dir")
        .flatten()
        .filter_map(|e| e.path().file_stem().map(|s| s.to_string_lossy().into_owned()))
        .collect();
    let listed: BTreeSet<String> = LOCALES.iter().map(|l| l.to_string()).collect();
    assert_eq!(shipped, listed);
}
