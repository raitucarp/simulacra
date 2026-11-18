//! Concept candidates of a post for "search concept": wikilinked titles and hashtags first
//! (the author marked them), then capitalised phrases (names, terms of art), then the longest
//! content words. Pure and language-tolerant: scripts without case simply skip the middle step.

use crate::domain::post::{HASHTAG_SIGIL, WIKILINK_CLOSE, WIKILINK_OPEN};

/// Candidates offered at most.
pub const MAX_CONCEPTS: usize = 8;
/// Shortest content word (in characters) worth offering.
const MIN_CONTENT_WORD_CHARS: usize = 7;
/// Longest capitalised phrase, in words.
const MAX_PHRASE_WORDS: usize = 4;
/// Words never offered as a concept on their own.
const STOPWORDS: &[&str] = &[
    "the", "a", "an", "and", "or", "but", "if", "then", "of", "to", "in", "on", "for", "with", "is", "are", "was",
    "were", "be", "it", "this", "that", "these", "those", "i", "we", "you", "they", "he", "she", "my", "our", "your",
    "not", "no", "so", "as", "at", "by", "from", "about", "because", "however", "therefore", "although", "without",
    "between", "through", "something", "everything", "nothing", "anything", "whatever", "another", "sometimes",
    "even", "also", "yet", "still", "perhaps", "maybe", "today", "yesterday", "here", "there", "when", "while", "what",
    "why", "how", "who", "which", "every", "each", "some", "many", "most", "more", "much", "very", "just", "only",
];
/// Possessive endings dropped from a word ("Spinoza's" is about "Spinoza").
const POSSESSIVES: &[&str] = &["'s", "’s"];
/// Apostrophes kept inside a word until its possessive ending is dropped.
const APOSTROPHES: [char; 2] = ['\'', '\u{2019}'];

/// Up to [`MAX_CONCEPTS`] concepts of `text`; wikilinks for which `is_post_link` holds (links
/// to other thoughts, not to terms) are skipped.
pub fn extract_concepts(text: &str, is_post_link: impl Fn(&str) -> bool) -> Vec<String> {
    let links = wikilinks(text).into_iter().filter(|link| !is_post_link(link));
    let tags = hashtags(text);
    let candidates = links.chain(tags).chain(capitalised_phrases(text)).chain(content_words(text));
    let mut seen: Vec<String> = Vec::new();
    candidates
        .filter(|concept| {
            let key = concept.to_lowercase();
            let fresh = !key.is_empty() && !seen.contains(&key);
            seen.push(key);
            fresh
        })
        .take(MAX_CONCEPTS)
        .collect()
}

fn wikilinks(text: &str) -> Vec<String> {
    text.split(WIKILINK_OPEN)
        .skip(1)
        .filter_map(|part| part.find(WIKILINK_CLOSE).map(|end| part[..end].trim().to_string()))
        .filter(|link| !link.is_empty())
        .collect()
}

fn hashtags(text: &str) -> impl Iterator<Item = String> + '_ {
    text.split_whitespace()
        .filter_map(|word| word.strip_prefix(HASHTAG_SIGIL))
        .map(|tag| tag.trim_matches(|c: char| !c.is_alphanumeric()).replace(['_', '-'], " "))
        .filter(|tag| !tag.is_empty())
}

/// The words of `text` with surrounding punctuation removed, and whether each ends a sentence.
fn words(text: &str) -> Vec<(String, bool)> {
    let without_links = text.replace(WIKILINK_OPEN, " ").replace(WIKILINK_CLOSE, " ");
    without_links
        .split_whitespace()
        .filter(|word| !word.starts_with(HASHTAG_SIGIL) && !word.starts_with('@') && !word.contains("://"))
        .map(|word| {
            let ends_sentence = word.ends_with(['.', '!', '?', ':', ';']);
            let bare = word.trim_matches(|c: char| !c.is_alphanumeric() && c != '-' && !APOSTROPHES.contains(&c));
            let bare = POSSESSIVES.iter().find_map(|ending| bare.strip_suffix(ending)).unwrap_or(bare);
            (bare.trim_matches(|c: char| !c.is_alphanumeric() && c != '-').to_string(), ends_sentence)
        })
        .filter(|(word, _)| !word.is_empty())
        .collect()
}

fn is_capitalised(word: &str) -> bool {
    word.chars().next().is_some_and(char::is_uppercase)
}

fn is_stopword(word: &str) -> bool {
    STOPWORDS.contains(&word.to_lowercase().as_str())
}

/// Runs of capitalised words; a lone capital that merely opens a sentence is skipped.
fn capitalised_phrases(text: &str) -> Vec<String> {
    let words = words(text);
    let is_term = |at: usize| words.get(at).is_some_and(|(word, _)| is_capitalised(word) && !is_stopword(word));
    let mut phrases = Vec::new();
    let mut run: Vec<&str> = Vec::new();
    for (at, (word, ends_sentence)) in words.iter().enumerate() {
        let opens_sentence = at == 0 || words[at - 1].1;
        let lone_opener = opens_sentence && run.is_empty() && (*ends_sentence || !is_term(at + 1));
        let joins = is_term(at) && !lone_opener && run.len() < MAX_PHRASE_WORDS;
        if joins {
            run.push(word);
        }
        if (!joins || *ends_sentence) && !run.is_empty() {
            phrases.push(run.join(" "));
            run.clear();
        }
    }
    phrases.extend((!run.is_empty()).then(|| run.join(" ")));
    phrases
}

/// Long content words, most frequent first, then by first appearance.
fn content_words(text: &str) -> Vec<String> {
    let mut counted: Vec<(String, usize, usize)> = Vec::new();
    for (position, (word, _)) in words(text).into_iter().enumerate() {
        let lower = word.to_lowercase();
        if lower.chars().count() < MIN_CONTENT_WORD_CHARS || is_stopword(&lower) || lower.chars().all(|c| c.is_ascii_digit()) {
            continue;
        }
        match counted.iter_mut().find(|(w, _, _)| *w == lower) {
            Some(entry) => entry.1 += 1,
            None => counted.push((lower, 1, position)),
        }
    }
    counted.sort_by(|a, b| b.1.cmp(&a.1).then(a.2.cmp(&b.2)));
    counted.into_iter().map(|(word, _, _)| word).collect()
}
