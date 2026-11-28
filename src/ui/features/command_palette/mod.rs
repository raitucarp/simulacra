//! The command palette (Ctrl+P): fuzzy search over screens, commands, people, posts, vault
//! documents and settings sections, with recent and frequent items when the query is empty.
//! A leading `>` (or Ctrl+Shift+P) narrows it to commands.
//!
//! - `registry`: [`CommandId`] and its data rows; extend it to add commands.
//! - `search`: pure fuzzy scoring and highlighting.
//! - `model`: query parsing, items, history, ranking and keyboard selection (pure).
//! - `collect`: the item snapshot taken from the store when the palette opens.
//! - `view`: the overlay entity, which emits [`PaletteEvent`]s for the shell to dispatch.

mod collect;
mod host;
pub mod model;
pub mod registry;
pub mod search;
mod view;

pub use collect::{collect_items, excerpt};
pub use host::{load_history, save_history, PaletteHost};
pub use model::{PaletteHistory, PaletteMode, PaletteTarget};
pub use registry::{spec, Availability, CommandCategory, CommandId, CommandKind, CommandSpec, COMMANDS};
pub use view::{
    CommandPalette, ConfirmSelection, DismissPalette, NextGroup, PaletteEvent, PreviousGroup, SelectNext,
    SelectPrevious,
};
