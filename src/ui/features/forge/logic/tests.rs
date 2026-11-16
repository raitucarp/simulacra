use super::*;
use crate::domain::portrait::{suggested_outfit, PortraitSlot, PortraitSpec};
use crate::state::forge::{validate, AgentDraft, ForgeKind, ForgeStep, ScoreField};
use crate::ui::illustration::GuidePose;

/// The forge root's key context.
const FORGE_CONTEXT: &str = crate::ui::features::forge::view::FORGE_KEY_CONTEXT;

const NETWORK: &str = "net_test";

fn blank(kind: ForgeKind) -> AgentDraft {
    AgentDraft::new(kind, NETWORK)
}

#[test]
fn rail_lists_the_kind_steps_and_flags_errors_on_visited_steps() {
    let draft = blank(ForgeKind::Agent);
    let issues = validate(&draft, &[]);
    let items = rail_items(&draft, &issues);
    assert_eq!(items.len(), ForgeStep::ALL.len());
    assert_eq!(items[0].number, 1);
    assert!(items[0].is_current);
    assert!(matches!(items[0].status, RailStatus::Issues(n) if n > 0), "blank name is an error");
    assert!(items[1..].iter().all(|item| item.status == RailStatus::Pending && !item.is_current));
}

#[test]
fn rail_marks_finished_steps_complete() {
    let mut draft = crate::state::forge::roll_random(7, ForgeKind::Persona, NETWORK, &[]);
    assert!(draft.advance());
    let issues = validate(&draft, &[]);
    draft.sync_completion(&issues);
    let items = rail_items(&draft, &issues);
    assert_eq!(items.len(), 4, "personas walk four steps");
    assert_eq!(items[0].status, RailStatus::Complete);
    assert!(items[1].is_current);
}

#[test]
fn save_bar_follows_dirty_state_and_errors() {
    let draft = blank(ForgeKind::Agent);
    let issues = validate(&draft, &[]);
    assert_eq!(save_bar_state(false, &issues), SaveBarState::Hidden);
    assert!(matches!(save_bar_state(true, &issues), SaveBarState::Blocked { errors } if errors > 0));
    assert_eq!(save_bar_state(true, &[]), SaveBarState::Ready);
    assert!(save_bar_state(true, &[]).can_save());
    assert!(!SaveBarState::Hidden.is_visible());
}

#[test]
fn autosave_waits_for_enough_edits_or_a_step_change() {
    let mut policy = AutosavePolicy::default();
    assert!(!policy.on_step_change(), "nothing to save yet");
    for _ in 1..AUTOSAVE_EVERY_EDITS {
        assert!(!policy.record_edit());
    }
    assert!(policy.on_step_change());
    assert!(policy.record_edit(), "the Nth edit triggers a save");
    policy.mark_saved();
    assert!(!policy.has_pending());
    assert!(!policy.on_step_change());
}

#[test]
fn guide_pose_follows_the_moment() {
    let step = |step, fresh| GuideMoment::Step { step, fresh };
    assert_eq!(guide_pose(GuideMoment::Waiting), GuidePose::Think);
    assert_eq!(guide_pose(GuideMoment::Celebrating), GuidePose::Cheer);
    assert_eq!(guide_pose(GuideMoment::Start), GuidePose::Point);
    assert_eq!(guide_pose(step(ForgeStep::Origin, true)), GuidePose::Point);
    assert_eq!(guide_pose(step(ForgeStep::Origin, false)), GuidePose::Idle);
    assert_eq!(guide_pose(step(ForgeStep::Review, true)), GuidePose::Read);
    assert_eq!(guide_pose(step(ForgeStep::Grimoire, false)), GuidePose::Read);
    assert_eq!(guide_tip_key(step(ForgeStep::Voice, false)), ForgeStep::Voice.guide_tip_key());
    assert_ne!(guide_tip_key(GuideMoment::Start), guide_tip_key(GuideMoment::Waiting));
}

#[test]
fn grid_cursor_wraps_sideways_and_stops_vertically() {
    let cursor = GridCursor::new(0, 7, 3);
    assert_eq!(cursor.moved(NavKey::Left).index, 6);
    assert_eq!(cursor.moved(NavKey::Right).index, 1);
    assert_eq!(cursor.moved(NavKey::Up).index, 0);
    assert_eq!(cursor.moved(NavKey::Down).index, 3);
    assert_eq!(GridCursor::new(5, 7, 3).moved(NavKey::Down).index, 5, "no card below");
    assert_eq!(GridCursor::new(6, 7, 3).moved(NavKey::Right).index, 0);
    assert_eq!(cursor.moved(NavKey::End).index, 6);
    assert_eq!(GridCursor::new(9, 7, 3).index, 6, "clamped into the grid");
    assert_eq!(GridCursor::new(0, 0, 3).moved(NavKey::Right).index, 0);
    assert_eq!(NavKey::from_key("down"), Some(NavKey::Down));
    assert_eq!(NavKey::from_key("q"), None);
}

#[test]
fn portrait_slots_round_trip_by_index() {
    let spec = PortraitSpec::random(3);
    for slot in PortraitSlot::ALL {
        for index in 0..slot_len(slot) {
            let edited = with_slot_index(spec, slot, index);
            assert_eq!(slot_index(&edited, slot), index, "{slot:?} #{index}");
        }
        let past_end = with_slot_index(spec, slot, slot_len(slot) + 4);
        assert_eq!(slot_index(&past_end, slot), slot_len(slot) - 1);
    }
}

#[test]
fn rerolling_a_slot_changes_only_that_slot() {
    let spec = PortraitSpec::random(11);
    for (seed, slot) in PortraitSlot::ALL.into_iter().enumerate() {
        let rolled = randomized_slot(spec, slot, seed as u64 + 100);
        assert_ne!(slot_index(&rolled, slot), slot_index(&spec, slot), "{slot:?} changed");
        for other in PortraitSlot::ALL.into_iter().filter(|s| *s != slot) {
            assert_eq!(slot_index(&rolled, other), slot_index(&spec, other));
        }
    }
}

#[test]
fn score_bands_and_levels() {
    assert_eq!(ScoreBand::from_score(0), ScoreBand::VeryLow);
    assert_eq!(ScoreBand::from_score(BAND_THRESHOLDS[2]), ScoreBand::Balanced);
    assert_eq!(ScoreBand::from_score(100), ScoreBand::VeryHigh);
    assert_eq!(score_level_key(ScoreField::Openness, 90), ScoreBand::VeryHigh.label_key());
    let level = crate::state::forge::QuotientLevel::from_score(90).label_key();
    assert_eq!(score_level_key(ScoreField::FluidIntelligence, 90), level);
}

#[test]
fn occupation_presets_dress_like_the_outfit_hints() {
    for preset in OccupationPreset::ALL {
        assert_eq!(suggested_outfit(preset.key()), Some(preset.outfit()), "{preset:?}");
    }
}

#[test]
fn time_seeds_differ() {
    assert_ne!(time_seed(), time_seed());
}

#[test]
fn slider_keys_move_by_notches_and_stop_at_the_ends() {
    let range = (0, 100, 1);
    assert_eq!(slider_key_value(50, "right", range), Some(51));
    assert_eq!(slider_key_value(50, "down", range), Some(49));
    assert_eq!(slider_key_value(50, "pageup", range), Some(50 + PAGE_NOTCHES));
    assert_eq!(slider_key_value(5, "pagedown", range), Some(0));
    assert_eq!(slider_key_value(50, "home", range), Some(0));
    assert_eq!(slider_key_value(50, "end", range), Some(100));
    assert_eq!(slider_key_value(100, "up", range), None, "already at the top");
    assert_eq!(slider_key_value(50, "a", range), None);
    assert_eq!(slider_key_value(60, "end", (0, 60, 1)), None, "a budget ceiling caps the end");
}

#[test]
fn hour_keys_move_start_or_end_wrapping_midnight() {
    use crate::state::forge::HourField::{ActiveHoursEnd, ActiveHoursStart};
    assert_eq!(hour_key_change("left", false, (0, 8)), Some((ActiveHoursStart, 23)));
    assert_eq!(hour_key_change("right", false, (9, 17)), Some((ActiveHoursStart, 10)));
    assert_eq!(hour_key_change("right", true, (9, 23)), Some((ActiveHoursEnd, 0)));
    assert_eq!(hour_key_change("up", false, (9, 17)), Some((ActiveHoursEnd, 18)));
    assert_eq!(hour_key_change("down", false, (9, 17)), Some((ActiveHoursEnd, 16)));
    assert_eq!(hour_key_change("enter", false, (9, 17)), None);
}

#[test]
fn every_step_gets_its_own_options_scroll_id() {
    let ids: Vec<_> = ForgeStep::ALL.iter().map(|step| options_scroll_id(*step)).collect();
    for (at, id) in ids.iter().enumerate() {
        assert!(ids[at + 1..].iter().all(|other| other != id), "{id} is shared, so scroll would carry over");
    }
    assert_eq!(options_scroll_id(ForgeStep::Voice), options_scroll_id(ForgeStep::Voice), "stable within a step");
}

fn contexts(sources: &[&str]) -> Vec<gpui_kit::KeyContext> {
    sources.iter().map(|source| gpui_kit::KeyContext::parse(source).expect("valid context")).collect()
}

#[test]
fn escape_closes_hint_cards_before_anything_else() {
    let in_field = contexts(&["Root", FORGE_CONTEXT, TEXT_INPUT_KEY_CONTEXT]);
    assert_eq!(dismiss_step(true, &in_field), DismissStep::CloseHints);
    assert_eq!(dismiss_step(true, &contexts(&["Root", FORGE_CONTEXT])), DismissStep::CloseHints);
}

#[test]
fn escape_in_a_text_field_only_leaves_the_field() {
    let in_field = contexts(&["Root", FORGE_CONTEXT, TEXT_INPUT_KEY_CONTEXT]);
    assert_eq!(dismiss_step(false, &in_field), DismissStep::LeaveTextField);
}

#[test]
fn escape_outside_a_text_field_closes_the_forge() {
    assert_eq!(dismiss_step(false, &contexts(&["Root", FORGE_CONTEXT])), DismissStep::CloseForge);
    assert_eq!(dismiss_step(false, &[]), DismissStep::CloseForge);
}
