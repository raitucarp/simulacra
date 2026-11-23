//! N1: the text a post showed at a version, for embeds pinned to it. The version shown comes
//! from the post itself, a kept one from its revisions, read once per post and cached until
//! its text changes. A locked post's versions hold only the placeholder (X5).

use super::AppState;
use crate::domain::data::history::SemVer;
use crate::domain::PostId;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/// The texts of one post's kept versions.
type Kept = Rc<HashMap<SemVer, Rc<str>>>;

/// Kept versions' texts per post, read on first use.
#[derive(Debug, Default)]
pub struct VersionTexts(RefCell<HashMap<PostId, Kept>>);

impl VersionTexts {
    /// Drops what was read for `id` (its text changed or it was removed).
    pub fn forget(&self, id: &PostId) {
        self.0.borrow_mut().remove(id);
    }

    fn of(&self, id: &PostId, read: impl FnOnce() -> HashMap<SemVer, Rc<str>>) -> Kept {
        if let Some(found) = self.0.borrow().get(id) {
            return Rc::clone(found);
        }
        let texts: Kept = Rc::new(read());
        self.0.borrow_mut().insert(id.clone(), Rc::clone(&texts));
        texts
    }
}

impl AppState {
    /// N1: the text the post `id` shows at `version` (`None`: the version shown now), when
    /// that version is kept.
    pub fn version_text(&self, id: &str, version: Option<SemVer>) -> Option<Rc<str>> {
        let post = self.get_post_by_id(id)?;
        let shown = post.extras.head.as_ref().map_or_else(SemVer::default, |head| head.version);
        match version {
            Some(wanted) if wanted != shown => {
                let kept = self.index.versions.of(&post.id, || self.post_revisions(&post.id).into_iter().filter_map(|r| Some((r.version?, Rc::from(r.content.as_str())))).collect());
                kept.get(&wanted).cloned()
            }
            _ => Some(Rc::from(post.content.as_str())),
        }
    }
}
