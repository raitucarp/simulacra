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
use crate::ui::features::forge::widgets::{IllustratedOptionGrid, ToggleCard};
use crate::state::forge::suggest::{EngineParam, Param};
use gpui_kit::*;

const SECTION: EngineSection = EngineSection::Output;
const MAPPING_ROOT: &str = "forge.ui.engine.map";

/// One mapping row: its i18n suffix and the path it edits.
type MappingSlot = (&'static str, fn(&mut OutputMapping) -> &mut String);

/// The post fields an answer can fill, in display order.
const MAPPING: [MappingSlot; 7] = [
    ("content", |m| &mut m.content),
    ("image_prompt", |m| &mut m.image_prompt),
    ("hashtags", |m| &mut m.hashtags),
    ("mood", |m| &mut m.mood),
    ("thread_parts", |m| &mut m.thread_parts),
    ("tint", |m| &mut m.tint),
    ("reactions", |m| &mut m.reactions),
];
/// sb C4: the switch letting the agent tint its posts (off by default).
const MAY_TINT_ID: &str = "forge-engine-may-tint";
const MAY_TINT_KEY: &str = "forge.ui.engine.may_tint";
const MAY_TINT_HINT_KEY: &str = "forge.ui.engine.may_tint_hint";
/// R8: the switch letting the agent react to the posts it answers (off by default).
const MAY_REACT_ID: &str = "forge-engine-may-react";
const MAY_REACT_KEY: &str = "forge.ui.engine.may_react";
const MAY_REACT_HINT_KEY: &str = "forge.ui.engine.may_react_hint";

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
        let grid = self.engine_cards((EngineParam::Output, "output"), (Some(kind), &OutputKind::ALL), handle, mutate::set_output_kind);
        let head = card(theme).child(self.engine_field(handle, "engine_output", grid)).children(option_line(kind, theme));
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
            .child(self.tint_card(structured.may_tint, handle))
            .child(self.react_card(structured.may_react, handle))
            .child(self.preview_card(structured, handle))
    }

    /// Whether the agent may give its posts a container tint (sb C4).
    fn tint_card(&self, may_tint: bool, handle: &ForgeHandle) -> Div {
        let set = handle.with(|this: &mut ForgeView, on: bool, _, cx| this.edit_engine(None, |e| mutate::set_may_tint(e, on), cx));
        let toggle = ToggleCard::custom(MAY_TINT_ID, tr(MAY_TINT_KEY), may_tint, &self.theme, move |on, w, cx| set(on, w, cx)).hint(Some(tr(MAY_TINT_HINT_KEY)));
        card(&self.theme).child(toggle)
    }

    /// Whether the agent may react to the posts it answers (R8), within its hourly limit.
    fn react_card(&self, may_react: bool, handle: &ForgeHandle) -> Div {
        let set = handle.with(|this: &mut ForgeView, on: bool, _, cx| this.edit_engine(None, |e| mutate::set_may_react(e, on), cx));
        let hint = rust_i18n::t!(MAY_REACT_HINT_KEY, limit = crate::domain::engine::AGENT_REACTIONS_PER_HOUR).to_string();
        let toggle = ToggleCard::custom(MAY_REACT_ID, tr(MAY_REACT_KEY), may_react, &self.theme, move |on, w, cx| set(on, w, cx)).hint(Some(hint));
        card(&self.theme).child(toggle)
    }

    fn schema_card(&self, output: &StructuredOutput, handle: &ForgeHandle) -> Div {
        let theme = &self.theme;
        let pick = handle.with(|this: &mut ForgeView, preset: SchemaPreset, _, cx| {
            this.engine.sample = None;
            this.edit_engine(None, |e| mutate::apply_preset(e, preset), cx);
        });
        let presets = IllustratedOptionGrid::new(engine_id("preset"), Some(output.preset), theme, move |p, w, cx| pick(p, w, cx))
            .suggested(self.picks(Param::Engine(EngineParam::SchemaPreset)));
        let schema = self.engine_input_for((EngineParam::Schema, "schema".to_string()), &output.schema, (SECTION, handle), mutate::edit_schema);
        card(theme)
            .gap(theme.section_gap)
            .child(self.engine_field(handle, "engine_schema_preset", presets))
            .children(option_line(output.preset, theme))
            .child(self.engine_field(handle, "engine_schema", schema.multiline(CODE_ROWS.0, CODE_ROWS.1)))
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
        card(theme).child(self.engine_field(handle, "engine_mapping", div().flex().flex_col().gap(theme.spacing_xs).children(rows)))
    }
}
