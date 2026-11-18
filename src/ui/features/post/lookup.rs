//! Post entity lookup abstraction shared by feed renderers.

use super::excerpt::excerpt_with;
use crate::domain::post::{tokenize_post_body, BodyToken};
use crate::domain::media::MediaCatalog;
use crate::domain::{Post, Profile};
use std::rc::Rc;
use std::borrow::Borrow;

/// Trait abstracting post entity lookup and wikilink excerpt resolution.
/// Enables zero-cost polymorphism between raw borrowed slices (&[Post]) and indexed AppState.
pub trait PostLookup: Copy {
    fn find_post(&self, id: &str) -> Option<&Post>;
    fn resolve_wikilink(&self, target_id: &str) -> String;
    /// The resolved author of `post` (collections without profiles answer the placeholder).
    fn author_of(&self, post: &Post) -> &Profile;
    /// Body tokens of `post` (cached by the store; collections tokenize on demand).
    fn body_tokens(&self, post: &Post) -> Rc<[BodyToken]>;
    /// Cached link previews and image scans (sb C3); collections have none.
    fn media_catalog(&self) -> Option<&MediaCatalog> {
        None
    }
    /// Whether link previews may be shown (Settings › Privacy & Data); off shows plain links.
    fn link_previews_enabled(&self) -> bool {
        false
    }
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
            #[inline]
            fn author_of(&self, _post: &Post) -> &Profile {
                Profile::unknown()
            }
            #[inline]
            fn body_tokens(&self, post: &Post) -> Rc<[BodyToken]> {
                tokenize_post_body(&post.content).into()
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
    #[inline]
    fn author_of(&self, post: &Post) -> &Profile {
        crate::state::AppState::author_of(self, post)
    }
    #[inline]
    fn body_tokens(&self, post: &Post) -> Rc<[BodyToken]> {
        match self.index.meta(post.id.as_str()) {
            Some(meta) => Rc::clone(&meta.tokens),
            None => tokenize_post_body(&post.content).into(),
        }
    }
    #[inline]
    fn media_catalog(&self) -> Option<&MediaCatalog> {
        Some(&self.media)
    }
    #[inline]
    fn link_previews_enabled(&self) -> bool {
        self.preferences().privacy.link_previews
    }
}
