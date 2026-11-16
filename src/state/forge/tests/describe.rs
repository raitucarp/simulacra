use super::*;
use crate::domain::{EnneagramType, HumorStyle, MbtiType};
use crate::state::forge::describe::{describable_traits, DESCRIBABLE_TEXT};

/// A typical well-behaved reply wrapped in a markdown fence with chatter around it.
const FENCED_REPLY: &str = "Sure! Here is the sheet:\n```json\n{\n  \"name\": \"Mira Halden\",\n  \"occupation\": \"Marine biologist\",\n  \"age\": 41,\n  \"openness\": 88,\n  \"extraversion\": \"35\",\n  \"mbti\": \"INTP\",\n  \"enneagram\": \"5w6\",\n  \"humor_style\": \"dry deadpan wit\"\n}\n```\nLet me know if you want changes.";

/// A sloppy reply: unknown keys, invalid enum values, wrong types and out-of-range numbers.
const SLOPPY_REPLY: &str = r#"{"name": "Kai", "favorite_color": "teal", "mbti": "XYZW", "temperament": 7, "openness": 140, "age": -3, "bio": "", "assembly_strategy": "custom_template", "pronouns": null}"#;

#[test]
fn prompt_lists_every_allowed_key_and_option() {
    let prompt = describe_prompt("  A cheerful marine biologist from Makassar.  ");
    assert!(prompt.contains("A cheerful marine biologist from Makassar."));
    for field in DESCRIBABLE_TEXT {
        assert!(prompt.contains(field.path()), "{}", field.path());
    }
    for field in describable_traits() {
        let line = prompt.lines().find(|l| l.starts_with(&format!("- {}:", field.path()))).expect("trait line");
        for key in field.option_keys() {
            assert!(line.contains(key), "{} lacks {key}", field.path());
        }
    }
    assert!(prompt.contains("openness") && prompt.contains("age") && prompt.contains("100"));
    assert!(!prompt.contains("assembly_strategy"), "Grimoire fields are not described");
    assert!(!prompt.contains("prompt_template"));
}

#[test]
fn fenced_replies_parse_into_typed_edits() {
    let patch = parse_described(FENCED_REPLY).expect("parse");
    assert!(patch.warnings.is_empty(), "{:?}", patch.warnings);
    let edits = &patch.edits;
    assert!(edits.contains(&DraftEdit::Text(TextField::Name, "Mira Halden".into())));
    assert!(edits.contains(&DraftEdit::Count(CountField::Age, 41)));
    assert!(edits.contains(&DraftEdit::Score(ScoreField::Extraversion, 35)), "numeric strings are accepted");
    assert!(edits.contains(&DraftEdit::Trait(TraitValue::Mbti(MbtiType::Intp))), "case-insensitive keys");
    assert!(edits.contains(&DraftEdit::Trait(TraitValue::Enneagram(EnneagramType::Type5w6))), "label shorthand");
    assert!(edits.contains(&DraftEdit::Trait(TraitValue::HumorStyle(HumorStyle::DryDeadpanWit))), "spaced key");
}

#[test]
fn sloppy_replies_keep_the_good_parts_and_warn_about_the_rest() {
    let patch = parse_described(SLOPPY_REPLY).expect("parse");
    assert!(patch.edits.contains(&DraftEdit::Text(TextField::Name, "Kai".into())));
    assert!(patch.edits.contains(&DraftEdit::Score(ScoreField::Openness, 100)));
    assert!(patch.edits.contains(&DraftEdit::Count(CountField::Age, 0)));
    assert!(!patch.edits.iter().any(|e| e.path() == "bio"), "empty text is ignored");
    let w = &patch.warnings;
    assert!(w.contains(&PatchWarning::UnknownField("favorite_color".into())));
    assert!(w.contains(&PatchWarning::UnknownField("assembly_strategy".into())), "Grimoire is off limits");
    assert!(w.contains(&PatchWarning::InvalidOption { field: TraitField::Mbti, value: "XYZW".into() }));
    assert!(w.contains(&PatchWarning::WrongType("temperament".into())));
    assert!(w.iter().any(|x| matches!(x, PatchWarning::OutOfRange { field, .. } if field == "openness")));
    assert!(w.iter().any(|x| matches!(x, PatchWarning::OutOfRange { field, .. } if field == "age")));
    assert_eq!(w.len(), 6);
    assert_eq!(w[0].message_key(), format!("forge.describe.{}", w[0].key()));
}

#[test]
fn unreadable_replies_are_errors() {
    assert_eq!(parse_described("I could not do that."), Err(ParseError::NoJsonObject));
    assert_eq!(parse_described("} backwards {"), Err(ParseError::NoJsonObject));
    let malformed = parse_described("```json\n{\"name\": \"Kai\",}\n```").unwrap_err();
    assert!(matches!(malformed, ParseError::Malformed(_)));
    assert_eq!(malformed.message_key(), "forge.describe.error_malformed");
    assert!(matches!(parse_described("[1, 2]"), Err(ParseError::NoJsonObject)));
}

#[test]
fn applying_a_patch_is_one_undo_step() {
    let mut draft = agent();
    let patch = parse_described(FENCED_REPLY).expect("parse");
    assert!(draft.apply_patch(&patch));
    assert_eq!(draft.profile().name, "Mira Halden");
    assert_eq!(draft.profile().config.mbti, MbtiType::Intp);
    assert!(draft.undo());
    assert!(!draft.is_dirty());
}
