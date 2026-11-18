//! The image ideas sheet overlay (sb D3), while open.

use super::super::binder::Binder;
use super::super::view::AppView;
use crate::ui::features::image_prompt::{render_ideas_sheet, IdeasInputs};
use gpui_kit::*;
use std::rc::Rc;

impl AppView {
    pub(super) fn image_ideas_sheet(&self, bind: &Binder) -> Option<Stateful<Div>> {
        let sheet = self.image_ideas.sheet.as_ref()?;
        let inputs = IdeasInputs {
            sheet,
            prompt: &self.image_ideas.prompt,
            focus: &self.image_ideas.focus,
            theme: &self.theme,
            on_event: Rc::new(bind.with(Self::on_image_ideas_event)),
        };
        Some(render_ideas_sheet(inputs))
    }
}
