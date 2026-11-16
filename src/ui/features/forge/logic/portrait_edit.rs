//! Choosing portrait parts by position, for the per-slot thumbnail choosers.

use crate::domain::portrait::*;

/// Re-rolls tried before accepting that a slot landed on its current option again.
const REROLL_ATTEMPTS: usize = 8;

/// Index of `item` in `all` (0 when absent).
fn position<T: PartialEq>(all: &[T], item: &T) -> usize {
    all.iter().position(|candidate| candidate == item).unwrap_or_default()
}

/// `all[index]`, clamped to the last option.
fn pick<T: Copy>(all: &[T], index: usize) -> T {
    all[index.min(all.len().saturating_sub(1))]
}

/// Number of options `slot` offers.
pub fn slot_len(slot: PortraitSlot) -> usize {
    slot_option_keys(slot).len()
}

/// Position of the option `spec` uses on `slot` (in `slot_option_keys(slot)` order).
pub fn slot_index(spec: &PortraitSpec, slot: PortraitSlot) -> usize {
    match slot {
        PortraitSlot::Back => position(Backdrop::ALL, &spec.back),
        PortraitSlot::Face => position(FaceShape::ALL, &spec.face),
        PortraitSlot::Skin => position(SkinTone::ALL, &spec.skin),
        PortraitSlot::Eyes => position(EyeStyle::ALL, &spec.eyes),
        PortraitSlot::Brows => position(BrowStyle::ALL, &spec.brows),
        PortraitSlot::Mouth => position(MouthStyle::ALL, &spec.mouth),
        PortraitSlot::Hair => position(HairStyle::ALL, &spec.hair),
        PortraitSlot::HairColor => position(HairColor::ALL, &spec.hair_color),
        PortraitSlot::Outfit => position(OutfitStyle::ALL, &spec.outfit),
        PortraitSlot::Accessory => position(Accessory::ALL, &spec.accessory),
    }
}

/// `spec` with option `index` chosen on `slot` (clamped to the slot's last option).
pub fn with_slot_index(spec: PortraitSpec, slot: PortraitSlot, index: usize) -> PortraitSpec {
    let mut next = spec;
    match slot {
        PortraitSlot::Back => next.back = pick(Backdrop::ALL, index),
        PortraitSlot::Face => next.face = pick(FaceShape::ALL, index),
        PortraitSlot::Skin => next.skin = pick(SkinTone::ALL, index),
        PortraitSlot::Eyes => next.eyes = pick(EyeStyle::ALL, index),
        PortraitSlot::Brows => next.brows = pick(BrowStyle::ALL, index),
        PortraitSlot::Mouth => next.mouth = pick(MouthStyle::ALL, index),
        PortraitSlot::Hair => next.hair = pick(HairStyle::ALL, index),
        PortraitSlot::HairColor => next.hair_color = pick(HairColor::ALL, index),
        PortraitSlot::Outfit => next.outfit = pick(OutfitStyle::ALL, index),
        PortraitSlot::Accessory => next.accessory = pick(Accessory::ALL, index),
    }
    next
}

/// `spec` with `slot` re-rolled from `seed` to a different option when the slot has one;
/// every other part is kept.
pub fn randomized_slot(spec: PortraitSpec, slot: PortraitSlot, seed: u64) -> PortraitSpec {
    let mut rng = PortraitRng::new(seed);
    let mut next = spec;
    for _ in 0..REROLL_ATTEMPTS {
        next.randomize_part(slot, &mut rng);
        if slot_index(&next, slot) != slot_index(&spec, slot) {
            break;
        }
    }
    next
}
