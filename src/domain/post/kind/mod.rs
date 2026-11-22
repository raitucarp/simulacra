//! What sort of thought a post is. The kind's key is persisted in the `posts.kind` column and
//! a feeling's emojis, intensity and note in `posts.kind_json` (M1); posts stored before
//! kinds existed read back as [`PostKind::Standard`].

mod feeling;
mod wire;

#[cfg(test)]
mod tests;

pub use feeling::{FeelingProblem, Intensity, MAX_FEELING_EMOJIS, MAX_FEELING_NOTE_CHARS};

use crate::domain::document::keyed::keyed_enum;

keyed_enum! {
    /// The key of each kind. Stored in `posts.kind`; never rename one.
    KindKey {
        /// An ordinary thought.
        Standard => "standard",
        /// A note on how the author feels.
        Feeling => "feeling",
        /// A question put to the network.
        Question => "question",
    }
}

/// The form a post takes.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum PostKind {
    /// An ordinary thought.
    #[default]
    Standard,
    /// How the author feels (M1): one to [`MAX_FEELING_EMOJIS`] emojis, how strongly, and a
    /// short optional note.
    Feeling { emojis: Vec<String>, intensity: Intensity, note: String },
    /// A question put to the network.
    Question,
}

impl PostKind {
    /// The kind's key.
    pub fn kind_key(&self) -> KindKey {
        match self {
            Self::Standard => KindKey::Standard,
            Self::Feeling { .. } => KindKey::Feeling,
            Self::Question => KindKey::Question,
        }
    }

    /// Stable key (the `posts.kind` column, locale keys).
    pub fn key(&self) -> &'static str {
        self.kind_key().key()
    }

    /// Whether this is an ordinary thought (serde skips the field then).
    pub fn is_standard(&self) -> bool {
        *self == Self::Standard
    }

    /// Whether this is a feeling.
    pub fn is_feeling(&self) -> bool {
        matches!(self, Self::Feeling { .. })
    }

    /// A validated feeling: emojis trimmed (blanks and repeats dropped), the note trimmed.
    pub fn feeling(emojis: &[String], intensity: Intensity, note: &str) -> Result<Self, FeelingProblem> {
        let (emojis, note) = feeling::checked(emojis, note)?;
        Ok(Self::Feeling { emojis, intensity, note })
    }

    /// A feeling's emojis and note as one line of text (`None` for other kinds, or a
    /// feeling without emojis).
    pub fn feeling_text(&self) -> Option<String> {
        let Self::Feeling { emojis, note, .. } = self else { return None };
        (!emojis.is_empty()).then(|| format!("{} {note}", emojis.concat()).trim().to_string())
    }

    /// The kind stored under `key` with the `payload` of `posts.kind_json` (`Standard` for
    /// an unknown key; a feeling whose payload cannot be read keeps no emojis).
    pub fn from_stored(key: &str, payload: &str) -> Self {
        match KindKey::from_key(key).unwrap_or(KindKey::Standard) {
            KindKey::Standard => Self::Standard,
            KindKey::Question => Self::Question,
            KindKey::Feeling => wire::feeling_from_json(payload),
        }
    }

    /// What `posts.kind_json` stores: a feeling's emojis, intensity and note; empty otherwise.
    pub fn payload_json(&self) -> String {
        wire::payload_json(self)
    }
}
