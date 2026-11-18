//! The transcript as prompts quote it: each turn labelled with its round, speaker, side and
//! role, the speakers labelled `D1…Dn` in setup order so a judge can name them. Long
//! transcripts keep the newest turns whole and shorten older ones.

use super::model::{DebateSetup, Turn};
use crate::domain::arena::reply::quoted;
use crate::domain::ProfileId;

/// Label prefix of a debater (`D1` is the first in setup order).
pub const DEBATER_LABEL_PREFIX: &str = "D";
/// Characters of the whole transcript at most, and of an older turn once shortened.
pub const MAX_TRANSCRIPT_CHARS: usize = 9_000;
const SHORT_TURN_CHARS: usize = 320;
/// Newest turns always quoted whole.
const WHOLE_TURNS: usize = 4;
const ELLIPSIS: char = '…';

/// The label of the debater at `index` (0-based).
pub fn debater_label(index: usize) -> String {
    format!("{DEBATER_LABEL_PREFIX}{}", index + 1)
}

/// The label of `agent` in `setup` (`MOD` for the moderator).
pub fn speaker_label(setup: &DebateSetup, agent: &ProfileId) -> String {
    setup.debaters.iter().position(|debater| &debater.agent == agent).map(debater_label).unwrap_or_else(|| "MOD".to_string())
}

/// The agent a `D2`-style label names.
pub fn labelled_agent(setup: &DebateSetup, label: &str) -> Option<ProfileId> {
    let digits = label.trim().trim_start_matches(DEBATER_LABEL_PREFIX).trim_start_matches(&DEBATER_LABEL_PREFIX.to_lowercase());
    let index = digits.parse::<usize>().ok()?.checked_sub(1)?;
    setup.debaters.get(index).map(|debater| debater.agent.clone())
}

fn shortened(text: &str) -> String {
    if text.chars().count() <= SHORT_TURN_CHARS {
        return text.to_string();
    }
    let mut short: String = text.chars().take(SHORT_TURN_CHARS).collect();
    short.push(ELLIPSIS);
    short
}

/// One turn as `[R2 · D1 Ada · for · rebuttal] text`.
fn line(setup: &DebateSetup, turn: &Turn, name: &dyn Fn(&ProfileId) -> String, whole: bool) -> String {
    let slot = &turn.slot;
    let text = if whole { turn.text.clone() } else { shortened(&turn.text) };
    let label = speaker_label(setup, &slot.speaker);
    format!("[R{} · {label} {} · {} · {}] {}", slot.round, quoted(&name(&slot.speaker)), slot.side.key(), slot.role.key().replace('_', " "), quoted(text.trim()))
}

/// The transcript of `turns`; `name` gives a speaker's display name.
pub fn render_transcript(setup: &DebateSetup, turns: &[Turn], name: &dyn Fn(&ProfileId) -> String) -> String {
    let whole_from = turns.len().saturating_sub(WHOLE_TURNS);
    let lines: Vec<String> = turns.iter().enumerate().map(|(index, turn)| line(setup, turn, name, index >= whole_from)).collect();
    let joined = lines.join("\n\n");
    let skip = joined.chars().count().saturating_sub(MAX_TRANSCRIPT_CHARS);
    joined.chars().skip(skip).collect()
}

/// The debaters as `D1 = Ada (for)` lines.
pub fn roster(setup: &DebateSetup, name: &dyn Fn(&ProfileId) -> String) -> String {
    let lines: Vec<String> = setup.debaters.iter().enumerate().map(|(index, debater)| format!("{} = {} ({})", debater_label(index), quoted(&name(&debater.agent)), debater.side.key())).collect();
    lines.join("\n")
}
