//! What sort of thought a post is. Persisted in the `posts.kind` column; posts stored before
//! kinds existed read back as [`PostKind::Standard`].

use crate::domain::document::keyed::keyed_enum;

keyed_enum! {
    /// The form a post takes. Keys are stored in `posts.kind`; never rename one.
    PostKind {
        /// An ordinary thought.
        Standard => "standard",
        /// A note on how the author feels.
        Feeling => "feeling",
        /// A question put to the network.
        Question => "question",
    }
}

impl Default for PostKind {
    fn default() -> Self {
        Self::Standard
    }
}

impl PostKind {
    /// Whether this is an ordinary thought (serde skips the field then).
    pub fn is_standard(&self) -> bool {
        *self == Self::Standard
    }

    /// The kind stored under `key` (`Standard` for anything unknown).
    pub fn from_stored(key: &str) -> Self {
        Self::from_key(key).unwrap_or_default()
    }
}
