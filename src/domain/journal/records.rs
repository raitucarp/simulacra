//! What the four private record kinds share: an id, and pure replace / remove by id over an
//! immutable list.

use super::ids::RecordId;

/// A record kept by id.
pub trait Record: Clone {
    fn id(&self) -> &RecordId;
}

/// `records` with `record` inserted, or replacing the one with its id (keeping its place).
pub fn upserted<T: Record>(records: &[T], record: T) -> Vec<T> {
    if records.iter().any(|r| r.id() == record.id()) {
        records.iter().map(|r| if r.id() == record.id() { record.clone() } else { r.clone() }).collect()
    } else {
        records.iter().cloned().chain(std::iter::once(record)).collect()
    }
}

/// `records` without the one with `id`.
pub fn removed<T: Record>(records: &[T], id: &RecordId) -> Vec<T> {
    records.iter().filter(|r| r.id() != id).cloned().collect()
}

/// The record with `id`.
pub fn find<'a, T: Record>(records: &'a [T], id: &RecordId) -> Option<&'a T> {
    records.iter().find(|r| r.id() == id)
}
