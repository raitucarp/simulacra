//! Versions of the user's posts (V3, V4): every edit makes a new version the post shows, and
//! the version it showed is kept as a revision under its own id. Versions form a tree: an
//! edit of the version shown continues its line, an edit of an older one starts a branch.
//! Each kept version keeps the symbols and tint it showed (V10).

use super::{require_persona, AppState};
use crate::domain::data::history::{edited_post, is_real_edit, Attachments, Bump, PostRecord, Revision, RevisionId, RevisionSubject, SemVer, VersionHead};
use crate::domain::{first_thread_problem, Post, PostId};
use crate::error::{AppError, Result};
use jiff::Timestamp;

/// An edit of a post's text (V3, V4).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PostEdit {
    pub content: String,
    /// The bump the user picked over the suggested one.
    pub bump: Option<Bump>,
    /// What the edit is about (optional).
    pub note: String,
    /// The older version edited from (`None`: the version shown), V4.
    pub base: Option<RevisionId>,
    /// The symbols and tint the new version shows (`None`: the post's own), V10.
    pub look: Option<PostRecord>,
}

impl PostEdit {
    /// An edit of the version shown to `content`, with the suggested bump and no note.
    pub fn text(content: impl Into<String>) -> Self {
        Self { content: content.into(), ..Self::default() }
    }
}

/// How the version shown becomes a new one.
pub(super) struct VersionStep<'a> {
    pub bump: Bump,
    pub note: String,
    /// The older version edited from, if not the one shown.
    pub base: Option<&'a Revision>,
}

fn missing(id: &PostId) -> AppError {
    AppError::DatabaseError(format!("post '{id}' not found"))
}

impl AppState {
    /// Every earlier version of the post `id`, newest first (none without a vault).
    pub fn post_revisions(&self, id: &PostId) -> Vec<Revision> {
        let subject = RevisionSubject::Post(id.clone());
        self.db.as_ref().and_then(|db| db.revisions_of(subject).ok()).unwrap_or_default()
    }

    /// The version the post `id` shows.
    pub fn post_head(&self, id: &PostId) -> Option<VersionHead> {
        let post = self.get_post(id)?;
        Some(VersionHead::of(post.extras.head.as_ref(), &self.post_revisions(id), post.created_at))
    }

    /// Saves `edit` of the user's own post `id` (agents' posts are never edited) as a new
    /// version, keeping the one it showed. `Ok(None)` when nothing changed.
    pub fn edit_post_version(&mut self, id: &PostId, edit: &PostEdit, now: Timestamp) -> Result<Option<Revision>> {
        let post = self.get_post(id).cloned().ok_or_else(|| missing(id))?;
        require_persona(self.author_of(&post))?;
        let shown = PostRecord::of(&post);
        let mut look = edit.look.clone().unwrap_or_else(|| shown.clone());
        look.attachments = look.attachments.or_else(|| shown.attachments.clone()); // V11: kept unless the edit sends them
        super::symbols::check_symbols(self.author_of(&post), &look.symbols)?;
        if let Some(problem) = first_thread_problem(&[&edit.content], look.attachments.as_ref().is_some_and(Attachments::any)) {
            return Err(AppError::DatabaseError(format!("{:?}", problem.issue)));
        }
        let revisions = self.post_revisions(id);
        let base = match &edit.base {
            Some(base) => Some(revisions.iter().find(|r| &r.id == base).ok_or_else(|| AppError::DatabaseError(format!("version '{}' not found", base.as_str())))?),
            None => None,
        };
        if base.is_none() && !is_real_edit(&post.content, &edit.content) && look == shown {
            return Ok(None);
        }
        let from = base.map_or(post.content.as_str(), |base| base.content.as_str());
        let step = VersionStep { bump: edit.bump.unwrap_or_else(|| Bump::between(from, &edit.content)), note: edit.note.clone(), base };
        let mut edited = edited_post(&post, &edit.content);
        look.apply(&mut edited);
        self.commit_version(edited, step, &revisions, now).map(Some)
    }

    /// Replaces the text of the user's own post `id` as a new version with the suggested bump.
    /// `Ok(None)` when nothing changed.
    pub fn edit_post(&mut self, id: &PostId, content: &str) -> Result<Option<Revision>> {
        self.edit_post_version(id, &PostEdit::text(content), Timestamp::now())
    }

    /// Shows `edited` (the post `edited.id` changed) as the version after `step`, keeping the
    /// version it showed (numbered among `revisions`, every earlier version of the post).
    pub(super) fn commit_version(&mut self, mut edited: Post, step: VersionStep<'_>, revisions: &[Revision], now: Timestamp) -> Result<Revision> {
        let current = self.get_post(&edited.id).ok_or_else(|| missing(&edited.id))?;
        let head = VersionHead::of(current.extras.head.as_ref(), revisions, current.created_at);
        let taken: Vec<SemVer> = revisions.iter().filter_map(|r| r.version).chain([head.version]).collect();
        let from = step.base.map_or_else(|| head.clone(), VersionHead::from_revision);
        let next = VersionHead { id: RevisionId::generate(), version: from.next(step.bump, &taken), parent: Some(from.id), bump: Some(step.bump), note: step.note.trim().to_string(), edited_at: Some(now) };
        let kept = Revision { record_json: PostRecord::of(current).to_json(), ..head.into_revision(RevisionSubject::Post(edited.id.clone()), &current.content, now) };
        edited.extras.head = Some(next);
        if let Some(ref db) = self.db {
            db.commit_post_version(edited.clone(), kept.clone(), None)?;
        }
        self.replace_post_body(edited);
        Ok(kept)
    }
}
