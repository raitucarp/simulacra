//! Updates the host pushes into the panel: a publish error, the active profile and the theme.

use super::ComposerModalPanel;
use crate::domain::Profile;
use crate::theme::ThemeTokens;
use gpui_kit::*;

impl ComposerModalPanel {
    /// Shows why the host could not publish the submitted thread.
    pub fn set_error(&mut self, message: String, cx: &mut Context<Self>) {
        self.error = Some(message);
        cx.notify();
    }

    pub fn set_profile(&mut self, profile: Option<&Profile>, cx: &mut Context<Self>) {
        if self.profile.as_ref() != profile {
            self.profile = profile.cloned();
            cx.notify();
        }
    }

    pub fn set_theme(&mut self, theme: &ThemeTokens, cx: &mut Context<Self>) {
        if &self.theme != theme {
            self.theme = theme.clone();
            cx.notify();
        }
    }
}
