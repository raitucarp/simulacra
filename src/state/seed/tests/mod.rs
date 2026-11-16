//! Seed fidelity tests.
//!
//! `seed_snapshot.json` was generated from the original hand-written Rust seed literals
//! (before the move to `assets/seed/*.json`), with runtime timestamps replaced by a
//! placeholder. The JSON-backed seeds must reproduce it exactly. Posts were since slimmed to
//! reference their author by `author_id` instead of embedding the profile.

use crate::state::seed::{seed_documents, seed_networks, seed_posts, seed_profiles, seed_trending};
use chrono::Utc;
use serde_json::{json, Value};

const SNAPSHOT_JSON: &str = include_str!("seed_snapshot.json");
const TIMESTAMP_FIELD: &str = "created_at";
const TIMESTAMP_PLACEHOLDER: &str = "<now>";

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
    let before = Utc::now();
    let posts = seed_posts();
    let documents = seed_documents();
    let after = Utc::now();

    assert!(posts.iter().all(|p| p.created_at >= before && p.created_at <= after));
    assert!(documents.iter().all(|d| d.created_at >= before && d.created_at <= after));
}
