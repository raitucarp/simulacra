//! The custom HTTP tools: one editor per tool (name, description, method, URL template,
//! headers, a JSON schema of its arguments, a key) plus "Add another tool".

use super::super::super::super::common::{card, heading};
use super::headers::HeaderAccess;
use super::kit::{engine_id, json_line, CODE_ROWS, NOTE_ROWS};
use crate::domain::engine::{HeaderPair, HttpMethod, HttpToolDef, ToolGrant};
use crate::domain::EngineConfig;
use crate::state::forge::engine::{mutate, object_problem};
use crate::state::forge::EngineSection;
use crate::ui::features::forge::text::{tr, tr_with};
use crate::ui::features::forge::view::{ForgeHandle, ForgeView};
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::Sizable;
use crate::state::forge::suggest::EngineParam;
use gpui_kit::*;

const SECTION: EngineSection = EngineSection::Tools;
const TITLE_KEY: &str = "forge.ui.engine.custom_tool_title";
const ADD_KEY: &str = "forge.ui.engine.custom_tool_add";
const REMOVE_KEY: &str = "forge.ui.engine.custom_tool_remove";
const NAME_HINT_KEY: &str = "forge.ui.engine.custom_tool_name_hint";
const URL_HINT_KEY: &str = "forge.ui.engine.url_template_hint";
const ARG_NUMBER: &str = "number";

fn tool_headers(engine: &mut EngineConfig, index: usize) -> Option<&mut Vec<HeaderPair>> {
    mutate::custom_tool_mut(engine, index).map(|tool| &mut tool.headers)
}

impl ForgeView {
    /// Every custom tool's editor and the "Add another tool" button.
    pub(super) fn custom_tools_card(&self, handle: &ForgeHandle) -> Div {
        let theme = &self.theme;
        let tools = self.engine().tools.iter().filter_map(|grant| match grant {
            ToolGrant::CustomHttp(def) => Some(def),
            _ => None,
        });
        let editors: Vec<Div> = tools.enumerate().map(|(index, def)| self.custom_tool_editor(index, def, handle)).collect();
        let add = handle.engine(|e: &mut EngineConfig, ()| mutate::add_custom_tool(e));
        let button = Button::new(engine_id("tool-add")).outline().small().icon(LucideIcon::Plus).label(tr(ADD_KEY)).on_click(move |_, w, cx| add((), w, cx));
        div().flex().flex_col().gap(theme.section_gap).children(editors).child(div().flex().child(button))
    }

    fn custom_tool_editor(&self, index: usize, def: &HttpToolDef, handle: &ForgeHandle) -> Div {
        let theme = &self.theme;
        let input = |(param, name): (EngineParam, &str), value: &str, set: fn(&mut HttpToolDef, String)| {
            self.engine_input_for((param, format!("tool-{index}-{name}")), value, (SECTION, handle), move |e, text| {
                if let Some(tool) = mutate::custom_tool_mut(e, index) {
                    set(tool, text);
                }
            })
        };
        let name = input((EngineParam::ToolName, "name"), &def.name, |tool, text| tool.name = text).placeholder(tr(NAME_HINT_KEY));
        let description = input((EngineParam::ToolDescription, "description"), &def.description, |tool, text| tool.description = text).multiline(NOTE_ROWS.0, NOTE_ROWS.1);
        let url = input((EngineParam::UrlTemplate, "url"), &def.url_template, |tool, text| tool.url_template = text).placeholder(tr(URL_HINT_KEY));
        let params = input((EngineParam::ParamsSchema, "params"), &def.params_schema, |tool, text| tool.params_schema = text).multiline(CODE_ROWS.0, CODE_ROWS.1);
        let methods = self.engine_cards((EngineParam::Method, &format!("tool-{index}-method")), (Some(def.method), &HttpMethod::ALL), handle, move |e, method| {
            if let Some(tool) = mutate::custom_tool_mut(e, index) {
                tool.method = method;
            }
        });
        let headers = self.headers_editor(&format!("tool-{index}"), &def.headers, (tool_headers as HeaderAccess, index), (SECTION, handle));
        let key = self.credential_picker(&format!("tool-{index}"), (def.credential.as_deref(), None), handle, move |e, id| {
            if let Some(tool) = mutate::custom_tool_mut(e, index) {
                tool.credential = id;
            }
        });
        card(theme)
            .gap(theme.section_gap)
            .child(self.tool_header(index, handle))
            .child(self.engine_field(handle, "engine_tool_name", name))
            .child(self.engine_field(handle, "engine_tool_description", description))
            .child(self.engine_field(handle, "engine_method", methods.compact()))
            .child(self.engine_field(handle, "engine_url_template", url))
            .child(self.engine_field(handle, "engine_headers", headers))
            .child(self.engine_field(handle, "engine_params_schema", div().flex().flex_col().gap(theme.spacing_xs).child(params).child(json_line(object_problem(&def.params_schema), theme))))
            .child(self.engine_field(handle, "engine_credential", key))
    }

    /// "Custom tool N" and its remove button.
    fn tool_header(&self, index: usize, handle: &ForgeHandle) -> Div {
        let theme = &self.theme;
        let remove = handle.engine(move |e: &mut EngineConfig, ()| mutate::remove_custom_tool(e, index));
        let button = Button::new(engine_id(format!("tool-{index}-remove"))).ghost().xsmall().icon(LucideIcon::Trash).label(tr(REMOVE_KEY));
        let title = tr_with(TITLE_KEY, &[(ARG_NUMBER, (index + 1).to_string())]);
        div().flex().items_center().justify_between().child(heading(title, theme)).child(button.on_click(move |_, w, cx| remove((), w, cx)))
    }
}
