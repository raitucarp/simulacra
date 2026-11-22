//! The search executor (Q3): ranked text, filters, meaning, fusion, and what never surfaces.

use super::{resolve_date, SearchRequest};
use crate::domain::crypto::KdfParams;
use crate::domain::embedding::{EmbeddingOwner, ScoredOwner};
use crate::domain::post::lock::LockPassword;
use crate::domain::post::Tint;
use crate::domain::search_query::{PostQuery, QueryDate};
use crate::domain::symbols::{EntrySymbol, PostSymbols, Signifier};
use crate::domain::PostId;
use crate::state::{AppState, ThreadAnchor, ThreadDraft};
use jiff::civil::date;
use jiff::tz::TimeZone;
use jiff::{SignedDuration, Timestamp};

const FAST: KdfParams = KdfParams { memory_kib: 8, iterations: 1, lanes: 1 };

pub(crate) fn run(state: &AppState, query: &str, meaning: &[ScoredOwner]) -> Vec<PostId> {
    let query = PostQuery::parse(query);
    let request = SearchRequest { query: &query, meaning, limit: 50, today: date(2026, 11, 23), tz: TimeZone::UTC, include_versions: false };
    state.search(&request).into_iter().map(|hit| hit.post_id).collect()
}

fn post(state: &mut AppState, draft: ThreadDraft) -> PostId {
    state.submit_thread(&[draft], &ThreadAnchor::default()).expect("posted").remove(0)
}

fn text(state: &mut AppState, content: &str) -> PostId {
    post(state, ThreadDraft::text(content))
}

fn meant(ids: &[&PostId]) -> Vec<ScoredOwner> {
    ids.iter().enumerate().map(|(i, id)| ScoredOwner { owner: EmbeddingOwner::post(id), score: 0.9 - i as f32 * 0.1, best_chunk: 0 }).collect()
}

/// Runs `test` against the full-text index and against the in-memory fallback.
fn both_ways(test: impl Fn(&mut AppState)) {
    let mut indexed = AppState::new_in_memory();
    test(&mut indexed);
    let mut in_memory = AppState::new_in_memory();
    in_memory.db = None;
    test(&mut in_memory);
}

fn sorted(mut ids: Vec<PostId>) -> Vec<PostId> {
    ids.sort();
    ids
}

#[test]
fn words_rank_best_first_and_every_word_is_needed() {
    both_ways(|state| {
        let dense = text(state, "Quillwort quillwort quillwort by the pond");
        let thin = text(state, "A long walk past many ponds where a single quillwort grows among reeds");
        text(state, "pond life without the plant");
        assert_eq!(run(state, "quillw", &[]), vec![dense.clone(), thin.clone()]);
        assert_eq!(run(state, "quillwort reeds", &[]), vec![thin.clone()]);
        assert!(run(state, "quillwort marsupial", &[]).is_empty());
    });
}

#[test]
fn phrases_need_their_words_in_order_and_exclusions_drop_posts() {
    both_ways(|state| {
        let starter = text(state, "My sourdough starter recipe, finally");
        let other = text(state, "A recipe for a sourdough starter");
        let burnt = text(state, "sourdough starter recipe, burnt again");
        assert_eq!(sorted(run(state, "\"starter recipe\"", &[])), sorted(vec![starter.clone(), burnt.clone()]));
        assert_eq!(run(state, "\"starter recipe\" -burnt", &[]), vec![starter.clone()]);
        assert_eq!(run(state, "sourdough -\"starter recipe\"", &[]), vec![other]);
    });
}

#[test]
fn filters_narrow_and_negated_filters_exclude() {
    let mut state = AppState::new_in_memory();
    let task = post(&mut state, ThreadDraft { symbols: PostSymbols::entry(EntrySymbol::Task), tint: Some(Tint::Amber), ..ThreadDraft::text("zephyr task") });
    let mut flagged = PostSymbols::entry(EntrySymbol::Note);
    flagged.add_signifier(Signifier::Priority);
    let note = post(&mut state, ThreadDraft { symbols: flagged, ..ThreadDraft::text("zephyr note with `code`") });
    let todo = text(&mut state, "zephyr list\n- [ ] buy rye\n- [x] feed starter");
    assert_eq!(run(&state, "zephyr symbol:task", &[]), vec![task.clone()]);
    assert_eq!(run(&state, "zephyr symbol:task_open tint:amber", &[]), vec![task.clone()]);
    assert_eq!(run(&state, "zephyr symbol:priority has:code", &[]), vec![note.clone()]);
    assert_eq!(run(&state, "zephyr has:todo is:open", &[]), vec![todo.clone()]);
    assert_eq!(sorted(run(&state, "zephyr is:open", &[])), sorted(vec![task.clone(), todo.clone()]));
    assert_eq!(sorted(run(&state, "zephyr -symbol:task", &[])), sorted(vec![note.clone(), todo.clone()]));
    assert_eq!(run(&state, "zephyr kind:question", &[]), Vec::<PostId>::new());
    let author = state.author_of(state.get_post(&task).expect("post")).handle.clone();
    assert_eq!(run(&state, &format!("zephyr from:{author} -has:todo -has:code"), &[]), vec![task]);
}

#[test]
fn dates_bound_the_days_and_unknown_bounds_are_left_out() {
    let mut state = AppState::new_in_memory();
    let old = text(&mut state, "glacier memo");
    let new = text(&mut state, "glacier note");
    let at = |day: &str| format!("{day}T12:00:00Z").parse::<Timestamp>().expect("time");
    state.posts.iter_mut().find(|p| p.id == old).expect("old").created_at = at("2026-01-10");
    state.posts.iter_mut().find(|p| p.id == new).expect("new").created_at = at("2026-11-22");
    assert_eq!(run(&state, "glacier before:2026-01-11", &[]), vec![old.clone()]);
    assert_eq!(run(&state, "glacier after:yesterday", &[]), vec![new.clone()]);
    assert!(run(&state, "glacier after:2026-11-23", &[]).is_empty(), "after: starts at its day");
    assert_eq!(run(&state, "glacier after:someday", &[]).len(), 2);
    assert_eq!(resolve_date(&QueryDate::Raw("last-week".into()), date(2026, 11, 23)), Some(date(2026, 11, 16)));
    assert_eq!(resolve_date(&QueryDate::Raw("LAST-YEAR".into()), date(2026, 11, 23)), Some(date(2025, 11, 23)));
}

#[test]
fn meaning_and_words_fuse_into_one_list() {
    let mut state = AppState::new_in_memory();
    let both = text(&mut state, "tidepool creatures at low tide");
    let words = text(&mut state, "tidepool notes");
    let meaning = text(&mut state, "anemones and crabs on the rocks");
    let found = run(&state, "tidepool", &meant(&[&meaning, &both]));
    assert_eq!(found[0], both, "found both ways, it ranks first");
    assert_eq!(sorted(found), sorted(vec![both, words, meaning.clone()]));
    assert_eq!(run(&state, "-tidepool", &meant(&[&meaning])), vec![meaning.clone()], "meaning alone, minus a word");
    let negative = vec![ScoredOwner { owner: EmbeddingOwner::post(&meaning), score: -0.2, best_chunk: 0 }];
    assert!(run(&state, "kelp", &negative).is_empty(), "opposite meanings are no match");
}

#[test]
fn a_filter_only_query_lists_newest_first_and_a_blank_one_nothing() {
    let mut state = AppState::new_in_memory();
    let first = post(&mut state, ThreadDraft { tint: Some(Tint::Violet), ..ThreadDraft::text("one") });
    let second = post(&mut state, ThreadDraft { tint: Some(Tint::Violet), ..ThreadDraft::text("two") });
    assert_eq!(run(&state, "tint:violet", &[]), vec![second.clone(), first]);
    assert!(state.set_archived(&second, true).expect("archived"));
    assert_eq!(run(&state, "is:archived tint:violet", &[]), vec![second]);
    assert!(run(&state, "   ", &[]).is_empty());
}

#[test]
fn locked_and_scheduled_posts_never_surface_by_text_or_meaning() {
    both_ways(|state| {
        let secret = "Quetzalcoatl orchard confession";
        let password = LockPassword::new("seven tidy lanterns");
        let locked = state.submit_locked_thread(&[ThreadDraft::text(secret)], &ThreadAnchor::default(), &password, FAST).expect("locked").remove(0);
        let now = Timestamp::now();
        state.schedule_thread(&[ThreadDraft::text("Quetzalcoatl later")], &ThreadAnchor::default(), now + SignedDuration::from_hours(1), now).expect("scheduled");
        for query in ["quetzalcoatl", "orchard", "\"orchard confession\"", "confession is:locked"] {
            assert!(run(state, query, &meant(&[&locked])).is_empty(), "{query}");
        }
        assert_eq!(run(state, "is:locked", &[]), vec![locked.clone()], "only filters find it");
        assert!(run(state, "-quetzalcoatl", &meant(&[&locked])).is_empty());
    });
}

#[test]
fn saved_searches_survive_a_reload_of_the_vault() {
    use crate::domain::search_query::SavedSearches;
    let state = AppState::new_in_memory();
    assert!(state.saved_searches().items.is_empty());
    let mut saved = SavedSearches::default();
    saved.save("rye has:image");
    state.store_saved_searches(&saved);
    assert_eq!(state.saved_searches(), saved);
    let mut bare = AppState::new_in_memory();
    bare.db = None;
    assert!(bare.saved_searches().items.is_empty(), "nothing without a vault");
}

// Q6: old versions

fn with_versions(state: &AppState, query: &str) -> Vec<(PostId, Option<String>)> {
    let query = PostQuery::parse(query);
    let request = SearchRequest { query: &query, meaning: &[], limit: 50, today: date(2026, 11, 23), tz: TimeZone::UTC, include_versions: true };
    state.search(&request).into_iter().map(|hit| (hit.post_id, hit.earlier)).collect()
}

#[test]
fn old_versions_are_searched_only_when_asked_and_rank_after_current_text() {
    let mut state = AppState::new_in_memory();
    let edited = text(&mut state, "a zeppelin at dawn");
    state.edit_post(&edited, "a dirigible at dusk").expect("edited");
    let current = text(&mut state, "the zeppelin keeper");
    assert_eq!(run(&state, "zeppelin", &[]), vec![current.clone()], "off by default");
    assert_eq!(with_versions(&state, "zeppelin"), vec![(current, None), (edited.clone(), Some("a zeppelin at dawn".to_string()))]);
    assert_eq!(with_versions(&state, "dirigible"), vec![(edited.clone(), None)], "its current text matches first");
    assert_eq!(with_versions(&state, "zeppelin dawn -dirigible"), Vec::new(), "exclusions read the post as it is now");
}

#[test]
fn sealed_versions_of_a_locked_post_are_never_searched() {
    use crate::domain::post::lock::seal_text;
    use crate::state::PostEdit;
    let mut state = AppState::new_in_memory();
    let (secret, password) = ("Quetzalcoatl orchard confession", LockPassword::new("seven tidy lanterns"));
    let locked = state.submit_locked_thread(&[ThreadDraft::text(secret)], &ThreadAnchor::default(), &password, FAST).expect("locked").remove(0);
    let reworded = "Quetzalcoatl orchard, reworded";
    let sealed = seal_text(reworded, &password, FAST).expect("sealed");
    state.edit_locked_post(&locked, &PostEdit::text(reworded), sealed, secret, Timestamp::now()).expect("edited");
    for query in ["quetzalcoatl", "orchard", "confession", "reworded"] {
        assert!(with_versions(&state, query).is_empty(), "{query}");
        let expression = crate::storage::db::query_match_expression(&[query.to_string()], &[]).expect("terms");
        assert!(state.db.as_ref().expect("vault").score_revisions(&expression, 10).expect("read").is_empty(), "{query} is not indexed");
    }
}
