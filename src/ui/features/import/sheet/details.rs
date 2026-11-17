//! Editable metadata: title, creators, language and date, each a labeled input.

use super::ImportSheet;
use gpui_kit::component::input::{Input, InputState};
use gpui_kit::*;
use rust_i18n::t;

impl ImportSheet {
    /// A section heading in the sheet body.
    pub(super) fn section_title(&self, key: &str) -> Div {
        let theme = &self.theme;
        div()
            .text_size(theme.font_size_micro)
            .font_weight(FontWeight::BOLD)
            .text_color(theme.text_muted)
            .child(t!(key).to_string())
    }

    fn field(&self, label_key: &str, state: &Entity<InputState>) -> Div {
        let theme = &self.theme;
        div()
            .flex()
            .items_center()
            .gap(theme.control_gap)
            .child(
                div()
                    .w(theme.import_meta_label_width)
                    .flex_none()
                    .text_size(theme.font_size_label)
                    .text_color(theme.text_secondary)
                    .child(t!(label_key).to_string()),
            )
            .child(div().flex_1().min_w_0().child(Input::new(state).w_full().h(theme.settings_input_height).text_size(theme.font_size_label)))
    }

    pub(super) fn details(&self) -> Div {
        let theme = &self.theme;
        let inputs = &self.inputs;
        div()
            .flex()
            .flex_col()
            .gap(theme.stack_gap)
            .child(self.section_title("import.details"))
            .child(self.field("import.field.title", &inputs.title))
            .child(self.field("import.field.creators", &inputs.creators))
            .child(
                div()
                    .flex()
                    .gap(theme.control_gap)
                    .child(div().flex_1().min_w_0().child(self.field("import.field.language", &inputs.language)))
                    .child(div().flex_1().min_w_0().child(self.field("import.field.date", &inputs.date))),
            )
    }
}
