//! The apply step, the only stage with side effects: the vault document (through the
//! adapter), the kept original (size-capped, under the media folder) and the posts (one
//! batch through the vault worker).

mod original;
mod request;

#[cfg(test)]
mod tests;

pub use original::{keep_original, max_bytes, original_path, IMPORTS_DIR_NAME};
pub use request::{vault_entry, ImportRequest, OriginalFile};

use super::plan::{build_posts, effective_post_limit, plan_posts, SourceInfo};
use super::vault_adapter::store_document;
use crate::domain::post::PostOrigin;
use crate::error::Result;
use crate::state::AppState;
use std::path::PathBuf;

/// What an import created.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImportOutcome {
    pub document_id: String,
    pub title: String,
    pub posts: usize,
    /// Where the original was kept, if it was.
    pub original: Option<PathBuf>,
    /// Why the original was not kept, when keeping failed.
    pub original_error: Option<String>,
}

impl AppState {
    /// Applies an import: stores the document, keeps the original when the preference says
    /// so, then publishes the selected units as posts when asked to.
    pub fn apply_import(&mut self, request: ImportRequest) -> Result<ImportOutcome> {
        let prefs = self.preferences().clone();
        let kept = request.original.as_ref().filter(|_| prefs.import.keep_original).map(|file| self.keep_file(file, prefs.import.max_file_mb));
        let (original, original_error) = match kept {
            Some(Ok(path)) => (Some(path), None),
            Some(Err(err)) => (None, Some(err.to_string())),
            None => (None, None),
        };
        let mut entry = request.entry.clone();
        entry.original_file = original.clone();
        entry.source_name = request.original.as_ref().map(|file| file.name.clone()).unwrap_or_default();
        let document_id = store_document(self, &entry)?;
        let posts = match request.create_posts {
            true => self.publish_units(&request, &document_id, &entry.title, effective_post_limit(prefs.writing.post_length_limit))?,
            false => 0,
        };
        Ok(ImportOutcome { document_id, title: entry.title, posts, original, original_error })
    }

    /// Copies the original under the media folder, refusing files over the size cap.
    fn keep_file(&self, file: &OriginalFile, max_file_mb: u32) -> Result<PathBuf> {
        let path = original_path(&self.media_dir(), &file.sha256, file.extension.as_deref());
        keep_original(&path, &file.bytes, max_bytes(max_file_mb))
    }

    /// Plans, builds and publishes the selected units, citing `document_id`.
    fn publish_units(&mut self, request: &ImportRequest, document_id: &str, title: &str, limit: usize) -> Result<usize> {
        let author = self.ensure_author(&request.author)?;
        let source = SourceInfo { document_id: document_id.to_string(), title: title.to_string(), chapter_titles: request.chapter_titles.clone() };
        let plan = plan_posts(&request.units, &source, limit);
        let posts = build_posts(&plan, &author, &self.active_network_id, jiff::Timestamp::now(), PostOrigin::Import, Self::fresh_post_id)?;
        self.publish_batch(posts)
    }
}
