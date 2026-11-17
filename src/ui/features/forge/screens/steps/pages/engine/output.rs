//! Answer shape: free text, or a JSON schema (from a preset or written by hand) whose fields
//! map onto the post, with a live rendered-post preview of a sample answer.

use super::super::super::super::common::card;
use super::kit::{engine_id, json_line, option_line, CODE_ROWS};
use crate::domain::engine::{OutputKind, OutputMapping, OutputMode, SchemaPreset, StructuredOutput};
use crate::domain::EngineConfig;
use crate::state::forge::engine::{mutate, object_problem};
use crate::state::forge::EngineSection;
use crate::ui::features::forge::text::tr;
use crate::ui::features::forge::view::{ForgeHandle, ForgeView};
use crate::ui::features::forge::widgets::IllustratedOptionGrid;
use gpui_kit::*;

const SECTION: EngineSection = EngineSection::Output;
const MAPPING_ROOT: &str = "forge.ui.engine.map";

/// One mapping row: its i18n suffix and the path it edits.
type MappingSlot = (&'static str, fn(&mut OutputMapping) -> &mut String);

/// The post fields an answer can fill, in display order.
const MAPPING: [MappingSlot; 5] = [
    ("content", |m| &mut m.content),
    ("image_prompt", |m| &mut m.image_prompt),
    ("hashtags", |m| &mut m.hashtags),
    ("mood", |m| &mut m.mood),
    ("thread_parts", |m| &mut m.thread_parts),
];

/// Mutable access to the structured output, when the agent answers in JSON.
pub(super) fn structured(engine: &mut EngineConfig) -> Option<&mut StructuredOutput> {
    match &mut engine.output {
        OutputMode::JsonSchema(structured) => Some(structured),
        OutputMode::FreeText => None,
    }
}

impl ForgeView {
    pub(super) fn output_section(&self, handle: &ForgeHandle) -> Div {
        let theme = &self.theme;
        let kind = OutputKind::of(&self.engine().output);
        let grid = self.engine_cards("output", (Some(kind), &OutputKind::ALL), handle, mutate::set_output_kind);
        let head = card(theme).child(self.engine_field("engine_output", grid)).children(option_line(kind, theme));
        let Some(structured) = self.engine().structured() else {
            return div().flex().flex_col().gap(theme.section_gap).child(head);
        };
        div()
            .flex()
            .flex_col()
            .gap(theme.section_gap)
            .child(head)
            .child(self.schema_card(structured, handle))
            .child(self.mapping_card(&structured.mapping, handle))
            .child(self.preview_card(structured, handle))
    }

    fn schema_card(&self, output: &StructuredOutput, handle: &ForgeHandle) -> Div {
        let theme = &self.theme;
        let pick = handle.with(|this: &mut ForgeView, preset: SchemaPreset, _, cx| {
            this.engine.sample = None;
            this.edit_engine(None, |e| mutate::apply_preset(e, preset), cx);
        });
        let presets = IllustratedOptionGrid::new(engine_id("preset"), Some(output.preset), theme, move |p, w, cx| pick(p, w, cx));
        let schema = self.engine_input("schema", &output.schema, (SECTION, handle), mutate::edit_schema);
        card(theme)
            .gap(theme.section_gap)
            .child(self.engine_field("engine_schema_preset", presets))
            .children(option_line(output.preset, theme))
            .child(self.engine_field("engine_schema", schema.multiline(CODE_ROWS.0, CODE_ROWS.1)))
            .child(json_line(object_problem(&output.schema), theme))
    }

    /// One path input per post field.
    fn mapping_card(&self, mapping: &OutputMapping, handle: &ForgeHandle) -> Div {
        let theme = &self.theme;
        let mut current = mapping.clone();
        let rows = MAPPING.iter().map(|&(key, slot)| {
            let value = slot(&mut current).clone();
            let input = self.engine_input(format!("map-{key}"), &value, (SECTION, handle), move |e, text| {
                if let Some(output) = structured(e) {
                    *slot(&mut output.mapping) = text;
                }
            });
            let label = div().w(theme.forge_option_card_width).flex_shrink_0().text_size(theme.font_size_xs).text_color(theme.text_secondary).child(tr(&format!("{MAPPING_ROOT}.{key}")));
            div().flex().items_center().gap(theme.spacing_sm).child(label).child(div().flex_1().min_w_0().child(input))
        });
        card(theme).child(self.engine_field("engine_mapping", div().flex().flex_col().gap(theme.spacing_xs).children(rows)))
    }
}
