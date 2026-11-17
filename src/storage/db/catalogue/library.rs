//! The subject vocabulary and the collections.

use crate::domain::document::{Collection, CollectionId, Library, Subject, SubjectId};
use crate::domain::NetworkId;
use crate::error::Result;
use crate::storage::db::{collect_rows, db_err, Database};
use rusqlite::{params, Row};

const UPSERT_SUBJECT: &str = r#"
    INSERT INTO subjects (id, label, scope_note, broader_id) VALUES (?1, ?2, ?3, ?4)
    ON CONFLICT(id) DO UPDATE SET label = excluded.label, scope_note = excluded.scope_note,
        broader_id = excluded.broader_id;
"#;
const CLEAR_RELATIONS: &str = "DELETE FROM subject_relations WHERE subject_id = ?1";
const RELATE: &str = "INSERT OR IGNORE INTO subject_relations (subject_id, related_id) VALUES (?1, ?2)";
const SELECT_SUBJECTS: &str = r#"
    SELECT id, label, scope_note, broader_id,
        (SELECT json_group_array(related_id) FROM subject_relations WHERE subject_id = subjects.id)
    FROM subjects ORDER BY label COLLATE NOCASE
"#;
/// Deleting a term hands its narrower terms to its own broader term and unfiles it.
const REPARENT_NARROWER: &str = "UPDATE subjects SET broader_id = (SELECT broader_id FROM subjects WHERE id = ?1) WHERE broader_id = ?1";
const DELETE_SUBJECT_STATEMENTS: [&str; 3] = [
    "DELETE FROM document_subjects WHERE subject_id = ?1",
    "DELETE FROM subject_relations WHERE subject_id = ?1 OR related_id = ?1",
    "DELETE FROM subjects WHERE id = ?1",
];
const UPSERT_COLLECTION: &str = r#"
    INSERT INTO collections (id, network_id, name, description) VALUES (?1, ?2, ?3, ?4)
    ON CONFLICT(id) DO UPDATE SET network_id = excluded.network_id, name = excluded.name,
        description = excluded.description;
"#;
const SELECT_COLLECTIONS: &str = "SELECT id, network_id, name, description FROM collections ORDER BY name COLLATE NOCASE";
const DELETE_COLLECTION_STATEMENTS: [&str; 2] =
    ["DELETE FROM collection_documents WHERE collection_id = ?1", "DELETE FROM collections WHERE id = ?1"];

impl Database {
    /// Saves (inserts or replaces) one vocabulary term and its related terms.
    pub fn save_subject(&self, subject: &Subject) -> Result<()> {
        let transaction = self.conn.unchecked_transaction().map_err(db_err)?;
        let broader = subject.broader.as_ref().map(SubjectId::as_str);
        let id = subject.id.as_str();
        self.conn.execute(UPSERT_SUBJECT, params![id, subject.label, subject.scope_note, broader]).map_err(db_err)?;
        self.conn.execute(CLEAR_RELATIONS, [id]).map_err(db_err)?;
        for related in &subject.related {
            self.conn.execute(RELATE, [id, related.as_str()]).map_err(db_err)?;
        }
        transaction.commit().map_err(db_err)
    }

    /// Deletes a term: its narrower terms move up to its broader term and documents filed
    /// under it lose the term.
    pub fn delete_subject(&self, id: &SubjectId) -> Result<()> {
        let transaction = self.conn.unchecked_transaction().map_err(db_err)?;
        self.conn.execute(REPARENT_NARROWER, [id.as_str()]).map_err(db_err)?;
        for statement in DELETE_SUBJECT_STATEMENTS {
            self.conn.execute(statement, [id.as_str()]).map_err(db_err)?;
        }
        transaction.commit().map_err(db_err)
    }

    /// Saves (inserts or replaces) a collection.
    pub fn save_collection(&self, collection: &Collection) -> Result<()> {
        let values = params![collection.id.as_str(), collection.network_id.as_str(), collection.name, collection.description];
        self.conn.execute(UPSERT_COLLECTION, values).map_err(db_err)?;
        Ok(())
    }

    /// Deletes a collection (its documents stay).
    pub fn delete_collection(&self, id: &CollectionId) -> Result<()> {
        let transaction = self.conn.unchecked_transaction().map_err(db_err)?;
        for statement in DELETE_COLLECTION_STATEMENTS {
            self.conn.execute(statement, [id.as_str()]).map_err(db_err)?;
        }
        transaction.commit().map_err(db_err)
    }

    /// The whole vocabulary and every collection.
    pub fn load_library(&self) -> Result<Library> {
        let mut subjects = self.conn.prepare(SELECT_SUBJECTS).map_err(db_err)?;
        let subjects = collect_rows(subjects.query_map([], subject_from_row).map_err(db_err)?)?;
        let mut collections = self.conn.prepare(SELECT_COLLECTIONS).map_err(db_err)?;
        let collections = collect_rows(collections.query_map([], collection_from_row).map_err(db_err)?)?;
        Ok(Library { subjects, collections })
    }
}

fn subject_from_row(row: &Row<'_>) -> rusqlite::Result<Subject> {
    let related: Option<String> = row.get(4)?;
    Ok(Subject {
        id: SubjectId::new(row.get::<_, String>(0)?),
        label: row.get(1)?,
        scope_note: row.get(2)?,
        broader: row.get::<_, Option<String>>(3)?.map(SubjectId::new),
        related: related.and_then(|json| serde_json::from_str(&json).ok()).unwrap_or_default(),
    })
}

fn collection_from_row(row: &Row<'_>) -> rusqlite::Result<Collection> {
    Ok(Collection {
        id: CollectionId::new(row.get::<_, String>(0)?),
        network_id: NetworkId::new(row.get::<_, String>(1)?),
        name: row.get(2)?,
        description: row.get(3)?,
    })
}
