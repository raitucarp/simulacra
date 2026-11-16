//! Named literals (element ids, fallbacks, text snippets) used by the composers.

/// Avatar initial shown by the inline composer when no profile is active.
pub(super) const INLINE_FALLBACK_INITIAL: &str = "Y";
/// Avatar initial shown by the composer modal when no profile is active.
pub(super) const MODAL_FALLBACK_INITIAL: &str = "U";
/// Handle shown by the composer modal when no profile is active.
pub(super) const MODAL_FALLBACK_HANDLE: &str = "@user";

/// Token inserted at the cursor by the "insert link" button (opens a `[[post_id]]` wikilink).
pub(super) const WIKILINK_OPEN_TOKEN: &str = "[[";
/// Token inserted at the cursor by the "insert tag" buttons (the hashtag sigil).
pub(super) const HASHTAG_TOKEN: &str = "#";

/// Modal title prefix in reply mode (followed by the parent author's handle).
pub(super) const REPLY_TITLE_PREFIX: &str = "Reply to";
/// Modal title prefix in quote mode (followed by the quoted author's handle).
pub(super) const QUOTE_TITLE_PREFIX: &str = "Quote";
/// Prefix rendered before the community topic label.
pub(super) const TOPIC_PREFIX: &str = "> ";

/// Quote characters stripped from a manually typed image path.
pub(super) const PATH_QUOTE_DOUBLE: char = '"';
pub(super) const PATH_QUOTE_SINGLE: char = '\'';

// Inline composer element ids.
pub(super) const INSERT_LINK_BTN_ID: &str = "insert-link-btn";
pub(super) const INSERT_TAG_BTN_ID: &str = "insert-tag-btn";
pub(super) const SUBMIT_POST_BTN_ID: &str = "submit-post-btn";
pub(super) const INLINE_TEXTAREA_WRAPPER_ID: &str = "composer-textarea-wrapper";

// Composer modal element ids.
pub(super) const MODAL_BACKDROP_ID: &str = "composer-modal-backdrop";
pub(super) const MODAL_DIALOG_ID: &str = "composer-modal-dialog";
pub(super) const MODAL_CANCEL_BTN_ID: &str = "modal-cancel-btn";
pub(super) const MODAL_TEXTAREA_WRAPPER_ID: &str = "modal-composer-textarea-wrapper";
pub(super) const MODAL_CLEAR_LINK_BTN_ID: &str = "modal-clear-link-btn";
pub(super) const MODAL_IMAGE_INPUT_ROW_ID: &str = "modal-image-input-row";
pub(super) const MODAL_BROWSE_IMAGE_BTN_ID: &str = "modal-browse-image-btn";
pub(super) const MODAL_CLEAR_IMAGE_INPUT_BTN_ID: &str = "modal-clear-image-input-btn";
pub(super) const MODAL_IMAGE_PREVIEW_CARD_ID: &str = "modal-image-preview-card";
pub(super) const MODAL_CLEAR_IMAGE_BTN_ID: &str = "modal-clear-image-btn";
pub(super) const MODAL_PICK_IMAGE_BTN_ID: &str = "modal-pick-image-btn";
pub(super) const MODAL_TOGGLE_IMAGE_INPUT_BTN_ID: &str = "modal-toggle-image-input-btn";
pub(super) const MODAL_TOGGLE_LINK_BTN_ID: &str = "modal-toggle-link-btn";
pub(super) const MODAL_TAG_BTN_ID: &str = "modal-tag-btn";
pub(super) const MODAL_MORE_BTN_ID: &str = "modal-more-options-btn";
pub(super) const MODAL_SUBMIT_POST_BTN_ID: &str = "modal-submit-post-btn";
pub(super) const MODAL_THREAD_SCROLL_ID: &str = "modal-thread-scroll";
pub(super) const MODAL_ADD_DRAFT_BTN_ID: &str = "modal-add-draft-btn";
pub(super) const MODAL_SPLIT_DRAFT_BTN_ID: &str = "modal-split-draft-btn";
pub(super) const MODAL_REMOVE_DRAFT_BTN_ID: &str = "modal-remove-draft-btn";

/// Auto-grow row range `(min, max)` of the modal draft text area.
pub(super) const MODAL_COMPOSER_ROWS: (usize, usize) = (2, 8);
/// Placeholder of the modal link input.
pub(super) const MODAL_LINK_PLACEHOLDER: &str = "https://...";
/// File dialog title of the modal image picker.
pub(super) const PICK_IMAGE_TITLE: &str = "Select Image";
