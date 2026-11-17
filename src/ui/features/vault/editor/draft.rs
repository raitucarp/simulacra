//! The editor's values as plain data, and their conversion to a catalogue record.

use crate::domain::document::intake::{validate, FieldProblem};
use crate::domain::document::record::{
    format_creators, parse_creators, CatalogueRecord, Discipline, DocumentForm, DocumentKind, Extent, Identifier,
    IdentifierScheme, Period,
};
use crate::domain::document::{GroundingDocument, Library};

/// Separators of the subject and collection lists.
const LIST_SEPARATORS: [char; 2] = [',', ';'];
const LIST_JOINER: &str = ", ";

/// Every editable value of the add / edit form.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct EditorDraft {
    pub title: String,
    pub subtitle: String,
    pub kind: DocumentKind,
    pub creators: String,
    pub date: String,
    pub publisher: String,
    pub language: String,
    pub scheme: Option<IdentifierScheme>,
    pub identifier: String,
    pub pages: String,
    pub rights: String,
    pub description: String,
    pub content: String,
    pub subjects: String,
    pub collections: String,
    pub discipline: Option<Discipline>,
    pub form: Option<DocumentForm>,
    pub period: Option<Period>,
    pub place: String,
}

/// The labels of a comma- or semicolon-separated list.
pub fn split_list(text: &str) -> Vec<String> {
    text.split(LIST_SEPARATORS).map(str::trim).filter(|item| !item.is_empty()).map(str::to_string).collect()
}

impl EditorDraft {
    /// The draft of an existing document (subject and collection names from `library`).
    pub fn of_document(doc: &GroundingDocument, library: &Library) -> Self {
        let record = &doc.record;
        let subjects = record.subjects.iter().filter_map(|id| library.subject(id)).map(|s| s.label.clone());
        let collections = record.collections.iter().filter_map(|id| library.collection(id)).map(|c| c.name.clone());
        Self {
            title: doc.title.clone(),
            subtitle: record.subtitle.clone(),
            kind: record.kind,
            creators: format_creators(&record.creators),
            date: record.date.clone(),
            publisher: record.publisher.clone(),
            language: record.language.clone(),
            scheme: record.identifier.scheme,
            identifier: record.identifier.value.clone(),
            pages: record.extent.pages.map(|p| p.to_string()).unwrap_or_default(),
            rights: record.rights.clone(),
            description: record.description.clone(),
            content: doc.content.clone(),
            subjects: subjects.collect::<Vec<_>>().join(LIST_JOINER),
            collections: collections.collect::<Vec<_>>().join(LIST_JOINER),
            discipline: record.classification.discipline,
            form: record.classification.form,
            period: record.classification.period,
            place: record.classification.place.clone(),
        }
    }

    /// Pages as a number: none when blank, `Some(0)` (rejected) when unreadable.
    fn pages(&self) -> Option<u32> {
        let pages = self.pages.trim();
        (!pages.is_empty()).then(|| pages.parse::<u32>().unwrap_or(0))
    }

    /// `base` with the draft's fields written over it (subjects, collections, reading and
    /// provenance stay as in `base`).
    pub fn record(&self, base: &CatalogueRecord) -> CatalogueRecord {
        let mut record = base.clone();
        record.subtitle = self.subtitle.trim().to_string();
        record.kind = self.kind;
        record.creators = parse_creators(&self.creators);
        record.date = self.date.trim().to_string();
        record.publisher = self.publisher.trim().to_string();
        record.language = self.language.trim().to_string();
        record.identifier = match self.scheme {
            Some(scheme) if !self.identifier.trim().is_empty() => Identifier::new(scheme, self.identifier.trim()),
            _ => Identifier::default(),
        };
        record.extent = Extent { pages: self.pages(), ..base.extent };
        record.rights = self.rights.trim().to_string();
        record.description = self.description.trim().to_string();
        record.classification.discipline = self.discipline;
        record.classification.form = self.form;
        record.classification.period = self.period;
        record.classification.place = self.place.trim().to_string();
        if record.kind != base.kind && record.format == base.kind.default_format() {
            record.format.clear();
        }
        record
    }

    /// Problems of the draft over `base` (empty when it can be saved).
    pub fn problems(&self, base: &CatalogueRecord) -> Vec<FieldProblem> {
        validate(&self.title, &self.record(base))
    }

    pub fn subject_labels(&self) -> Vec<String> {
        split_list(&self.subjects)
    }

    pub fn collection_names(&self) -> Vec<String> {
        split_list(&self.collections)
    }
}
