//! Reactions in the store: the acting persona reacts and claps on a post's original, every
//! change is saved, and a restart reads them back.

use crate::domain::reaction::CLAP;
use crate::domain::ProfileId;
use crate::state::store::AppState;
use crate::storage::{Database, Vault};

const FIRE: &str = "\u{1F525}";

#[test]
fn reacting_toggles_the_acting_personas_emoji_and_survives_a_restart() {
    let vault = Vault::spawn(Database::in_memory().expect("db")).expect("vault");
    let mut state = AppState::fallback_seed(Some(vault.clone()));
    let post = state.submit_post("a thought to react to").expect("post");
    let before = state.engagement;
    assert_eq!(state.toggle_reaction(&post, FIRE).map(|r| r.count), Some(1));
    assert!(state.engagement > before, "recommendations see the reaction");
    state.clap(&post, 3);
    state.unclap(&post, 1);
    let me = state.active_profile_id.clone();
    assert_eq!(state.reactions_of(&post).map(|set| set.claps_by(&me)), Some(2));
    assert!(state.toggle_reaction(&post, "not an emoji").is_none());
    assert!(state.toggle_reaction(&crate::domain::PostId::new("missing"), FIRE).is_none());

    let loaded = AppState::load_from_db(&vault).expect("reload");
    let set = loaded.reactions_of(&post).expect("read back");
    assert_eq!(set.emoji_of(&me), vec![FIRE]);
    assert_eq!(set.claps_by(&me), 2);

    let mut state = loaded;
    assert_eq!(state.toggle_reaction(&post, FIRE).map(|r| r.count), Some(0), "a second pick takes it back");
    assert!(AppState::load_from_db(&vault).expect("reload").reactions_of(&post).is_some_and(|set| set.emoji_of(&me).is_empty()));
}

#[test]
fn reactions_land_on_the_original_of_an_instant_repost_and_go_with_a_deleted_post() {
    let mut state = AppState::new_in_memory();
    let original = state.submit_post("worth reposting").expect("post");
    state.repost(&original).expect("repost");
    let wrapper = state.posts.iter().find(|p| p.repost_id.as_ref() == Some(&original)).map(|p| p.id.clone()).expect("wrapper");
    state.toggle_reaction(&wrapper, FIRE);
    assert!(state.reactions_of(&original).is_some() && state.reactions_of(&wrapper).is_none());
    let agent = ProfileId::new("agent_x");
    state.add_reaction_as(&original, &agent, CLAP, jiff::Timestamp::now());
    assert_eq!(state.reactions_of(&original).map(|set| set.claps()), Some(1));
    state.delete_post(&original).expect("delete");
    assert!(state.reactions_of(&original).is_none());
}

#[test]
fn an_agent_reacts_within_its_hourly_limit() {
    use crate::domain::engine::AGENT_REACTIONS_PER_HOUR;
    let mut state = AppState::new_in_memory();
    let posts: Vec<_> = (0..AGENT_REACTIONS_PER_HOUR + 2).map(|n| state.submit_post(&format!("thought {n}")).expect("post")).collect();
    let (agent, now) = (ProfileId::new("agent_x"), jiff::Timestamp::now());
    let emoji = vec![FIRE.to_string(), "\u{1F914}".to_string()];
    assert_eq!(state.react_as_agent(&posts[0], &agent, &emoji, now), 2);
    assert_eq!(state.react_as_agent(&posts[0], &agent, &emoji, now), 0, "the same emoji again adds nothing");
    let added: usize = posts[1..].iter().map(|post| state.react_as_agent(post, &agent, &emoji[..1], now)).sum();
    assert_eq!(added, AGENT_REACTIONS_PER_HOUR - 2, "the limit stops the rest");
    let later = now + jiff::SignedDuration::from_secs(2 * 3_600);
    assert_eq!(state.react_as_agent(posts.last().unwrap(), &agent, &emoji[..1], later), 1, "an hour later it may react again");
}
