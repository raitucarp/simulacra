use super::*;
use crate::domain::post::PostExtras;

#[test]
fn a_label_is_one_trimmed_line_within_the_limit() {
    assert_eq!(BranchPoint::named("  What if\n we used   a queue? ").map(|b| b.label), Some("What if we used a queue?".to_string()));
    assert_eq!(BranchPoint::named("   "), None);
    assert_eq!(BranchPoint::named(&"x".repeat(MAX_BRANCH_LABEL_CHARS + 1)), None);
    assert!(BranchPoint::named(&"é".repeat(MAX_BRANCH_LABEL_CHARS)).is_some(), "counted in characters");
}

#[test]
fn the_pointer_round_trips_and_is_absent_from_older_extras() {
    let extras = PostExtras { branch: BranchPoint::named("What if not?"), ..Default::default() };
    let json = serde_json::to_string(&extras).expect("json");
    assert_eq!(json, r#"{"branch":{"label":"What if not?"}}"#);
    assert_eq!(serde_json::from_str::<PostExtras>(&json).expect("back"), extras);
    assert_eq!(serde_json::from_str::<PostExtras>("{}").expect("old").branch, None);
    assert!(!extras.has_details(), "a branch pointer alone is nothing to show behind the post");
}
