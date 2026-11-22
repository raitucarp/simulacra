//! Post symbols are set only by the user's personas, on submit and on edit (S2).

use crate::domain::symbols::{EntryState, EntrySymbol, PostSymbols, Signifier};
use crate::domain::{Post, PostId};
use crate::error::AppError;
use crate::state::store::{AppState, ThreadAnchor, ThreadDraft};

fn task() -> PostSymbols {
    let mut symbols = PostSymbols::entry(EntrySymbol::Task);
    symbols.add_signifier(Signifier::Priority);
    symbols
}

fn seeded_agent(state: &AppState) -> crate::domain::Profile {
    state.user_profiles.iter().find(|p| p.is_agent()).cloned().expect("a seeded agent")
}

#[test]
fn a_persona_submits_a_post_with_symbols() {
    let mut state = AppState::new_in_memory();
    let draft = ThreadDraft { symbols: task(), ..ThreadDraft::text("Call the bakery") };
    let ids = state.submit_thread(&[draft], &ThreadAnchor::default()).expect("published");
    assert_eq!(state.get_post(&ids[0]).unwrap().extras.symbols, task());
}

#[test]
fn an_agent_profile_cannot_submit_symbols_or_an_inconsistent_set() {
    let mut state = AppState::new_in_memory();
    let agent = seeded_agent(&state);
    state.active_profile_id = agent.id.clone();
    let draft = ThreadDraft { symbols: task(), ..ThreadDraft::text("Not mine") };
    assert_eq!(state.submit_thread(&[draft], &ThreadAnchor::default()), Err(AppError::NotAPersona(agent.id.to_string())));
    assert!(state.submit_thread(&[ThreadDraft::text("plain")], &ThreadAnchor::default()).is_ok(), "no symbols, no guard");

    let mut state = AppState::new_in_memory();
    let note = PostSymbols { state: Some(EntryState::Done), ..PostSymbols::entry(EntrySymbol::Note) };
    let bad = ThreadDraft { symbols: note, ..ThreadDraft::text("A note is never done") };
    assert_eq!(state.submit_thread(&[bad], &ThreadAnchor::default()), Err(AppError::InvalidSymbols));
}

#[test]
fn set_post_symbols_changes_only_the_users_own_posts() {
    let mut state = AppState::new_in_memory();
    let agent = seeded_agent(&state);
    state.commit_post(Post::create("by_agent", "net1", agent.id.as_str(), "a thought").unwrap()).unwrap();
    assert_eq!(state.set_post_symbols(&PostId::new("by_agent"), task()), Err(AppError::NotAPersona(agent.id.to_string())));
    assert!(state.set_post_symbols(&PostId::new("missing"), task()).is_err());

    let own = state.submit_post("my words").unwrap();
    assert_eq!(state.set_post_symbols(&own, task()), Ok(true));
    assert_eq!(state.set_post_symbols(&own, task()), Ok(false), "already set");
    assert_eq!(state.edit_post(&own, "my words"), Ok(None), "symbols are not a text edit");
    assert_eq!(state.set_post_symbols(&own, PostSymbols::default()), Ok(true));
    assert!(state.get_post(&own).unwrap().extras.symbols.is_empty());
}

#[test]
fn an_agent_reply_is_stored_without_symbols() {
    let mut state = AppState::new_in_memory();
    let agent = seeded_agent(&state);
    let mut reply = Post::create("agent_reply", "net1", agent.id.as_str(), "Done already").unwrap();
    reply.extras.symbols = task();
    state.insert_reply_after_parent(reply).unwrap();
    assert!(state.get_post(&PostId::new("agent_reply")).unwrap().extras.symbols.is_empty());
}
