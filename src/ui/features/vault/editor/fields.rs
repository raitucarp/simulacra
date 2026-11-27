//! The single-line text fields of the editor and where each lives in the draft.

use super::draft::EditorDraft;
use crate::ui::features::vault::text::tr;

/// A single-line field of the editor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TextField {
    Title,
    Subtitle,
    Creators,
    Date,
    Publisher,
    Language,
    Identifier,
    Pages,
    Rights,
    Place,
    Subjects,
    Collections,
}

impl TextField {
    pub const ALL: [Self; 12] = [
        Self::Title,
        Self::Subtitle,
        Self::Creators,
        Self::Date,
        Self::Publisher,
        Self::Language,
        Self::Identifier,
        Self::Pages,
        Self::Rights,
        Self::Place,
        Self::Subjects,
        Self::Collections,
    ];

    /// Locale key stem (`vault.field.<key>` label, `vault.hint.<key>` placeholder).
    pub const fn key(self) -> &'static str {
        match self {
            Self::Title => "title",
            Self::Subtitle => "subtitle",
            Self::Creators => "creators",
            Self::Date => "date",
            Self::Publisher => "publisher",
            Self::Language => "language",
            Self::Identifier => "identifier",
            Self::Pages => "pages",
            Self::Rights => "rights",
            Self::Place => "place",
            Self::Subjects => "subjects",
            Self::Collections => "collections",
        }
    }

    pub fn label(self) -> String {
        tr(format!("vault.field.{}", self.key()))
    }

    /// The field's hint. Sample values read "e.g. 1677": shown bare, they looked like data
    /// already filled in. The publisher's hint says what goes there instead.
    pub fn placeholder(self) -> String {
        let hint = tr(format!("vault.hint.{}", self.key()));
        match self {
            Self::Publisher => hint,
            _ => rust_i18n::t!("vault.hint.example", example = hint).to_string(),
        }
    }

    /// The draft value of this field.
    pub fn get(self, draft: &EditorDraft) -> &str {
        match self {
            Self::Title => &draft.title,
            Self::Subtitle => &draft.subtitle,
            Self::Creators => &draft.creators,
            Self::Date => &draft.date,
            Self::Publisher => &draft.publisher,
            Self::Language => &draft.language,
            Self::Identifier => &draft.identifier,
            Self::Pages => &draft.pages,
            Self::Rights => &draft.rights,
            Self::Place => &draft.place,
            Self::Subjects => &draft.subjects,
            Self::Collections => &draft.collections,
        }
    }

    /// Writes `value` into this field of `draft`.
    pub fn set(self, draft: &mut EditorDraft, value: String) {
        let slot = match self {
            Self::Title => &mut draft.title,
            Self::Subtitle => &mut draft.subtitle,
            Self::Creators => &mut draft.creators,
            Self::Date => &mut draft.date,
            Self::Publisher => &mut draft.publisher,
            Self::Language => &mut draft.language,
            Self::Identifier => &mut draft.identifier,
            Self::Pages => &mut draft.pages,
            Self::Rights => &mut draft.rights,
            Self::Place => &mut draft.place,
            Self::Subjects => &mut draft.subjects,
            Self::Collections => &mut draft.collections,
        };
        *slot = value;
    }
}
