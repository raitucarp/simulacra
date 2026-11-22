//! How a kind is written: the `posts.kind_json` payload of a feeling, and the serde form of
//! [`PostKind`] inside a post's JSON (a plain key, as before kinds carried data, or
//! `{"feeling": {…}}`). Field names are persisted: never rename one.

use super::{Intensity, PostKind};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// A feeling's stored fields.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
struct FeelingPayload {
    emojis: Vec<String>,
    intensity: Intensity,
    #[serde(skip_serializing_if = "String::is_empty")]
    note: String,
}

/// A kind as written in a post's JSON.
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum Wire {
    Key(String),
    Feeling { feeling: FeelingPayload },
}

/// The feeling stored as `json` (no emojis when it cannot be read).
pub(super) fn feeling_from_json(json: &str) -> PostKind {
    let FeelingPayload { emojis, intensity, note } = serde_json::from_str(json).unwrap_or_default();
    PostKind::Feeling { emojis, intensity, note }
}

pub(super) fn payload_json(kind: &PostKind) -> String {
    match kind {
        PostKind::Feeling { emojis, intensity, note } => serde_json::to_string(&FeelingPayload { emojis: emojis.clone(), intensity: *intensity, note: note.clone() }).unwrap_or_default(),
        _ => String::new(),
    }
}

impl Serialize for PostKind {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Feeling { emojis, intensity, note } => Wire::Feeling { feeling: FeelingPayload { emojis: emojis.clone(), intensity: *intensity, note: note.clone() } }.serialize(serializer),
            other => Wire::Key(other.key().to_string()).serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for PostKind {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(match Wire::deserialize(deserializer)? {
            Wire::Key(key) => PostKind::from_stored(&key, ""),
            Wire::Feeling { feeling } => PostKind::Feeling { emojis: feeling.emojis, intensity: feeling.intensity, note: feeling.note },
        })
    }
}
