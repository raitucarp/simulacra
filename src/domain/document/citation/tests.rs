use super::{cite, CitationSource, CitationStyle};
use crate::domain::document::record::{CatalogueRecord, Creator, CreatorRole, DocumentKind, Identifier, IdentifierScheme};

fn ethics() -> CatalogueRecord {
    CatalogueRecord {
        kind: DocumentKind::Book,
        subtitle: "Demonstrated in Geometrical Order".into(),
        creators: vec![Creator::author("Baruch Spinoza"), Creator::new("R. H. M. Elwes", CreatorRole::Translator)],
        date: "1677".into(),
        publisher: "Rieuwertsz".into(),
        ..Default::default()
    }
}

fn text(title: &str, record: &CatalogueRecord, style: CitationStyle) -> String {
    cite(CitationSource { title, record }, style, 1).text()
}

#[test]
fn author_date_leads_with_name_and_year() {
    assert_eq!(
        text("Ethics", &ethics(), CitationStyle::AuthorDate),
        "Spinoza, B. (1677). Ethics: Demonstrated in Geometrical Order (R. H. M. Elwes, Trans.). Rieuwertsz."
    );
}

#[test]
fn notes_and_bibliography_spells_out_names_and_ends_with_the_year() {
    assert_eq!(
        text("Ethics", &ethics(), CitationStyle::NotesBibliography),
        "Spinoza, Baruch. Ethics: Demonstrated in Geometrical Order. Translated by R. H. M. Elwes. Rieuwertsz, 1677."
    );
}

#[test]
fn numeric_numbers_the_reference_and_uses_initials() {
    let citation = cite(CitationSource { title: "Ethics", record: &ethics() }, CitationStyle::Numeric, 3);
    assert_eq!(citation.text(), "[3] B. Spinoza, Ethics: Demonstrated in Geometrical Order, Rieuwertsz, 1677.");
}

#[test]
fn whole_works_are_emphasised_and_parts_are_quoted() {
    let book = cite(CitationSource { title: "Ethics", record: &ethics() }, CitationStyle::NotesBibliography, 1);
    assert!(book.parts.iter().any(|part| part.emphasis && part.text.starts_with("Ethics")));
    let article = CatalogueRecord {
        kind: DocumentKind::Article,
        creators: vec![Creator::author("Alan Turing")],
        date: "1950".into(),
        identifier: Identifier::new(IdentifierScheme::Doi, "10.1093/mind/LIX.236.433"),
        ..Default::default()
    };
    let notes = text("Computing Machinery and Intelligence", &article, CitationStyle::NotesBibliography);
    assert_eq!(notes, "Turing, Alan. \u{201c}Computing Machinery and Intelligence.\u{201d} 1950. https://doi.org/10.1093/mind/LIX.236.433.");
    let numeric = text("Computing Machinery and Intelligence", &article, CitationStyle::Numeric);
    assert_eq!(numeric, "[1] A. Turing, \u{201c}Computing Machinery and Intelligence,\u{201d} 1950. https://doi.org/10.1093/mind/LIX.236.433");
    let cited = cite(CitationSource { title: "Computing Machinery and Intelligence", record: &article }, CitationStyle::AuthorDate, 1);
    assert!(cited.parts.iter().all(|part| !part.emphasis));
}

#[test]
fn several_authors_are_joined_per_style() {
    let record = CatalogueRecord {
        kind: DocumentKind::Book,
        creators: ["Alfred North Whitehead", "Bertrand Russell", "Ada Lovelace"].map(Creator::author).to_vec(),
        date: "1910".into(),
        ..Default::default()
    };
    assert!(text("Principia", &record, CitationStyle::AuthorDate).starts_with("Whitehead, A. N., Russell, B., & Lovelace, A. (1910)."));
    assert!(text("Principia", &record, CitationStyle::NotesBibliography)
        .starts_with("Whitehead, Alfred North, Bertrand Russell, and Ada Lovelace. Principia."));
    assert!(text("Principia", &record, CitationStyle::Numeric).starts_with("[1] A. N. Whitehead, B. Russell, and A. Lovelace, Principia"));
}

#[test]
fn missing_authors_and_dates_degrade_gracefully() {
    let bare = CatalogueRecord { kind: DocumentKind::Note, ..Default::default() };
    assert_eq!(text("Field notes", &bare, CitationStyle::AuthorDate), "Field notes (n.d.).");
    assert_eq!(text("Field notes", &bare, CitationStyle::NotesBibliography), "\u{201c}Field notes.\u{201d}");
    assert_eq!(text("Field notes", &bare, CitationStyle::Numeric), "[1] \u{201c}Field notes.\u{201d}");
}
