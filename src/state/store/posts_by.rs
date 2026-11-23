//! One author's posts, newest first, a page at a time.

use super::AppState;
use crate::domain::{Post, ProfileId};

/// One page of a newest-first listing: page `number` (counted from 0) of `size` items.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Page {
    pub number: usize,
    pub size: usize,
}

impl Page {
    /// The first page of `size` items.
    pub const fn first(size: usize) -> Self {
        Self { number: 0, size }
    }

    /// The page after this one.
    pub const fn next(self) -> Self {
        Self { number: self.number + 1, ..self }
    }

    /// How many items come before this page.
    pub const fn offset(self) -> usize {
        self.number.saturating_mul(self.size)
    }
}

impl AppState {
    /// The posts `profile` wrote, in every network and of every kind and status (replies and
    /// reposts included, scheduled ones too), newest first (ties by id), on `page`. Empty
    /// past the last page.
    pub fn posts_by(&self, profile: &ProfileId, page: Page) -> Vec<&Post> {
        self.written_by(profile, page, |_| true)
    }

    /// The feelings `profile` posted (M4: a profile's feelings filter), newest first, on
    /// `page`.
    pub fn feelings_by(&self, profile: &ProfileId, page: Page) -> Vec<&Post> {
        self.written_by(profile, page, |post| post.kind.is_feeling())
    }

    /// How many posts `profile` wrote (W7: the count on its Posts tab).
    pub fn post_count_by(&self, profile: &ProfileId) -> usize {
        self.posts.iter().chain(&self.scheduled).filter(|post| &post.author_id == profile).count()
    }

    fn written_by(&self, profile: &ProfileId, page: Page, keep: impl Fn(&Post) -> bool) -> Vec<&Post> {
        let mut written: Vec<&Post> = self.posts.iter().chain(&self.scheduled).filter(|post| &post.author_id == profile && keep(post)).collect();
        written.sort_by(|a, b| b.created_at.cmp(&a.created_at).then_with(|| b.id.as_str().cmp(a.id.as_str())));
        written.into_iter().skip(page.offset()).take(page.size).collect()
    }
}
