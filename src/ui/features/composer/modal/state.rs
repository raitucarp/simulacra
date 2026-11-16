//! Derived state of the composer modal: title and the reply / quote target.

use super::super::consts::{QUOTE_TITLE_PREFIX, REPLY_TITLE_PREFIX};
use super::{ComposerModalMode, ComposerModalView};
use crate::domain::Post;
use rust_i18n::t;

impl ComposerModalView {
    /// Dialog title for the current mode.
    pub(super) fn title_text(&self) -> String {
        match &self.mode {
            ComposerModalMode::NewThread => t!("composer.new_thread").to_string(),
            ComposerModalMode::Reply(parent) => {
                format!("{REPLY_TITLE_PREFIX} {}", parent.author.display_handle())
            }
            ComposerModalMode::Quote(target) => {
                format!("{QUOTE_TITLE_PREFIX} {}", target.author.display_handle())
            }
        }
    }

    pub(super) fn reply_parent(&self) -> Option<&Post> {
        match &self.mode {
            ComposerModalMode::Reply(parent) => Some(parent),
            _ => None,
        }
    }

    pub(super) fn quote_target(&self) -> Option<&Post> {
        match &self.mode {
            ComposerModalMode::Quote(target) => Some(target),
            _ => None,
        }
    }
}
