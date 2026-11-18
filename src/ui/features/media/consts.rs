//! Element ids and the play-badge, skeleton and caption proportions of the media views.

/// Element id of a gallery tile (keyed by its index inside the post card).
pub(super) const GALLERY_TILE_ID: &str = "media-tile";
/// Element id of a post's link preview card, video card and link chip.
pub(super) const LINK_CARD_ID: &str = "media-link-card";
pub(super) const VIDEO_CARD_ID: &str = "media-video-card";
pub(super) const LINK_CHIP_ID: &str = "media-link-chip";
/// Element ids of the lightbox and its buttons.
pub(super) const LIGHTBOX_ID: &str = "media-lightbox";
pub(super) const LIGHTBOX_CLOSE_ID: &str = "media-lightbox-close";
pub(super) const LIGHTBOX_PREVIOUS_ID: &str = "media-lightbox-previous";
pub(super) const LIGHTBOX_NEXT_ID: &str = "media-lightbox-next";
/// Width of the second skeleton line relative to the first.
pub(super) const SKELETON_SHORT_LINE: f32 = 0.6;
/// Lines of title and description shown on a card.
pub(super) const TITLE_LINES: usize = 2;
pub(super) const DESCRIPTION_LINES: usize = 2;
/// Separator between the label and the domain of a card's meta row.
pub(super) const META_SEPARATOR: &str = " · ";
/// Scheme prefixes dropped from a chip's text.
pub(super) const SHOWN_SCHEMES: [&str; 2] = ["https://", "http://"];
