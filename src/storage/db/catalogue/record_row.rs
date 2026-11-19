//! The scalar columns of a `catalogue_records` row, derived from a document.

use crate::domain::document::query::filing_title;
use crate::domain::GroundingDocument;
use rusqlite::types::Value;

/// Parameters of the record upsert, in column order (`?1` = document id).
pub fn record_params(doc: &GroundingDocument) -> Vec<Value> {
    let record = &doc.record;
    let class = &record.classification;
    let key = |key: Option<&str>| Value::Text(key.unwrap_or_default().to_string());
    vec![
        Value::Text(doc.id.clone()),
        Value::Text(serde_json::to_string(record).unwrap_or_default()),
        Value::Text(record.kind.key().to_string()),
        Value::Text(filing_title(&doc.title)),
        record.published_year().map_or(Value::Null, |year| Value::Integer(i64::from(year))),
        Value::Text(record.language.trim().to_lowercase()),
        key(class.discipline.map(|d| d.key())),
        key(class.form.map(|f| f.key())),
        key(class.period.map(|p| p.key())),
        Value::Text(class.place.trim().to_string()),
        Value::Text(record.reading.status.key().to_string()),
        Value::Integer(i64::from(record.reading.progress)),
        record.updated_at.map_or(Value::Null, |at| Value::Text(at.to_rfc3339())),
    ]
}

/// Upsert of one record row (parameters from [`record_params`]).
pub const UPSERT_RECORD: &str = r#"
    INSERT INTO catalogue_records (document_id, record_json, kind, sort_title, year_published, language,
        discipline, form, period, place, reading_status, reading_progress, updated_at)
    VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)
    ON CONFLICT(document_id) DO UPDATE SET
        record_json = excluded.record_json, kind = excluded.kind, sort_title = excluded.sort_title,
        year_published = excluded.year_published, language = excluded.language,
        discipline = excluded.discipline, form = excluded.form, period = excluded.period,
        place = excluded.place, reading_status = excluded.reading_status,
        reading_progress = excluded.reading_progress, updated_at = excluded.updated_at;
"#;

/// Creator names of a document, for the full-text index.
pub fn creator_text(doc: &GroundingDocument) -> String {
    doc.record.creators.iter().map(|c| c.name.as_str()).collect::<Vec<_>>().join(" ")
}
