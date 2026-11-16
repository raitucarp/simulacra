use super::clock::{hour_at, segment};
use super::{chronotype_icon, week_summary, WeekSummary};
use crate::agent::schedule::{HourWindow, WeekPreview};
use crate::domain::agent_config::AgentConfig;
use crate::domain::{HumanChronotype, PostingCadence};
use crate::ui::icons::LucideIcon;

const CENTER: (f32, f32) = (100.0, 100.0);
const RADII: (f32, f32) = (60.0, 80.0);

#[test]
fn midnight_is_at_the_top_and_hours_run_clockwise() {
    assert_eq!(hour_at(0.1, -50.0), 0);
    assert_eq!(hour_at(50.0, 0.1), 6);
    assert_eq!(hour_at(-0.1, 50.0), 12);
    assert_eq!(hour_at(-50.0, -0.1), 18);
    assert_eq!(hour_at(-0.1, -50.0), 23);
}

#[test]
fn every_segment_lies_inside_the_ring_and_round_trips_to_its_hour() {
    for hour in 0..24 {
        let points = segment(hour, CENTER, RADII);
        assert!(!points.is_empty());
        let (mut sx, mut sy) = (0.0, 0.0);
        for (x, y) in &points {
            let distance = ((x - CENTER.0).powi(2) + (y - CENTER.1).powi(2)).sqrt();
            assert!(distance >= RADII.0 - 0.01 && distance <= RADII.1 + 0.01, "hour {hour}: {distance}");
            sx += x;
            sy += y;
        }
        let count = points.len() as f32;
        assert_eq!(hour_at(sx / count - CENTER.0, sy / count - CENTER.1), hour);
    }
}

#[test]
fn the_week_headline_follows_the_scheduler_switches() {
    let mut week = WeekPreview::default();
    week.posts[2][9] = 3;
    let active = AgentConfig { is_active: true, posting_cadence: PostingCadence::Hourly, ..AgentConfig::default() };
    assert_eq!(week_summary(&active, &week), WeekSummary::Posts(3));
    let off = AgentConfig { is_active: false, ..active.clone() };
    assert_eq!(week_summary(&off, &week), WeekSummary::Off);
    let manual = AgentConfig { posting_cadence: PostingCadence::ManualOnly, ..active };
    assert_eq!(week_summary(&manual, &week), WeekSummary::Manual);
}

#[test]
fn early_chronotypes_get_the_sun_and_late_ones_the_moon() {
    assert_eq!(chronotype_icon(HumanChronotype::EarlyBirdLark), LucideIcon::Sun);
    assert_eq!(chronotype_icon(HumanChronotype::NightOwlInsomniac), LucideIcon::Moon);
    let peak = crate::agent::schedule::peak_window(HumanChronotype::NightOwlInsomniac);
    assert_eq!(peak, HourWindow::new(22, 5));
}
