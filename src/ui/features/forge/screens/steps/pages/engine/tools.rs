//! Tools: which tools the agent may call before it posts (multi-select cards), the web search
//! backend and key, the custom HTTP tools and how many tool rounds it gets.

use super::super::super::super::common::{card, muted};
use super::kit::{engine_id, option_line};
use crate::domain::engine::{SearchBackend, ToolGrant, ToolKind, WebSearchConfig, MAX_TOOL_STEPS_LIMIT};
use crate::domain::EngineConfig;
use crate::state::forge::engine::{mutate, search_credential_kind};
use crate::ui::features::forge::text::{tr, tr_count, trait_label};
use crate::ui::features::forge::view::{ForgeHandle, ForgeView};
use crate::ui::features::forge::widgets::{MultiSelectGrid, NumberSlider};
use crate::state::forge::suggest::{EngineParam, Param};
use gpui_kit::*;

const NONE_KEY: &str = "forge.ui.engine.tools_none";
const STEPS_KEY: &str = "forge.ui.engine.tool_steps";
const GRANTED_KEY: &str = "forge.ui.engine.tools_granted";
const SEPARATOR: &str = ", ";
/// Fewest tool rounds the slider offers.
const MIN_TOOL_STEPS: u16 = 1;
const TOOL_STEPS_STEP: u16 = 1;

impl ForgeView {
    pub(super) fn tools_section(&self, handle: &ForgeHandle) -> Div {
        let theme = &self.theme;
        let engine = self.engine();
        let granted = mutate::granted(engine);
        let set = handle.engine(|e: &mut EngineConfig, kinds| mutate::set_tools(e, &kinds));
        let grid = MultiSelectGrid::new(engine_id("tools"), &granted, theme, move |kinds, w, cx| set(kinds, w, cx))
            .with_descriptions()
            .suggested(self.picks(Param::Engine(EngineParam::Tools)));
        let summary = match granted.len() {
            0 => muted(tr(NONE_KEY), theme),
            count => {
                let names = granted.iter().map(trait_label).collect::<Vec<_>>().join(SEPARATOR);
                muted(format!("{} · {names}", tr_count(GRANTED_KEY, count, &[])), theme)
            }
        };
        let head = card(theme).child(self.engine_field(handle, "engine_tools", grid)).child(summary);
        let search = engine.tools.iter().find_map(|grant| match grant {
            ToolGrant::WebSearch(search) => Some(self.search_card(search, handle)),
            _ => None,
        });
        let customs = granted.contains(ToolKind::CustomHttp).then(|| self.custom_tools_card(handle));
        let steps = (!granted.is_empty()).then(|| self.steps_card(engine.max_tool_steps, handle));
        div().flex().flex_col().gap(theme.section_gap).child(head).children(search).children(customs).children(steps)
    }

    fn search_card(&self, search: &WebSearchConfig, handle: &ForgeHandle) -> Div {
        let theme = &self.theme;
        let grid = self.engine_cards((EngineParam::SearchBackend, "search"), (Some(search.backend), &SearchBackend::ALL), handle, |e, backend| {
            if let Some(search) = mutate::web_search_mut(e) {
                search.backend = backend;
                search.credential = None;
            }
        });
        let kind = search_credential_kind(search.backend);
        let key = kind.map(|kind| {
            let picker = self.credential_picker("search", (search.credential.as_deref(), Some(kind)), handle, |e, id| {
                if let Some(search) = mutate::web_search_mut(e) {
                    search.credential = id;
                }
            });
            self.engine_field(handle, "engine_credential", picker)
        });
        card(theme).child(self.engine_field(handle, "engine_search_backend", grid.compact())).children(option_line(search.backend, theme)).children(key)
    }

    fn steps_card(&self, steps: u8, handle: &ForgeHandle) -> Div {
        let theme = &self.theme;
        let set = handle.engine(|e: &mut EngineConfig, value: u16| e.max_tool_steps = value as u8);
        let range = (MIN_TOOL_STEPS, u16::from(MAX_TOOL_STEPS_LIMIT), TOOL_STEPS_STEP);
        let slider = NumberSlider::new(engine_id("tool-steps"), tr(STEPS_KEY), u16::from(steps), range, theme, move |v, w, cx| set(v, w, cx))
            .hint(self.slider_hint(Param::Engine(EngineParam::MaxToolSteps), &tr(STEPS_KEY)));
        card(theme).child(self.engine_field(handle, "engine_max_tool_steps", slider))
    }
}
