//! The order of speaking, pure, from the setup alone:
//!
//! - **Oxford:** each round the sides alternate, For first, pairing speakers in setup order
//!   (For 1, Against 1, For 2, Against 2…); the first round opens, the last closes, the
//!   rounds between rebut.
//! - **Lincoln–Douglas:** each round the affirmative (For) speaks and is cross-examined by
//!   the negative, then the negative speaks and is cross-examined; closings follow, negative
//!   first, so the affirmative has the last word.
//! - **Socratic round-robin:** each round the next participant asks one question and every
//!   other participant answers it in turn.
//!
//! A moderator opens before the first round and wraps up after the last.

use super::model::{DebateFormat, DebateSetup, Side, Slot, TurnRole};
use crate::domain::ProfileId;

/// The role of a speech in `round` of `rounds`.
fn speech_role(round: u8, rounds: u8) -> TurnRole {
    match round {
        1 => TurnRole::Opening,
        last if last == rounds => TurnRole::Closing,
        _ => TurnRole::Rebuttal,
    }
}

fn slot(round: u8, speaker: &ProfileId, side: Side, role: TurnRole, addressee: Option<&ProfileId>) -> Slot {
    Slot { round, speaker: speaker.clone(), side, role, addressee: addressee.cloned() }
}

/// For and Against speakers interleaved.
fn alternating(setup: &DebateSetup) -> Vec<(&ProfileId, Side)> {
    let (pro, con) = (setup.on_side(Side::For), setup.on_side(Side::Against));
    (0..pro.len().max(con.len()))
        .flat_map(|i| [pro.get(i).map(|agent| (*agent, Side::For)), con.get(i).map(|agent| (*agent, Side::Against))])
        .flatten()
        .collect()
}

fn oxford(setup: &DebateSetup) -> Vec<Slot> {
    let order = alternating(setup);
    (1..=setup.rounds).flat_map(|round| order.iter().map(move |(agent, side)| slot(round, agent, *side, speech_role(round, setup.rounds), None)).collect::<Vec<_>>()).collect()
}

fn lincoln_douglas(setup: &DebateSetup) -> Vec<Slot> {
    let (Some(aff), Some(neg)) = (setup.on_side(Side::For).first().copied(), setup.on_side(Side::Against).first().copied()) else { return Vec::new() };
    let role = |round: u8| if round == 1 { TurnRole::Opening } else { TurnRole::Rebuttal };
    let rounds = (1..=setup.rounds).flat_map(|round| {
        [
            slot(round, aff, Side::For, role(round), None),
            slot(round, neg, Side::Against, TurnRole::CrossExamination, Some(aff)),
            slot(round, neg, Side::Against, role(round), None),
            slot(round, aff, Side::For, TurnRole::CrossExamination, Some(neg)),
        ]
    });
    let closings = [slot(setup.rounds, neg, Side::Against, TurnRole::Closing, None), slot(setup.rounds, aff, Side::For, TurnRole::Closing, None)];
    rounds.chain(closings).collect()
}

fn socratic(setup: &DebateSetup) -> Vec<Slot> {
    let people: Vec<&ProfileId> = setup.debaters.iter().map(|debater| &debater.agent).collect();
    if people.is_empty() {
        return Vec::new();
    }
    (1..=setup.rounds)
        .flat_map(|round| {
            let asker = people[usize::from(round - 1) % people.len()];
            let answers = people.iter().filter(move |agent| **agent != asker).map(move |agent| slot(round, agent, Side::Open, TurnRole::Answer, Some(asker)));
            std::iter::once(slot(round, asker, Side::Open, TurnRole::Question, None)).chain(answers).collect::<Vec<_>>()
        })
        .collect()
}

/// Every slot of the debate in speaking order.
pub fn schedule(setup: &DebateSetup) -> Vec<Slot> {
    let body = match setup.format {
        DebateFormat::Oxford => oxford(setup),
        DebateFormat::LincolnDouglas => lincoln_douglas(setup),
        DebateFormat::Socratic => socratic(setup),
    };
    let Some(moderator) = setup.moderator.as_ref() else { return body };
    let intro = slot(0, moderator, Side::Open, TurnRole::Moderation, None);
    let outro = slot(setup.rounds, moderator, Side::Open, TurnRole::Moderation, None);
    std::iter::once(intro).chain(body).chain(std::iter::once(outro)).collect()
}
