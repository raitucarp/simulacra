//! The persona guard (F3) and one author's posts, paged (F5).

use crate::domain::{Post, PostId, Profile, ProfileId};
use crate::error::AppError;
use crate::state::store::{require_persona, AppState, Page};

fn at(hour: i8) -> jiff::Timestamp {
    crate::time::utc(2026, 3, 10, hour, 0, 0)
}

fn written(id: &str, author: &str, hour: i8) -> Post {
    Post { created_at: at(hour), ..Post::create(id, "net1", author, "a thought").unwrap() }
}

fn page_ids(state: &AppState, author: &str, page: Page) -> Vec<String> {
    state.posts_by(&ProfileId::new(author), page).into_iter().map(|p| p.id.as_str().to_string()).collect()
}

#[test]
fn require_persona_admits_the_primary_profile_and_alter_egos() {
    let primary = Profile::new_primary("user_main", "Me", "@me");
    let alt = Profile::new_alt("persona_1", "Other me", "@other");
    assert_eq!(require_persona(&primary), Ok(&primary));
    assert_eq!(require_persona(&alt), Ok(&alt));
}

#[test]
fn require_persona_refuses_agents() {
    let agent = Profile::new_agent("agent_1", "Ada", "@ada", "Logician");
    assert_eq!(require_persona(&agent), Err(AppError::NotAPersona("agent_1".into())));
}

#[test]
fn edit_post_is_guarded_by_the_persona_check() {
    let mut state = AppState::new_in_memory();
    let agent = state.user_profiles.iter().find(|p| p.is_agent()).cloned().expect("a seeded agent");
    let agent_post = written("by_agent", agent.id.as_str(), 9);
    state.commit_post(agent_post).unwrap();
    assert_eq!(state.edit_post(&PostId::new("by_agent"), "rewritten"), Err(AppError::NotAPersona(agent.id.to_string())));

    let own = state.submit_post("my first words").unwrap();
    assert!(state.edit_post(&own, "my second words").unwrap().is_some());
    assert_eq!(state.get_post(&own).unwrap().content, "my second words");
}

#[test]
fn posts_by_pages_one_authors_posts_newest_first() {
    let mut state = AppState::new_in_memory();
    for (id, hour) in [("w3", 3), ("w1", 1), ("w5", 5), ("w2", 2), ("w4", 4)] {
        state.commit_post(written(id, "writer", hour)).unwrap();
    }
    state.commit_post(written("elsewhere", "someone_else", 6)).unwrap();

    let first = Page::first(2);
    assert_eq!(page_ids(&state, "writer", first), vec!["w5", "w4"]);
    assert_eq!(page_ids(&state, "writer", first.next()), vec!["w3", "w2"]);
    assert_eq!(page_ids(&state, "writer", first.next().next()), vec!["w1"]);
    assert!(page_ids(&state, "writer", first.next().next().next()).is_empty());
    assert_eq!(page_ids(&state, "writer", Page::first(10)).len(), 5);
    assert!(page_ids(&state, "nobody", first).is_empty());
}

#[test]
fn posts_by_breaks_ties_by_id() {
    let mut state = AppState::new_in_memory();
    for id in ["tie_a", "tie_c", "tie_b"] {
        state.commit_post(written(id, "writer", 7)).unwrap();
    }
    assert_eq!(page_ids(&state, "writer", Page::first(3)), vec!["tie_c", "tie_b", "tie_a"]);
}
