//! Enneagram type-with-wing and instinctual stacking.

use crate::domain::trait_option::{trait_enum, trait_table};

trait_enum! {
    /// Enneagram of Personality type with wing.
    pub enum EnneagramType in "enneagram" {
        Type1w9("type1w9", "1w9 · The Idealist"),
        Type1w2("type1w2", "1w2 · The Advocate"),
        Type2w1("type2w1", "2w1 · The Altruist"),
        Type2w3("type2w3", "2w3 · The Host"),
        Type3w2("type3w2", "3w2 · The Enchanter"),
        Type3w4("type3w4", "3w4 · The Virtuoso"),
        Type4w3("type4w3", "4w3 · The Aristocrat"),
        Type4w5("type4w5", "4w5 · The Bohemian"),
        #[default]
        Type5w4("type5w4", "5w4 · The Iconoclast"),
        Type5w6("type5w6", "5w6 · The Problem Solver"),
        Type6w5("type6w5", "6w5 · The Defender"),
        Type6w7("type6w7", "6w7 · The Confidant"),
        Type7w6("type7w6", "7w6 · The Entertainer"),
        Type7w8("type7w8", "7w8 · The Opportunist"),
        Type8w7("type8w7", "8w7 · The Maverick"),
        Type8w9("type8w9", "8w9 · The Diplomat-Bear"),
        Type9w8("type9w8", "9w8 · The Referee"),
        Type9w1("type9w1", "9w1 · The Dreamer"),
    }
}

impl EnneagramType {
    /// Type-with-wing and archetype title (e.g. `"5w4 · The Iconoclast"`); identical to the label.
    #[inline]
    pub const fn title(self) -> &'static str {
        self.label()
    }
}

trait_table! {
    EnneagramType::code {
        Type1w9 => "1w9",
        Type1w2 => "1w2",
        Type2w1 => "2w1",
        Type2w3 => "2w3",
        Type3w2 => "3w2",
        Type3w4 => "3w4",
        Type4w3 => "4w3",
        Type4w5 => "4w5",
        Type5w4 => "5w4",
        Type5w6 => "5w6",
        Type6w5 => "6w5",
        Type6w7 => "6w7",
        Type7w6 => "7w6",
        Type7w8 => "7w8",
        Type8w7 => "8w7",
        Type8w9 => "8w9",
        Type9w8 => "9w8",
        Type9w1 => "9w1",
    }
}

trait_enum! {
    /// Enneagram Instinctual Stacking (Self-Preservation, Social, One-to-One/Sexual).
    pub enum InstinctualVariant in "instinctual_variant" {
        #[default]
        SpSo("sp_so", "sp/so (Self-Preservation / Social)"),
        SpSx("sp_sx", "sp/sx (Self-Preservation / Intimate)"),
        SoSp("so_sp", "so/sp (Social / Self-Preservation)"),
        SoSx("so_sx", "so/sx (Social / Intimate)"),
        SxSp("sx_sp", "sx/sp (Intimate / Self-Preservation)"),
        SxSo("sx_so", "sx/so (Intimate / Social)"),
    }
}

trait_table! {
    InstinctualVariant::code {
        SpSo => "sp/so",
        SpSx => "sp/sx",
        SoSp => "so/sp",
        SoSx => "so/sx",
        SxSp => "sx/sp",
        SxSo => "sx/so",
    }
}
