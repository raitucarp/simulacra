//! The text side of a search: ranked word and phrase matches, from the full-text index when
//! there is a vault and in memory otherwise (or when the index cannot answer), plus the
//! test for excluded words and phrases.

use super::TEXT_CANDIDATES;
use crate::domain::document::query::MIN_TERM_CHARS;
use crate::domain::search_query::PostQuery;
use crate::domain::{Post, PostId};
use crate::state::AppState;
use crate::storage::db::{query_match_expression, search_terms};

/// The active network's posts matching every word and phrase of `query`, best first (none
/// without words or phrases). Locked posts never match: only their placeholder is stored.
pub(super) fn ranked(state: &AppState, query: &PostQuery) -> Vec<PostId> {
    let Some(expression) = query_match_expression(&query.words, &query.phrases) else { return Vec::new() };
    let indexed = state.db.as_ref().and_then(|db| db.score_posts(&expression, TEXT_CANDIDATES).ok());
    let found: Vec<PostId> = match indexed {
        Some(found) => found.into_iter().map(|(id, _)| id).collect(),
        None => in_memory(state, query),
    };
    let wanted = |id: &PostId| state.get_post(id).is_some_and(|post| post.network_id == state.active_network_id && !post.is_locked());
    found.into_iter().filter(wanted).collect()
}

/// Word and phrase matching over the posts held in memory, most occurrences first.
fn in_memory(state: &AppState, query: &PostQuery) -> Vec<PostId> {
    let mut scored: Vec<(usize, &Post)> = state
        .current_network_posts()
        .filter_map(|post| {
            let text = Haystack::of(&post.content);
            let words: Option<usize> = query.words.iter().flat_map(|word| word_terms(word)).map(|term| text.prefixed(&term)).map(|n| (n > 0).then_some(n)).sum();
            let phrases: Option<usize> = query.phrases.iter().map(|phrase| text.phrase(phrase)).map(|n| (n > 0).then_some(n)).sum();
            Some((words? + phrases?, post))
        })
        .collect();
    scored.sort_by_key(|(count, _)| std::cmp::Reverse(*count));
    scored.into_iter().map(|(_, post)| post.id.clone()).collect()
}

fn word_terms(word: &str) -> impl Iterator<Item = String> + '_ {
    search_terms(word).filter(|term| term.chars().count() >= MIN_TERM_CHARS)
}

/// Whether `post` holds an excluded word or phrase of `query`.
pub(super) fn excluded(post: &Post, query: &PostQuery) -> bool {
    if query.excluded_words.is_empty() && query.excluded_phrases.is_empty() {
        return false;
    }
    let text = Haystack::of(&post.content);
    query.excluded_words.iter().flat_map(|word| word_terms(word)).any(|term| text.prefixed(&term) > 0)
        || query.excluded_phrases.iter().any(|phrase| text.phrase(phrase) > 0)
}

/// A text as its lower-cased terms.
struct Haystack {
    terms: Vec<String>,
    /// The terms joined by single spaces, with a space on both ends.
    joined: String,
}

impl Haystack {
    fn of(text: &str) -> Self {
        let terms: Vec<String> = search_terms(text).collect();
        let joined = format!(" {} ", terms.join(" "));
        Self { terms, joined }
    }

    /// How many terms start with `term`.
    fn prefixed(&self, term: &str) -> usize {
        self.terms.iter().filter(|t| t.starts_with(term)).count()
    }

    /// How many times `phrase`'s terms follow each other (0 for a phrase without terms).
    fn phrase(&self, phrase: &str) -> usize {
        let terms: Vec<String> = search_terms(phrase).collect();
        if terms.is_empty() {
            return 0;
        }
        self.joined.matches(&format!(" {} ", terms.join(" "))).count()
    }
}
