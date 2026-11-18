//! Named constants for the post item feature: element ids, labels, and formats.

/// `chrono` format string for post timestamps.
pub(crate) const TIME_FORMAT: &str = "%H:%M";
/// Middle dot separating metadata segments.
pub(crate) const META_SEPARATOR: &str = "·";

/// Element id prefix of the whole post card.
pub(crate) const POST_ITEM_ID_PREFIX: &str = "post-item-";
/// Element id prefix of an embedded quote card (suffixed with the quoted post id).
pub(crate) const QUOTE_CARD_ID_PREFIX: &str = "quote-card-";
/// Element id prefix of the like action button.
pub(crate) const LIKE_BTN_ID_PREFIX: &str = "like-btn-";
/// Element id prefix of the reply action button.
pub(crate) const REPLY_BTN_ID_PREFIX: &str = "reply-btn-";
/// Element id prefix of the repost action button.
pub(crate) const REPOST_BTN_ID_PREFIX: &str = "repost-btn-";
/// Element id prefix of the quote action button.
pub(crate) const QUOTE_BTN_ID_PREFIX: &str = "quote-btn-";
/// Element id prefix of the linked-references action button.
pub(crate) const REFS_BTN_ID_PREFIX: &str = "refs-btn-";
/// Element id prefix of the share action button.
pub(crate) const SHARE_BTN_ID_PREFIX: &str = "share-btn-";
/// Element id prefix of the overflow (ellipsis) menu button.
pub(crate) const OVERFLOW_BTN_ID_PREFIX: &str = "post-overflow-btn-";


/// Line break character.
pub(crate) const LINE_BREAK: char = '\n';
/// Carriage return stripped from line ends.
pub(crate) const CARRIAGE_RETURN: char = '\r';
