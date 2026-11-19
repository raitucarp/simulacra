//! Suggestions follow the rest of the draft.

use super::*;
use crate::domain::engine::{OutputKind, TextSource};
use crate::domain::traits::{BodyForm, HumanGender, MbtiType, PostFormat, RhetoricalMode};
use crate::state::forge::engine::mutate::{set_output_kind, set_text_source};
use crate::state::forge::ScoreField;

fn texts(param: Param, draft: &AgentDraft) -> Vec<String> {
    match suggestions_for(param, &SuggestContext::of(draft)) {
        FieldSuggestions::Text(hints) => hints.examples.iter().map(|ex| keys_as_text(&ex.value)).collect(),
        other => panic!("not text: {other:?}"),
    }
}

fn band(param: Param, draft: &AgentDraft) -> (u16, u16) {
    range_hint(param, &SuggestContext::of(draft)).expect("range").typical
}

fn with(edit: TraitValue) -> AgentDraft {
    let mut draft = agent();
    draft.edit(DraftEdit::Trait(edit));
    draft
}

#[test]
fn personality_bands_follow_the_type_letters() {
    let extravert = band(Param::Score(ScoreField::Extraversion), &with(TraitValue::Mbti(MbtiType::Enfp)));
    let introvert = band(Param::Score(ScoreField::Extraversion), &with(TraitValue::Mbti(MbtiType::Intp)));
    assert!(extravert.0 > introvert.1, "{extravert:?} vs {introvert:?}");
}

#[test]
fn embodiment_picks_follow_the_form() {
    let robot = with(TraitValue::Form(BodyForm::Robot));
    let ctx = SuggestContext::of(&robot);
    assert_eq!(picks_for(Param::Trait(crate::state::forge::TraitField::Material), &ctx), vec!["metal"]);
    let forms = picks_for(Param::Trait(crate::state::forge::TraitField::Form), &ctx);
    assert!(forms.iter().all(|key| BodyForm::ALL.iter().any(|form| form.key() == *key && form.kind() == BodyKind::Machine)));
}

#[test]
fn pronouns_lead_with_the_gender_or_the_body() {
    let she = texts(Param::Text(TextField::Pronouns), &with(TraitValue::Gender(HumanGender::Female)));
    assert_eq!(she[0], keys::example_key("pronouns", 2));
    let cat = texts(Param::Text(TextField::Pronouns), &with(TraitValue::Form(BodyForm::Cat)));
    assert_eq!(cat[0], keys::example_key("pronouns", 4));
    assert_eq!(she.len(), keys::KEYED_EXAMPLES);
}

#[test]
fn handles_and_initials_come_from_the_name() {
    let mut draft = agent();
    draft.edit(DraftEdit::Text(TextField::Name, "Ada Lovelace".into()));
    let handles = texts(Param::Text(TextField::Handle), &draft);
    assert!(handles.contains(&"@ada_lovelace".to_string()), "{handles:?}");
    assert!(handles.contains(&"@adalovelace".to_string()), "{handles:?}");
    assert!(texts(Param::Text(TextField::AvatarInitial), &draft).starts_with(&["A".to_string(), "AL".to_string()]));
}

#[test]
fn tritypes_start_at_the_core_with_one_digit_per_center() {
    let draft = with(TraitValue::Enneagram(crate::domain::traits::EnneagramType::Type5w4));
    for tritype in texts(Param::Text(TextField::Tritype), &draft) {
        let digits: Vec<char> = tritype.chars().collect();
        assert_eq!(digits.len(), 3);
        assert_eq!(digits[0], '5');
        assert!(digits[1..].iter().any(|d| "891".contains(*d)) && digits[1..].iter().any(|d| "234".contains(*d)), "{tritype}");
    }
}

#[test]
fn structured_answers_lower_the_temperature_band() {
    let mut draft = agent();
    let mut engine = draft.profile().config.engine.clone();
    set_text_source(&mut engine, TextSource::Own);
    draft.edit(DraftEdit::Engine(Box::new(engine.clone()), None));
    let free = band(Param::Engine(EngineParam::Temperature), &draft);
    set_output_kind(&mut engine, OutputKind::JsonSchema);
    draft.edit(DraftEdit::Engine(Box::new(engine), None));
    let structured = band(Param::Engine(EngineParam::Temperature), &draft);
    assert!(structured.1 < free.0 + free.1, "{structured:?} vs {free:?}");
    assert!(structured.0 < free.0);
}

#[test]
fn post_format_picks_follow_the_mode_and_clashes_are_found() {
    let mut draft = with(TraitValue::RhetoricalMode(RhetoricalMode::Narrative));
    let format = Param::Trait(crate::state::forge::TraitField::PostFormat);
    assert_eq!(picks_for(format, &SuggestContext::of(&draft))[0], "field_note");
    assert!(choice::active_clashes(format, draft.profile()).is_empty());
    draft.edit(DraftEdit::Trait(TraitValue::PostFormat(PostFormat::Aphorism)));
    let clashes = choice::active_clashes(format, draft.profile());
    assert_eq!(clashes.len(), 1);
    assert_eq!(clashes[0].other.key, "narrative");
}

#[test]
fn joiners_append_with_their_separator() {
    assert_eq!(Joiner::Comma.join("tea", "chess"), "tea, chess");
    assert_eq!(Joiner::Space.join("  ", "#Doubt"), "#Doubt");
    assert_eq!(Joiner::Line.join("one\n", "two"), "one\ntwo");
}

#[test]
fn units_read_as_people_expect() {
    assert_eq!(RangeUnit::Hundredths.format(75), "0.75");
    assert_eq!(RangeUnit::Hour.format(7), "07:00");
    assert_eq!(RangeUnit::Percent.format(30), "30%");
    assert_eq!(RangeUnit::PerMille.format(1500), "1.5×");
}
