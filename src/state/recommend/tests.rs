//! Recommendations over a small store: the user writes about ethics, agent Ada about optics,
//! Bo about cooking; vectors put ethics on x, optics on y and cooking on z.

use super::*;
use crate::domain::embedding::{l2_normalized, StoredChunk};
use jiff::SignedDuration;

const MODEL: &str = "m";

struct World {
    state: AppState,
    index: VectorIndex,
    ada: Profile,
}

fn now() -> Timestamp {
    crate::time::utc(2026, 11, 19, 12, 0, 0)
}

/// Adds a post by `author` an hour apart from the others, embedded at `vector`.
fn post(world: &mut World, author: &str, text: &str, vector: [f32; 3], reply_to: Option<&PostId>) -> PostId {
    let id = world.state.submit_post(text).unwrap();
    let age = SignedDuration::from_hours(world.state.posts.len() as i64);
    let stored = world.state.posts.iter_mut().find(|p| p.id == id).unwrap();
    (stored.author_id, stored.created_at, stored.reply_to_id) = (ProfileId::new(author), now() - age, reply_to.cloned());
    let chunk = StoredChunk { index: 0, model: MODEL.into(), content_hash: "h".into(), vector: l2_normalized(vector.to_vec()) };
    world.index = std::mem::take(&mut world.index).with_chunks([(EmbeddingOwner::post(&id), chunk)]);
    id
}

fn world() -> World {
    let state = AppState::new_in_memory();
    let mut ada = Profile::new_agent("ada", "Ada", "@ada", "optician");
    ada.network_id = Some(state.active_network_id.clone());
    World { state, index: VectorIndex::default(), ada }
}

fn recommender(world: &World) -> Recommender<'_> {
    Recommender { state: &world.state, index: &world.index, model: MODEL, now: now() }
}

#[test]
fn for_you_ranks_other_peoples_posts_by_the_users_interest() {
    let mut w = world();
    let me = w.state.active_profile_id.as_str().to_owned();
    let mine = post(&mut w, &me, "virtue", [1.0, 0.0, 0.0], None);
    let cooking = post(&mut w, "bo", "bread", [0.0, 0.0, 1.0], None);
    let ethics = post(&mut w, "ada", "duty", [0.9, 0.1, 0.0], None);
    let scores = recommender(&w).for_you();
    assert!(scores[&ethics] > scores[&cooking], "{scores:?}");
    assert!(scores[&mine] < 0.3, "the user's own post scores on freshness alone");
}

#[test]
fn with_no_embedded_engagement_there_is_no_ranking() {
    let mut w = world();
    post(&mut w, "bo", "bread", [0.0, 0.0, 1.0], None);
    assert!(recommender(&w).for_you().is_empty());
    assert!(recommender(&w).for_profile(&w.ada, 3).is_empty());
    assert_eq!(recommender(&w).agent_answer(&w.ada, &w.state.active_network_id.clone()), None);
}

#[test]
fn a_persona_is_recommended_posts_near_what_it_writes() {
    let mut w = world();
    post(&mut w, "ada", "lenses", [0.0, 1.0, 0.0], None);
    let optics = post(&mut w, "bo", "prisms", [0.1, 0.9, 0.0], None);
    post(&mut w, "bo", "bread", [0.0, 0.0, 1.0], None);
    let top = recommender(&w).for_profile(&w.ada, 1);
    assert_eq!(top.iter().map(|r| &r.post).collect::<Vec<_>>(), [&optics], "its own post is never recommended back");
}

#[test]
fn an_agent_answers_the_closest_post_it_has_not_answered() {
    let mut w = world();
    let network = w.state.active_network_id.clone();
    post(&mut w, "ada", "lenses", [0.0, 1.0, 0.0], None);
    let near = post(&mut w, "bo", "prisms", [0.1, 0.9, 0.0], None);
    let farther = post(&mut w, "bo", "mirrors", [0.5, 0.5, 0.0], None);
    post(&mut w, "bo", "bread", [0.0, 0.0, 1.0], None);
    assert_eq!(recommender(&w).agent_answer(&w.ada, &network), Some(near.clone()));
    post(&mut w, "ada", "a reply", [0.0, 1.0, 0.0], Some(&near));
    assert_eq!(recommender(&w).agent_answer(&w.ada, &network), Some(farther), "answered posts are skipped");
}

#[test]
fn related_posts_come_from_other_conversations() {
    let mut w = world();
    let root = post(&mut w, "bo", "duty", [1.0, 0.0, 0.0], None);
    post(&mut w, "ada", "same thread", [1.0, 0.0, 0.0], Some(&root));
    let elsewhere = post(&mut w, "ada", "obligation", [0.9, 0.1, 0.0], None);
    post(&mut w, "bo", "bread", [0.0, 0.0, 1.0], None);
    let source = w.state.get_post_by_id(root.as_str()).unwrap().clone();
    let related = recommender(&w).related(&source, 2);
    assert_eq!(related[0].owner, EmbeddingOwner::post(&elsewhere));
    assert!(related.iter().all(|hit| hit.owner.id != root.as_str()), "{related:?}");
    assert_eq!(related.len(), 2, "the reply in its own thread is left out, cooking fills in");
}

#[test]
fn a_like_shapes_the_users_interest_and_bumps_the_engagement_revision() {
    let mut w = world();
    let liked = post(&mut w, "ada", "duty", [1.0, 0.0, 0.0], None);
    let near = post(&mut w, "bo", "virtue", [0.9, 0.1, 0.0], None);
    let cooking = post(&mut w, "bo", "bread", [0.0, 0.0, 1.0], None);
    assert!(recommender(&w).for_you().is_empty(), "no engagement yet");
    let before = w.state.engagement;
    w.state.toggle_like(&liked);
    assert!(w.state.engagement > before, "cached recommendations see the like");
    let scores = recommender(&w).for_you();
    assert!(scores[&near] > scores[&cooking], "{scores:?}");
}

#[test]
fn reactions_and_claps_shape_interest_like_a_like_does() {
    let mut w = world();
    let reacted = post(&mut w, "ada", "duty", [1.0, 0.0, 0.0], None);
    let near = post(&mut w, "bo", "virtue", [0.9, 0.1, 0.0], None);
    let cooking = post(&mut w, "bo", "bread", [0.0, 0.0, 1.0], None);
    assert!(recommender(&w).for_you().is_empty(), "no engagement yet");
    let before = w.state.engagement;
    w.state.toggle_reaction(&reacted, "\u{1F525}");
    assert!(w.state.engagement > before, "cached recommendations see the reaction");
    let scores = recommender(&w).for_you();
    assert!(scores[&near] > scores[&cooking], "{scores:?}");

    let me = w.state.active_profile_id.clone();
    let network = w.state.active_network_id.clone();
    let weight = |state: &AppState| signals_of(state, &me, &network).iter().filter(|s| s.post == cooking).map(|s| s.weight).sum::<f32>();
    w.state.clap(&cooking, 1);
    let one_clap = weight(&w.state);
    w.state.clap(&cooking, 40);
    assert!(weight(&w.state) > one_clap, "more claps say more");
    assert!(weight(&w.state) <= 0.8 + f32::EPSILON, "up to a cap");
    let agent = ProfileId::new("ada");
    w.state.add_reaction_as(&near, &agent, "\u{1F914}", now());
    assert!(signals_of(&w.state, &agent, &network).iter().any(|s| s.post == near && s.at == now()), "an agent's reactions are its signals, at the moment it reacted");
}
