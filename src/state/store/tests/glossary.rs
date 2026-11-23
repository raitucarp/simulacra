//! N17: defining terms in the user's posts: one post per term, never a locked post nor anyone
//! else's; the glossary lists them, and an agent's glossary keeps only what it may read.

use super::locked::locked_post;
use crate::domain::post::glossary::DefineProblem;
use crate::domain::ProfileId;
use crate::state::store::AppState;
use jiff::Timestamp;

#[test]
fn a_term_is_defined_once_and_can_be_taken_back() {
    let mut state = AppState::new_in_memory();
    let first = state.submit_post("Entropy is why rooms get messy.").expect("posted");
    let second = state.submit_post("More on entropy.").expect("posted");
    assert_eq!(state.define_term(&first, "  ", "Disorder.", Timestamp::now()).err(), Some(DefineProblem::NoTerm));
    assert_eq!(state.define_term(&first, "entropy", " ", Timestamp::now()).err(), Some(DefineProblem::NoDefinition));
    state.define_term(&first, "Entropy", "How many ways a state can be arranged.", Timestamp::now()).expect("defined");
    assert_eq!(state.define_term(&second, "ENTROPY.", "Something else.", Timestamp::now()).err(), Some(DefineProblem::Taken(first.clone())));
    state.define_term(&first, "entropy", "Disorder, counted.", Timestamp::now()).expect("redefined in the same post");
    assert_eq!(state.glossary().lookup("entropy").map(|entry| entry.text.clone()).as_deref(), Some("Disorder, counted."));
    state.undefine_term(&first).expect("undefined");
    assert!(state.glossary().is_empty());
}

#[test]
fn a_locked_post_never_defines_a_term() {
    let mut state = AppState::new_in_memory();
    let locked = locked_post(&mut state);
    assert_eq!(state.define_term(&locked, "quetzal", "The secret word.", Timestamp::now()).err(), Some(DefineProblem::Locked));
    assert!(state.glossary().is_empty());
}

#[test]
fn an_agent_reads_only_the_definitions_it_may_read() {
    let mut state = AppState::new_in_memory();
    let mut agent = crate::domain::Profile::new_agent("ada", "Ada", "@ada", "historian");
    agent.network_id = Some(state.active_network_id.clone());
    state.user_profiles.push(agent);
    let open = state.submit_post("Public note.").expect("posted");
    let private = state.submit_post("Private note.").expect("posted");
    state.define_term(&open, "agora", "A public square.", Timestamp::now()).expect("defined");
    state.define_term(&private, "hearth", "Where the family gathers.", Timestamp::now()).expect("defined");
    state.set_audience(&private, Some(crate::domain::post::audience::Audience::OnlyMe)).expect("scoped");
    let theirs = state.glossary_for(&ProfileId::new("ada"));
    assert!(theirs.lookup("agora").is_some() && theirs.lookup("hearth").is_none(), "N8: the scoped definition stays with the user");
    assert_eq!(state.glossary().entries().len(), 2);
}
