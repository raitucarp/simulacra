//! Post entity lookup abstraction shared by feed renderers.

use super::excerpt::excerpt_with;
use crate::domain::Post;
use std::borrow::Borrow;

/// Trait abstracting post entity lookup and wikilink excerpt resolution.
/// Enables zero-cost polymorphism between raw borrowed slices (&[Post]) and indexed AppState.
pub trait PostLookup: Copy {
    fn find_post(&self, id: &str) -> Option<&Post>;
    fn resolve_wikilink(&self, target_id: &str) -> String;
}

/// Linear id search over any collection of owned or borrowed posts.
#[inline]
fn find_by_id<'a, P: Borrow<Post> + 'a>(
    items: impl Iterator<Item = &'a P>,
    id: &str,
) -> Option<&'a Post> {
    items
        .map(<P as Borrow<Post>>::borrow)
        .find(|post| post.id.as_str() == id)
}

/// Implements [`PostLookup`] for borrowed post collections via linear search.
macro_rules! impl_collection_lookup {
    ($([$($generics:tt)*] $ty:ty),* $(,)?) => {$(
        impl<$($generics)*> PostLookup for $ty {
            #[inline]
            fn find_post(&self, id: &str) -> Option<&Post> {
                find_by_id(self.iter(), id)
            }
            #[inline]
            fn resolve_wikilink(&self, target_id: &str) -> String {
                excerpt_with(target_id, |id| self.find_post(id))
            }
        }
    )*};
}

impl_collection_lookup!(
    [] &[Post],
    [const N: usize] &[Post; N],
    [] &Vec<Post>,
    [] &[&Post],
);

impl PostLookup for &crate::state::AppState {
    #[inline]
    fn find_post(&self, id: &str) -> Option<&Post> {
        self.get_post_by_id(id)
    }
    #[inline]
    fn resolve_wikilink(&self, target_id: &str) -> String {
        self.resolve_wikilink_fast(target_id)
    }
}
