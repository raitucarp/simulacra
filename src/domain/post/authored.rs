//! A post bundled with its resolved author, for consumers that live outside the store
//! (composer modal, agent prompts) and therefore cannot resolve `author_id` themselves.

use super::Post;
use crate::domain::profile::Profile;
use std::ops::Deref;

/// A post together with a snapshot of the profile that wrote it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthoredPost {
    pub post: Post,
    pub author: Profile,
}

impl AuthoredPost {
    /// Bundles `post` with its resolved `author`.
    pub fn new(post: Post, author: Profile) -> Self {
        Self { post, author }
    }
}

impl Deref for AuthoredPost {
    type Target = Post;

    #[inline]
    fn deref(&self) -> &Post {
        &self.post
    }
}
