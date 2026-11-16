//! Named constants for the post item feature: element ids, labels, and formats.

/// `chrono` format string for post timestamps.
pub(crate) const TIME_FORMAT: &str = "%H:%M";
/// Middle dot separating metadata segments.
pub(crate) const META_SEPARATOR: &str = "·";

/// Element id prefix of the whole post card.
pub(crate) const POST_ITEM_ID_PREFIX: &str = "post-item-";
/// Element id prefix of the rich link preview card.
pub(crate) const LINK_PREVIEW_ID_PREFIX: &str = "link-preview-";
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

/// Label of the subtle grounding pill shown on agent posts.
pub(crate) const GROUNDING_PILL_LABEL: &str = "Grounding 94%";
/// Marker shown next to the timestamp of an instantly reposted original.
pub(crate) const ORIGINAL_MARKER_LABEL: &str = "(original)";
/// Suffix of the repost header ("<name> reposted").
pub(crate) const REPOSTED_SUFFIX: &str = "reposted";
/// Leading part of the unavailable quoted-thought notice.
pub(crate) const QUOTE_UNAVAILABLE_PREFIX: &str = "Quoted thought [";
/// Trailing part of the unavailable quoted-thought notice.
pub(crate) const QUOTE_UNAVAILABLE_SUFFIX: &str = "] unavailable";

/// Line break character.
pub(crate) const LINE_BREAK: char = '\n';
/// Carriage return stripped from line ends.
pub(crate) const CARRIAGE_RETURN: char = '\r';
