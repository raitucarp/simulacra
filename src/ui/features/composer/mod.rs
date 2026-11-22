//! Post composers: the inline Threads-style composer view, the composer modal dialog and
//! the stateful composer modal panel entity that hosts it.

mod avatar;
mod consts;
mod format;
mod format_keys;
pub mod inline;
pub mod keys;
mod messages;
mod preview;
mod schedule_picker;
pub mod modal;
pub mod panel;
mod snippet;
mod symbol_picker; // S5
mod tint_picker;
mod version_picker; // V3

pub use inline::{PostComposerView, SubmitPostCallback};
pub use format::format_label_key;
pub use symbol_picker::{choice_glyph, choice_label, SymbolCallback, SymbolPickerView};
pub use tint_picker::{tint_label_key, TintCallback};
pub use version_picker::{bump_label_key, version_label, BumpCallback, VersionView};
pub use format_keys::{format_action, FormatBold, FormatCode, FormatItalic, FormatSpoiler, FormatUnderline};
pub use keys::{is_submit_chord, SubmitPostDraft, KEY_ENTER, KEY_ENTER_CR, KEY_ENTER_LF};
pub use messages::{publish_error_message, thread_problem_message, visible_problem_message};
pub use modal::{
    ComposerModalMode, ComposerModalView, DraftActionCallback, DraftEditor, ModalActionCallback,
    ModalCallbacks, ModalCloseCallback, ModalSubmitCallback, PasteCallback,
};
pub use panel::{ComposerModalEvent, ComposerModalPanel, ComposerSubmission};
