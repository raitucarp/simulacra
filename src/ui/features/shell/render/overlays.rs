//! Title bar and the floating overlays: language menu, workspace switcher, composer modal.

use super::super::binder::Binder;
use super::super::view::AppView;
use crate::ui::features::composer::ComposerModalPanel;
use crate::ui::features::popovers::language::LanguagePopover;
use crate::ui::features::popovers::workspace::WorkspacePopover;
use crate::ui::features::title_bar::AppTitleBar;
use gpui_kit::*;

impl AppView {
    /// Window title bar with sidebar, language and settings toggles.
    pub(super) fn title_bar(&self, bind: &Binder) -> AppTitleBar {
        AppTitleBar::new(
            self.state.current_network().cloned(),
            self.chrome.current_language,
            self.theme.clone(),
            bind.act(Self::toggle_left_sidebar),
            bind.act(Self::toggle_right_sidebar),
            bind.act(Self::toggle_lang_menu),
            bind.act(Self::toggle_settings),
        )
    }

    /// Language menu, while open.
    pub(super) fn language_popover(&self, bind: &Binder) -> Option<LanguagePopover> {
        self.chrome.is_lang_menu_open.then(|| {
            LanguagePopover::new(
                self.chrome.current_language,
                self.theme.clone(),
                bind.with(Self::select_language),
                bind.act(Self::dismiss_lang_menu),
            )
        })
    }

    /// Workspace (network) switcher, while open.
    pub(super) fn workspace_popover(&self, bind: &Binder) -> Option<WorkspacePopover> {
        self.chrome.is_workspace_open.then(|| {
            WorkspacePopover::new(
                self.state.current_network().cloned(),
                self.state.networks.clone(),
                self.chrome.is_left_sidebar_collapsed,
                self.theme.clone(),
                bind.with(Self::select_network),
                bind.act(Self::dismiss_workspace),
                bind.act(Self::open_create_network),
            )
        })
    }

    /// Threads-style composer modal panel, while open.
    pub(super) fn composer_modal_view(&self, cx: &App) -> Option<Entity<ComposerModalPanel>> {
        self.composer_modal.read(cx).is_open().then(|| self.composer_modal.clone())
    }
}
