use super::*;
use crate::domain::post::PostExtras;

#[test]
fn a_title_is_one_line_and_names_one_series_in_any_case() {
    assert_eq!(series_title("  Learning\n  Rust "), Some("Learning Rust".to_string()));
    assert_eq!(series_title("   "), None);
    assert_eq!(series_title(&"x".repeat(MAX_SERIES_TITLE_CHARS + 1)), None);
    assert_eq!(series_id("Learning Rust"), series_id("learning rust"));
    assert_ne!(series_id("Learning Rust"), series_id("Learning Go"));
    assert!(series_id("Rust を学ぶ").starts_with("series_"), "any script has an id");
}

#[test]
fn goals_and_progress() {
    assert_eq!((parse_goal(" 30 "), parse_goal("0"), parse_goal("abc"), parse_goal("1001")), (Some(30), None, None, None));
    assert!((progress(12, Some(30)) - 0.4).abs() < f32::EPSILON);
    assert!((progress(40, Some(30)) - 1.0).abs() < f32::EPSILON, "never past the end");
    assert!((progress(5, None) - 1.0).abs() < f32::EPSILON, "open-ended: all posted so far");
    assert!(progress(0, None) < f32::EPSILON);
}

#[test]
fn the_entry_round_trips_and_older_rows_have_none() {
    let entry = SeriesEntry { id: series_id("Learning Rust"), title: "Learning Rust".into(), number: 12, goal: Some(30) };
    let extras = PostExtras { series: Some(Box::new(entry)), ..Default::default() };
    let json = serde_json::to_string(&extras).expect("json");
    assert!(json.contains(r#""number":12,"goal":30"#), "{json}");
    assert_eq!(serde_json::from_str::<PostExtras>(&json).expect("back"), extras);
    assert_eq!(serde_json::from_str::<PostExtras>("{}").expect("old").series, None);
    assert!(!extras.has_details());
}
