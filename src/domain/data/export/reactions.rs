//! Reactions in the bundle (R10): `reactions/reactions.json` lists every reaction and clap
//! given to an exported post (with the reacting profile's handle, for people reading it),
//! and [`read_reactions`] reads such a file back for an import.

use crate::domain::reaction::{normalize_emoji, Reaction};
use crate::domain::{Profile, ProfileId};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Where the reactions live inside the bundle.
pub const REACTIONS_PATH: &str = "reactions/reactions.json";

/// One exported reaction: the stored row and who gave it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct ExportedReaction {
    #[serde(flatten)]
    reaction: Reaction,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    handle: String,
}

/// The JSON file of `rows`, each with its profile's handle when known.
pub fn reactions_json(rows: &[Reaction], authors: &HashMap<ProfileId, Profile>) -> String {
    let exported: Vec<ExportedReaction> = rows
        .iter()
        .map(|row| ExportedReaction { reaction: row.clone(), handle: authors.get(&row.profile_id).map(|p| p.handle.clone()).unwrap_or_default() })
        .collect();
    serde_json::to_string_pretty(&exported).unwrap_or_default()
}

/// The reactions of a bundle's file; rows without a count or with text that is not an emoji
/// are left out. `Err` when the file is not a reactions list.
pub fn read_reactions(text: &str) -> Result<Vec<Reaction>, String> {
    let exported: Vec<ExportedReaction> = serde_json::from_str(text).map_err(|err| err.to_string())?;
    Ok(exported
        .into_iter()
        .filter_map(|row| {
            let emoji = normalize_emoji(&row.reaction.emoji)?;
            (row.reaction.count > 0).then_some(Reaction { emoji, ..row.reaction })
        })
        .collect())
}
