//! Multiple choice: the right answer among distractors taken from other items' answers.
//! Plausible distractors make recognition effortful (a desirable difficulty), so they are
//! ranked by a shared tag, then by closeness in meaning of their sources (when an embedding
//! index exists), then by a similar length.

use super::recall::normalize_answer;
use super::shuffle::shuffled;
use crate::domain::review::item::{ItemId, ReviewItem};

/// Distractors per question (four options in all).
pub const DISTRACTORS: usize = 3;
/// Weight of a shared tag against closeness in meaning (0–1).
const TAG_WEIGHT: f64 = 2.0;
/// Weight of a similar answer length.
const LENGTH_WEIGHT: f64 = 0.25;

/// One multiple-choice question.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChoiceQuestion {
    pub item_id: ItemId,
    pub options: Vec<String>,
    /// Index of the right option.
    pub correct: usize,
}

/// How alike two answer lengths are (1 = same length).
fn length_closeness(a: &str, b: &str) -> f64 {
    let (a, b) = (a.chars().count() as f64, b.chars().count() as f64);
    if a.max(b) == 0.0 { 1.0 } else { a.min(b) / a.max(b) }
}

/// How a question reads its items: an item's answer as text (posts resolve to their current
/// text) and how close in meaning its source is to the question's (0–1).
pub struct ChoiceSources<'a> {
    pub answer_of: &'a dyn Fn(&ReviewItem) -> String,
    pub affinity: &'a dyn Fn(&ReviewItem) -> f64,
}

/// Up to [`DISTRACTORS`] answers from `pool`, best first, none equal to the right answer
/// or to each other.
pub fn pick_distractors(card: &ReviewItem, pool: &[ReviewItem], sources: &ChoiceSources<'_>) -> Vec<String> {
    let answer = (sources.answer_of)(card);
    let key = normalize_answer(&answer);
    let mut scored: Vec<(f64, String)> = pool
        .iter()
        .filter(|other| other.id != card.id && other.subject.anchor().is_some())
        .map(|other| (other, (sources.answer_of)(other)))
        .filter(|(_, text)| !normalize_answer(text).is_empty() && normalize_answer(text) != key)
        .map(|(other, text)| {
            let tag = if card.shares_tag_with(other) { TAG_WEIGHT } else { 0.0 };
            (tag + (sources.affinity)(other) + LENGTH_WEIGHT * length_closeness(&answer, &text), text)
        })
        .collect();
    scored.sort_by(|a, b| b.0.total_cmp(&a.0).then_with(|| a.1.cmp(&b.1)));
    scored.into_iter().map(|(_, text)| text).fold(Vec::new(), |mut picked, text| {
        if picked.len() < DISTRACTORS && !picked.iter().any(|p: &String| normalize_answer(p) == normalize_answer(&text)) {
            picked.push(text);
        }
        picked
    })
}

/// The question for `card`, options in a seeded order; `None` without a single distractor.
pub fn build_choice(card: &ReviewItem, pool: &[ReviewItem], sources: &ChoiceSources<'_>, seed: u64) -> Option<ChoiceQuestion> {
    let distractors = pick_distractors(card, pool, sources);
    if distractors.is_empty() {
        return None;
    }
    let answer = (sources.answer_of)(card);
    let options = shuffled(std::iter::once(answer.clone()).chain(distractors).collect(), seed);
    let correct = options.iter().position(|option| *option == answer)?;
    Some(ChoiceQuestion { item_id: card.id.clone(), options, correct })
}
