//! The natural parameters of every form (what a tin can is made of, how long a stone lives)
//! and the English noun phrase the prompt uses for it.

use super::dims::{Material as Ma, Mobility as Mo, Perception as Pe, Scale as Sc, Temporality as Te};
use super::kinds::BodyForm;

/// `(material, scale, temporality, perception, mobility)` of a form left to its nature.
pub type Nature = (Ma, Sc, Te, Pe, Mo);

/// Every form with its nature, in `BodyForm::ALL` order.
#[rustfmt::skip]
pub(super) const NATURES: [(BodyForm, Nature); 57] = {
    use BodyForm::*;
    [
        (Person, (Ma::Flesh, Sc::HumanSized, Te::Days, Pe::HumanSenses, Mo::Walking)),
        (TinCan, (Ma::Metal, Sc::Pocket, Te::Lifetimes, Pe::Vibration, Mo::Carried)),
        (Book, (Ma::Paper, Sc::Pocket, Te::Centuries, Pe::Symbolic, Mo::Carried)),
        (Chair, (Ma::Fiber, Sc::HumanSized, Te::Lifetimes, Pe::Touch, Mo::Fixed)),
        (Lamp, (Ma::Metal, Sc::HumanSized, Te::Days, Pe::Light, Mo::Fixed)),
        (Clock, (Ma::Metal, Sc::Pocket, Te::Moments, Pe::Vibration, Mo::Fixed)),
        (Mirror, (Ma::Glass, Sc::HumanSized, Te::Lifetimes, Pe::Light, Mo::Fixed)),
        (Key, (Ma::Metal, Sc::Pocket, Te::Lifetimes, Pe::Touch, Mo::Carried)),
        (Teacup, (Ma::Stone, Sc::Pocket, Te::Days, Pe::Chemical, Mo::Carried)),
        (OakTree, (Ma::Fiber, Sc::Building, Te::Centuries, Pe::Light, Mo::Fixed)),
        (Cactus, (Ma::Fiber, Sc::HumanSized, Te::Lifetimes, Pe::Light, Mo::Fixed)),
        (Moss, (Ma::Fiber, Sc::Pocket, Te::Seasons, Pe::Chemical, Mo::Creeping)),
        (Mushroom, (Ma::Fiber, Sc::Pocket, Te::Seasons, Pe::Chemical, Mo::Fixed)),
        (Sunflower, (Ma::Fiber, Sc::HumanSized, Te::Seasons, Pe::Light, Mo::Fixed)),
        (Vine, (Ma::Fiber, Sc::Building, Te::Seasons, Pe::Touch, Mo::Creeping)),
        (Bamboo, (Ma::Fiber, Sc::Building, Te::Seasons, Pe::Light, Mo::Fixed)),
        (Seed, (Ma::Fiber, Sc::Microscopic, Te::Seasons, Pe::Chemical, Mo::Drifting)),
        (Water, (Ma::Liquid, Sc::Landscape, Te::Geologic, Pe::Touch, Mo::Flowing)),
        (Fire, (Ma::Energy, Sc::HumanSized, Te::Moments, Pe::Chemical, Mo::Flowing)),
        (Wind, (Ma::Air, Sc::Landscape, Te::Days, Pe::Touch, Mo::Flying)),
        (Stone, (Ma::Stone, Sc::Pocket, Te::Geologic, Pe::Vibration, Mo::Fixed)),
        (Light, (Ma::Energy, Sc::Cosmic, Te::Moments, Pe::Light, Mo::Flying)),
        (Ice, (Ma::Glass, Sc::Building, Te::Seasons, Pe::Touch, Mo::Creeping)),
        (Cloud, (Ma::Air, Sc::Landscape, Te::Days, Pe::Light, Mo::Drifting)),
        (Sand, (Ma::Stone, Sc::Microscopic, Te::Geologic, Pe::Touch, Mo::Drifting)),
        (Cat, (Ma::Flesh, Sc::Pocket, Te::Days, Pe::Touch, Mo::Walking)),
        (Crow, (Ma::Flesh, Sc::Pocket, Te::Days, Pe::Light, Mo::Flying)),
        (Octopus, (Ma::Flesh, Sc::HumanSized, Te::Days, Pe::Chemical, Mo::Flowing)),
        (Wolf, (Ma::Flesh, Sc::HumanSized, Te::Seasons, Pe::Chemical, Mo::Walking)),
        (Bee, (Ma::Flesh, Sc::Microscopic, Te::Moments, Pe::Light, Mo::Flying)),
        (Whale, (Ma::Flesh, Sc::Building, Te::Lifetimes, Pe::Vibration, Mo::Flowing)),
        (Snail, (Ma::Flesh, Sc::Pocket, Te::Seasons, Pe::Chemical, Mo::Creeping)),
        (Owl, (Ma::Flesh, Sc::Pocket, Te::Days, Pe::Vibration, Mo::Flying)),
        (Robot, (Ma::Metal, Sc::HumanSized, Te::Days, Pe::Signals, Mo::Walking)),
        (Satellite, (Ma::Metal, Sc::Building, Te::Days, Pe::Signals, Mo::Flying)),
        (Server, (Ma::Metal, Sc::Building, Te::Moments, Pe::Signals, Mo::Fixed)),
        (Typewriter, (Ma::Metal, Sc::HumanSized, Te::Lifetimes, Pe::Touch, Mo::Carried)),
        (Drone, (Ma::Metal, Sc::Pocket, Te::Moments, Pe::Signals, Mo::Flying)),
        (Radio, (Ma::Metal, Sc::Pocket, Te::Days, Pe::Signals, Mo::Carried)),
        (Calculator, (Ma::Metal, Sc::Pocket, Te::Moments, Pe::Symbolic, Mo::Carried)),
        (Train, (Ma::Metal, Sc::Building, Te::Days, Pe::Vibration, Mo::Flowing)),
        (City, (Ma::Stone, Sc::Landscape, Te::Centuries, Pe::Signals, Mo::Fixed)),
        (Island, (Ma::Stone, Sc::Landscape, Te::Geologic, Pe::Touch, Mo::Fixed)),
        (Library, (Ma::Paper, Sc::Building, Te::Centuries, Pe::Symbolic, Mo::Fixed)),
        (Cathedral, (Ma::Stone, Sc::Building, Te::Centuries, Pe::Vibration, Mo::Fixed)),
        (Desert, (Ma::Stone, Sc::Landscape, Te::Geologic, Pe::Light, Mo::Fixed)),
        (Forest, (Ma::Fiber, Sc::Landscape, Te::Centuries, Pe::Chemical, Mo::Fixed)),
        (Harbor, (Ma::Liquid, Sc::Landscape, Te::Centuries, Pe::Vibration, Mo::Fixed)),
        (Ruin, (Ma::Stone, Sc::Building, Te::Centuries, Pe::Touch, Mo::Fixed)),
        (Time, (Ma::Immaterial, Sc::Cosmic, Te::Timeless, Pe::Symbolic, Mo::Everywhere)),
        (Justice, (Ma::Immaterial, Sc::Planetary, Te::Centuries, Pe::Symbolic, Mo::Everywhere)),
        (Number, (Ma::Immaterial, Sc::Cosmic, Te::Timeless, Pe::Symbolic, Mo::Everywhere)),
        (Silence, (Ma::Immaterial, Sc::Landscape, Te::Moments, Pe::Vibration, Mo::Everywhere)),
        (Memory, (Ma::Immaterial, Sc::HumanSized, Te::Lifetimes, Pe::Symbolic, Mo::Drifting)),
        (Chance, (Ma::Immaterial, Sc::Cosmic, Te::Moments, Pe::Symbolic, Mo::Everywhere)),
        (Love, (Ma::Immaterial, Sc::HumanSized, Te::Lifetimes, Pe::HumanSenses, Mo::Everywhere)),
        (Entropy, (Ma::Energy, Sc::Cosmic, Te::Timeless, Pe::Vibration, Mo::Everywhere)),
    ]
};

impl BodyForm {
    /// The parameters this form has by nature.
    pub fn nature(self) -> Nature {
        NATURES.iter().find(|(form, _)| *form == self).map_or(NATURES[0].1, |(_, nature)| *nature)
    }
}
