//! The settings catalog (pure): categories' grouped rows, row ids and text keys, the action
//! commands, which rows are enabled, search, and the section list for the command palette.

mod commands;
mod entries;
mod groups;
mod palette;
mod rows;
mod rules;
mod search;

#[cfg(test)]
mod tests;

pub use commands::SettingsCommand;
pub use entries::search_entries;
pub use groups::{all_rows, groups, SettingsGroup};
pub use palette::{settings_sections, SettingsSectionEntry};
pub use rows::{DataFolder, EmbeddedPane, SettingRow, ThemeScale};
pub use rules::is_enabled;
pub use search::{find_ranges, hits_in, match_entry, merge_ranges, query_terms, search, SearchEntry, SearchHit};
