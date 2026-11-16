//! Every i18n key the forge UI uses exists in English and Indonesian.

use super::logic::{guide_tip_key, GuideMoment, OccupationPreset, ScoreBand};
use super::text::{tr_opt, trait_description, trait_label};
use crate::domain::{HumanGender, TraitOption};
use crate::state::forge::fields::SYSTEM_FIELDS;
use crate::state::forge::{fields_for_step, ForgeStep};
use std::collections::HashSet;
use std::path::{Path, PathBuf};

const EN: &str = include_str!("../../../../locales/en.yml");
const ID: &str = include_str!("../../../../locales/id.yml");
const INDENT: usize = 2;
const COMMENT: char = '#';
const FORGE_SOURCE: &str = "src/ui/features/forge";
/// Test files hold deliberately missing keys.
const TEST_FILE: &str = "tests.rs";
const KEY_ROOTS: [&str; 2] = ["\"forge.", "\"portrait."];
/// Count labels live under `<base>_one` / `<base>_other` (see `crate::i18n::t_count`).
const PLURAL_SUFFIXES: [&str; 2] = ["_one", "_other"];
const EXPLAIN_PARTS: [&str; 2] = ["what", "effect"];
/// Origin fields plus the avatar: the fields this task's pages explain.
const EXPLAINED_FIELDS: [&str; 10] =
    ["name", "handle", "description", "occupation", "location_culture", "age", "gender", "pronouns", "bio", "avatar"];

/// Dotted paths of every leaf in a (two-space indented) locale YAML file.
fn leaf_keys(yaml: &str) -> HashSet<String> {
    let mut stack: Vec<String> = Vec::new();
    let mut keys = HashSet::new();
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
            keys.insert(stack.join("."));
        }
    }
    keys
}

fn rust_files(dir: &Path, out: &mut Vec<PathBuf>) {
    for entry in std::fs::read_dir(dir).into_iter().flatten().flatten() {
        let path = entry.path();
        if path.is_dir() {
            rust_files(&path, out);
        } else if path.extension().is_some_and(|ext| ext == "rs") && !path.ends_with(TEST_FILE) {
            out.push(path);
        }
    }
}

/// Every `"forge.…"` / `"portrait.…"` literal in the forge UI sources.
fn literal_keys() -> HashSet<String> {
    let mut files = Vec::new();
    rust_files(&Path::new(env!("CARGO_MANIFEST_DIR")).join(FORGE_SOURCE), &mut files);
    let mut keys = HashSet::new();
    for file in files {
        let source = std::fs::read_to_string(&file).unwrap_or_default();
        for root in KEY_ROOTS {
            for (at, _) in source.match_indices(root) {
                let rest = &source[at + 1..];
                let key: String = rest.chars().take_while(|c| c.is_ascii_lowercase() || *c == '.' || *c == '_').collect();
                keys.insert(key.trim_end_matches('.').to_string());
            }
        }
    }
    keys
}

fn generated_keys() -> Vec<String> {
    let mut keys: Vec<String> = ScoreBand::ALL.iter().map(|band| band.label_key()).collect();
    keys.extend(OccupationPreset::ALL.iter().map(|preset| preset.label_key()));
    keys.extend([GuideMoment::Start, GuideMoment::Waiting, GuideMoment::Celebrating].map(guide_tip_key));
    for field in EXPLAINED_FIELDS {
        keys.extend(EXPLAIN_PARTS.iter().map(|part| format!("forge.explain.{field}.{part}")));
    }
    keys.extend(HumanGender::ALL.iter().flat_map(|gender| [gender.label_key(), gender.description_key()]));
    keys
}

#[test]
fn forge_ui_keys_exist_in_english_and_indonesian() {
    let wanted: Vec<String> = literal_keys().into_iter().chain(generated_keys()).collect();
    assert!(wanted.len() > 50, "the source scan found the keys");
    for (locale, yaml) in [("en", EN), ("id", ID)] {
        let available = leaf_keys(yaml);
        let missing: Vec<&String> = wanted
            .iter()
            .filter(|key| !available.contains(*key) && !available.iter().any(|leaf| leaf.starts_with(&format!("{key}."))))
            .filter(|key| !PLURAL_SUFFIXES.iter().all(|suffix| available.contains(&format!("{key}{suffix}"))))
            .collect();
        assert!(missing.is_empty(), "{locale} is missing {missing:?}");
    }
}

#[test]
fn origin_fields_all_have_explainers() {
    let system: Vec<&str> = SYSTEM_FIELDS.iter().map(|(path, _)| *path).collect();
    for path in fields_for_step(ForgeStep::Origin).into_iter().filter(|path| !system.contains(path)) {
        assert!(EXPLAINED_FIELDS.contains(&path), "{path} has no explainer");
    }
}

#[test]
fn flat_trait_keys_resolve_at_runtime() {
    assert!(tr_opt(&HumanGender::NonBinary.label_key()).is_some());
    assert!(trait_description(HumanGender::Female).is_some());
    assert!(!trait_label(HumanGender::Male).is_empty());
    assert!(tr_opt("forge.ui.no_such_key").is_none());
}

#[test]
fn cadence_texts_show_the_scheduler_gap_not_a_placeholder() {
    use crate::agent::schedule::cadence_rule;
    use crate::domain::PostingCadence;
    for cadence in PostingCadence::ALL {
        let texts = [Some(trait_label(cadence)), trait_description(cadence)];
        for text in texts.into_iter().flatten() {
            assert!(!text.contains("%{"), "{cadence:?}: unfilled placeholder in {text:?}");
        }
    }
    for cadence in [PostingCadence::OnEveryNewPost, PostingCadence::Every15Minutes] {
        let minutes = cadence_rule(cadence).expect("autonomous cadence").min_interval_minutes.to_string();
        let description = trait_description(cadence).expect("described");
        assert!(description.contains(&minutes), "{cadence:?}: {description:?} lacks {minutes}");
    }
}

#[test]
fn forge_count_labels_follow_the_plural_rule() {
    use crate::i18n::plural_key_in;
    let english = |key: String, count: usize| {
        let text = crate::_rust_i18n_try_translate("en", &key).expect("translated").to_string();
        text.replace("%{count}", &count.to_string())
    };
    assert_eq!(english(plural_key_in("en", "forge.ui.save_blocked", 1), 1), "Fix 1 issue before saving.");
    assert_eq!(english(plural_key_in("en", "forge.ui.save_blocked", 3), 3), "Fix 3 issues before saving.");
    assert_eq!(english(plural_key_in("en", "forge.ui.origin.age_years", 1), 1), "1 year");
    assert_eq!(english(plural_key_in("en", "forge.ui.grimoire.chars", 0), 0), "0 characters");
}
