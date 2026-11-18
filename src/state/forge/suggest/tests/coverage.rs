//! Every Forge parameter has suggestions, in every context, and every text they use exists —
//! the suggestion twin of the illustration and locale coverage tests.

use super::*;
use crate::state::forge::fields::{mapped_fields, ENGINE_PATH, SYSTEM_FIELDS};
use std::fs;
use std::path::Path;

/// Parameters that only get advice: an imported file, a stored key, a server's reply shape.
const GUIDE_ONLY: [Param; 3] = [
    Param::OptionalText(crate::state::forge::OptionalTextField::AvatarPath),
    Param::Engine(EngineParam::Credential),
    Param::Engine(EngineParam::Mapping),
];
const MIN_EXAMPLES: usize = 4;
const MAX_EXAMPLES: usize = 8;
const MAX_PICKS: usize = 3;
const FORGE_UI: &str = "src/ui/features/forge";
const CAPTION_CALL: &str = "engine_field(handle, \"";

fn check(param: Param, suggestions: &FieldSuggestions) -> Result<(), String> {
    let fail = |why: &str| Err(format!("{}: {why}", param.path()));
    match suggestions {
        FieldSuggestions::Text(hints) => {
            if !(MIN_EXAMPLES..=MAX_EXAMPLES).contains(&hints.examples.len()) {
                return fail(&format!("{} examples", hints.examples.len()));
            }
            if !resolves(&hints.tip) || !hints.examples.iter().all(|ex| resolves(&ex.label) && resolves(&ex.value)) {
                return fail("a text is missing");
            }
        }
        FieldSuggestions::Range(hint) => {
            let ordered = hint.valid.0 <= hint.typical.0
                && hint.typical.0 <= hint.recommended
                && hint.recommended <= hint.typical.1
                && hint.typical.1 <= hint.valid.1;
            if !ordered || !resolves(&hint.why) {
                return fail("range out of order or reason missing");
            }
        }
        FieldSuggestions::Choice(hints) => {
            let options = choice::option_keys(param);
            if hints.picks.is_empty() || hints.picks.len() > MAX_PICKS || !hints.picks.iter().all(|pick| options.contains(pick)) {
                return fail(&format!("picks {:?}", hints.picks));
            }
            if !resolves(&hints.why) {
                return fail("reason missing");
            }
        }
        FieldSuggestions::Guide(advice) if GUIDE_ONLY.contains(&param) && resolves(advice) => {}
        FieldSuggestions::Guide(_) => return fail("only advice"),
    }
    Ok(())
}

#[test]
fn every_parameter_has_suggestions_in_every_context() {
    let failures: Vec<String> = contexts()
        .iter()
        .flat_map(|draft| {
            let ctx = SuggestContext::of(draft);
            Param::all().into_iter().filter_map(move |param| check(param, &suggestions_for(param, &ctx)).err())
        })
        .collect();
    assert!(failures.is_empty(), "{failures:#?}");
}

#[test]
fn every_registry_field_is_a_parameter() {
    let system: Vec<&str> = SYSTEM_FIELDS.iter().map(|(path, _)| *path).collect();
    let missing: Vec<&str> = mapped_fields()
        .into_iter()
        .map(|(path, _)| path)
        .filter(|path| !system.contains(path) && *path != ENGINE_PATH && Param::from_path(path).is_none())
        .collect();
    assert!(missing.is_empty(), "fields without suggestions: {missing:?}");
}

fn collect(dir: &Path, out: &mut Vec<String>) {
    for entry in fs::read_dir(dir).expect("forge ui dir").flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect(&path, out);
        } else if path.extension().is_some_and(|ext| ext == "rs") {
            out.push(fs::read_to_string(&path).unwrap_or_default());
        }
    }
}

#[test]
fn every_engine_caption_has_suggestions() {
    let mut sources = Vec::new();
    collect(Path::new(FORGE_UI), &mut sources);
    let captions: Vec<String> = sources
        .iter()
        .flat_map(|source| source.match_indices(CAPTION_CALL).map(move |(at, _)| &source[at + CAPTION_CALL.len()..]))
        .filter_map(|rest| rest.split('"').next().map(str::to_string))
        .collect();
    assert!(!captions.is_empty());
    let uncovered: Vec<&String> =
        captions.iter().filter(|path| Param::from_path(path).is_none() && !GROUP_CAPTIONS.contains(&path.as_str())).collect();
    assert!(uncovered.is_empty(), "engine captions without suggestions: {uncovered:?}");
}

#[test]
fn every_combination_reason_and_ui_text_exists() {
    let missing: Vec<String> = choice::combo_ids().into_iter().map(keys::combo_key).filter(|key| !has_key(key)).collect();
    assert!(missing.is_empty(), "{missing:?}");
    for param in Param::all() {
        for combo in choice::combos_for(param) {
            assert!(choice::option_keys(combo.mine.param).contains(&combo.mine.key), "{combo:?}");
            assert!(choice::option_keys(combo.other.param).contains(&combo.other.key), "{combo:?}");
        }
    }
}

#[test]
fn every_option_has_a_label() {
    for param in Param::all() {
        for key in choice::option_keys(param) {
            assert!(choice::option_label_key(param, key).is_some(), "{}: {key}", param.path());
        }
    }
}

#[test]
fn suggested_personality_fits_the_budget_for_every_type() {
    use crate::domain::traits::{IdentityVariant, MbtiType};
    use crate::state::forge::attributes::BUDGET_FIELDS;
    for mbti in MbtiType::ALL {
        for identity in IdentityVariant::ALL {
            let mut draft = agent();
            draft.edit(DraftEdit::Trait(TraitValue::Mbti(mbti)));
            draft.edit(DraftEdit::Trait(TraitValue::IdentityVariant(identity)));
            let ctx = SuggestContext::of(&draft);
            let total: u16 = BUDGET_FIELDS.iter().filter_map(|field| range_hint(Param::Score(*field), &ctx)).map(|hint| hint.recommended).sum();
            assert!(total <= crate::state::forge::PERSONALITY_POINT_BUDGET, "{mbti:?} {identity:?}: {total}");
        }
    }
}
