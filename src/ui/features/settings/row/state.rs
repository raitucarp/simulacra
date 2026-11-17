//! What a row renders from: the shared context, its highlight ranges, element ids and
//! whether it differs from its default.

use super::super::callbacks::SettingsActions;
use super::super::catalog::{SearchHit, SettingRow, ThemeScale};
use super::super::consts::ROW_ID_PREFIX;
use super::super::props::{SettingsData, SettingsInputs};
use crate::theme::{Density, ThemeTokens, TypeScale};
use gpui_kit::SharedString;
use std::ops::Range;

/// Everything a row needs besides its own id.
pub(in crate::ui::features::settings) struct RowContext<'a> {
    pub theme: &'a ThemeTokens,
    pub data: &'a SettingsData,
    pub inputs: &'a SettingsInputs,
    pub actions: &'a SettingsActions,
}

/// Search matches to highlight in a row's title and description.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(in crate::ui::features::settings) struct RowHighlight {
    pub title: Vec<Range<usize>>,
    pub description: Vec<Range<usize>>,
}

impl RowHighlight {
    /// The ranges of a search hit.
    pub fn of(hit: &SearchHit) -> Self {
        Self { title: hit.title_ranges.clone(), description: hit.description_ranges.clone() }
    }
}

/// `settings-row-<row id>-<part>`.
pub(in crate::ui::features::settings) fn element_id(row: SettingRow, part: &str) -> SharedString {
    SharedString::from(format!("{ROW_ID_PREFIX}{}-{part}", row.id()))
}

/// Whether `row` differs from its default (rows without a value never do).
pub(in crate::ui::features::settings) fn is_modified(row: SettingRow, data: &SettingsData) -> bool {
    match row {
        SettingRow::Field(field) => field.is_modified(&data.prefs),
        SettingRow::Theme(ThemeScale::Density) => data.density != Density::default(),
        SettingRow::Theme(ThemeScale::FontSize) => data.type_scale != TypeScale::default(),
        SettingRow::Action(_) | SettingRow::Path(_) | SettingRow::Embedded(_) => false,
    }
}
