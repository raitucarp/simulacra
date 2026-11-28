//! M1: the canvas's right-click menu. A right press selects the node under the pointer (as a
//! click does), so the menu offers the details bar's actions on it: open it, and reset the
//! view.

use super::details::open_label_key;
use super::input::xy;
use super::panel::{MindGraphEvent, MindGraphPanel};
use crate::ui::icons::LucideIcon;
use crate::ui::menus::{render_menu, MenuItem, MenuSection, MenuSpec, MenuText};
use gpui_kit::component::menu::PopupMenu;
use gpui_kit::*;

impl MindGraphPanel {
    /// Selects the node under a right press (or clears the selection off every node).
    pub(super) fn on_secondary_press(&mut self, event: &MouseDownEvent, _: &mut Window, cx: &mut Context<Self>) {
        let hit = self.hit(xy(event.position)).map(|idx| self.scene.model.nodes()[idx].node.clone());
        if let Some(node) = hit.clone() {
            cx.emit(MindGraphEvent::Select(node));
        }
        self.selected = hit;
        cx.notify();
    }
}

fn panel_item(id: &'static str, key: &'static str, icon: LucideIcon, panel: &WeakEntity<MindGraphPanel>, run: fn(&mut MindGraphPanel, &mut Context<MindGraphPanel>)) -> MenuItem {
    let panel = panel.clone();
    MenuItem::new(id, MenuText::key(key)).icon(icon).on_click(move |_, cx| {
        let _ = panel.update(cx, |this, cx| run(this, cx));
    })
}

/// The menu builder, read when it opens (after the press picked its node).
pub(super) fn canvas_menu(panel: WeakEntity<MindGraphPanel>) -> impl Fn(PopupMenu, &mut Window, &mut Context<PopupMenu>) -> PopupMenu + 'static {
    move |menu, window, cx| {
        let open_key = panel.upgrade().and_then(|entity| entity.read(cx).selected_node().map(|node| open_label_key(node.node.kind())));
        let open = open_key.map(|key| panel_item("graph.node.open", key, LucideIcon::ExternalLink, &panel, MindGraphPanel::open_selected));
        let reset = panel_item("graph.reset_view", "graph.reset_view", LucideIcon::RotateCcw, &panel, MindGraphPanel::reset_view);
        let spec = MenuSpec::new().section(MenuSection::new().entry_opt(open)).section(MenuSection::new().entry(reset));
        render_menu(menu, &spec, window, cx)
    }
}
