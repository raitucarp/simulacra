//! Items follow their post: once the post is deleted the item is retired (left out of every
//! review for good), while its log stays for the statistics. A map is never retired; its
//! missing posts simply drop out of the drill.

use super::{Anchor, ReviewItem};

/// Whether `item` should be retired now: anchored to a post that no longer exists.
pub fn retired_now(item: &ReviewItem, post_exists: &dyn Fn(&str) -> bool) -> bool {
    !item.retired && matches!(item.subject.anchor(), Some(Anchor::Post { id }) if !post_exists(id))
}

/// The items that must be retired now, already marked retired (to be saved).
pub fn retire_orphans(items: &[ReviewItem], post_exists: &dyn Fn(&str) -> bool) -> Vec<ReviewItem> {
    items.iter().filter(|item| retired_now(item, post_exists)).map(|item| ReviewItem { retired: true, ..item.clone() }).collect()
}
