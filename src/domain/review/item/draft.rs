//! An item as the learner writes it on a post or a passage, its problems, the new item it
//! becomes and the edit of an existing one (the schedule survives an edit).

use super::{Anchor, AnchorSide, ItemId, ReviewItem, Subject};
use crate::domain::review::connections::ConnectionMap;
use crate::domain::review::schedule::ItemSchedule;
use crate::domain::NetworkId;
use jiff::Timestamp;
use jiff::civil::Date;

/// What the learner wrote on a source.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ItemDraft {
    pub anchor: Anchor,
    pub side: AnchorSide,
    /// The question (side Back) or the answer (side Front).
    pub written: String,
    pub tags: Vec<String>,
}

/// Why a draft cannot be saved yet.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DraftProblem {
    MissingQuestion,
    MissingAnswer,
    /// A passage anchor without any text.
    MissingPassage,
}

impl DraftProblem {
    /// Locale key of the message.
    pub const fn message_key(self) -> &'static str {
        match self {
            Self::MissingQuestion => "review.learn.problem_question",
            Self::MissingAnswer => "review.learn.problem_answer",
            Self::MissingPassage => "review.learn.problem_passage",
        }
    }
}

/// The first problem of `draft`, if any.
pub fn draft_problem(draft: &ItemDraft) -> Option<DraftProblem> {
    match (&draft.anchor, draft.side) {
        (Anchor::Passage { text, .. }, _) if text.trim().is_empty() => Some(DraftProblem::MissingPassage),
        (_, AnchorSide::Back) if draft.written.trim().is_empty() => Some(DraftProblem::MissingQuestion),
        (_, AnchorSide::Front) if draft.written.trim().is_empty() => Some(DraftProblem::MissingAnswer),
        _ => None,
    }
}

/// The network, the moment and the review day new items are created in.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NewItemContext {
    pub network_id: NetworkId,
    pub created_at: Timestamp,
    pub today: Date,
}

fn subject_of(draft: &ItemDraft) -> Subject {
    Subject::Anchored { anchor: draft.anchor.clone(), side: draft.side, written: draft.written.trim().to_string() }
}

/// The new item of `draft` (none while it has a problem).
pub fn new_item(draft: &ItemDraft, context: &NewItemContext, id: ItemId) -> Option<ReviewItem> {
    draft_problem(draft).is_none().then(|| ReviewItem {
        id,
        network_id: context.network_id.clone(),
        subject: subject_of(draft),
        tags: draft.tags.clone(),
        created_at: context.created_at,
        schedule: ItemSchedule::new_item(context.today),
        suspended: false,
        retired: false,
    })
}

/// A new item drilling `map` as a whole.
pub fn new_map_item(map: ConnectionMap, context: &NewItemContext, id: ItemId) -> ReviewItem {
    ReviewItem {
        id,
        network_id: context.network_id.clone(),
        subject: Subject::Connections { map },
        tags: Vec::new(),
        created_at: context.created_at,
        schedule: ItemSchedule::new_item(context.today),
        suspended: false,
        retired: false,
    }
}

/// `item` rewritten from `draft`, keeping its schedule and history (none while the draft
/// has a problem).
pub fn edited_item(item: &ReviewItem, draft: &ItemDraft) -> Option<ReviewItem> {
    draft_problem(draft).is_none().then(|| ReviewItem { subject: subject_of(draft), tags: draft.tags.clone(), ..item.clone() })
}

/// Tags typed as a comma-separated list: trimmed, without a leading `#`, deduplicated.
pub fn parse_tags(text: &str) -> Vec<String> {
    text.split(',').map(|tag| tag.trim().trim_start_matches('#').to_string()).filter(|tag| !tag.is_empty()).fold(Vec::new(), |mut tags, tag| {
        if !tags.iter().any(|t: &String| t.eq_ignore_ascii_case(&tag)) {
            tags.push(tag);
        }
        tags
    })
}
