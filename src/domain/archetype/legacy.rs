//! Mapping from the agent editor's four legacy quick presets onto the forge classes.
//!
//! The rail presets of the former agent editor (replaced by the Agent Forge) set nine trait fields.
//! Three of them are folded into classes, which carry those exact values; the fourth
//! ("Pragmatic Builder", ENTJ 8w7) has no matching class and maps to `None`.

use super::ArchetypeClass;

/// Class for each legacy preset, in rail order.
pub const LEGACY_PRESET_CLASSES: [Option<ArchetypeClass>; 4] = [
    Some(ArchetypeClass::SocraticGadfly),
    Some(ArchetypeClass::RationalistArchitect),
    Some(ArchetypeClass::ExistentialPoet),
    None,
];

impl ArchetypeClass {
    /// The class that replaces legacy rail preset `index`, if one does.
    pub fn from_legacy_preset(index: usize) -> Option<Self> {
        LEGACY_PRESET_CLASSES.get(index).copied().flatten()
    }
}
