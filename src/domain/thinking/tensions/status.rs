//! What the user did with a pair the finder showed.

use crate::domain::document::keyed::keyed_enum;

keyed_enum! {
    /// The state of a pair; the key is stored in the tensions table.
    TensionStatus {
        /// Still to look at.
        Open => "open",
        /// Reconciled (by a reply, an edit or a decision).
        Resolved => "resolved",
        /// Not a real tension; never shown again for these texts.
        Dismissed => "dismissed",
    }
}

impl Default for TensionStatus {
    fn default() -> Self {
        Self::Open
    }
}
