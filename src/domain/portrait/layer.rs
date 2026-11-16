//! Portrait layers (stacked asset templates) and slots (everything a player can re-roll).

use serde::{Deserialize, Serialize};

/// Asset folder of portrait parts, relative to the asset root.
pub const PORTRAIT_ASSET_ROOT: &str = "illustrations/portrait";
/// File extension of portrait part templates.
pub const PORTRAIT_ASSET_EXT: &str = "svg";
/// i18n namespace of portrait labels (`portrait.<slot>` and `portrait.<slot>_<variant>`).
pub const PORTRAIT_I18N_ROOT: &str = "portrait";

/// One stacked template layer of a generated portrait.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PortraitLayer {
    Back,
    Face,
    Outfit,
    Mouth,
    Eyes,
    Brows,
    Hair,
    Accessory,
}

impl PortraitLayer {
    /// Paint order, bottom to top (see `assets/illustrations/STYLE.md`).
    pub const STACK: [PortraitLayer; 8] = [
        Self::Back, Self::Face, Self::Outfit, Self::Mouth,
        Self::Eyes, Self::Brows, Self::Hair, Self::Accessory,
    ];

    /// Asset folder name of this layer.
    pub const fn key(self) -> &'static str {
        match self {
            Self::Back => "back",
            Self::Face => "face",
            Self::Outfit => "outfit",
            Self::Mouth => "mouth",
            Self::Eyes => "eyes",
            Self::Brows => "brows",
            Self::Hair => "hair",
            Self::Accessory => "accessory",
        }
    }

    /// Asset path of `variant` on this layer: `illustrations/portrait/<layer>/<variant>.svg`.
    pub fn asset_path(self, variant: &str) -> String {
        format!("{PORTRAIT_ASSET_ROOT}/{}/{variant}.{PORTRAIT_ASSET_EXT}", self.key())
    }
}

/// Everything in a [`PortraitSpec`](super::PortraitSpec) that can be chosen or re-rolled:
/// the eight layers plus the two color choices.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PortraitSlot {
    Back,
    Face,
    Skin,
    Eyes,
    Brows,
    Mouth,
    Hair,
    HairColor,
    Outfit,
    Accessory,
}

impl PortraitSlot {
    /// Every slot in editor order.
    pub const ALL: [PortraitSlot; 10] = [
        Self::Back, Self::Face, Self::Skin, Self::Eyes, Self::Brows,
        Self::Mouth, Self::Hair, Self::HairColor, Self::Outfit, Self::Accessory,
    ];

    /// Stable snake_case identifier (also the i18n suffix: `portrait.<key>`).
    pub const fn key(self) -> &'static str {
        match self {
            Self::Skin => "skin",
            Self::HairColor => "hair_color",
            other => match other.layer() {
                Some(layer) => layer.key(),
                None => "",
            },
        }
    }

    /// The asset layer this slot selects, if it is one.
    pub const fn layer(self) -> Option<PortraitLayer> {
        Some(match self {
            Self::Back => PortraitLayer::Back,
            Self::Face => PortraitLayer::Face,
            Self::Eyes => PortraitLayer::Eyes,
            Self::Brows => PortraitLayer::Brows,
            Self::Mouth => PortraitLayer::Mouth,
            Self::Hair => PortraitLayer::Hair,
            Self::Outfit => PortraitLayer::Outfit,
            Self::Accessory => PortraitLayer::Accessory,
            Self::Skin | Self::HairColor => return None,
        })
    }

    /// i18n key of this slot's label (`portrait.hair_color`).
    pub fn label_key(self) -> String {
        format!("{PORTRAIT_I18N_ROOT}.{}", self.key())
    }

    /// i18n key of one of this slot's options (`portrait.hair_side_part`).
    pub fn option_label_key(self, option_key: &str) -> String {
        format!("{PORTRAIT_I18N_ROOT}.{}_{option_key}", self.key())
    }
}
