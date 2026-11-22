use super::*;

fn emojis(list: &[&str]) -> Vec<String> {
    list.iter().map(|emoji| emoji.to_string()).collect()
}

fn calm() -> PostKind {
    PostKind::feeling(&emojis(&["😌", "🌿"]), Intensity::new(2), " after the walk ").expect("valid")
}

#[test]
fn a_feeling_keeps_its_emojis_intensity_and_trimmed_note() {
    let PostKind::Feeling { emojis, intensity, note } = calm() else { panic!("a feeling") };
    assert_eq!(emojis, ["😌", "🌿"]);
    assert_eq!(intensity.level(), 2);
    assert_eq!(note, "after the walk");
    assert!(calm().is_feeling() && !calm().is_standard());
    assert_eq!(calm().key(), "feeling");
}

#[test]
fn feelings_need_one_to_three_distinct_emojis_and_a_short_note() {
    let none = PostKind::feeling(&emojis(&[" ", ""]), Intensity::default(), "");
    assert_eq!(none, Err(FeelingProblem::NoEmoji));
    let repeated = PostKind::feeling(&emojis(&["🙂", "🙂", "🙂", "🙂"]), Intensity::default(), "").expect("repeats collapse");
    assert!(matches!(repeated, PostKind::Feeling { ref emojis, .. } if emojis.len() == 1));
    assert_eq!(PostKind::feeling(&emojis(&["🙂", "😢", "😡", "😴"]), Intensity::default(), ""), Err(FeelingProblem::TooManyEmojis));
    let long = "a".repeat(MAX_FEELING_NOTE_CHARS + 1);
    assert_eq!(PostKind::feeling(&emojis(&["🙂"]), Intensity::default(), &long), Err(FeelingProblem::NoteTooLong));
    assert_eq!(FeelingProblem::NoteTooLong.key(), "note_too_long");
}

#[test]
fn intensity_stays_on_its_five_step_scale() {
    assert_eq!(Intensity::new(0), Intensity::MIN);
    assert_eq!(Intensity::new(9), Intensity::MAX);
    assert_eq!(Intensity::default().level(), 3);
    assert_eq!(Intensity::ALL.map(Intensity::level), [1, 2, 3, 4, 5]);
}

#[test]
fn the_stored_key_and_payload_read_back() {
    for kind in [PostKind::Standard, PostKind::Question, calm()] {
        assert_eq!(PostKind::from_stored(kind.key(), &kind.payload_json()), kind);
    }
    assert_eq!(PostKind::Standard.payload_json(), "");
    assert_eq!(PostKind::from_stored("unknown", ""), PostKind::Standard);
    assert!(matches!(PostKind::from_stored("feeling", "not json"), PostKind::Feeling { ref emojis, .. } if emojis.is_empty()));
}

#[test]
fn a_post_json_reads_old_keys_and_new_feelings() {
    let old: PostKind = serde_json::from_str("\"question\"").expect("old form");
    assert_eq!(old, PostKind::Question);
    let json = serde_json::to_string(&calm()).expect("json");
    assert!(json.starts_with("{\"feeling\""), "{json}");
    assert_eq!(serde_json::from_str::<PostKind>(&json).expect("back"), calm());
    assert_eq!(serde_json::to_string(&PostKind::Question).expect("json"), "\"question\"");
}

#[test]
fn a_feeling_reads_as_its_emojis_and_note() {
    assert_eq!(calm().feeling_text().as_deref(), Some("😌🌿 after the walk"));
    let bare = PostKind::feeling(&emojis(&["😴"]), Intensity::MAX, "").expect("valid");
    assert_eq!(bare.feeling_text().as_deref(), Some("😴"));
    assert_eq!(PostKind::Question.feeling_text(), None);
}
