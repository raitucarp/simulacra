//! The material and the dialogue so far as quoted data for the prompt: the source between
//! `<material>` tags, each exchange with its level and move, the learner's words between
//! `<learner>` tags (never instructions).

use super::source::cut;
use super::{Exchange, TutorSession};

/// Exchanges quoted at most (the most recent).
pub const MAX_QUOTED_EXCHANGES: usize = 8;
/// Longest quoted answer, in characters.
pub const MAX_QUOTED_ANSWER_CHARS: usize = 1_200;

/// The source block.
pub fn material_block(session: &TutorSession) -> String {
    format!("<material title=\"{}\">\n{}\n</material>", session.title.replace('"', "'"), session.material.trim())
}

fn exchange_block(exchange: &Exchange) -> String {
    let question = &exchange.question;
    let answer = exchange.answer.as_deref().map_or_else(|| "(no answer yet)".to_string(), |text| format!("<learner>{}</learner>", cut(text, MAX_QUOTED_ANSWER_CHARS)));
    let assessment = exchange.assessment.as_ref().map(|a| {
        let reached = a.reached.map_or("none", |level| level.key());
        format!("\nAssessed: {} (reached {reached})", a.verdict.key())
    });
    format!("<exchange level=\"{}\" move=\"{}\">\nTutor: {}\nLearner: {answer}{}\n</exchange>", question.level.key(), question.kind.key(), question.text, assessment.unwrap_or_default())
}

/// The last [`MAX_QUOTED_EXCHANGES`] exchanges, oldest first.
pub fn dialogue_block(exchanges: &[Exchange]) -> String {
    let skip = exchanges.len().saturating_sub(MAX_QUOTED_EXCHANGES);
    let blocks: Vec<String> = exchanges.iter().skip(skip).map(exchange_block).collect();
    format!("<dialogue>\n{}\n</dialogue>", blocks.join("\n"))
}
