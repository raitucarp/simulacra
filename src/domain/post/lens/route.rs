//! N16: which agent takes which lens. Each lens goes to the agent whose knowledge (its name,
//! role, bio and topics, in words) shares most with the lens's own words; every agent takes
//! one lens at most, so a fan never asks more lenses than the room has agents.

use super::{FanEntry, Lens};
use crate::domain::post::question::words;
use crate::domain::ProfileId;

/// Words that say an agent knows the field of `lens`.
const fn lens_words(lens: Lens) -> &'static str {
    match lens {
        Lens::Ethics => "ethics ethicist moral morality philosophy philosopher justice values virtue rights",
        Lens::History => "history historian historical past archive heritage ancient medieval century",
        Lens::Economics => "economics economist economy market markets finance trade money cost price",
        Lens::Science => "science scientist research evidence physics biology chemistry experiment data",
        Lens::Psychology => "psychology psychologist mind behaviour behavior cognitive emotion therapist bias",
        Lens::Culture => "culture cultural anthropology anthropologist art arts literature language society",
        Lens::Ecology => "ecology ecologist environment climate nature biodiversity species sustainability",
        Lens::Law => "law lawyer legal justice court judge rights regulation policy",
    }
}

/// How well `knowledge` fits `lens`: the share of the lens's words it holds, in `[0, 1]`.
pub fn lens_fit(lens: Lens, knowledge: &str) -> f32 {
    let (wanted, known) = (words(lens_words(lens)), words(knowledge));
    let shared = wanted.iter().filter(|word| known.contains(*word)).count();
    if wanted.is_empty() { 0.0 } else { shared as f32 / wanted.len() as f32 }
}

/// The fan of `lenses` over `room` (each agent with its knowledge, in words): each lens in
/// turn takes the best-fitting agent not taken yet (ties to the earlier agent).
pub fn assign(lenses: &[Lens], room: &[(ProfileId, String)]) -> Vec<FanEntry> {
    let mut free: Vec<&(ProfileId, String)> = room.iter().collect();
    let mut entries = Vec::new();
    for lens in lenses.iter().copied() {
        let best = free.iter().enumerate().fold(None, |best: Option<(usize, f32)>, (at, (_, knowledge))| {
            let fit = lens_fit(lens, knowledge);
            if best.is_none_or(|(_, top)| fit > top) { Some((at, fit)) } else { best }
        });
        let Some((at, _)) = best else { break };
        entries.push(FanEntry { agent: free.remove(at).0.clone(), lens });
    }
    entries
}
