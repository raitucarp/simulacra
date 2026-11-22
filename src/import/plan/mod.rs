//! Pure planning: selected units become post drafts (threads when a unit is longer than one
//! post), each citing its source document, chapter and paragraph, in reading order. A dry
//! run counts what applying the plan would create.

mod author;
mod posts;

#[cfg(test)]
mod tests;

pub use author::{pseudonym_handle, AuthorChoice};
pub use posts::{build_posts, IMPORT_POST_SPACING_MS};

use super::segment::Unit;
use crate::domain::post::{SourceKind, SourceLocator, SourceRef};
use crate::domain::{split_into_thread, MAX_POST_CHARS};

/// Separator between the parts of a citation label.
const CITATION_SEPARATOR: &str = " · ";
/// Marks a chapter without a title, and a paragraph number.
const SECTION_SIGN: &str = "§";
const PARAGRAPH_SIGN: &str = "¶";

/// The vault document the posts cite.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceInfo {
    pub document_id: String,
    pub title: String,
    /// Title of each chapter (section), by index.
    pub chapter_titles: Vec<Option<String>>,
}

/// One unit as a thread of post texts with its citation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PostDraft {
    pub thread: Vec<String>,
    pub source: SourceRef,
}

/// Every draft, in reading order.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ImportPlan {
    pub drafts: Vec<PostDraft>,
}

/// What applying a plan would create.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct DryRun {
    pub units: usize,
    pub posts: usize,
    /// Drafts that need more than one post.
    pub threads: usize,
    pub characters: usize,
}

/// The per-post limit: the preference, never above what a post may hold.
pub fn effective_post_limit(preferred: u32) -> usize {
    (preferred as usize).clamp(1, MAX_POST_CHARS)
}

/// The citation of `unit`: a vault-document source narrowed to its chapter and paragraph.
pub fn cite(unit: &Unit, source: &SourceInfo) -> SourceRef {
    let position = unit.position;
    let one_based = |index: usize| u32::try_from(index + 1).unwrap_or(u32::MAX);
    let chapter_title = source.chapter_titles.get(position.chapter).cloned().flatten();
    let locator = SourceLocator {
        chapter: one_based(position.chapter),
        chapter_title: chapter_title.clone(),
        paragraph: position.paragraph.map(one_based),
        sentence: position.sentence.map(one_based),
    };
    let mut label = vec![source.title.clone(), chapter_title.unwrap_or_else(|| format!("{SECTION_SIGN}{}", locator.chapter))];
    if let Some(paragraph) = locator.paragraph {
        let sentence = locator.sentence.map(|s| format!(".{s}")).unwrap_or_default();
        label.push(format!("{PARAGRAPH_SIGN}{paragraph}{sentence}"));
    }
    SourceRef::new(SourceKind::VaultDoc, source.document_id.clone(), label.join(CITATION_SEPARATOR)).at(locator)
}

/// Drafts for `units` (already filtered to the selection), split at `post_limit` characters.
pub fn plan_posts<'a>(units: impl IntoIterator<Item = &'a Unit>, source: &SourceInfo, post_limit: usize) -> ImportPlan {
    let drafts = units
        .into_iter()
        .map(|unit| PostDraft { thread: unit_thread(&unit.text, post_limit), source: cite(unit, source) })
        .filter(|draft| !draft.thread.is_empty())
        .collect();
    ImportPlan { drafts }
}

/// The posts of one unit: an exported locked post's seal stays whole (X6), anything else is
/// split at `post_limit` characters.
fn unit_thread(text: &str, post_limit: usize) -> Vec<String> {
    match crate::domain::post::lock::read_lock_block(text) {
        Some(_) => vec![text.trim().to_string()],
        None => split_into_thread(text, post_limit),
    }
}

/// Counts of `plan`.
pub fn dry_run(plan: &ImportPlan) -> DryRun {
    plan.drafts.iter().fold(DryRun::default(), |run, draft| DryRun {
        units: run.units + 1,
        posts: run.posts + draft.thread.len(),
        threads: run.threads + usize::from(draft.thread.len() > 1),
        characters: run.characters + draft.thread.iter().map(|post| post.chars().count()).sum::<usize>(),
    })
}
