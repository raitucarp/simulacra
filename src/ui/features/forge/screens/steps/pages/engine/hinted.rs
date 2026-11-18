//! Engine text settings with example chips under them (sb D1): the chips write through the
//! same setter as typing, as their own undo step.

use super::kit::engine_id;
use crate::domain::EngineConfig;
use crate::state::forge::suggest::EngineParam;
use crate::state::forge::EngineSection;
use crate::ui::features::forge::view::{ForgeHandle, ForgeView};
use crate::ui::features::forge::widgets::ForgeTextInput;
use std::rc::Rc;

impl ForgeView {
    /// [`ForgeView::engine_input`] for setting `param`, with its example chips underneath.
    pub(super) fn engine_input_for(
        &self,
        (param, name): (EngineParam, String),
        value: &str,
        (section, handle): (EngineSection, &ForgeHandle),
        set: impl Fn(&mut EngineConfig, String) + 'static,
    ) -> ForgeTextInput {
        let set: Rc<dyn Fn(&mut EngineConfig, String)> = Rc::new(set);
        let typing = set.clone();
        let input = self.engine_input(&name, value, (section, handle), move |engine, text| typing(engine, text));
        match self.engine_examples(param, (engine_id(&name).as_ref(), value), (section, handle), set) {
            Some(chips) => input.footer(chips),
            None => input,
        }
    }
}
