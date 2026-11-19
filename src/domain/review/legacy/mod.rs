//! Carrying the retired standalone flashcards over to items anchored on their sources
//! (the user_version 5 migration). A card citing a post or a document passage becomes an
//! item on that source with its question kept and its schedule intact; free-form cards
//! (a typed sentence) and the reverse half of a concept pair are discarded. A post keeps
//! one item (the first card made on it).

mod cloze;

use super::item::{Anchor, AnchorSide, ItemId, ReviewItem, Subject};
use super::schedule::ItemSchedule;
use crate::domain::post::SourceLocator;
use crate::domain::NetworkId;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use std::collections::HashSet;

/// Direction of a legacy concept card asked as written (term shown, definition asked).
const FORWARD_CONCEPT: &str = "term_to_definition";

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum LegacyKind {
    Basic { front: String, back: String },
    Cloze { text: String, ordinal: u32 },
    Concept { term: String, definition: String, direction: String },
}

#[derive(Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
enum LegacySource {
    Post { id: String },
    Document {
        id: String,
        #[serde(default)]
        locator: Option<SourceLocator>,
    },
    Sentence {},
}

/// One `review_cards` row, columns as stored.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LegacyRow {
    pub id: String,
    pub network_id: String,
    pub kind_json: String,
    pub source_json: String,
    pub tags_json: String,
    pub created_at: DateTime<Utc>,
    pub schedule_json: String,
    pub suspended: bool,
}

/// The question written on the source and, for a passage, the passage text.
fn question_and_passage(kind: LegacyKind) -> Option<(String, String)> {
    match kind {
        LegacyKind::Basic { front, back } => Some((front, back)),
        LegacyKind::Cloze { text, ordinal } => Some((cloze::prompt_text(&text, ordinal), cloze::plain_text(&text))),
        LegacyKind::Concept { term, definition, direction } if direction == FORWARD_CONCEPT => Some((term, definition)),
        LegacyKind::Concept { .. } => None,
    }
}

/// The subject a legacy card becomes, if it is kept.
pub fn legacy_subject(kind_json: &str, source_json: &str) -> Option<Subject> {
    let kind: LegacyKind = serde_json::from_str(kind_json).ok()?;
    let source: LegacySource = serde_json::from_str(source_json).ok()?;
    let (question, passage) = question_and_passage(kind)?;
    let anchor = match source {
        LegacySource::Post { id } => Anchor::Post { id },
        LegacySource::Document { id, locator } => Anchor::passage(id, locator, passage.trim()),
        LegacySource::Sentence {} => return None,
    };
    let written = question.trim().to_string();
    (!written.is_empty()).then_some(Subject::Anchored { anchor, side: AnchorSide::Back, written })
}

fn item_of(row: &LegacyRow) -> Option<ReviewItem> {
    let subject = legacy_subject(&row.kind_json, &row.source_json)?;
    let schedule: ItemSchedule = serde_json::from_str(&row.schedule_json).ok()?;
    Some(ReviewItem {
        id: ItemId::new(row.id.clone()),
        network_id: NetworkId::new(row.network_id.clone()),
        subject,
        tags: serde_json::from_str(&row.tags_json).unwrap_or_default(),
        created_at: row.created_at,
        schedule,
        suspended: row.suspended,
        retired: false,
    })
}

/// The items `rows` become, oldest first, one per post.
pub fn migrated_items(rows: &[LegacyRow]) -> Vec<ReviewItem> {
    let mut sorted: Vec<&LegacyRow> = rows.iter().collect();
    sorted.sort_by(|a, b| a.created_at.cmp(&b.created_at).then_with(|| a.id.cmp(&b.id)));
    let mut posts_taken: HashSet<(String, String)> = HashSet::new();
    sorted
        .into_iter()
        .filter_map(item_of)
        .filter(|item| match item.subject.anchor() {
            Some(anchor @ Anchor::Post { .. }) => posts_taken.insert((item.network_id.as_str().to_string(), anchor.key())),
            _ => true,
        })
        .collect()
}
