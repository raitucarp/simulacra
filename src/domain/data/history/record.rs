//! What a post's version keeps beside its text (V10, V11): its symbols, its tint and its
//! attachments, stored as the revision's record (`revisions.record_json`). The symbols are
//! the record's top level, so a record kept by an entry state move before tints were kept
//! (S7) reads the same. A post revision without a record (a text edit before V10) keeps
//! nothing to restore, and one without attachments (before V11) leaves them as they are.

use crate::domain::media::ImageAttachment;
use crate::domain::post::Tint;
use crate::domain::symbols::PostSymbols;
use crate::domain::Post;
use serde::{Deserialize, Serialize};

/// The link and images of one version of a post (V11).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Attachments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link_url: Option<String>,
    /// The first image (older posts carry only this).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_path: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub images: Vec<ImageAttachment>,
}

impl Attachments {
    /// A link and images as the composer sends them (the first image is the post's image).
    pub fn new(link_url: Option<String>, images: Vec<ImageAttachment>) -> Self {
        Self { link_url, image_path: images.first().map(|image| image.path.clone()), images }
    }

    /// Whether the version carries anything (a post may then have no text).
    pub fn any(&self) -> bool {
        self.link_url.is_some() || self.image_path.is_some() || !self.images.is_empty()
    }
}

/// The symbols, tint and attachments of one version of a post.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct PostRecord {
    #[serde(flatten)]
    pub symbols: PostSymbols,
    #[serde(skip_serializing_if = "Option::is_none", deserialize_with = "crate::domain::post::lenient_tint")]
    pub tint: Option<Tint>,
    /// `None` in records kept before attachments were (V11).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Attachments>,
}

impl PostRecord {
    /// What `post` shows beside its text.
    pub fn of(post: &Post) -> Self {
        let attachments = Attachments { link_url: post.link_url.clone(), image_path: post.image_path.clone(), images: post.extras.images.clone() };
        Self { symbols: post.extras.symbols.clone(), tint: post.extras.tint, attachments: Some(attachments) }
    }

    /// The record as stored.
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }

    /// The record stored as `json`; `None` when nothing was kept (or it cannot be read).
    pub fn parse(json: &str) -> Option<Self> {
        (!json.trim().is_empty()).then(|| serde_json::from_str(json).ok()).flatten()
    }

    /// `post` showing this record's symbols, tint and (when kept) attachments.
    pub fn apply(&self, post: &mut Post) {
        post.extras.symbols = self.symbols.clone();
        post.extras.tint = self.tint;
        if let Some(attachments) = &self.attachments {
            (post.link_url, post.image_path, post.extras.images) = (attachments.link_url.clone(), attachments.image_path.clone(), attachments.images.clone());
        }
    }
}

/// What changed beside the text between two versions' records (V10).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct RecordChanges {
    /// The tint before and after, when it changed.
    pub tint: Option<(Option<Tint>, Option<Tint>)>,
    pub symbols: bool,
    /// The link or the images changed (V11).
    pub attachments: bool,
}

/// The changes from record `from` to record `to` (stored JSON); nothing when either version
/// kept no record.
pub fn record_changes(from: &str, to: &str) -> RecordChanges {
    let (Some(a), Some(b)) = (PostRecord::parse(from), PostRecord::parse(to)) else { return RecordChanges::default() };
    let attachments = a.attachments.is_some() && b.attachments.is_some() && a.attachments != b.attachments;
    RecordChanges { tint: (a.tint != b.tint).then_some((a.tint, b.tint)), symbols: a.symbols != b.symbols, attachments }
}
