//! The export bundle (sb F2): a zip of portable files.
//!
//! - `threads/*.md`: one Markdown file per thread (front matter, `[[wikilinks]]` kept), with
//!   the images it shows copied to `threads/media/`.
//! - `documents/*.md`: vault records as Markdown, and the kept original files in
//!   `documents/originals/` when asked for.
//! - `review/items.csv` and `review/items.json`: the review items.
//! - `agents/*.json`: agent and persona configurations, secrets redacted.
//! - `settings.json`: an explicit allow-list of settings; credentials and API keys never.
//! - `journal/journal.json`: only when "Include private journal" is ticked.
//! - `manifest.json`: format and app versions, scope and counts.
//!
//! [`plan_export`] decides what goes in (and is the dialog's preview); [`bundle_files`]
//! turns the plan into files. Writing the zip happens at the edge (`storage::export`).

mod bundle;
mod markdown;
mod plan;
mod secrets;
mod tables;
mod threads;

pub use bundle::{bundle_files, BundleFile, FileBody, MANIFEST_PATH};
pub use markdown::{document_markdown, slug, thread_markdown, thread_title};
pub use plan::{plan_export, Counts, ExportPlan, ThreadExport};
pub use secrets::{redacted_profile, SETTINGS_ALLOW_LIST};
pub use tables::{csv_field, review_csv};
pub use threads::thread_root;

use crate::domain::data::saved::CollectionEntry;
use crate::domain::document::{CollectionId, Library};
use crate::domain::journal::JournalBook;
use crate::domain::review::ReviewItem;
use crate::domain::{GroundingDocument, Post, Profile, ProfileId};
use chrono::{DateTime, NaiveDate, Utc};
use std::collections::HashMap;
use std::path::PathBuf;

/// Version of the bundle layout written in the manifest.
pub const BUNDLE_FORMAT_VERSION: u32 = 1;
/// `format` field of the manifest.
pub const BUNDLE_FORMAT: &str = "simulacra-export";

/// What to export.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExportScope {
    Everything,
    /// The posts and documents of one collection.
    Collection(CollectionId),
    /// Posts, documents and journal records of these days (inclusive, local calendar).
    DateRange { from: NaiveDate, to: NaiveDate },
    /// One persona's posts and the documents it added.
    Persona(ProfileId),
}

impl ExportScope {
    /// Stable key (manifest, locale keys `data.export.scope.<key>`).
    pub const fn key(&self) -> &'static str {
        match self {
            Self::Everything => "everything",
            Self::Collection(_) => "collection",
            Self::DateRange { .. } => "date_range",
            Self::Persona(_) => "persona",
        }
    }
}

/// The export dialog's choices.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExportOptions {
    pub scope: ExportScope,
    pub include_originals: bool,
    /// Off unless the user ticks "Include private journal".
    pub include_journal: bool,
}

/// Everything an export may draw from, copied off the store (so the zip is written off the
/// UI thread). `settings` holds only allow-listed rows; `journal` is `None` unless ticked.
#[derive(Debug, Clone)]
pub struct ExportSource {
    pub network_name: String,
    /// The active network's posts.
    pub posts: Vec<Post>,
    /// Every known author, archived ones included.
    pub authors: HashMap<ProfileId, Profile>,
    /// The active network's profiles (agents and personas).
    pub profiles: Vec<Profile>,
    pub documents: Vec<GroundingDocument>,
    pub library: Library,
    pub review: Vec<ReviewItem>,
    pub entries: Vec<CollectionEntry>,
    pub settings: Vec<(String, String)>,
    pub journal: Option<JournalBook>,
    pub media_dir: PathBuf,
    pub exported_at: DateTime<Utc>,
    pub app_version: String,
}

#[cfg(test)]
mod tests;
