//! N8: a post's scope decides which agents read it: in their recommendations (what they
//! answer next), in the tools' snapshot of the network, and never a locked post.

use super::locked::locked_post;
use crate::agent::tools::ToolContext;
use crate::domain::post::audience::Audience;
use crate::domain::{Profile, ProfileId};
use crate::state::recommend::candidates_for;
use crate::state::store::AppState;

fn with_agents() -> (AppState, ProfileId, ProfileId) {
    let mut state = AppState::new_in_memory();
    for (id, name) in [("ada", "Ada"), ("bo", "Bo")] {
        let mut agent = Profile::new_agent(id, name, &format!("@{id}"), "thinker");
        agent.network_id = Some(state.active_network_id.clone());
        state.user_profiles.push(agent);
    }
    (state, ProfileId::new("ada"), ProfileId::new("bo"))
}

#[test]
fn a_scope_keeps_a_post_from_the_agents_outside_it() {
    let (mut state, ada, bo) = with_agents();
    let open = state.submit_post("For everyone.").expect("open");
    let mine = state.submit_post("Only for me.").expect("mine");
    let shared = state.submit_post("For Ada.").expect("shared");
    assert_eq!(state.set_audience(&mine, Some(Audience::OnlyMe)), Ok(true));
    assert_eq!(state.set_audience(&mine, Some(Audience::OnlyMe)), Ok(false), "already so");
    state.set_audience(&shared, Some(Audience::Agents { agents: vec![ada.clone()] })).expect("shared with Ada");
    let network = state.active_network_id.clone();
    let ids = |who: &ProfileId| candidates_for(&state, who, &network).into_iter().map(|c| c.post).collect::<Vec<_>>();
    assert!(ids(&ada).contains(&open) && ids(&ada).contains(&shared) && !ids(&ada).contains(&mine));
    assert!(ids(&bo).contains(&open) && !ids(&bo).contains(&shared) && !ids(&bo).contains(&mine));
    let me = state.active_profile_id.clone();
    assert!(state.may_read(&me, state.get_post(&mine).expect("mine")), "the user's personas always read");
    let tools = ToolContext::capture_for(&bo, &network, &state.posts, &state.user_profiles, &state.documents);
    let seen: Vec<&str> = tools.posts.iter().map(|post| post.content.as_str()).collect();
    assert!(seen.contains(&"For everyone.") && !seen.contains(&"Only for me.") && !seen.contains(&"For Ada."), "{seen:?}");
    let names: Vec<String> = state.audience_agents().into_iter().map(|(_, name)| name).collect();
    assert!(names.iter().any(|n| n == "Ada") && names.iter().any(|n| n == "Bo"), "{names:?}");
}

#[test]
fn no_scope_lets_an_agent_read_a_locked_post() {
    let (mut state, ada, _) = with_agents();
    let locked = locked_post(&mut state);
    state.set_audience(&locked, Some(Audience::Agents { agents: vec![ada.clone()] })).expect("scoped");
    assert!(!state.may_read(&ada, state.get_post(&locked).expect("locked")));
    let network = state.active_network_id.clone();
    assert!(!candidates_for(&state, &ada, &network).iter().any(|candidate| candidate.post == locked));
    let tools = ToolContext::capture_for(&ada, &network, &state.posts, &state.user_profiles, &state.documents);
    assert!(tools.posts.iter().all(|post| post.id != locked.as_str()));
}
