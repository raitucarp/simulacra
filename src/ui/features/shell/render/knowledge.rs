//! The Knowledge page (sb F2): the Library / Saved switch over the vault or the Saved view.

use super::super::binder::Binder;
use super::super::view::AppView;
use crate::ui::features::data::{render_knowledge_switch, render_saved_page};
use gpui_kit::*;

impl AppView {
    /// The switch, then the vault (library) or the Saved view.
    pub(super) fn knowledge_page(&self, bind: &Binder) -> AnyElement {
        let on_event = self.data_action(bind);
        let content = if self.data.showing_saved {
            render_saved_page(&self.saved_view(), on_event.clone(), &self.theme).into_any_element()
        } else {
            self.vault.panel.clone().into_any_element()
        };
        div()
            .size_full()
            .flex()
            .flex_col()
            .child(render_knowledge_switch(self.data.showing_saved, on_event, &self.theme))
            .child(div().flex_1().min_h_0().w_full().child(content))
            .into_any_element()
    }
}
