use super::gaps::geometry::{label_anchor, outline, point, radar, ring};
use super::items::matches_words;
use super::state::{LearnTool, LearningUi, PalaceScreen, DEFAULT_HORIZON, GOAL_HORIZONS};
use super::{LearningData, TutorStatus};
use crate::domain::learning::{GapReport, GoalId, PalaceId, SessionId, Walk};
use crate::ui::features::mind_graph::viewport::Frame;

fn frame() -> Frame {
    Frame { x: 10.0, y: 20.0, width: 200.0, height: 200.0 }
}

#[test]
fn the_radar_starts_at_twelve_and_scales_values_along_each_axis() {
    let (centre, radius) = radar(&frame());
    assert_eq!(centre, (110.0, 120.0));
    let top = point(&frame(), 0, 1.0);
    assert!((top.0 - centre.0).abs() < 1e-3 && (top.1 - (centre.1 - radius)).abs() < 1e-3, "{top:?}");
    let half = point(&frame(), 0, 0.5);
    assert!((half.1 - (centre.1 - radius / 2.0)).abs() < 1e-3);
    assert_eq!(point(&frame(), 3, 2.0), point(&frame(), 3, 1.0), "values are clamped");
    assert_eq!(outline(&frame(), &[0.0; 6]), vec![centre; 6]);
    assert_eq!(ring(&frame(), 1.0).len(), 6);
    let label = label_anchor(&frame(), 0);
    assert!(label.1 < top.1 && label.1 > frame().y, "labels sit outside the radar, inside the frame");
}

#[test]
fn the_chooser_matches_every_word() {
    assert!(matches_words("What is Necessity in the Ethics?", "necessity ethics"));
    assert!(!matches_words("What is necessity?", "necessity mind"));
    assert!(matches_words("anything", "  "));
}

#[test]
fn the_tab_starts_on_the_tutor_with_a_month_horizon() {
    let ui = LearningUi::new();
    assert_eq!(ui.tool, LearnTool::Tutor);
    assert_eq!(ui.new_goal_days, DEFAULT_HORIZON);
    assert!(GOAL_HORIZONS.contains(&DEFAULT_HORIZON));
    assert!(ui.walk().is_none());
    let walking = LearningUi { palace: PalaceScreen::Walk(Walk::new(PalaceId("p".into()), Vec::new())), ..ui };
    assert!(walking.walk().is_some_and(Walk::is_finished));
    assert_eq!(LearnTool::ALL.map(LearnTool::key), ["tutor", "goals", "gaps", "palace"]);
}

#[test]
fn tutor_status_and_radar_lookups() {
    let id = SessionId("tutor_1".into());
    assert!(TutorStatus::Running(id.clone()).is_running(&id));
    assert_eq!(TutorStatus::Failed(id.clone(), "no key".into()).failure(&id), Some("no key"));
    assert_eq!(TutorStatus::Idle.failure(&id), None);
    let mut data = LearningData::default();
    let goal = GoalId("goal_1".into());
    data.goal_gaps.insert(goal.clone(), GapReport { entries: Vec::new(), average: [0.5; 6] });
    assert_eq!(data.radar(Some(&goal)).average, [0.5; 6]);
    assert_eq!(data.radar(None).average, [0.0; 6], "without a goal the overall radar shows");
    assert_eq!(data.radar(Some(&GoalId("other".into()))).average, [0.0; 6]);
}
