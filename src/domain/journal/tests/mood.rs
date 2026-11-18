use super::day;
use crate::domain::journal::activity::{activity_by_day, review_sessions_by_day, ActivityByDay, ActivityKind, DayActivity};
use crate::domain::journal::mood::correlate::{associate, associations, pearson, MoodMeasure, Strength, MIN_DAYS};
use crate::domain::journal::mood::summary::{by_hour, by_weekday, daily_means, MoodMean};
use crate::domain::journal::mood::{parse_tags, Affect, AffectRegion, CheckinDraft, MoodCheckin};
use crate::domain::journal::RecordId;
use chrono::{Duration, FixedOffset, TimeZone, Utc};
use std::collections::BTreeMap;

fn checkin(offset: i64, hour: u32, valence: f32, energy: u8) -> MoodCheckin {
    let zone = FixedOffset::east_opt(7 * 3600).unwrap();
    let at = zone.from_local_datetime(&day(offset).and_hms_opt(hour, 0, 0).unwrap()).unwrap();
    let draft = CheckinDraft { affect: Some(Affect::new(valence, 0.0)), energy, ..CheckinDraft::default() };
    draft.into_checkin(RecordId::new(format!("m{offset}-{hour}")), at, day(offset)).unwrap()
}

#[test]
fn affect_regions_follow_the_circumplex() {
    let region = |v, a| Affect::new(v, a).region();
    assert_eq!(region(0.9, 0.0), AffectRegion::Pleasant);
    assert_eq!(region(0.7, 0.7), AffectRegion::Excited);
    assert_eq!(region(0.0, 0.9), AffectRegion::Alert);
    assert_eq!(region(-0.7, 0.7), AffectRegion::Tense);
    assert_eq!(region(-0.9, 0.05), AffectRegion::Unpleasant);
    assert_eq!(region(-0.6, -0.6), AffectRegion::Sad);
    assert_eq!(region(0.0, -0.9), AffectRegion::Tired);
    assert_eq!(region(0.6, -0.6), AffectRegion::Calm);
    assert_eq!(region(0.1, -0.1), AffectRegion::Neutral);
    assert_eq!(region(5.0, 0.0), AffectRegion::Pleasant, "points are clamped onto the grid");
}

#[test]
fn grid_clicks_map_to_points_and_back() {
    let top_left = Affect::from_grid(0.0, 0.0);
    assert_eq!((top_left.valence, top_left.arousal), (-1.0, 1.0));
    let point = Affect::from_grid(0.75, 0.25);
    let (x, y) = point.grid_position();
    assert!((x - 0.75).abs() < 1e-6 && (y - 0.25).abs() < 1e-6);
    let (lx, ly) = AffectRegion::Alert.label_position();
    assert!((lx - 0.5).abs() < 1e-6 && ly < 0.5, "alert sits at the top");
    assert_eq!(AffectRegion::Neutral.label_position(), (0.5, 0.5));
}

#[test]
fn drafts_need_a_point_and_clean_their_fields() {
    assert!(CheckinDraft::default().into_checkin(RecordId::new("x"), Utc::now().fixed_offset(), day(0)).is_none());
    let draft = CheckinDraft { affect: Some(Affect::new(0.2, 0.2)), energy: 9, sleep_hours: Some(30.0), tags: vec!["#Work, walk".into()], note: "  ok ".into(), wants_support: true };
    let saved = draft.into_checkin(RecordId::new("x"), Utc::now().fixed_offset(), day(0)).unwrap();
    assert_eq!((saved.energy, saved.sleep_hours, saved.note.as_str(), saved.wants_support), (5, Some(24.0), "ok", true));
    assert_eq!(saved.tags, ["work", "walk"]);
    assert_eq!(parse_tags("a a, #b"), ["a", "b"]);
}

#[test]
fn summaries_by_day_weekday_and_hour() {
    let list = vec![checkin(0, 9, 0.5, 4), checkin(0, 21, -0.5, 2), checkin(7, 9, 1.0, 5)];
    let daily = daily_means(&list);
    assert_eq!(daily.get(&day(0)).map(|m| (m.valence, m.energy, m.count)), Some((0.0, 3.0, 2)));
    let weekday = chrono::Datelike::weekday(&day(0)).num_days_from_monday() as usize;
    assert_eq!(by_weekday(&list)[weekday].count, 3, "same weekday a week apart");
    let hours = by_hour(&list);
    assert_eq!((hours[9].count, hours[21].count, hours[12].count), (2, 1, 0), "local hours of the check-ins");
}

#[test]
fn pearson_and_its_edge_cases() {
    assert!((pearson(&[(1.0, 2.0), (2.0, 4.0), (3.0, 6.0)]).unwrap() - 1.0).abs() < 1e-9);
    assert!((pearson(&[(1.0, 3.0), (2.0, 2.0), (3.0, 1.0)]).unwrap() + 1.0).abs() < 1e-9);
    assert_eq!(pearson(&[(1.0, 1.0), (1.0, 2.0)]), None, "no variance");
    assert_eq!(pearson(&[(1.0, 1.0)]), None);
}

fn mood_days(days: usize, valence: impl Fn(usize) -> f32) -> BTreeMap<chrono::NaiveDate, MoodMean> {
    (0..days).map(|i| (day(i as i64), MoodMean { valence: valence(i), arousal: 0.0, energy: 3.0, count: 1 })).collect()
}

fn focus(days: usize, minutes: impl Fn(usize) -> u32) -> ActivityByDay {
    (0..days).map(|i| (day(i as i64), DayActivity { focus_minutes: minutes(i), ..DayActivity::default() })).collect()
}

#[test]
fn small_samples_give_no_figure() {
    let few = mood_days(MIN_DAYS - 1, |i| i as f32);
    let link = associate(&few, &focus(MIN_DAYS - 1, |i| i as u32), ActivityKind::FocusMinutes, MoodMeasure::Valence);
    assert_eq!((link.strength, link.r, link.days), (Strength::TooFewDays, None, MIN_DAYS - 1));
}

#[test]
fn associations_report_strength_direction_and_days() {
    let mood = mood_days(10, |i| i as f32 / 10.0);
    let strong = associate(&mood, &focus(10, |i| 10 * i as u32), ActivityKind::FocusMinutes, MoodMeasure::Valence);
    assert_eq!((strong.strength, strong.days, strong.is_positive()), (Strength::Strong, 10, true));
    let flat = associate(&mood, &focus(10, |_| 25), ActivityKind::FocusMinutes, MoodMeasure::Valence);
    assert_eq!(flat.strength, Strength::NoVariation);
    let missing = associate(&mood, &ActivityByDay::new(), ActivityKind::PostsWritten, MoodMeasure::Valence);
    assert_eq!(missing.strength, Strength::NoVariation, "days without activity count as zero");
    assert_eq!(associations(&mood, &ActivityByDay::new()).len(), ActivityKind::ALL.len() * MoodMeasure::ALL.len());
    let energy = associate(&mood, &focus(10, |i| 10 * i as u32), ActivityKind::FocusMinutes, MoodMeasure::Energy);
    assert_eq!(energy.strength, Strength::NoVariation, "constant energy has nothing to associate");
}

#[test]
fn review_sessions_split_on_long_gaps_and_days() {
    let t0 = Utc.with_ymd_and_hms(2026, 3, 2, 8, 0, 0).unwrap();
    let answers = [(day(0), t0), (day(0), t0 + Duration::minutes(5)), (day(0), t0 + Duration::minutes(50)), (day(1), t0 + Duration::days(1))];
    let sessions = review_sessions_by_day(&answers);
    assert_eq!((sessions.get(&day(0)), sessions.get(&day(1))), (Some(&2), Some(&1)));
    let merged = activity_by_day(&BTreeMap::from([(day(0), 3)]), &sessions, &BTreeMap::from([(day(2), 25)]));
    assert_eq!(merged.get(&day(0)), Some(&DayActivity { posts: 3, review_sessions: 2, focus_minutes: 0 }));
    assert_eq!(merged.get(&day(2)).map(|a| a.focus_minutes), Some(25));
}
