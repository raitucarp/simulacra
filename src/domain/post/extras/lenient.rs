//! N16–N20: the newer `extras` fields read leniently: a value this version cannot read (written
//! by a newer one, or damaged) reads as absent instead of failing the whole extras column,
//! which would also lose the post's lock.

use serde::de::DeserializeOwned;
use serde::{Deserialize, Deserializer};

/// `Option<T>` from any JSON value; `None` when it does not read as `T`.
pub fn lenient<'de, D: Deserializer<'de>, T: DeserializeOwned>(deserializer: D) -> Result<Option<T>, D::Error> {
    let value = Option::<serde_json::Value>::deserialize(deserializer)?;
    Ok(value.and_then(|value| serde_json::from_value(value).ok()))
}
