//! The right panel as a drawer: in the compact layout class it slides over the content from
//! the right edge, under the title bar, above a scrim that closes it.

use super::super::binder::Binder;
use super::super::panels::ShellSection;
use super::super::view::AppView;
use crate::ui::atoms::ElevationExt;
use crate::ui::responsive::LayoutClass;
use gpui_kit::*;

const DRAWER_SCRIM_ID: &str = "right-drawer-scrim";
const DRAWER_ID: &str = "right-drawer";

impl AppView {
    /// The open drawer (compact class only, never over the Agent Forge).
    pub(super) fn right_drawer(&self, bind: &Binder, class: LayoutClass) -> Option<Div> {
        let open = class.is_compact() && self.chrome.is_right_drawer_open && self.forge.is_none();
        open.then(|| {
            let t = &self.theme;
            let close = bind.act(Self::close_right_drawer);
            let scrim = div().id(DRAWER_SCRIM_ID).absolute().inset_0().bg(t.bg_scrim).on_click(move |_, window, cx| close(window, cx));
            let panel = div()
                .id(DRAWER_ID)
                .absolute()
                .top_0()
                .right_0()
                .bottom_0()
                .w(t.right_drawer_width)
                .max_w_full()
                .occlude()
                .bg(t.bg_app)
                .border_l(t.border_width_hairline)
                .border_color(t.border)
                .elevation_shadow(t)
                .child(self.panels.cached(ShellSection::RightSidebar));
            div().absolute().top(t.titlebar_height).left_0().right_0().bottom_0().child(scrim).child(panel)
        })
    }
}
