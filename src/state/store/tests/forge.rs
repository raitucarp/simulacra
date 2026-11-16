use crate::state::forge::{can_save, roll_random, validate, AgentDraft, ForgeKind, ForgeStep};
use crate::state::store::AppState;

#[test]
fn forge_autosave_persists_and_clears() {
    let state = AppState::new_in_memory();
    assert!(state.load_forge_autosave().is_none());
    let mut draft = roll_random(42, ForgeKind::Agent, state.active_network_id.clone(), &state.user_profiles);
    draft.go_to(ForgeStep::Voice);
    state.save_forge_autosave(&draft);
    let restored = state.load_forge_autosave().expect("autosave restored");
    assert_eq!(restored.profile(), draft.profile());
    assert_eq!(restored.step(), ForgeStep::Voice);
    state.clear_forge_autosave();
    assert!(state.load_forge_autosave().is_none());
}

#[test]
fn a_committed_forge_draft_joins_the_active_network() {
    let mut state = AppState::new_in_memory();
    let mut draft = roll_random(9, ForgeKind::Agent, state.active_network_id.clone(), &state.user_profiles);
    assert!(can_save(&validate(&draft, &state.user_profiles)));
    let profile = draft.commit();
    state.update_profile(profile.clone()).expect("save forged agent");
    assert!(state.current_network_agents().iter().any(|p| p.id == profile.id));
    let reopened = AgentDraft::edit_existing(profile);
    assert!(validate(&reopened, &state.user_profiles).is_empty(), "no clash with itself");
}

#[test]
fn the_forge_guide_switch_persists() {
    let state = AppState::new_in_memory();
    assert!(!state.forge_guide_hidden(), "shown by default");
    state.set_forge_guide_hidden(true);
    assert!(state.forge_guide_hidden());
    state.set_forge_guide_hidden(false);
    assert!(!state.forge_guide_hidden());
}
