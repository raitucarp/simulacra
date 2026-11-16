//! [`PortraitSpec`]: the full, serializable description of a procedurally generated portrait.

use super::colors::{HairColor, SkinTone};
use super::layer::{PortraitLayer, PortraitSlot};
use super::parts::*;
use super::rng::PortraitRng;
use serde::{Deserialize, Serialize};

/// Odds (numerator / denominator) that a random portrait wears no accessory.
const BARE_ODDS: (u64, u64) = (1, 2);

/// Every choice that makes up a generated portrait. Missing JSON fields fall back to the
/// defaults, so older or partial specs always load.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(default)]
pub struct PortraitSpec {
    /// Seed the spec was rolled from (kept so "re-roll" can continue deterministically).
    pub seed: u64,
    pub back: Backdrop,
    pub face: FaceShape,
    pub skin: SkinTone,
    pub eyes: EyeStyle,
    pub brows: BrowStyle,
    pub mouth: MouthStyle,
    pub hair: HairStyle,
    pub hair_color: HairColor,
    pub outfit: OutfitStyle,
    pub accessory: Accessory,
}

impl PortraitSpec {
    /// A fully random portrait; the same seed always yields the same spec.
    pub fn random(seed: u64) -> Self {
        let mut rng = PortraitRng::new(seed);
        let mut spec = Self { seed, ..Self::default() };
        for slot in PortraitSlot::ALL {
            spec.randomize_part(slot, &mut rng);
        }
        spec
    }

    /// Re-rolls one slot, leaving every other choice untouched.
    pub fn randomize_part(&mut self, slot: PortraitSlot, rng: &mut PortraitRng) {
        match slot {
            PortraitSlot::Back => self.back = rng.pick(Backdrop::ALL),
            PortraitSlot::Face => self.face = rng.pick(FaceShape::ALL),
            PortraitSlot::Skin => self.skin = rng.pick(SkinTone::ALL),
            PortraitSlot::Eyes => self.eyes = rng.pick(EyeStyle::ALL),
            PortraitSlot::Brows => self.brows = rng.pick(BrowStyle::ALL),
            PortraitSlot::Mouth => self.mouth = rng.pick(MouthStyle::ALL),
            PortraitSlot::Hair => self.hair = rng.pick(HairStyle::ALL),
            PortraitSlot::HairColor => self.hair_color = rng.pick(HairColor::ALL),
            PortraitSlot::Outfit => self.outfit = rng.pick(OutfitStyle::ALL),
            PortraitSlot::Accessory => self.accessory = random_accessory(rng),
        }
    }

    /// Asset key of the variant chosen on `layer`.
    pub const fn variant_key(&self, layer: PortraitLayer) -> &'static str {
        match layer {
            PortraitLayer::Back => self.back.key(),
            PortraitLayer::Face => self.face.key(),
            PortraitLayer::Outfit => self.outfit.key(),
            PortraitLayer::Mouth => self.mouth.key(),
            PortraitLayer::Eyes => self.eyes.key(),
            PortraitLayer::Brows => self.brows.key(),
            PortraitLayer::Hair => self.hair.key(),
            PortraitLayer::Accessory => self.accessory.key(),
        }
    }

    /// Asset paths of every layer in paint order (bottom to top).
    pub fn layer_paths(&self) -> Vec<String> {
        PortraitLayer::STACK.iter().map(|layer| layer.asset_path(self.variant_key(*layer))).collect()
    }
}

/// No accessory half the time, otherwise one of the real accessories.
fn random_accessory(rng: &mut PortraitRng) -> Accessory {
    let (numerator, denominator) = BARE_ODDS;
    if rng.chance(numerator, denominator) {
        return Accessory::None;
    }
    rng.pick(&Accessory::ALL[1..])
}
