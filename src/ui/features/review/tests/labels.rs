use crate::domain::review::schedule::{ItemSchedule, Phase};
use crate::domain::review::Grade;
use crate::ui::features::review::{REVIEW_DONE_ART, REVIEW_EMPTY_ART};
use crate::ui::illustration::EmptyKey;

fn english(key: &str) -> String {
    rust_i18n::t!(key, locale = "en").to_string()
}

#[test]
fn every_label_key_has_an_english_text() {
    let mut keys: Vec<String> = Grade::ALL.iter().flat_map(|g| [format!("review.grade.{}", g.key()), format!("review.grade.{}_hint", g.key())]).collect();
    keys.extend(["question", "answer", "connections"].map(|k| format!("review.kind.{k}")));
    keys.extend(["new", "learning", "young", "mature", "suspended"].map(|k| format!("review.state.{k}")));
    keys.extend(["guess", "unsure", "sure"].map(|k| format!("review.confidence.{k}")));
    keys.extend(["choice", "recall", "order"].iter().flat_map(|k| [format!("review.quiz.{k}.title"), format!("review.quiz.{k}.description"), format!("review.quiz.{k}.done_tip")]));
    keys.extend(["exact", "close", "wrong", "compare"].map(|k| format!("review.recall.verdict.{k}")));
    keys.extend(["today", "stats", "items"].map(|k| format!("review.tab.{k}")));
    keys.extend(["front", "back"].iter().flat_map(|k| [format!("review.learn.{k}.switch"), format!("review.learn.{k}.field"), format!("review.learn.{k}.tip")]));
    keys.extend(["sentences", "paragraphs"].map(|k| format!("review.learn.{k}")));
    keys.extend(["unexpected", "direction", "label", "missing"].map(|k| format!("review.drill.mistake.{k}")));
    for key in keys {
        assert_ne!(english(&key), key, "untranslated {key}");
    }
}

#[test]
fn intervals_read_in_days_months_and_years() {
    use crate::ui::features::review::text::{interval_label, tr};
    assert_eq!(interval_label(0), tr("review.interval.soon"));
    assert!(interval_label(4).contains('4'));
    assert!(interval_label(45).contains("1.5"), "{}", interval_label(45));
    assert!(interval_label(800).contains("2.2"), "{}", interval_label(800));
}

#[test]
fn due_labels_cover_new_today_overdue_and_later() {
    use crate::ui::features::review::text::due_label;
    let today = jiff::civil::date(2026, 3, 10);
    let mut schedule = ItemSchedule::new_item(today);
    let new = due_label(&schedule, today);
    schedule.phase = Phase::Review;
    let due_today = due_label(&schedule, today);
    schedule.due = today - crate::time::days(3);
    let overdue = due_label(&schedule, today);
    assert!(overdue.contains('3'));
    assert!(new != due_today && due_today != overdue);
}

#[test]
fn the_empty_states_are_typed_illustrations() {
    assert_eq!(EmptyKey::ReviewEmpty.path(), REVIEW_EMPTY_ART);
    assert_eq!(EmptyKey::ReviewDone.path(), REVIEW_DONE_ART);
}
