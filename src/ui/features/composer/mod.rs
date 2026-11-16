//! Post composers: the inline Threads-style composer view, the composer modal dialog and
//! the stateful composer modal panel entity that hosts it.

mod avatar;
mod consts;
pub mod inline;
pub mod keys;
mod messages;
pub mod modal;
pub mod panel;

pub use inline::{PostComposerView, SubmitPostCallback};
pub use keys::{is_submit_chord, SubmitPostDraft, KEY_ENTER, KEY_ENTER_CR, KEY_ENTER_LF};
pub use messages::{publish_error_message, thread_problem_message, visible_problem_message};
pub use modal::{
    ComposerModalMode, ComposerModalView, DraftActionCallback, DraftEditor, ModalActionCallback,
    ModalCallbacks, ModalCloseCallback, ModalSubmitCallback,
};
pub use panel::{ComposerModalEvent, ComposerModalPanel, ComposerSubmission};
