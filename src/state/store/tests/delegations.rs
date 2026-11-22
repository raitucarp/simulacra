//! D12: items handed to an agent wait for its progress reply, which is asked for once.

use crate::domain::{Post, PostId};
use crate::state::store::AppState;

fn agent_handle(state: &AppState) -> (String, crate::domain::ProfileId) {
    let agent = state.current_network_agents()[0].clone();
    (agent.handle.trim_start_matches('@').to_string(), agent.id)
}

#[test]
fn items_for_an_agent_of_the_network_wait_until_asked_about() {
    let mut state = AppState::new_in_memory();
    let (handle, agent) = agent_handle(&state);
    let text = format!("Launch\n[ ] write the notes @{handle}\n[/] book the room @{}\n[x] done already @{handle}\n[ ] ask @nobody_here", handle.to_uppercase());
    let id = state.submit_post(&text).unwrap();
    let jobs = state.pending_delegations();
    assert_eq!(jobs.iter().map(|job| (job.index, job.item.as_str())).collect::<Vec<_>>(), vec![(0, "write the notes"), (1, "book the room")]);
    assert!(jobs.iter().all(|job| job.agent.id == agent && job.post_id == id));
    state.mark_progress_asked(&id, 0).unwrap();
    assert_eq!(state.pending_delegations().iter().map(|job| job.index).collect::<Vec<_>>(), vec![1]);
    assert!(state.get_post(&id).unwrap().extras.head.is_none(), "no version");
}

#[test]
fn an_edit_does_not_ask_again_and_agents_posts_are_never_delegations() {
    let mut state = AppState::new_in_memory();
    let (handle, agent) = agent_handle(&state);
    let id = state.submit_post(&format!("[ ] draft the plan @{handle}")).unwrap();
    state.mark_progress_asked(&id, 0).unwrap();
    let edited = format!("Plan\n[ ] draft the plan @{handle}");
    state.edit_post_version(&id, &crate::state::PostEdit::text(edited), jiff::Timestamp::now()).unwrap();
    assert!(state.pending_delegations().is_empty());
    let network = state.active_network_id.clone();
    state.commit_post(Post::create("agent_list", network.as_str(), agent.as_str(), &format!("[ ] theirs @{handle}")).unwrap()).unwrap();
    assert!(state.pending_delegations().is_empty());
    assert!(state.mark_progress_asked(&PostId::new("agent_list"), 0).is_err());
}
