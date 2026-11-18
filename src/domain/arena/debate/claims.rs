//! Which claims still stand: the transcript is mapped with the thinking tools' argument
//! extraction (each turn is one "post", cited by its turn reference) and labelled with their
//! grounded semantics, so the page lists every claim with its speaker and standing.

use super::record::Debate;
use crate::domain::thinking::argument::{analyse, ArgumentMap, NodeRole, Standing, ThreadPost};
use crate::domain::ProfileId;

/// One claim of the debate and where it stands.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClaimStanding {
    pub text: String,
    pub speaker: Option<ProfileId>,
    pub standing: Standing,
    /// Index of the turn it was made in.
    pub turn: Option<usize>,
}

/// The turns as the extraction's thread posts, in speaking order.
pub fn turn_posts(debate: &Debate, name: &dyn Fn(&ProfileId) -> String) -> Vec<ThreadPost> {
    debate.turns.iter().enumerate().map(|(index, turn)| ThreadPost { id: debate.turn_ref(index), author: name(&turn.slot.speaker), text: turn.text.clone() }).collect()
}

/// The turn index `map` node sources point at.
fn turn_of(debate: &Debate, source: &str) -> Option<usize> {
    (0..debate.turns.len()).find(|index| debate.turn_ref(*index).as_str() == source)
}

/// Every claim of `map` with its speaker and standing: standing claims first, then contested,
/// then defeated, each in transcript order.
pub fn claim_standings(debate: &Debate, map: &ArgumentMap) -> Vec<ClaimStanding> {
    let report = analyse(map);
    let mut claims: Vec<ClaimStanding> = map
        .nodes
        .iter()
        .zip(&report.standing)
        .filter(|(node, _)| node.role == NodeRole::Claim)
        .map(|(node, standing)| {
            let turn = turn_of(debate, node.source.as_str());
            let speaker = turn.and_then(|index| debate.turns.get(index)).map(|turn| turn.slot.speaker.clone());
            ClaimStanding { text: node.text.clone(), speaker, standing: *standing, turn }
        })
        .collect();
    let rank = |standing: Standing| match standing {
        Standing::Accepted => 0,
        Standing::Undecided => 1,
        Standing::Rejected => 2,
    };
    claims.sort_by_key(|claim| (rank(claim.standing), claim.turn));
    claims
}
