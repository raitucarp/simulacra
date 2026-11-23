//! N17: the index of defined terms, and where a run of words mentions them (the longest term
//! first; a plural "-s" of a term's last word still mentions it).

use super::{term_key, word_key};
use crate::domain::{Post, PostId};
use std::collections::HashMap;
use std::rc::Rc;

/// One defined term.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GlossaryEntry {
    pub term: String,
    pub text: String,
    /// The post that defines it.
    pub post: PostId,
}

/// Every defined term, by its key.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Glossary {
    entries: HashMap<String, Rc<GlossaryEntry>>,
    /// The most words a term has.
    longest: usize,
}

impl Glossary {
    /// The terms `posts` define; a locked post defines none, and the newest of two
    /// definitions of one term wins.
    pub fn from_posts<'a>(posts: impl IntoIterator<Item = &'a Post>) -> Self {
        let mut defining: Vec<&Post> = posts.into_iter().filter(|post| !post.is_locked() && post.extras.defines.is_some()).collect();
        defining.sort_by_key(|post| post.created_at);
        let mut glossary = Self::default();
        for post in defining {
            let Some(definition) = post.extras.defines.as_deref() else { continue };
            let key = term_key(&definition.term);
            glossary.longest = glossary.longest.max(key.split(' ').count());
            glossary.entries.insert(key, Rc::new(GlossaryEntry { term: definition.term.clone(), text: definition.text.clone(), post: post.id.clone() }));
        }
        glossary
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// The entry of `term` (in any case, with any punctuation around it).
    pub fn lookup(&self, term: &str) -> Option<&Rc<GlossaryEntry>> {
        self.entries.get(&term_key(term))
    }

    /// Every entry, by term.
    pub fn entries(&self) -> Vec<&Rc<GlossaryEntry>> {
        let mut all: Vec<_> = self.entries.values().collect();
        all.sort_by_key(|entry| entry.term.to_lowercase());
        all
    }

    fn entry_of(&self, keys: &[String]) -> Option<&Rc<GlossaryEntry>> {
        if keys.iter().any(String::is_empty) {
            return None;
        }
        let key = keys.join(" ");
        let singular = key.strip_suffix('s').filter(|_| key.len() > 3);
        self.entries.get(&key).or_else(|| singular.and_then(|key| self.entries.get(key)))
    }

    /// For each of `words` (in order, as written), the term it is part of, if any.
    pub fn mark_words(&self, words: &[&str]) -> Vec<Option<Rc<GlossaryEntry>>> {
        let keys: Vec<String> = words.iter().map(|word| word_key(word)).collect();
        let mut marks = vec![None; words.len()];
        let mut at = 0;
        while at < words.len() {
            let widest = self.longest.min(words.len() - at);
            let found = (1..=widest).rev().find_map(|len| self.entry_of(&keys[at..at + len]).map(|entry| (len, entry.clone())));
            let Some((len, entry)) = found else {
                at += 1;
                continue;
            };
            marks[at..at + len].iter_mut().for_each(|mark| *mark = Some(entry.clone()));
            at += len;
        }
        marks
    }

    /// The distinct entries `text` mentions, in order.
    pub fn mentions(&self, text: &str) -> Vec<Rc<GlossaryEntry>> {
        let words: Vec<&str> = text.split_whitespace().collect();
        let mut found: Vec<Rc<GlossaryEntry>> = Vec::new();
        for entry in self.mark_words(&words).into_iter().flatten() {
            if !found.iter().any(|seen| seen.post == entry.post) {
                found.push(entry);
            }
        }
        found
    }
}
