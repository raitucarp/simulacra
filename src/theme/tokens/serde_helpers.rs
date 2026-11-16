//! Serde adapters persisting gpui colors and pixels as plain JSON numbers.

/// Custom serde helper for gpui_kit::Rgba
pub(super) mod rgba_serde {
    use gpui_kit::{rgba, Rgba};
    use serde::{Deserialize, Deserializer, Serializer};

    /// Channel scale used to quantize a unit float into one byte.
    const CHANNEL_MAX: f32 = 255.0;
    const RED_SHIFT: u32 = 24;
    const GREEN_SHIFT: u32 = 16;
    const BLUE_SHIFT: u32 = 8;

    fn channel(value: f32) -> u32 {
        (value * CHANNEL_MAX).round() as u32
    }

    pub fn serialize<S>(color: &Rgba, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Serialize as u32 hex 0xRRGGBBAA
        let val = (channel(color.r) << RED_SHIFT)
            | (channel(color.g) << GREEN_SHIFT)
            | (channel(color.b) << BLUE_SHIFT)
            | channel(color.a);
        serializer.serialize_u32(val)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> std::result::Result<Rgba, D::Error>
    where
        D: Deserializer<'de>,
    {
        let val = u32::deserialize(deserializer)?;
        Ok(rgba(val))
    }
}

/// Helper for Pixels serialization
pub(super) mod px_serde {
    use gpui_kit::{px, Pixels};
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(pixels: &Pixels, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_f32(pixels.as_f32())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> std::result::Result<Pixels, D::Error>
    where
        D: Deserializer<'de>,
    {
        let val = f32::deserialize(deserializer)?;
        Ok(px(val))
    }
}
