//! Entry templates: a morning page (intention, priorities, gratitude), an evening review
//! (what went well, learnt, surprised, tomorrow's if-then plan) and a weekly review
//! (highlights, lessons, what to change, next week). A template is a list of headed sections
//! the user fills in; the headings are localized by the caller.

use serde::{Deserialize, Serialize};

/// Markdown mark of a section heading.
const HEADING_MARK: &str = "## ";

/// A template of a new entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Template {
    Morning,
    Evening,
    Weekly,
}

impl Template {
    pub const ALL: [Self; 3] = [Self::Morning, Self::Evening, Self::Weekly];

    pub const fn key(self) -> &'static str {
        match self {
            Self::Morning => "morning",
            Self::Evening => "evening",
            Self::Weekly => "weekly",
        }
    }

    /// Locale keys of its section headings, in order.
    pub fn section_keys(self) -> Vec<String> {
        let sections: &[&str] = match self {
            Self::Morning => &["intention", "priorities", "grateful"],
            Self::Evening => &["went_well", "learned", "surprised", "if_then"],
            Self::Weekly => &["highlights", "lessons", "change", "next_week"],
        };
        sections.iter().map(|section| format!("journal.section.{section}")).collect()
    }
}

/// The body a template starts with: each heading followed by an empty line to write on.
pub fn template_body(headings: &[String]) -> String {
    headings.iter().map(|heading| format!("{HEADING_MARK}{heading}\n\n")).collect::<String>().trim_end().to_string() + "\n"
}
