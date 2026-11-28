//! Title bar and the floating overlays: the network menu, composer modal.

use super::super::binder::Binder;
use super::super::view::AppView;
use crate::ui::features::composer::ComposerModalPanel;
use crate::ui::features::popovers::workspace::MENU_ANCHOR_ID;
use crate::ui::features::title_bar::AppTitleBar;
use gpui_kit::*;

impl AppView {
    /// Window title bar with sidebar, language and settings controls.
    pub(super) fn title_bar(&self, bind: &Binder) -> AppTitleBar {
        AppTitleBar::new(
            self.state.current_network().cloned(),
            self.chrome.current_language,
            self.theme.clone(),
            bind.act(Self::toggle_left_sidebar),
            bind.act(Self::toggle_right_sidebar),
            bind.with(Self::select_language),
            bind.act(Self::toggle_settings),
        )
        .appearance_toggle(self.state.theme_settings.appearance, bind.act(Self::cycle_appearance))
        .app_menu(self.palette.shell_focus.clone()) // M1
    }

    /// The network menu while it is open: under the switcher (right of the icon rail when
    /// collapsed), over a scrim that closes it.
    pub(super) fn workspace_popover(&self, bind: &Binder) -> Option<Div> {
        let menu = self.chrome.workspace_menu.view()?;
        let theme = &self.theme;
        let left = if self.is_left_rail() { theme.left_sidebar_collapsed_width } else { theme.spacing_md };
        let dismiss = bind.act(Self::dismiss_workspace);
        let anchor = div().id(MENU_ANCHOR_ID).absolute().top(theme.workspace_trigger_height).left(left).on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation()).child(menu);
        Some(div().absolute().inset_0().occlude().on_mouse_down(MouseButton::Left, move |_, window, cx| dismiss(window, cx)).child(anchor))
    }

    /// Threads-style composer modal panel, while open.
    pub(super) fn composer_modal_view(&self, cx: &App) -> Option<Entity<ComposerModalPanel>> {
        self.composer_modal.read(cx).is_open().then(|| self.composer_modal.clone())
    }
}
