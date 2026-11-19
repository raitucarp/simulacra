//! A journal entry: Markdown text written on a day (wikilinks to posts and vault documents
//! use the stream's `[[...]]` syntax), with the prompt or template it started from. A day
//! usually has one entry; more are allowed.

use super::ids::RecordId;
use super::prompts::Prompt;
use super::templates::Template;
use jiff::Timestamp;
use jiff::civil::Date;
use serde::{Deserialize, Serialize};

/// Characters of the first line shown as an entry's title.
pub const TITLE_CHARS: usize = 80;
/// Marks a shortened title.
const ELLIPSIS: char = '…';
/// Markdown heading marks stripped from a title.
const HEADING_MARKS: &[char] = &['#', ' '];

/// One entry of the journal.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JournalEntry {
    pub id: RecordId,
    /// The journal day it belongs to (local, after the night's rollover).
    pub day: Date,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
    pub body: String,
    #[serde(default)]
    pub prompt: Option<Prompt>,
    #[serde(default)]
    pub template: Option<Template>,
}

impl super::records::Record for JournalEntry {
    fn id(&self) -> &RecordId {
        &self.id
    }
}

impl JournalEntry {
    /// A new entry on `day`.
    pub fn new(id: RecordId, day: Date, now: Timestamp, body: impl Into<String>) -> Self {
        Self { id, day, created_at: now, updated_at: now, body: body.into(), prompt: None, template: None }
    }

    /// The entry with `body` written at `now` (the day and creation time stay).
    pub fn edited(&self, body: impl Into<String>, now: Timestamp) -> Self {
        Self { body: body.into(), updated_at: now, ..self.clone() }
    }

    /// Whether nothing but whitespace was written.
    pub fn is_blank(&self) -> bool {
        self.body.trim().is_empty()
    }

    /// The first written line without heading marks, shortened to [`TITLE_CHARS`].
    pub fn title(&self) -> String {
        let line = self.body.lines().map(|line| line.trim_start_matches(HEADING_MARKS).trim()).find(|line| !line.is_empty()).unwrap_or_default();
        match line.char_indices().nth(TITLE_CHARS) {
            Some((cut, _)) => format!("{}{ELLIPSIS}", line[..cut].trim_end()),
            None => line.to_string(),
        }
    }

    /// Words written (whitespace separated).
    pub fn word_count(&self) -> usize {
        self.body.split_whitespace().count()
    }
}

/// Entries newest first (by day, then creation time).
pub fn newest_first(entries: &[JournalEntry]) -> Vec<&JournalEntry> {
    let mut sorted: Vec<&JournalEntry> = entries.iter().collect();
    sorted.sort_by(|a, b| b.day.cmp(&a.day).then(b.created_at.cmp(&a.created_at)));
    sorted
}

/// The entries of `day`, oldest first.
pub fn entries_on(entries: &[JournalEntry], day: Date) -> Vec<&JournalEntry> {
    let mut on_day: Vec<&JournalEntry> = entries.iter().filter(|entry| entry.day == day).collect();
    on_day.sort_by_key(|entry| entry.created_at);
    on_day
}
