//! The cache key of a pair: a content hash of both sides, independent of their order, that
//! changes as soon as either text is edited (so a stale verdict is never reused).

use super::side::TensionSide;
use crate::domain::embedding::content_hash;

/// The key of the unordered pair `left`, `right`.
pub fn pair_key(left: &TensionSide, right: &TensionSide) -> String {
    let a = (left.key(), left.text.trim());
    let b = (right.key(), right.text.trim());
    let (first, second) = if a <= b { (a, b) } else { (b, a) };
    content_hash(&[first.0.as_str(), first.1, second.0.as_str(), second.1])
}
