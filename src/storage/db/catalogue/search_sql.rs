//! SQL fragments of a catalogue search.

use crate::domain::document::facets::Facet;
use crate::domain::document::query::SortKey;

pub const BASE_SELECT: &str = "SELECT d.id, ";
/// FTS5 rank weighting the columns (id, title, creators, subjects, abstract, body);
/// lower is better.
pub const TEXT_SCORE: &str = "bm25(documents_fts, 0.0, 4.0, 3.0, 2.5, 2.0, 1.0) AS score \
    FROM documents d LEFT JOIN catalogue_records c ON c.document_id = d.id";
pub const NO_SCORE: &str = "0.0 AS score FROM documents d LEFT JOIN catalogue_records c ON c.document_id = d.id";
pub const FTS_JOIN: &str = " JOIN documents_fts ON documents_fts.document_id = d.id WHERE documents_fts MATCH ? AND";
pub const NETWORK_FILTER: &str = " d.network_id = ?";
pub const WHERE: &str = " WHERE";

/// Posts of the document's network that link to it by wikilink (id or title) or source.
const LINK_COUNT: &str = r#"(SELECT COUNT(*) FROM posts p WHERE p.network_id = d.network_id AND (
    EXISTS (SELECT 1 FROM json_each(CASE WHEN json_valid(p.linked_posts_json) THEN p.linked_posts_json ELSE '[]' END) j
        WHERE trim(j.value) = d.id OR lower(trim(j.value)) = lower(trim(d.title)))
    OR EXISTS (SELECT 1 FROM json_each(CASE WHEN json_valid(p.extras_json) THEN p.extras_json ELSE '{}' END, '$.sources') s
        WHERE json_extract(s.value, '$.kind') = 'vault_doc' AND json_extract(s.value, '$.id') = d.id)))"#;

/// The column or expression a facet is stored in.
fn facet_target(facet: Facet) -> &'static str {
    match facet {
        Facet::Kind => "c.kind",
        Facet::Discipline => "c.discipline",
        Facet::Form => "c.form",
        Facet::Period => "c.period",
        Facet::Place => "c.place",
        Facet::Language => "c.language",
        Facet::Status => "c.reading_status",
        Facet::Subject => "d.id IN (SELECT document_id FROM document_subjects WHERE subject_id",
        Facet::Collection => "d.id IN (SELECT document_id FROM collection_documents WHERE collection_id",
    }
}

/// ` AND <target> IN (?, ?)` for `count` values (a sub-select is closed after the list).
pub fn facet_clause(facet: Facet, count: usize) -> String {
    let placeholders = vec!["?"; count.max(1)].join(", ");
    let closing = if matches!(facet, Facet::Subject | Facet::Collection) { ")" } else { "" };
    format!(" AND {} IN ({placeholders}){closing}", facet_target(facet))
}

/// ` ORDER BY ...` for `key`, ties broken by filing title then id.
pub fn order_clause(key: SortKey, descending: bool) -> String {
    let direction = if descending { "DESC" } else { "ASC" };
    let primary = match key {
        // bm25: smaller is a better match, so "best first" is ascending.
        SortKey::Relevance => format!("score {}", if descending { "ASC" } else { "DESC" }),
        SortKey::Title => format!("c.sort_title {direction}"),
        SortKey::DateAdded => format!("d.created_at {direction}"),
        SortKey::DatePublished => format!("c.year_published {direction}"),
        SortKey::MostLinked => format!("{LINK_COUNT} {direction}"),
    };
    format!(" ORDER BY {primary}, c.sort_title ASC, d.id ASC")
}
