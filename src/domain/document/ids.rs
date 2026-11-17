//! Identifiers of vault documents, controlled-vocabulary subjects and collections.

use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};

macro_rules! catalogue_id {
    ($(#[$meta:meta])* $name:ident) => {
        $(#[$meta])*
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize, Default)]
        #[serde(transparent)]
        pub struct $name(String);

        impl $name {
            pub fn new(id: impl Into<String>) -> Self {
                Self(id.into())
            }

            pub fn as_str(&self) -> &str {
                &self.0
            }
        }

        impl Display for $name {
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                f.write_str(&self.0)
            }
        }

        impl From<&str> for $name {
            fn from(id: &str) -> Self {
                Self::new(id)
            }
        }

        impl std::borrow::Borrow<str> for $name {
            fn borrow(&self) -> &str {
                &self.0
            }
        }
    };
}

catalogue_id!(
    /// A Knowledge Vault document (the `documents.id` column).
    DocumentId
);
catalogue_id!(
    /// A term of the subject vocabulary.
    SubjectId
);
catalogue_id!(
    /// A user-defined collection (shelf).
    CollectionId
);

/// Prefixes of generated identifiers.
pub const DOCUMENT_ID_PREFIX: &str = "doc_";
pub const SUBJECT_ID_PREFIX: &str = "subj_";
pub const COLLECTION_ID_PREFIX: &str = "coll_";

/// A fresh random identifier with `prefix`.
pub fn generated_id(prefix: &str) -> String {
    format!("{prefix}{}", uuid::Uuid::new_v4().simple())
}
