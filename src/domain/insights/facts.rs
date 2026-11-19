//! The per-post and per-author facts the charts count, derived once from the store's posts
//! and profiles (local time from a fixed UTC offset, so the result is reproducible).

use crate::domain::{Post, Profile, ProfileId};
use jiff::Timestamp;
use jiff::civil::Date;
use jiff::tz::Offset;

/// What a post is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PostKind {
    Thought,
    Reply,
    Quote,
    Repost,
    /// Imported from a file or captured into the Inbox.
    Captured,
}

impl PostKind {
    pub const ALL: [Self; 5] = [Self::Thought, Self::Reply, Self::Quote, Self::Repost, Self::Captured];
    pub const COUNT: usize = Self::ALL.len();

    pub fn of(post: &Post) -> Self {
        if post.is_repost() {
            Self::Repost
        } else if post.is_reply() {
            Self::Reply
        } else if post.is_quote() {
            Self::Quote
        } else if post.extras.origin.is_some() {
            Self::Captured
        } else {
            Self::Thought
        }
    }

    pub const fn key(self) -> &'static str {
        match self {
            Self::Thought => "thought",
            Self::Reply => "reply",
            Self::Quote => "quote",
            Self::Repost => "repost",
            Self::Captured => "captured",
        }
    }

    pub fn index(self) -> usize {
        Self::ALL.iter().position(|k| *k == self).unwrap_or_default()
    }
}

/// An author of the analysed posts.
#[derive(Debug, Clone, PartialEq)]
pub struct AuthorFacts {
    pub id: ProfileId,
    pub name: String,
    pub is_agent: bool,
    /// Big Five plus honesty–humility (0–100), for agents.
    pub traits: Option<[u8; TRAIT_AXES]>,
}

/// Axes of the trait radar.
pub const TRAIT_AXES: usize = 6;
/// Locale suffixes of the radar axes, in value order.
pub const TRAIT_KEYS: [&str; TRAIT_AXES] = ["openness", "conscientiousness", "extraversion", "agreeableness", "neuroticism", "honesty"];

impl AuthorFacts {
    pub fn of(profile: &Profile) -> Self {
        let c = &profile.config;
        let traits = profile.is_agent().then_some([c.openness, c.conscientiousness, c.extraversion, c.agreeableness, c.neuroticism, c.honesty_humility]);
        Self { id: profile.id.clone(), name: profile.name.clone(), is_agent: profile.is_agent(), traits }
    }
}

/// One post as the charts see it.
#[derive(Debug, Clone, PartialEq)]
pub struct PostFacts {
    pub author: usize,
    pub kind: PostKind,
    pub created_at: Timestamp,
    pub day: Date,
    /// Monday = 0.
    pub weekday: usize,
    pub hour: usize,
    pub chars: usize,
    pub words: usize,
    /// Mean words per sentence (0 for text-less posts).
    pub words_per_sentence: f64,
}

const SENTENCE_ENDS: [char; 3] = ['.', '!', '?'];

impl PostFacts {
    /// Facts of `post` written by author `author`, in the local time of `offset`.
    pub fn of(post: &Post, author: usize, offset: Offset) -> Self {
        let local = offset.to_datetime(post.created_at);
        let words = post.content.split_whitespace().count();
        let sentences = post.content.split(SENTENCE_ENDS).filter(|s| !s.trim().is_empty()).count().max(1);
        Self {
            author,
            kind: PostKind::of(post),
            created_at: post.created_at,
            day: local.date(),
            weekday: local.weekday().to_monday_zero_offset() as usize,
            hour: local.hour() as usize,
            chars: post.content.chars().count(),
            words,
            words_per_sentence: if words == 0 { 0.0 } else { words as f64 / sentences as f64 },
        }
    }
}
