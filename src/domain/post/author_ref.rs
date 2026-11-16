//! Deserialization of a post's author reference: a bare id, or (legacy JSON) the embedded
//! author profile, from which only the id is kept.

use crate::domain::ids::ProfileId;
use serde::{Deserialize, Deserializer};

#[derive(Deserialize)]
#[serde(untagged)]
enum AuthorRef {
    Id(ProfileId),
    Snapshot { id: ProfileId },
}

/// Accepts `"author_id": "u1"` as well as the legacy `"author": { "id": "u1", ... }`.
pub(super) fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<ProfileId, D::Error> {
    Ok(match AuthorRef::deserialize(deserializer)? {
        AuthorRef::Id(id) | AuthorRef::Snapshot { id } => id,
    })
}
