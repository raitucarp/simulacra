//! Title bar and the floating overlays: language menu, workspace switcher, composer modal.

use super::super::binder::Binder;
use super::super::view::AppView;
use crate::ui::features::composer::ComposerModalView;
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
            )
        })
    }

    /// Threads-style composer modal, while open.
    pub(super) fn composer_modal_view(&self, bind: &Binder, cx: &App) -> Option<ComposerModalView> {
        let modal = &self.composer_modal;
        if !modal.is_open {
            return None;
        }
        let draft_length = modal.composer_input.read(cx).value().chars().count();
        Some(ComposerModalView::new(
            self.theme.clone(),
            self.state.current_profile().cloned(),
            modal.mode.clone(),
            modal.composer_input.clone(),
            modal.link_input.clone(),
            modal.image_input.clone(),
            modal.image_path.clone(),
            modal.is_link_input_open,
            modal.is_image_input_open,
            draft_length,
            bind.act(Self::close_composer_modal),
            bind.act(Self::submit_composer_modal),
            bind.act(Self::toggle_modal_link),
            bind.act(Self::toggle_modal_image),
            bind.act(Self::pick_modal_image),
            bind.act(Self::clear_modal_image),
            bind.act(Self::clear_modal_link),
        ))
    }
}
