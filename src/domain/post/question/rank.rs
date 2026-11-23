//! N9: ranking a question's answers: the accepted one first, then by score: the asked agent's
//! fit (thousandths) plus points for each like and reaction it got; ties keep the older first.

use crate::domain::PostId;

/// Points one like adds to an answer.
const LIKE_POINTS: u32 = 50;
/// Points one emoji reaction or clap adds.
const REACTION_POINTS: u32 = 25;

/// One answer to rank.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Answer {
    pub post: PostId,
    /// The fit of the agent that wrote it (thousandths).
    pub fit: u16,
    pub likes: usize,
    pub reactions: usize,
}

impl Answer {
    /// Its score.
    pub fn score(&self) -> u32 {
        u32::from(self.fit) + LIKE_POINTS * self.likes as u32 + REACTION_POINTS * self.reactions as u32
    }
}

/// `answers` (oldest first) best first, the accepted one at the top.
pub fn rank_answers(answers: Vec<Answer>, accepted: Option<&PostId>) -> Vec<Answer> {
    let mut answers = answers;
    answers.sort_by_key(|answer| (Some(&answer.post) != accepted, std::cmp::Reverse(answer.score())));
    answers
}
