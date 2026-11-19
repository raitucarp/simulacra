//! The "Your current beliefs" section of an agent's system prompt: the most salient beliefs
//! with their credence, at most [`DOSSIER_BELIEFS`] of them and at most
//! [`DOSSIER_TOKEN_BUDGET`] estimated tokens, so the dossier stays small.

use super::book::BeliefBook;
use crate::domain::embedding::estimate_tokens;
use crate::domain::ProfileId;
use jiff::Timestamp;

/// Beliefs listed at most.
pub const DOSSIER_BELIEFS: usize = 8;
/// Estimated tokens the section may take at most (heading included).
pub const DOSSIER_TOKEN_BUDGET: u32 = 260;
const HEADING: &str = "[YOUR CURRENT BELIEFS]\nYour present views, each with how sure you are (0 = surely false, 1 = surely true). \
Speak from them, and revise them openly when you meet a good reason.";

/// One line of the section: `- (0.72) proposition`.
fn line(proposition: &str, credence: f32) -> String {
    format!("- ({credence:.2}) {proposition}")
}

/// The section for `agent`, or an empty string when it holds no beliefs.
pub fn beliefs_section(book: &BeliefBook, agent: &ProfileId, now: Timestamp) -> String {
    let lines: Vec<String> = book.ranked(agent, now).into_iter().take(DOSSIER_BELIEFS).map(|belief| line(&belief.proposition, belief.credence.unit())).collect();
    if lines.is_empty() {
        return String::new();
    }
    let mut kept: Vec<&str> = Vec::new();
    let mut spent = estimate_tokens([HEADING]);
    for text in &lines {
        let cost = estimate_tokens([text.as_str()]);
        if spent.saturating_add(cost) > DOSSIER_TOKEN_BUDGET {
            break;
        }
        spent += cost;
        kept.push(text);
    }
    if kept.is_empty() {
        return String::new();
    }
    format!("{HEADING}\n{}", kept.join("\n"))
}
