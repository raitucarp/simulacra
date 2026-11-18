//! Tests of the Beliefs tab: sparkline geometry and wording.

use super::render::text::{change_text, reason_text, source_text};
use super::spark::spark_points;
use crate::domain::beliefs::{BeliefId, BeliefOrigin, BeliefSource, Credence, Revision, RevisionCause};
use crate::domain::ProfileId;

fn revision(old: Option<u8>, new: Option<u8>, cause: RevisionCause, reason: &str) -> Revision {
    Revision {
        belief: BeliefId::new("b"),
        agent: ProfileId::new("a"),
        proposition: "P".into(),
        old: old.map(|p| Credence::from_percent(p.into())),
        new: new.map(|p| Credence::from_percent(p.into())),
        cause,
        reason: reason.into(),
        source: BeliefSource::new(BeliefOrigin::Debate, "d", "Cars"),
        at: chrono::Utc::now(),
    }
}

#[test]
fn sparkline_points_span_the_box_on_a_fixed_scale() {
    let points = spark_points(&[0.0, 1.0, 0.5], 100.0, 20.0, 2.0);
    assert_eq!(points, vec![(2.0, 18.0), (50.0, 2.0), (98.0, 10.0)]);
    assert_eq!(spark_points(&[0.25], 100.0, 20.0, 2.0), vec![(50.0, 14.0)]);
    assert!(spark_points(&[], 100.0, 20.0, 2.0).is_empty());
}

#[test]
fn revisions_read_as_changes_with_reasons() {
    assert_eq!(change_text(&revision(Some(70), Some(40), RevisionCause::Evidence, "x")), "0.70 → 0.40");
    assert_eq!(reason_text(&revision(Some(70), Some(40), RevisionCause::Evidence, "Data")), "Data");
    let displaced = revision(Some(70), None, RevisionCause::Displaced, "");
    assert!(!reason_text(&displaced).contains("beliefs."), "the cause is translated");
    assert!(change_text(&displaced).contains("0.70"));
    let source = source_text(&BeliefSource::new(BeliefOrigin::Debate, "d", "Cars"));
    assert!(source.ends_with(": Cars") && !source.contains("beliefs."));
    for origin in BeliefOrigin::ALL {
        assert!(!source_text(&BeliefSource::new(*origin, "", "")).contains("beliefs."));
    }
}
