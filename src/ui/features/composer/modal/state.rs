//! Derived state of the composer modal: title and the reply / quote target.

use super::{ComposerModalMode, ComposerModalView};
use crate::domain::AuthoredPost;
use rust_i18n::t;

impl ComposerModalView {
    /// Dialog title for the current mode.
    pub(super) fn title_text(&self) -> String {
        match &self.mode {
            ComposerModalMode::NewThread => t!("composer.new_thread").to_string(),
            ComposerModalMode::Reply(parent) => {
                t!("composer.reply_title", handle = parent.author.display_handle()).to_string()
            }
            ComposerModalMode::Quote(target) => {
                t!("composer.quote_title", handle = target.author.display_handle()).to_string()
            }
            ComposerModalMode::Edit(_) => t!("data.edit.title").to_string(),
        }
    }

    /// Whether the dialog edits a post (sb F2: one text, no attachments or added posts).
    pub(super) fn is_editing(&self) -> bool {
        matches!(self.mode, ComposerModalMode::Edit(_))
    }

    /// Whether the "Schedule…" picker is offered: for new posts, and while editing one that
    /// still waits to go out (Y4).
    pub(super) fn can_schedule(&self) -> bool {
        match &self.mode {
            ComposerModalMode::Edit(target) => target.post.lifecycle.is_scheduled(),
            _ => true,
        }
    }

    pub(super) fn reply_parent(&self) -> Option<&AuthoredPost> {
        match &self.mode {
            ComposerModalMode::Reply(parent) => Some(parent),
            _ => None,
        }
    }

    pub(super) fn quote_target(&self) -> Option<&AuthoredPost> {
        match &self.mode {
            ComposerModalMode::Quote(target) => Some(target),
            _ => None,
        }
    }
}
