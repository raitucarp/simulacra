//! Search at 100 000 posts (Q7): a budget test like the graph bench. It builds a scratch vault
//! in a fresh temp folder, so plain `cargo test` leaves it out; run it with
//! `cargo test --lib search::bench -- --ignored --nocapture` (numbers: docs/decisions/Q7).

use super::{SearchRequest, TEXT_CANDIDATES};
use crate::domain::data::history::{Revision, RevisionSubject};
use crate::domain::post::Tint;
use crate::domain::search_query::PostQuery;
use crate::domain::symbols::{EntrySymbol, PostSymbols};
use crate::domain::{NetworkId, Post, PostId, ProfileId};
use crate::state::AppState;
use crate::storage::db::query_match_expression;
use crate::storage::{Database, Vault};
use jiff::tz::TimeZone;
use std::path::PathBuf;
use std::time::{Duration, Instant};

const POSTS: usize = 100_000;
/// Earlier versions kept, one for every tenth post.
const VERSION_EVERY: usize = 10;
/// Distinct made-up words, used about as unevenly as words in prose.
const VOCABULARY: usize = 20_000;
const WORDS_PER_POST: usize = 16;
const RESULTS: usize = 100;
/// Slowest query allowed, in an optimised test build on a loaded machine.
const QUERY_BUDGET: Duration = Duration::from_millis(400);
#[cfg(unoptimized)]
const UNOPTIMISED_SLOWDOWN: u32 = 8;
#[cfg(not(unoptimized))]
const UNOPTIMISED_SLOWDOWN: u32 = 1;
const SYLLABLES: [&str; 16] = ["ka", "lo", "mi", "nu", "re", "so", "ta", "vi", "ze", "po", "du", "fe", "gar", "hin", "jos", "bel"];

/// Word `n` of the made-up vocabulary: its base-16 digits as syllables.
fn word(mut n: usize) -> String {
    let mut out = String::new();
    loop {
        out.push_str(SYLLABLES[n % SYLLABLES.len()]);
        n /= SYLLABLES.len();
        if n == 0 {
            return out;
        }
    }
}

/// The timed queries: a common, a mid and a rare word, two words, a phrase, a short prefix,
/// words with filters, filters alone.
fn queries() -> Vec<(String, bool)> {
    let (common, mid, rare) = (word(83), word(400), word(19_000));
    let text = [
        common.clone(), mid.clone(), rare, format!("{common} {mid}"), "\"quiet harbour\"".into(), common[..3].to_string(),
        format!("{mid} has:todo"), format!("{common} -{mid} tint:amber"), "has:code is:open".into(), "tint:amber after:2026-01-01".into(),
    ];
    text.into_iter().map(|q| (q, false)).chain([("oldword7".to_string(), true), (mid, true)]).collect()
}

/// A deterministic xorshift.
struct Dice(u64);

impl Dice {
    fn roll(&mut self, bound: usize) -> usize {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        (self.0 % bound.max(1) as u64) as usize
    }

    /// A word index skewed toward the small ones, as in prose.
    fn skewed(&mut self) -> usize {
        self.roll(VOCABULARY) * self.roll(VOCABULARY) / VOCABULARY
    }
}

fn post(i: usize, dice: &mut Dice, network: &NetworkId, author: &ProfileId) -> Post {
    let mut text = (0..WORDS_PER_POST).map(|_| word(dice.skewed())).collect::<Vec<_>>().join(" ");
    match i % 20 {
        0 => text.push_str("\n- [ ] follow up"),
        1 => text.push_str("\n```rust\nfn main() {}\n```"),
        2 => text.push_str(" by the quiet harbour"),
        _ => {}
    }
    let mut post = Post::create(format!("bench_{i}"), network.clone(), author.clone(), &text).expect("post");
    post.created_at = crate::time::utc(2025, 1, 1, 0, 0, 0) + jiff::SignedDuration::from_mins(i as i64 * 10);
    post.extras.tint = i.is_multiple_of(7).then_some(Tint::Amber);
    post.extras.symbols = if i.is_multiple_of(11) { PostSymbols::entry(EntrySymbol::Task) } else { PostSymbols::default() };
    post
}

/// A fresh temp folder, removed when dropped.
struct Scratch(PathBuf);

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn scratch() -> Scratch {
    let dir = std::env::temp_dir().join(format!("simulacra-search-bench-{}-{}", std::process::id(), jiff::Timestamp::now().as_nanosecond()));
    std::fs::create_dir_all(&dir).expect("temp folder");
    Scratch(dir)
}

/// The vault filled with [`POSTS`] posts and their versions; the time writing and loading took.
fn filled(dir: &Scratch) -> (AppState, Duration, Duration) {
    let vault = Vault::spawn(Database::open(dir.0.join("bench.db")).expect("db")).expect("vault");
    let seeded = AppState::load_from_db(&vault).expect("seeded");
    let (network, author) = (seeded.active_network_id.clone(), seeded.active_profile_id.clone());
    let mut dice = Dice(0x9E37_79B9_7F4A_7C15);
    let posts: Vec<Post> = (0..POSTS).map(|i| post(i, &mut dice, &network, &author)).collect();
    let versions: Vec<Revision> = (0..POSTS)
        .step_by(VERSION_EVERY)
        .map(|i| Revision::of(RevisionSubject::Post(PostId::new(format!("bench_{i}"))), &format!("an earlier draft about oldword{}", i % 100), jiff::Timestamp::now()))
        .collect();
    let started = Instant::now();
    vault.read(move |db| db.save_posts(&posts).and_then(|_| db.save_revisions(&versions))).expect("saved");
    let written = started.elapsed();
    let started = Instant::now();
    let state = AppState::load_from_db(&vault).expect("loaded");
    (state, written, started.elapsed())
}

/// The hits, the whole search's time and the full-text index's share of it.
fn timed(state: &AppState, input: &str, include_versions: bool) -> (usize, Duration, Duration) {
    let query = PostQuery::parse(input);
    let request = SearchRequest { query: &query, meaning: &[], limit: RESULTS, today: jiff::civil::date(2026, 11, 23), tz: TimeZone::UTC, include_versions };
    let started = Instant::now();
    let found = state.search(&request).len();
    let took = started.elapsed();
    let started = Instant::now();
    let _ = query_match_expression(&query.words, &query.phrases).map(|e| state.db.as_ref().expect("vault").score_posts(&e, TEXT_CANDIDATES));
    (found, took, started.elapsed())
}

#[test]
#[ignore = "benchmark: builds a 100 000 post vault; run with --ignored --nocapture"]
fn one_hundred_thousand_posts_search_within_budget() {
    let dir = scratch();
    let (state, written, loaded) = filled(&dir);
    assert!(state.posts.len() >= POSTS);
    println!("write {POSTS} posts + {} versions, indexed: {written:?}; load: {loaded:?}", POSTS / VERSION_EVERY);
    let mut slowest = Duration::ZERO;
    for (input, versions) in queries() {
        let (found, took, index) = timed(&state, &input, versions);
        println!("{input:32} versions={versions:5} hits={found:3} {took:?} (index {index:?})");
        assert!(found > 0, "{input}");
        slowest = slowest.max(took);
    }
    assert!(slowest < QUERY_BUDGET * UNOPTIMISED_SLOWDOWN, "slowest query {slowest:?}");
}
