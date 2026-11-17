//! Faceted classification: discipline, form, period and place.
//!
//! The discipline scheme is a small built-in set of ten general main classes, the way general
//! library classifications divide all knowledge; it is deliberately not any published scheme.

use crate::domain::document::keyed::keyed_enum;
use serde::{Deserialize, Serialize};

keyed_enum! {
    /// Main class of knowledge a work belongs to.
    Discipline {
        GeneralWorks => "general_works",
        Philosophy => "philosophy",
        Religion => "religion",
        SocialSciences => "social_sciences",
        Language => "language",
        Sciences => "sciences",
        Technology => "technology",
        Arts => "arts",
        Literature => "literature",
        History => "history",
    }
}

keyed_enum! {
    /// Form or genre of the work (how it is written, not what it is about).
    DocumentForm {
        Treatise => "treatise",
        Essay => "essay",
        Dialogue => "dialogue",
        Aphorisms => "aphorisms",
        Reference => "reference",
        Manual => "manual",
        Fiction => "fiction",
        Poetry => "poetry",
        Correspondence => "correspondence",
        Lecture => "lecture",
        Report => "report",
    }
}

keyed_enum! {
    /// Historical period the work comes from or treats.
    Period {
        Antiquity => "antiquity",
        Medieval => "medieval",
        EarlyModern => "early_modern",
        Modern => "modern",
        Contemporary => "contemporary",
    }
}

/// First year of each period after antiquity (periods are broad and conventional).
const MEDIEVAL_FROM: i32 = 500;
const EARLY_MODERN_FROM: i32 = 1500;
const MODERN_FROM: i32 = 1800;
const CONTEMPORARY_FROM: i32 = 1945;

impl Period {
    /// The conventional period of a year (a suggestion for the editor).
    pub const fn of_year(year: i32) -> Self {
        if year < MEDIEVAL_FROM {
            Self::Antiquity
        } else if year < EARLY_MODERN_FROM {
            Self::Medieval
        } else if year < MODERN_FROM {
            Self::EarlyModern
        } else if year < CONTEMPORARY_FROM {
            Self::Modern
        } else {
            Self::Contemporary
        }
    }
}

/// The four facets of a document's classification; every one is optional.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Classification {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discipline: Option<Discipline>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub form: Option<DocumentForm>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<Period>,
    /// Free-text place of origin or subject place ("Amsterdam", "Vienna").
    #[serde(skip_serializing_if = "String::is_empty")]
    pub place: String,
}
