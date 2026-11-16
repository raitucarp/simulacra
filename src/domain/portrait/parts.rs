//! Typed variants of every portrait layer: one variant per
//! `assets/illustrations/portrait/<layer>/<variant>.svg` template.

use super::colors::{HairColor, SkinTone};
use super::layer::{PortraitLayer, PortraitSlot};
use serde::{Deserialize, Serialize};

/// Generates a part enum (serialized as its snake_case asset key) with `ALL`, `LAYER` and `key()`.
macro_rules! portrait_part {
    (
        $(#[$meta:meta])* $name:ident in $layer:ident {
            $first:ident => $first_key:literal $(, $variant:ident => $key:literal)* $(,)?
        }
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
        #[serde(rename_all = "snake_case")]
        pub enum $name {
            #[default]
            $first,
            $($variant,)*
        }

        impl $name {
            /// Every variant; the first one is the default.
            pub const ALL: &'static [Self] = &[Self::$first, $(Self::$variant,)*];
            /// Layer this part is drawn on.
            pub const LAYER: PortraitLayer = PortraitLayer::$layer;

            /// Asset file stem (also the serde name and the i18n suffix).
            pub const fn key(self) -> &'static str {
                match self {
                    Self::$first => $first_key,
                    $(Self::$variant => $key,)*
                }
            }
        }
    };
}

portrait_part! {
    /// Full-canvas scene behind the bust.
    Backdrop in Back {
        Plain => "plain", Geometric => "geometric", Library => "library", Meadow => "meadow",
        Starfield => "starfield", Sunburst => "sunburst",
    }
}

portrait_part! {
    /// Head silhouette (with ears and neck), filled with the skin tone.
    FaceShape in Face {
        Oval => "oval", Round => "round", Square => "square", Heart => "heart", Long => "long",
        Diamond => "diamond",
    }
}

portrait_part! {
    /// Eye shape and expression.
    EyeStyle in Eyes {
        Round => "round", Almond => "almond", Dot => "dot", Happy => "happy", Narrow => "narrow",
        Sleepy => "sleepy", Wide => "wide", Wink => "wink",
    }
}

portrait_part! {
    /// Eyebrows, filled with the hair color.
    BrowStyle in Brows {
        Arched => "arched", Bushy => "bushy", Furrowed => "furrowed", Raised => "raised",
        Straight => "straight", Thin => "thin",
    }
}

portrait_part! {
    /// Mouth expression.
    MouthStyle in Mouth {
        Smile => "smile", Cat => "cat", Frown => "frown", Grin => "grin", Laugh => "laugh",
        Neutral => "neutral", Open => "open", Smirk => "smirk",
    }
}

portrait_part! {
    /// Hairstyle, filled with the hair color.
    HairStyle in Hair {
        Crop => "crop", Bald => "bald", Bob => "bob", Braids => "braids", Bun => "bun", Buzz => "buzz",
        Curly => "curly", Long => "long", Ponytail => "ponytail", SidePart => "side_part",
        Spiky => "spiky", Wavy => "wavy",
    }
}

portrait_part! {
    /// Garment on the shoulders.
    OutfitStyle in Outfit {
        Hoodie => "hoodie", Cardigan => "cardigan", LabCoat => "lab_coat", Robe => "robe",
        ScholarGown => "scholar_gown", Suit => "suit", Toga => "toga", Tunic => "tunic",
    }
}

portrait_part! {
    /// Optional finishing touch drawn above the hair.
    Accessory in Accessory {
        None => "none", Glasses => "glasses", Headphones => "headphones", Laurel => "laurel",
        Monocle => "monocle", Scarf => "scarf",
    }
}

/// Every variant key of `layer`, in `ALL` order (used by asset guards and pickers).
pub fn layer_variant_keys(layer: PortraitLayer) -> Vec<&'static str> {
    fn keys<T: Copy>(all: &[T], key: fn(T) -> &'static str) -> Vec<&'static str> {
        all.iter().map(|v| key(*v)).collect()
    }
    match layer {
        PortraitLayer::Back => keys(Backdrop::ALL, Backdrop::key),
        PortraitLayer::Face => keys(FaceShape::ALL, FaceShape::key),
        PortraitLayer::Outfit => keys(OutfitStyle::ALL, OutfitStyle::key),
        PortraitLayer::Mouth => keys(MouthStyle::ALL, MouthStyle::key),
        PortraitLayer::Eyes => keys(EyeStyle::ALL, EyeStyle::key),
        PortraitLayer::Brows => keys(BrowStyle::ALL, BrowStyle::key),
        PortraitLayer::Hair => keys(HairStyle::ALL, HairStyle::key),
        PortraitLayer::Accessory => keys(Accessory::ALL, Accessory::key),
    }
}

/// Every option key of `slot` (variant keys for layers, color keys for skin and hair color);
/// labels are `slot.option_label_key(key)`.
pub fn slot_option_keys(slot: PortraitSlot) -> Vec<&'static str> {
    match (slot, slot.layer()) {
        (_, Some(layer)) => layer_variant_keys(layer),
        (PortraitSlot::Skin, None) => SkinTone::ALL.iter().map(|tone| tone.key()).collect(),
        (_, None) => HairColor::ALL.iter().map(|color| color.key()).collect(),
    }
}
