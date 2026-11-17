//! The palette's pure model: query parsing, items, history, ranking and selection.

mod history;
mod items;
mod query;
mod rank;
mod selection;

#[cfg(test)]
mod tests;

pub use history::{HistoryEntry, PaletteHistory};
pub use items::{PaletteGroup, PaletteItem, PaletteTarget};
pub use query::{parse_query, PaletteMode, PaletteQuery, COMMAND_PREFIX};
pub use rank::{is_compact, rank, RankedItem, ResultGroup};
pub use selection::{locate, next_group_start, previous_group_start, result_count, stepped};
