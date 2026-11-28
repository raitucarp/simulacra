//! The shared, declarative menu model (M1, `docs/decisions/M1-menu-architecture.md`).
//!
//! A [`MenuSpec`] is plain data: labelled sections of items and submenus, each item with its
//! stable id, label (and the locale key it came from), icon, optional `Action` (for its key
//! hint), tick, disabled reason, destructive styling and click handler. Items that do not
//! apply are simply not added (the builders' visibility predicates), so one spec renders
//! the "…" dropdown, the right-click context menu and anything that walks it (tests, docs).
//!
//! [`render_menu`] is the one renderer to the kit's `PopupMenu`; [`rules`] checks every spec
//! against the architecture's structure rules.

mod command;
mod entry;
mod host;
mod item;
mod query;
mod render;
mod rows;
pub mod post_ids;
pub mod rules;
mod spec;
mod text;

pub use command::{command_item, command_items, COMMAND_PREFIX};
pub use entry::{MenuEntry, Submenu};
pub use host::OpenMenu;
pub use item::{MenuHandler, MenuItem};
pub use query::MenuPath;
pub use render::{context_menu_of, render_menu};
pub use spec::{MenuSection, MenuSpec};
pub use text::MenuText;

#[cfg(test)]
mod tests;
