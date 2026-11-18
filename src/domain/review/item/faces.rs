//! The two faces of an item: which one is the source (shown as it appears in the stream or
//! as a quoted passage) and which one the learner wrote. A post face always resolves to the
//! post's current text, so an edited post is reviewed as edited.

use super::{Anchor, AnchorSide, ReviewItem, Subject};

/// One face of an item.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Face {
    /// The anchored post or passage itself.
    Source(Anchor),
    /// Text the learner wrote.
    Written(String),
    /// A connection map (drawn, not read).
    Map,
}

/// Both faces of an item.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ItemFaces {
    pub prompt: Face,
    pub answer: Face,
}

impl ItemFaces {
    pub fn of(subject: &Subject) -> Self {
        match subject {
            Subject::Anchored { anchor, side, written } => {
                let (source, written) = (Face::Source(anchor.clone()), Face::Written(written.trim().to_string()));
                match side {
                    AnchorSide::Front => Self { prompt: source, answer: written },
                    AnchorSide::Back => Self { prompt: written, answer: source },
                }
            }
            Subject::Connections { .. } => Self { prompt: Face::Map, answer: Face::Map },
        }
    }
}

impl Face {
    /// The face as plain text; `post_text` gives a post's current text (none once deleted).
    pub fn text(&self, post_text: &dyn Fn(&str) -> Option<String>) -> String {
        match self {
            Self::Source(Anchor::Post { id }) => post_text(id).unwrap_or_default().trim().to_string(),
            Self::Source(Anchor::Passage { text, .. }) => text.trim().to_string(),
            Self::Written(text) => text.clone(),
            Self::Map => String::new(),
        }
    }
}

impl ReviewItem {
    pub fn faces(&self) -> ItemFaces {
        ItemFaces::of(&self.subject)
    }

    /// The prompt as plain text.
    pub fn prompt_text(&self, post_text: &dyn Fn(&str) -> Option<String>) -> String {
        self.faces().prompt.text(post_text)
    }

    /// The answer as plain text (quiz options and typed-answer checks).
    pub fn answer_text(&self, post_text: &dyn Fn(&str) -> Option<String>) -> String {
        self.faces().answer.text(post_text)
    }
}
