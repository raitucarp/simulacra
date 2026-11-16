//! Occupation → outfit suggestion, so a freshly forged agent dresses for their job.

use super::parts::OutfitStyle;

/// Lowercase occupation fragments (English and Indonesian) and the outfit they suggest.
/// The first fragment found in the occupation wins, so more specific entries come first.
const OUTFIT_HINTS: &[(&str, OutfitStyle)] = &[
    ("professor", OutfitStyle::ScholarGown),
    ("lectur", OutfitStyle::ScholarGown),
    ("academ", OutfitStyle::ScholarGown),
    ("scholar", OutfitStyle::ScholarGown),
    ("dosen", OutfitStyle::ScholarGown),
    ("scien", OutfitStyle::LabCoat),
    ("research", OutfitStyle::LabCoat),
    ("chemist", OutfitStyle::LabCoat),
    ("physicist", OutfitStyle::LabCoat),
    ("biolog", OutfitStyle::LabCoat),
    ("doctor", OutfitStyle::LabCoat),
    ("physician", OutfitStyle::LabCoat),
    ("ilmuwan", OutfitStyle::LabCoat),
    ("peneliti", OutfitStyle::LabCoat),
    ("dokter", OutfitStyle::LabCoat),
    ("philosoph", OutfitStyle::Toga),
    ("orator", OutfitStyle::Toga),
    ("filsuf", OutfitStyle::Toga),
    ("monk", OutfitStyle::Robe),
    ("priest", OutfitStyle::Robe),
    ("mystic", OutfitStyle::Robe),
    ("theolog", OutfitStyle::Robe),
    ("biksu", OutfitStyle::Robe),
    ("lawyer", OutfitStyle::Suit),
    ("executive", OutfitStyle::Suit),
    ("manager", OutfitStyle::Suit),
    ("banker", OutfitStyle::Suit),
    ("consult", OutfitStyle::Suit),
    ("diplomat", OutfitStyle::Suit),
    ("pengacara", OutfitStyle::Suit),
    ("writer", OutfitStyle::Cardigan),
    ("poet", OutfitStyle::Cardigan),
    ("novelist", OutfitStyle::Cardigan),
    ("librar", OutfitStyle::Cardigan),
    ("teacher", OutfitStyle::Cardigan),
    ("therap", OutfitStyle::Cardigan),
    ("penulis", OutfitStyle::Cardigan),
    ("penyair", OutfitStyle::Cardigan),
    ("guru", OutfitStyle::Cardigan),
    ("farmer", OutfitStyle::Tunic),
    ("artisan", OutfitStyle::Tunic),
    ("craft", OutfitStyle::Tunic),
    ("bard", OutfitStyle::Tunic),
    ("petani", OutfitStyle::Tunic),
    ("engineer", OutfitStyle::Hoodie),
    ("develop", OutfitStyle::Hoodie),
    ("program", OutfitStyle::Hoodie),
    ("student", OutfitStyle::Hoodie),
    ("hacker", OutfitStyle::Hoodie),
    ("mahasiswa", OutfitStyle::Hoodie),
];

/// The outfit that best fits a free-text `occupation`, or `None` when nothing matches.
pub fn suggested_outfit(occupation: &str) -> Option<OutfitStyle> {
    let lowered = occupation.to_lowercase();
    OUTFIT_HINTS.iter().find(|(fragment, _)| lowered.contains(fragment)).map(|(_, outfit)| *outfit)
}
