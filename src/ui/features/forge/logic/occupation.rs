//! Quick-pick occupations shown as illustrated cards on the Origin step, each dressed in
//! the outfit `suggested_outfit` would give it.

use crate::domain::portrait::OutfitStyle;

const OCCUPATION_ROOT: &str = "forge.ui.occupation";

/// A common occupation offered as a one-click card.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OccupationPreset {
    Philosopher,
    Scientist,
    Professor,
    Writer,
    Engineer,
    Lawyer,
    Mystic,
    Artisan,
}

impl OccupationPreset {
    pub const ALL: [Self; 8] = [
        Self::Philosopher, Self::Scientist, Self::Professor, Self::Writer,
        Self::Engineer, Self::Lawyer, Self::Mystic, Self::Artisan,
    ];

    pub const fn key(self) -> &'static str {
        match self {
            Self::Philosopher => "philosopher",
            Self::Scientist => "scientist",
            Self::Professor => "professor",
            Self::Writer => "writer",
            Self::Engineer => "engineer",
            Self::Lawyer => "lawyer",
            Self::Mystic => "mystic",
            Self::Artisan => "artisan",
        }
    }

    /// The outfit the occupation card shows (and the portrait switches to).
    pub const fn outfit(self) -> OutfitStyle {
        match self {
            Self::Philosopher => OutfitStyle::Toga,
            Self::Scientist => OutfitStyle::LabCoat,
            Self::Professor => OutfitStyle::ScholarGown,
            Self::Writer => OutfitStyle::Cardigan,
            Self::Engineer => OutfitStyle::Hoodie,
            Self::Lawyer => OutfitStyle::Suit,
            Self::Mystic => OutfitStyle::Robe,
            Self::Artisan => OutfitStyle::Tunic,
        }
    }

    /// i18n key of the occupation name (also the text written into the field).
    pub fn label_key(self) -> String {
        format!("{OCCUPATION_ROOT}.{}", self.key())
    }
}
