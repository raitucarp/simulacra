//! Form layers (stacked asset templates) and slots (everything the editor can choose).

use super::kind::FORM_I18N_ROOT;
use serde::{Deserialize, Serialize};

/// The reserved variant that draws nothing (no asset file).
pub const NONE_VARIANT: &str = "none";

/// One stacked template layer of a form body. Paint order comes from the kind's catalog.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FormLayer {
    Back,
    Aura,
    Body,
    Finish,
    Markings,
    Mouth,
    Eyes,
    Accessory,
}

impl FormLayer {
    /// Asset folder name, parts-map key and i18n suffix.
    pub const fn key(self) -> &'static str {
        match self {
            Self::Back => "back",
            Self::Aura => "aura",
            Self::Body => "body",
            Self::Finish => "finish",
            Self::Markings => "markings",
            Self::Mouth => "mouth",
            Self::Eyes => "eyes",
            Self::Accessory => "accessory",
        }
    }

    /// i18n key of this layer's label (`forms.layer_finish`).
    pub fn label_key(self) -> String {
        format!("{FORM_I18N_ROOT}.layer_{}", self.key())
    }

    /// i18n key of one variant: forms are `forms.form_<key>`, every other part `forms.v_<key>`.
    pub fn variant_label_key(self, variant: &str) -> String {
        match self {
            Self::Body => format!("{FORM_I18N_ROOT}.form_{variant}"),
            _ => format!("{FORM_I18N_ROOT}.v_{variant}"),
        }
    }
}

/// Which of the two form colors.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FormColor {
    Primary,
    Secondary,
}

impl FormColor {
    /// Template placeholder name (`{{primary}}`) and i18n suffix.
    pub const fn key(self) -> &'static str {
        match self {
            Self::Primary => "primary",
            Self::Secondary => "secondary",
        }
    }
}

/// Everything in a [`FormSpec`](super::FormSpec) the editor can choose or re-roll.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FormSlot {
    Layer(FormLayer),
    Color(FormColor),
}

impl FormSlot {
    /// Every slot in editor order: the layers, then the two colors.
    pub const ALL: [FormSlot; 10] = [
        Self::Layer(FormLayer::Body), Self::Layer(FormLayer::Finish), Self::Layer(FormLayer::Markings),
        Self::Color(FormColor::Primary), Self::Color(FormColor::Secondary), Self::Layer(FormLayer::Eyes),
        Self::Layer(FormLayer::Mouth), Self::Layer(FormLayer::Accessory), Self::Layer(FormLayer::Aura),
        Self::Layer(FormLayer::Back),
    ];

    /// i18n key of this slot's label.
    pub fn label_key(self) -> String {
        match self {
            Self::Layer(layer) => layer.label_key(),
            Self::Color(color) => format!("{FORM_I18N_ROOT}.{}", color.key()),
        }
    }
}
