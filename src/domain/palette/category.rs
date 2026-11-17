//! The gallery shelves full palettes are sorted onto.

use super::{ART_EXT, ART_ROOT, PALETTE_I18N_ROOT};
use serde::{Deserialize, Serialize};

/// Illustration folder of the category icons (160×120 cards).
pub const CATEGORY_ART_DIR: &str = "palette_categories";
const CATEGORY_GROUP: &str = "category";
/// Icon and i18n key of the "every category" filter.
pub const ALL_CATEGORIES_KEY: &str = "all";

/// A shelf of the start gallery; also the catalog folder of its palettes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaletteCategory {
    Thinkers,
    Objects,
    Nature,
    Creatures,
    Machines,
    Places,
    Concepts,
}

impl PaletteCategory {
    /// Every category, in tab order.
    pub const ALL: [Self; 7] = [Self::Thinkers, Self::Objects, Self::Nature, Self::Creatures, Self::Machines, Self::Places, Self::Concepts];

    /// Stable snake_case id: the catalog folder, icon file stem and i18n key.
    pub const fn key(self) -> &'static str {
        match self {
            Self::Thinkers => "thinkers",
            Self::Objects => "objects",
            Self::Nature => "nature",
            Self::Creatures => "creatures",
            Self::Machines => "machines",
            Self::Places => "places",
            Self::Concepts => "concepts",
        }
    }

    /// The category whose folder is `key`.
    pub fn from_key(key: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|category| category.key() == key)
    }

    /// i18n key of the tab label: `forge.palette.category.{key}`.
    pub fn label_key(self) -> String {
        category_label_key(self.key())
    }

    /// Asset path of the category icon.
    pub fn illustration(self) -> String {
        category_illustration(self.key())
    }
}

/// i18n key of a category tab (`all` for the unfiltered tab).
pub fn category_label_key(key: &str) -> String {
    format!("{PALETTE_I18N_ROOT}.{CATEGORY_GROUP}.{key}")
}

/// Icon path of a category tab (`all` for the unfiltered tab).
pub fn category_illustration(key: &str) -> String {
    format!("{ART_ROOT}/{CATEGORY_ART_DIR}/{key}.{ART_EXT}")
}
