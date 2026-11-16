//! Workspace/Network domain dropdown trigger and floating popover molecules with 100% Lucide SVG icons.

mod consts;
mod popover;
mod popover_item;
mod trigger;

use crate::domain::{Network, NetworkId};
use crate::theme::ThemeTokens;
use gpui_kit::{App, IntoElement, Window};
use rust_i18n::t;

pub type SelectNetworkCallback = Box<dyn Fn(NetworkId, &mut Window, &mut App) + 'static>;
pub type ToggleWorkspaceCallback = Box<dyn Fn(&mut Window, &mut App) + 'static>;

/// Atomic Workspace Trigger button rendered in the sidebar.
#[derive(IntoElement)]
pub struct WorkspaceTrigger {
    active_network: Option<Network>,
    is_open: bool,
    is_collapsed: bool,
    theme: ThemeTokens,
    on_toggle: ToggleWorkspaceCallback,
}

impl WorkspaceTrigger {
    pub fn new(
        active_network: Option<Network>,
        is_open: bool,
        is_collapsed: bool,
        theme: ThemeTokens,
        on_toggle: impl Fn(&mut Window, &mut App) + 'static,
    ) -> Self {
        Self {
            active_network,
            is_open,
            is_collapsed,
            theme,
            on_toggle: Box::new(on_toggle),
        }
    }
}

/// Atomic Floating Workspace Popover Menu rendered at root with 100% Lucide SVG icons.
#[derive(IntoElement)]
pub struct WorkspacePopover {
    active_network: Option<Network>,
    networks: Vec<Network>,
    is_collapsed: bool,
    theme: ThemeTokens,
    on_select: SelectNetworkCallback,
    on_dismiss: ToggleWorkspaceCallback,
}

impl WorkspacePopover {
    pub fn new(
        active_network: Option<Network>,
        networks: Vec<Network>,
        is_collapsed: bool,
        theme: ThemeTokens,
        on_select: impl Fn(NetworkId, &mut Window, &mut App) + 'static,
        on_dismiss: impl Fn(&mut Window, &mut App) + 'static,
    ) -> Self {
        Self {
            active_network,
            networks,
            is_collapsed,
            theme,
            on_select: Box::new(on_select),
            on_dismiss: Box::new(on_dismiss),
        }
    }
}

/// Localized "N docs · M agents" caption for a network.
fn docs_and_agents_caption(network: &Network) -> String {
    t!(
        "sidebar.docs_and_agents",
        docs = network.doc_count,
        agents = network.agent_count
    )
    .to_string()
}
