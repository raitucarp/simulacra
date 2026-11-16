//! Derived state of the composer modal: title, attached image and post eligibility.

use super::super::consts::{
    PATH_QUOTE_DOUBLE, PATH_QUOTE_SINGLE, QUOTE_TITLE_PREFIX, REPLY_TITLE_PREFIX,
};
use super::{ComposerModalMode, ComposerModalView};
use crate::domain::Post;
use gpui_kit::App;
use rust_i18n::t;
use std::path::Path;

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

    /// The picked image, or a manually typed path that points to an existing file.
    pub(super) fn effective_image_path(&self, cx: &App) -> Option<String> {
        let manual_image_val = self
            .image_input
            .read(cx)
            .value()
            .trim()
            .trim_matches(PATH_QUOTE_DOUBLE)
            .trim_matches(PATH_QUOTE_SINGLE)
            .to_string();
        self.image_path.clone().or_else(|| {
            if !manual_image_val.is_empty() && Path::new(&manual_image_val).exists() {
                Some(manual_image_val)
            } else {
                None
            }
        })
    }

    /// Whether the draft may be submitted (quotes require text; threads and replies accept media).
    pub(super) fn can_post(&self, has_image: bool, cx: &App) -> bool {
        let has_link = !self.link_input.read(cx).value().trim().is_empty();
        match &self.mode {
            ComposerModalMode::NewThread | ComposerModalMode::Reply(_) => {
                self.draft_length > 0 || has_image || has_link
            }
            ComposerModalMode::Quote(_) => self.draft_length > 0,
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
