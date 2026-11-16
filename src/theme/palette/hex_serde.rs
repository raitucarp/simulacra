//! Serde adapter persisting colors as `#rrggbb` / `#rrggbbaa` strings.

use crate::theme::color::{parse_hex, to_hex};
use gpui_kit::Rgba;
use serde::de::Error as _;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Serializes `color` as hex text.
pub fn serialize<S: Serializer>(color: &Rgba, serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(&to_hex(*color))
}

/// Parses hex text, rejecting anything that is not `#rgb`, `#rrggbb` or `#rrggbbaa`.
pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Rgba, D::Error> {
    let text = String::deserialize(deserializer)?;
    parse_hex(&text).ok_or_else(|| D::Error::custom(format!("invalid hex color `{text}`")))
}

/// A color that (de)serializes as hex text, for map values and other non-field positions.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HexColor(pub Rgba);

impl Serialize for HexColor {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serialize(&self.0, serializer)
    }
}

impl<'de> Deserialize<'de> for HexColor {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserialize(deserializer).map(HexColor)
    }
}
