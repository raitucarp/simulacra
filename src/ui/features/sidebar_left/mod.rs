//! The left navigation sidebar: an expanded column or a collapsed icon rail (Ctrl+B, and
//! automatically outside the wide layout class of `crate::ui::responsive`).
//!
//! Information architecture, top to bottom:
//! - **Context**: the network switcher (the scope everything below belongs to) and the
//!   collapse button.
//! - **Create and find**: the primary "New" menu and the command palette trigger.
//! - **Places**: the primary screens, each with its count. Only destinations live here.
//! - **Your things**: Pinned and Recent, then the network's Sources and Members; each section
//!   folds, remembers it, and lists a few items with "Show all".
//! - **You and your preferences** (footer): Settings, Appearance and the shortcut sheet next
//!   to the identity card. Appearance used to be a primary screen, but it is a preference, not
//!   a place, so it moved beside Settings.
//!
//! Pure logic lives in `mode`, `prefs` and `resolve`; the rest renders.

mod footer;
mod header;
mod icon_button;
mod item_row;
mod mode;
pub mod nav_button;
mod nav_list;
mod nav_menu; // M1
mod new_menu;
mod prefs;
mod render;
mod resolve;
mod section;
mod sections;
mod state;
mod tooltip;
pub mod user_card;

#[cfg(test)]
mod tests;

pub use mode::{sidebar_mode, sidebar_mode_for, toggled_preference, SidebarMode, SidebarPreference};
pub use nav_button::{render_nav_button, NavButtonSpec};
pub use prefs::{NavRef, SidebarPrefs, SidebarSection, PIN_CAPACITY, RECENT_CAPACITY};
pub use render::render_left_sidebar;
pub use new_menu::new_menu_spec;
pub use resolve::{resolve, resolve_all, ResolvedRef};
pub use state::SidebarState;
pub use user_card::render_interactive_user_card;

use crate::state::{AppState, NavigationTab};
use crate::theme::ThemeTokens;
use crate::ui::features::command_palette::CommandId;
use gpui_kit::{App, FocusHandle, Window};
use std::rc::Rc;

/// A shared callback with one argument.
pub type SidebarHandler<A> = Rc<dyn Fn(A, &mut Window, &mut App)>;

/// What the sidebar asks the shell to do.
#[derive(Clone)]
pub struct SidebarCallbacks {
    pub on_select_nav: SidebarHandler<NavigationTab>,
    /// Runs a registry command (New menu items, Settings, the palette, switching).
    pub on_command: SidebarHandler<CommandId>,
    /// Opens a pinned, recent, source or member item.
    pub on_open: SidebarHandler<NavRef>,
    pub on_toggle_pin: SidebarHandler<NavRef>,
    pub on_toggle_section: SidebarHandler<SidebarSection>,
    pub on_toggle_show_all: SidebarHandler<SidebarSection>,
}

/// Inputs of [`render_left_sidebar`].
pub struct LeftSidebarParams<'a> {
    pub state: &'a AppState,
    pub theme: &'a ThemeTokens,
    pub sidebar: &'a SidebarState,
    pub is_workspace_open: bool,
    /// Focus of the shell root: menu items dispatch their commands there.
    pub shell_focus: FocusHandle,
    pub callbacks: SidebarCallbacks,
    /// Cards waiting on the Review page, shown as its count (sb C1).
    pub review_due: Option<usize>,
}
