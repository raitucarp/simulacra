use super::*;
use crate::domain::embedding::{l2_normalized, EmbeddingOwner, StoredChunk, VectorIndex};
use crate::domain::PostId;
use jiff::{SignedDuration, Timestamp};

const MODEL: &str = "m";

fn now() -> Timestamp {
    crate::time::utc(2026, 11, 19, 12, 0, 0)
}

fn hours_ago(hours: i64) -> Timestamp {
    now() - SignedDuration::from_hours(hours)
}

fn id(name: &str) -> PostId {
    PostId::new(name)
}

/// Posts about ethics lie along x, about optics along y, about cooking along z.
fn index() -> VectorIndex {
    let row = |name: &str, v: [f32; 3]| (EmbeddingOwner::post(&id(name)), StoredChunk { index: 0, model: MODEL.into(), content_hash: "h".into(), vector: l2_normalized(v.to_vec()) });
    VectorIndex::from_rows([
        row("mine_ethics", [1.0, 0.0, 0.0]),
        row("mine_optics_old", [0.0, 1.0, 0.0]),
        row("ethics", [0.9, 0.1, 0.0]),
        row("optics", [0.1, 0.9, 0.0]),
        row("cooking", [0.0, 0.1, 0.9]),
    ])
}

fn signal(name: &str, at: Timestamp) -> Signal {
    Signal { post: id(name), weight: 1.0, at }
}

fn candidate(name: &str, hours: i64) -> Candidate {
    Candidate { post: id(name), created_at: hours_ago(hours) }
}

#[test]
fn recent_engagement_shapes_the_interest_more_than_old() {
    let signals = [signal("mine_ethics", hours_ago(1)), signal("mine_optics_old", hours_ago(24 * 365))];
    let interest = interest_vector(&index(), MODEL, &signals, now()).expect("embedded posts");
    assert!(interest[0] > 0.9 && interest[1] > 0.0, "{interest:?}");
    assert_eq!(interest_vector(&index(), MODEL, &[signal("unknown", now())], now()), None, "nothing embedded");
    assert_eq!(interest_vector(&index(), "other", &signals, now()), None, "another model");
}

#[test]
fn rank_all_puts_the_closest_meaning_first_and_keeps_unembedded_posts() {
    let interest = [1.0, 0.0, 0.0];
    let candidates = [candidate("cooking", 1), candidate("optics", 1), candidate("ethics", 1), candidate("fresh_unembedded", 0)];
    let ranked = rank_all(&index(), MODEL, &interest, &candidates, now(), Blend::FEED);
    let order: Vec<&str> = ranked.iter().map(|r| r.post.as_str()).collect();
    assert_eq!(order[0], "ethics");
    assert_eq!(ranked.len(), 4, "every candidate is ranked");
    let unembedded = ranked.iter().find(|r| r.post.as_str() == "fresh_unembedded").unwrap();
    assert_eq!(unembedded.similarity, 0.0);
    assert!((unembedded.score - Blend::FEED.freshness).abs() < 1e-6, "scored on freshness alone");
}

#[test]
fn freshness_can_outrank_a_slightly_closer_old_post() {
    let interest = l2_normalized(vec![1.0, 1.0, 0.0]);
    let candidates = [candidate("ethics", 24 * 14), candidate("optics", 1)];
    let ranked = rank_all(&index(), MODEL, &interest, &candidates, now(), Blend::FEED);
    assert_eq!(ranked[0].post.as_str(), "optics", "equal meaning, the newer wins");
    assert!((freshness(now(), hours_ago(48), 48.0) - 0.5).abs() < 1e-6);
    assert_eq!(freshness(now(), now() + SignedDuration::from_hours(1), 48.0), 1.0);
}

#[test]
fn recommend_only_returns_candidates_best_first() {
    let interest = [1.0, 0.0, 0.0];
    let candidates = [candidate("optics", 2), candidate("ethics", 2), candidate("cooking", 2)];
    let top = recommend(&index(), MODEL, &interest, &candidates, now(), Blend::PERSONA, 2);
    let order: Vec<&str> = top.iter().map(|r| r.post.as_str()).collect();
    assert_eq!(order, ["ethics", "optics"], "the profile's own posts are not candidates, so never returned");
    assert!(top[0].similarity > 0.9);
}
