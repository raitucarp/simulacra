//! The colour a post's container is tinted with (sb C1): a small named set whose actual
//! colours come from the theme (`theme::tint_colors`), so a tint reads well in every
//! palette, light or dark.

use crate::domain::document::keyed::keyed_enum;
use serde::{Deserialize, Deserializer};

keyed_enum! {
    /// A named container tint. Keys are persisted in `extras.tint`: never rename one.
    Tint {
        Amber => "amber",
        Rose => "rose",
        Sky => "sky",
        Mint => "mint",
        Violet => "violet",
        Slate => "slate",
    }
}

/// Reads `extras.tint` leniently: a tint this version does not know (written by a newer
/// one) or a malformed value reads as no tint instead of failing the whole extras column.
pub(super) fn lenient<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Option<Tint>, D::Error> {
    let value = Option::<serde_json::Value>::deserialize(deserializer)?;
    Ok(value.as_ref().and_then(serde_json::Value::as_str).and_then(Tint::from_key))
}

#[cfg(test)]
mod tests;
