//! Menus anchored to the title bar and sidebar (language, network), built on the shared menu
//! model and the kit's popup menu.

pub mod language;
pub mod workspace;

pub use language::language_menu_spec;
pub use workspace::network_menu_spec;
