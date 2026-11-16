use crate::state::forge::{roll_random, ForgeKind};
use crate::state::store::onboarding::needs_onboarding;
use crate::state::store::AppState;

#[test]
fn the_welcome_shows_only_before_it_is_done_and_before_a_thinker_exists() {
    assert!(needs_onboarding(false, 0));
    assert!(!needs_onboarding(true, 0), "completed or skipped");
    assert!(!needs_onboarding(false, 1), "they already made a thinker");
    assert!(!needs_onboarding(true, 3));
}

#[test]
fn seeded_agents_do_not_count_and_the_flag_persists() {
    let mut state = AppState::new_in_memory();
    assert_eq!(state.user_created_agents(), 0, "the seeded demo agents are not the user's");
    assert!(state.should_offer_onboarding());
    state.set_onboarding_done(true);
    assert!(state.onboarding_done());
    assert!(!state.should_offer_onboarding());
    state.set_onboarding_done(false);
    let draft = roll_random(5, ForgeKind::Agent, state.active_network_id.clone(), &state.user_profiles);
    state.update_profile(draft.profile().clone()).expect("save a forged agent");
    assert_eq!(state.user_created_agents(), 1);
    assert!(!state.should_offer_onboarding());
}
