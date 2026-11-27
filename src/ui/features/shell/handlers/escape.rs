//! Escape closes what the shell opens without giving it the keyboard: the language and network
//! menus, the import sheet and the atomicity sheet, then the vault's reader, form or facet drawer. The key reaches
//! the shell root from wherever focus is. Before, Escape left these open, and the next click
//! anywhere was spent closing the menu.

use super::super::view::AppView;
use crate::state::NavigationTab;
use gpui_kit::*;

const ESCAPE_KEY: &str = "escape";

impl AppView {
    /// Closes the open menu, else the import sheet; any other key passes on.
    pub(in crate::ui::features::shell) fn on_shell_key(&mut self, event: &KeyDownEvent, window: &mut Window, cx: &mut Context<Self>) {
        if event.keystroke.key != ESCAPE_KEY {
            return;
        }
        if self.chrome.is_lang_menu_open || self.chrome.is_workspace_open {
            (self.chrome.is_lang_menu_open, self.chrome.is_workspace_open) = (false, false);
            cx.stop_propagation();
            cx.notify();
        } else if self.import.session.is_some() {
            cx.stop_propagation();
            self.close_import(window, cx);
        } else if self.thinking.atomic_sheet.take().is_some() {
            cx.stop_propagation();
            cx.notify();
        } else if self.state.active_tab == NavigationTab::Knowledge && self.vault.panel.update(cx, |panel, cx| panel.escape(cx)) {
            cx.stop_propagation();
        }
    }
}
