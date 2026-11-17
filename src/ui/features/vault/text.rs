//! Localized labels of the catalogue vocabularies (`vault.<group>.<key>` locale keys).

use crate::domain::document::citation::CitationStyle;
use crate::domain::document::facets::Facet;
use crate::domain::document::intake::FieldProblem;
use crate::domain::document::query::SortKey;
use crate::domain::document::record::{CreatorRole, Discipline, DocumentForm, DocumentKind, IdentifierProblem, IdentifierScheme, Period, ReadingStatus};
use crate::domain::document::Library;
use crate::domain::document::{CollectionId, SubjectId};
use rust_i18n::t;

/// The translation of a key built at run time.
pub fn tr(key: String) -> String {
    t!(key.as_str()).to_string()
}

fn label(group: &str, key: &str) -> String {
    tr(format!("vault.{group}.{key}"))
}

pub fn kind_label(kind: DocumentKind) -> String {
    label("kind", kind.key())
}

pub fn discipline_label(discipline: Discipline) -> String {
    label("discipline", discipline.key())
}

pub fn form_label(form: DocumentForm) -> String {
    label("form", form.key())
}

pub fn period_label(period: Period) -> String {
    label("period", period.key())
}

pub fn status_label(status: ReadingStatus) -> String {
    label("status", status.key())
}

pub fn sort_label(key: SortKey) -> String {
    label("sort", key.key())
}

pub fn facet_label(facet: Facet) -> String {
    label("facet", facet.key())
}

pub fn style_label(style: CitationStyle) -> String {
    label("style", style.key())
}

pub fn role_label(role: CreatorRole) -> String {
    label("role", role.key())
}

pub fn scheme_label(scheme: IdentifierScheme) -> String {
    label("scheme", scheme.key())
}

/// The label of a facet value key as shown in the facet list and filter chips.
pub fn facet_value_label(facet: Facet, key: &str, library: &Library) -> String {
    let named = |name: Option<String>| name.unwrap_or_else(|| key.to_string());
    match facet {
        Facet::Kind => named(DocumentKind::from_key(key).map(kind_label)),
        Facet::Discipline => named(Discipline::from_key(key).map(discipline_label)),
        Facet::Form => named(DocumentForm::from_key(key).map(form_label)),
        Facet::Period => named(Period::from_key(key).map(period_label)),
        Facet::Status => named(ReadingStatus::from_key(key).map(status_label)),
        Facet::Subject => named(library.subject(&SubjectId::new(key)).map(|s| s.label.clone())),
        Facet::Collection => named(library.collection(&CollectionId::new(key)).map(|c| c.name.clone())),
        Facet::Language => key.to_uppercase(),
        Facet::Place => key.to_string(),
    }
}

/// The message of a validation problem.
pub fn problem_message(problem: FieldProblem) -> String {
    match problem {
        FieldProblem::TitleMissing => t!("vault.problem.title_missing"),
        FieldProblem::TitleTooLong => t!("vault.problem.title_too_long", max = crate::domain::document::intake::MAX_TITLE_CHARS),
        FieldProblem::DateUnreadable => t!("vault.problem.date_unreadable"),
        FieldProblem::LanguageUnreadable => t!("vault.problem.language_unreadable"),
        FieldProblem::Identifier(IdentifierProblem::BadIsbn) => t!("vault.problem.bad_isbn"),
        FieldProblem::Identifier(IdentifierProblem::BadDoi) => t!("vault.problem.bad_doi"),
        FieldProblem::Identifier(IdentifierProblem::BadUrl) => t!("vault.problem.bad_url"),
        FieldProblem::PagesZero => t!("vault.problem.pages_zero"),
    }
    .to_string()
}
