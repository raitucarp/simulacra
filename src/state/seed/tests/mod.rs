//! Seed fidelity tests.
//!
//! `seed_snapshot.json` was generated from the original hand-written Rust seed literals
//! (before the move to `assets/seed/*.json`), with runtime timestamps replaced by a
//! placeholder. The JSON-backed seeds must reproduce it exactly. Posts were since slimmed to
//! reference their author by `author_id` instead of embedding the profile.

use crate::state::seed::{seed_documents, seed_networks, seed_posts, seed_profiles, seed_trending};
use jiff::Timestamp;
use std::collections::HashSet;
use serde_json::{json, Value};

const SNAPSHOT_JSON: &str = include_str!("seed_snapshot.json");
const TIMESTAMP_FIELD: &str = "created_at";
const TIMESTAMP_PLACEHOLDER: &str = "<now>";
/// Upper bound of how far back the seed timeline reaches.
const SEED_SPAN_DAYS: i64 = 60;

/// Replaces the runtime timestamp of every record in a JSON array with the placeholder.
fn without_timestamps(mut records: Value) -> Value {
    for record in records.as_array_mut().expect("array of records") {
        record[TIMESTAMP_FIELD] = json!(TIMESTAMP_PLACEHOLDER);
    }
    records
}

fn current_seed_snapshot() -> Value {
    let profiles = seed_profiles();
    let posts = serde_json::to_value(seed_posts()).expect("serialize posts");
    let documents = serde_json::to_value(seed_documents()).expect("serialize documents");
    json!({
        "networks": seed_networks(),
        "profiles": profiles,
        "posts": without_timestamps(posts),
        "trending": seed_trending(),
        "documents": without_timestamps(documents),
    })
}

#[test]
fn seed_output_matches_pre_json_snapshot() {
    let expected: Value = serde_json::from_str(SNAPSHOT_JSON).expect("valid snapshot");
    let actual = current_seed_snapshot();
    let sections = ["networks", "profiles", "posts", "trending", "documents"];
    for section in sections {
        assert_eq!(actual[section], expected[section], "seed section `{section}` drifted");
    }
    assert_eq!(actual, expected);
}

#[test]
fn seed_timestamps_are_stamped_at_runtime() {
    let before = Timestamp::now();
    let posts = seed_posts();
    let documents = seed_documents();
    let after = Timestamp::now();

    assert!(posts.iter().all(|p| p.created_at <= after && p.created_at >= before - crate::time::days(SEED_SPAN_DAYS)));
    assert!(documents.iter().all(|d| d.created_at >= before && d.created_at <= after));
}

#[test]
fn seed_posts_spread_over_distinct_days_newest_first_and_causally_ordered() {
    let posts = seed_posts();
    let days: HashSet<_> = posts.iter().map(|p| crate::time::utc_date(p.created_at)).collect();
    assert!(days.len() > 1, "seed posts must not all share one heatmap day");
    assert!(posts.windows(2).all(|w| w[0].created_at > w[1].created_at), "strictly newest first, no ties");
    for post in &posts {
        let targets = [&post.reply_to_id, &post.quote_id, &post.repost_id];
        for target in targets.into_iter().flatten() {
            let target = posts.iter().find(|p| &p.id == target).expect("seed target exists");
            assert!(post.created_at > target.created_at, "{} predates {}", post.id, target.id);
        }
    }
}
