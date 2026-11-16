//! Left sidebar organism with Collapsible Icon-Rail (60px) to Expanded (260px) modes.
//! Features unified Personas & Agents navigation, per-network member roster, and interactive identity switching.

mod grounding;
mod members;
mod nav_list;
mod new_thread;
mod render;
mod section;
mod user_anchor;

pub mod nav_button;
pub mod user_card;

pub use nav_button::render_nav_button;
pub use render::render_left_sidebar;
pub use user_card::render_interactive_user_card;

use crate::domain::ProfileId;
use crate::state::{AppState, NavigationTab};
use crate::theme::ThemeTokens;
use gpui_kit::{App, Window};
use std::rc::Rc;

pub type SelectNavCallback = Box<dyn Fn(NavigationTab, &mut Window, &mut App) + 'static>;
pub type ToggleWsCallback = Box<dyn Fn(&mut Window, &mut App) + 'static>;
pub type NewThreadCallback = Box<dyn Fn(&mut Window, &mut App) + 'static>;
pub type SelectSidebarAgentCallback = Box<dyn Fn(ProfileId, &mut Window, &mut App) + 'static>;
pub type SwitchUserPersonaCallback = Box<dyn Fn(&mut Window, &mut App) + 'static>;

/// Shared handle to the navigation callback, cloned into every clickable row.
type NavHandle = Rc<SelectNavCallback>;

pub struct LeftSidebarParams<'a> {
    pub state: &'a AppState,
    pub theme: &'a ThemeTokens,
    pub is_workspace_open: bool,
    pub is_collapsed: bool,
    pub on_toggle_workspace: ToggleWsCallback,
    pub on_select_nav: SelectNavCallback,
    pub on_new_thread: NewThreadCallback,
    pub on_select_agent: Option<SelectSidebarAgentCallback>,
    pub on_switch_persona: Option<SwitchUserPersonaCallback>,
}
