//! Granting tools on the Engine step: the multi-select keeps each kept grant's settings, and
//! custom HTTP tools are edited by their position among the custom tools.

use crate::domain::engine::{EngineConfig, HttpToolDef, ToolGrant, ToolKind, WebSearchConfig};
use crate::domain::traits::OptionSet;

/// The tool kinds currently granted.
pub fn granted(engine: &EngineConfig) -> OptionSet<ToolKind> {
    OptionSet::new(engine.tools.iter().map(ToolGrant::kind))
}

/// Grants exactly `kinds`: kept grants keep their settings, new ones start from defaults.
pub fn set_tools(engine: &mut EngineConfig, kinds: &OptionSet<ToolKind>) {
    engine.tools.retain(|grant| kinds.contains(grant.kind()));
    for kind in ToolKind::ALL.into_iter().filter(|kind| kinds.contains(*kind)) {
        if !engine.tools.iter().any(|grant| grant.kind() == kind) {
            engine.tools.push(default_grant(kind));
        }
    }
}

fn default_grant(kind: ToolKind) -> ToolGrant {
    match kind {
        ToolKind::VaultSearch => ToolGrant::VaultSearch,
        ToolKind::ReadNetwork => ToolGrant::ReadNetwork,
        ToolKind::WebSearch => ToolGrant::WebSearch(WebSearchConfig::default()),
        ToolKind::WebFetch => ToolGrant::WebFetch,
        ToolKind::CustomHttp => ToolGrant::CustomHttp(HttpToolDef::default()),
    }
}

/// The web search settings, when web search is granted.
pub fn web_search_mut(engine: &mut EngineConfig) -> Option<&mut WebSearchConfig> {
    engine.tools.iter_mut().find_map(|grant| match grant {
        ToolGrant::WebSearch(search) => Some(search),
        _ => None,
    })
}

/// The `index`-th custom HTTP tool.
pub fn custom_tool_mut(engine: &mut EngineConfig, index: usize) -> Option<&mut HttpToolDef> {
    custom_tools_mut(engine).nth(index)
}

fn custom_tools_mut(engine: &mut EngineConfig) -> impl Iterator<Item = &mut HttpToolDef> {
    engine.tools.iter_mut().filter_map(|grant| match grant {
        ToolGrant::CustomHttp(def) => Some(def),
        _ => None,
    })
}

/// Adds another (blank) custom HTTP tool.
pub fn add_custom_tool(engine: &mut EngineConfig) {
    engine.tools.push(ToolGrant::CustomHttp(HttpToolDef::default()));
}

/// Removes the `index`-th custom HTTP tool.
pub fn remove_custom_tool(engine: &mut EngineConfig, index: usize) {
    let mut seen = 0;
    engine.tools.retain(|grant| {
        let ToolGrant::CustomHttp(_) = grant else { return true };
        seen += 1;
        seen - 1 != index
    });
}
