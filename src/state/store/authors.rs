//! Author resolution: posts carry an `author_id`, profiles live once in the store.

use super::AppState;
use crate::domain::post::AuthoredPost;
use crate::domain::profile::handle_key;
use crate::domain::{Post, Profile, ProfileId};

impl AppState {
    /// The live profile with `id`, else the archived profile of a deleted author.
    pub fn find_author(&self, id: &ProfileId) -> Option<&Profile> {
        self.user_profiles
            .iter()
            .find(|p| &p.id == id)
            .or_else(|| self.archived_authors.get(id))
    }

    /// The author of `post`, or the shared "unknown author" placeholder.
    #[inline]
    pub fn author_of(&self, post: &Post) -> &Profile {
        self.find_author(&post.author_id).unwrap_or_else(|| Profile::unknown())
    }

    /// `post` bundled with a snapshot of its author, for consumers outside the store.
    pub fn authored(&self, post: &Post) -> AuthoredPost {
        AuthoredPost::new(post.clone(), self.author_of(post).clone())
    }

    /// The live profile whose handle matches `handle`, ignoring case and `@` sigils.
    pub fn profile_by_handle(&self, handle: &str) -> Option<&Profile> {
        let key = handle_key(handle);
        self.user_profiles.iter().find(|p| handle_key(&p.handle) == key)
    }

    /// Whether a live profile other than `own` already uses `handle` (case-insensitive).
    pub fn handle_taken(&self, handle: &str, own: Option<&ProfileId>) -> bool {
        let key = handle_key(handle);
        self.user_profiles.iter().any(|p| Some(&p.id) != own && handle_key(&p.handle) == key)
    }
}
