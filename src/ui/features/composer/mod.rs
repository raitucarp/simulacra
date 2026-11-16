//! Post composers: the inline Threads-style composer view and the composer modal dialog.

mod avatar;
mod consts;
pub mod inline;
pub mod keys;
pub mod modal;

pub use inline::{PostComposerView, SubmitPostCallback};
pub use keys::{is_submit_chord, SubmitPostDraft, KEY_ENTER, KEY_ENTER_CR, KEY_ENTER_LF};
pub use modal::{
    ComposerModalMode, ComposerModalView, ModalActionCallback, ModalCloseCallback,
    ModalSubmitCallback,
};
