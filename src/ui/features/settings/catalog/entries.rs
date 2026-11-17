//! The translated search entries of every row (in the active locale).

use super::groups::all_rows;
use super::search::SearchEntry;
use crate::ui::features::settings::tabs::{DESCRIPTION, KEYWORDS, TITLE};
use crate::ui::features::settings::text::tr;

/// One entry per row, in window order.
pub fn search_entries() -> Vec<SearchEntry> {
    all_rows()
        .into_iter()
        .map(|(tab, row)| SearchEntry {
            tab,
            row,
            title: tr(&row.text_key(TITLE)),
            description: tr(&row.text_key(DESCRIPTION)),
            keywords: tr(&row.text_key(KEYWORDS)),
            context: tab.title(),
        })
        .collect()
}
