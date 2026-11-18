//! Multiple choice: the right answer among distractors taken from other cards' answers.
//! Plausible distractors make recognition effortful (a desirable difficulty), so they are
//! ranked by a shared tag, then by closeness in meaning of their sources (when an embedding
//! index exists), then by a similar length.

use super::recall::normalize_answer;
use super::shuffle::shuffled;
use crate::domain::review::card::{Card, CardId};

/// Distractors per question (four options in all).
pub const DISTRACTORS: usize = 3;
/// Weight of a shared tag against closeness in meaning (0–1).
const TAG_WEIGHT: f64 = 2.0;
/// Weight of a similar answer length.
const LENGTH_WEIGHT: f64 = 0.25;

/// One multiple-choice question.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChoiceQuestion {
    pub card_id: CardId,
    pub options: Vec<String>,
    /// Index of the right option.
    pub correct: usize,
}

/// How alike two answer lengths are (1 = same length).
fn length_closeness(a: &str, b: &str) -> f64 {
    let (a, b) = (a.chars().count() as f64, b.chars().count() as f64);
    if a.max(b) == 0.0 { 1.0 } else { a.min(b) / a.max(b) }
}

/// Up to [`DISTRACTORS`] answers from `pool`, best first, none equal to the right answer
/// or to each other. `affinity` scores how close in meaning a candidate's source is (0–1).
pub fn pick_distractors(card: &Card, pool: &[Card], affinity: &dyn Fn(&Card) -> f64) -> Vec<String> {
    let answer = card.kind.answer_text();
    let key = normalize_answer(&answer);
    let mut scored: Vec<(f64, String)> = pool
        .iter()
        .filter(|other| other.id != card.id)
        .map(|other| (other, other.kind.answer_text()))
        .filter(|(_, text)| !normalize_answer(text).is_empty() && normalize_answer(text) != key)
        .map(|(other, text)| {
            let tag = if card.shares_tag_with(other) { TAG_WEIGHT } else { 0.0 };
            (tag + affinity(other) + LENGTH_WEIGHT * length_closeness(&answer, &text), text)
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
pub fn build_choice(card: &Card, pool: &[Card], affinity: &dyn Fn(&Card) -> f64, seed: u64) -> Option<ChoiceQuestion> {
    let distractors = pick_distractors(card, pool, affinity);
    if distractors.is_empty() {
        return None;
    }
    let answer = card.kind.answer_text();
    let options = shuffled(std::iter::once(answer.clone()).chain(distractors).collect(), seed);
    let correct = options.iter().position(|option| *option == answer)?;
    Some(ChoiceQuestion { card_id: card.id.clone(), options, correct })
}
