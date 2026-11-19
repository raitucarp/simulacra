//! The settings categories as plain data for the command palette: id, localized title and
//! keywords. Opening one goes through `AppView::open_settings_section`.

use crate::ui::features::settings::tabs::SettingsTab;
use crate::ui::icons::LucideIcon;

/// Separator of the translated keyword list.
const KEYWORD_SEPARATOR: char = ',';

/// One settings category as the command palette lists it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SettingsSectionEntry {
    /// Stable id ([`SettingsTab::from_id`] turns it back into the category).
    pub id: &'static str,
    pub title: String,
    pub keywords: Vec<String>,
    pub icon: LucideIcon,
}

/// Every settings category in rail order, translated into the active locale.
pub fn settings_sections() -> Vec<SettingsSectionEntry> {
    SettingsTab::ALL
        .into_iter()
        .map(|tab| SettingsSectionEntry {
            id: tab.id(),
            title: tab.title(),
            keywords: split_keywords(&tab.keywords()),
            icon: tab.icon(),
        })
        .collect()
}

/// The non-blank, trimmed items of a comma-separated keyword list.
pub(super) fn split_keywords(raw: &str) -> Vec<String> {
    raw.split(KEYWORD_SEPARATOR).map(str::trim).filter(|word| !word.is_empty()).map(str::to_string).collect()
}
