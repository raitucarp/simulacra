//! The post actions sheet overlay (sb C2), while open.

use super::super::binder::Binder;
use super::super::view::AppView;
use crate::ui::features::post_actions::{render_actions_sheet, SheetInputs};
use gpui_kit::*;
use std::rc::Rc;

impl AppView {
    pub(super) fn post_actions_sheet(&self, bind: &Binder) -> Option<Stateful<Div>> {
        let sheet = self.post_actions.sheet.as_ref()?;
        let inputs = SheetInputs {
            sheet,
            input: &self.post_actions.input,
            focus: &self.post_actions.focus,
            theme: &self.theme,
            on_event: Rc::new(bind.with(Self::on_action_sheet_event)),
        };
        Some(render_actions_sheet(inputs))
    }
}
