//! N9: ask the room. A question post goes to the one to three agents whose knowledge fits it
//! best: the question's embedding against each agent's interest vector (what it writes and
//! answers), or, while either is not embedded yet, the words they share. The post keeps the
//! agents asked with their fit ([`Question`], `extras.question`); their replies are its
//! answers, ranked by fit and by how they were received, and the user can accept one.

mod rank;

pub use rank::{rank_answers, Answer};

use crate::domain::embedding::cosine;
use crate::domain::{PostId, ProfileId};
use serde::{Deserialize, Serialize};

/// The most agents a question goes to.
pub const MAX_ASKED: usize = 3;
/// How close to the best fit another agent must be to be asked too.
const RUNNER_UP_SHARE: f32 = 0.8;
/// A fit stored in thousandths, so the record stays exact.
pub const FIT_SCALE: f32 = 1000.0;

/// One agent asked, with how well its knowledge fits the question (thousandths, 0..=1000).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Asked {
    pub agent: ProfileId,
    #[serde(default)]
    pub fit: u16,
}

/// A post asked of the room (`extras.question`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Question {
    /// The agents asked, best fit first.
    pub asked: Vec<Asked>,
    /// The answer the user accepted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accepted: Option<PostId>,
}

impl Question {
    /// The fit of `agent`'s answers (0 when it was not asked).
    pub fn fit_of(&self, agent: &ProfileId) -> u16 {
        self.asked.iter().find(|asked| &asked.agent == agent).map_or(0, |asked| asked.fit)
    }
}

fn permille(score: f32) -> u16 {
    (score.clamp(0.0, 1.0) * FIT_SCALE).round() as u16
}

/// The agents to ask, best first: the best fit, and up to [`MAX_ASKED`] in all whose fit is
/// close to it. `scored` is each agent's fit in `[0, 1]`; ties go to the earlier agent.
pub fn pick(scored: Vec<(ProfileId, f32)>) -> Vec<Asked> {
    let mut scored = scored;
    scored.sort_by(|a, b| b.1.total_cmp(&a.1));
    let best = scored.first().map_or(0.0, |(_, score)| *score);
    scored.into_iter().enumerate().filter(|(at, (_, score))| *at == 0 || (*score > 0.0 && *score >= best * RUNNER_UP_SHARE)).take(MAX_ASKED).map(|(_, (agent, score))| Asked { agent, fit: permille(score) }).collect()
}

/// Routing by embeddings: each agent's fit is the cosine of its vector and the question's.
pub fn route_by_meaning(question: &[f32], agents: &[(ProfileId, Vec<f32>)]) -> Vec<Asked> {
    pick(agents.iter().map(|(agent, vector)| (agent.clone(), cosine(question, vector).max(0.0))).collect())
}

/// Routing by words (until things are embedded): each agent's fit is the share of the
/// question's words its own text holds.
pub fn route_by_words(question: &str, agents: &[(ProfileId, String)]) -> Vec<Asked> {
    let wanted = words(question);
    let fit = |text: &str| {
        let known = words(text);
        let shared = wanted.iter().filter(|word| known.contains(*word)).count();
        if wanted.is_empty() { 0.0 } else { shared as f32 / wanted.len() as f32 }
    };
    pick(agents.iter().map(|(agent, text)| (agent.clone(), fit(text))).collect())
}

/// The lower-case words of `text` long enough to say something (three letters or more).
pub fn words(text: &str) -> std::collections::HashSet<String> {
    text.split(|c: char| !c.is_alphanumeric()).filter(|word| word.chars().count() >= 3).map(str::to_lowercase).collect()
}

#[cfg(test)]
mod tests;
