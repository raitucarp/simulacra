//! [`AvatarMode`]: which of the four avatar kinds a profile shows.

use super::emblem::EmblemSpec;
use super::form::FormSpec;
use super::photo::PhotoCrop;
use super::spec::PortraitSpec;
use serde::{Deserialize, Serialize};

/// A profile's avatar: a generated portrait, a layered non-human body, a personality emblem, or
/// an imported photo.
///
/// JSON: `{"mode":"pcg", ...spec}`, `{"mode":"form", ...spec}`, `{"mode":"emblem", ...spec}` or
/// `{"mode":"photo","path":"avatars/<id>.png","crop":{...}}`.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(tag = "mode", rename_all = "snake_case")]
pub enum AvatarMode {
    /// Procedurally generated layered portrait.
    Pcg(PortraitSpec),
    /// Layered non-human body (creature, plant, object, element, machine, place, abstraction).
    Form(FormSpec),
    /// Flat geometric badge derived from MBTI, Enneagram and temperament.
    Emblem(EmblemSpec),
    /// Imported photo. `path` is relative to the vault folder (`avatars/<profile_id>.<ext>`);
    /// an absolute path (legacy or external) is used as-is.
    Photo {
        path: String,
        #[serde(default)]
        crop: PhotoCrop,
    },
}

impl AvatarMode {
    /// A photo avatar at `path` with the default (uncropped) framing.
    pub fn photo(path: impl Into<String>) -> Self {
        Self::Photo { path: path.into(), crop: PhotoCrop::default() }
    }
}
