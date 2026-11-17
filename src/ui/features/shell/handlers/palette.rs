//! Opening and closing the command palette and the shortcut sheet, and running what the
//! palette picked (remembered in its persisted history).

use super::super::view::AppView;
use crate::ui::features::command_palette::{collect_items, save_history, CommandPalette, PaletteEvent, PaletteMode, PaletteTarget};
use crate::ui::features::sidebar_left::NavRef;
use gpui_kit::*;

impl AppView {
    /// Opens the palette in `mode` (or switches the open one by reopening it).
    pub(crate) fn open_palette(&mut self, mode: PaletteMode, window: &mut Window, cx: &mut Context<Self>) {
        self.palette.is_sheet_open = false;
        let (items, history, theme) = (collect_items(&self.state), self.palette.history.clone(), self.theme.clone());
        let palette = cx.new(|cx| CommandPalette::new(items, history, mode, theme, window, cx));
        self.palette.subscription = Some(cx.subscribe_in(&palette, window, Self::on_palette_event));
        self.palette.palette = Some(palette);
        cx.notify();
    }

    fn on_palette_event(&mut self, _: &Entity<CommandPalette>, event: &PaletteEvent, window: &mut Window, cx: &mut Context<Self>) {
        match event {
            PaletteEvent::SemanticQuery(text) => self.on_semantic_query(text.clone(), window, cx),
            PaletteEvent::Dismiss => self.close_palette(window, cx),
            PaletteEvent::Run(target) => {
                self.close_palette(window, cx);
                self.palette.history = self.palette.history.recorded(&target.history_key());
                save_history(self.state.db.as_ref(), &self.palette.history);
                self.run_palette_target(target.clone(), window, cx);
            }
        }
    }

    /// Closes the palette and gives the keyboard back to the shell.
    fn close_palette(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.palette.palette = None;
        self.palette.subscription = None;
        window.focus(&self.palette.shell_focus, cx);
        cx.notify();
    }

    fn run_palette_target(&mut self, target: PaletteTarget, window: &mut Window, cx: &mut Context<Self>) {
        match target {
            PaletteTarget::Command(id) => self.run_command(id, window, cx),
            PaletteTarget::Profile(id) => self.open_profile(id, window, cx),
            PaletteTarget::Post(id) => self.reveal_post(id, window, cx),
            PaletteTarget::Document(id) => self.open_nav_item(NavRef::Document(id), window, cx),
            PaletteTarget::Settings(tab) => self.open_settings_tab(tab, window, cx),
        }
    }

    /// Shows or hides the shortcut sheet (it takes the keyboard so Escape closes it).
    pub(crate) fn toggle_shortcut_sheet(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let open = !self.palette.is_sheet_open;
        self.palette.is_sheet_open = open;
        let focus = if open { &self.palette.sheet_focus } else { &self.palette.shell_focus };
        window.focus(focus, cx);
        cx.notify();
    }

    /// When the focused element disappears (a closed menu or overlay), the shell root takes the
    /// keyboard so global shortcuts keep working.
    pub(in crate::ui::features::shell) fn refocus_on_focus_loss(window: &mut Window, cx: &mut Context<Self>) -> Subscription {
        cx.on_focus_lost(window, |this, window, cx| window.focus(&this.palette.shell_focus, cx))
    }

    /// Keeps global shortcuts reachable: focuses the shell root whenever nothing is focused.
    pub(in crate::ui::features::shell) fn claim_idle_focus(&self, window: &mut Window, cx: &mut Context<Self>) {
        if window.focused(cx).is_none() {
            window.focus(&self.palette.shell_focus, cx);
        }
    }
}
