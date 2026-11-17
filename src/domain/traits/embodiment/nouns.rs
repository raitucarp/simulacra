//! English noun phrases of every form, as the prompt says them ("You are a tin can").

use super::kinds::BodyForm;
use crate::domain::trait_option::trait_table;

trait_table! {
    /// The form as an English noun phrase with its article.
    BodyForm::noun {
        Person => "a person",
        TinCan => "a tin can", Book => "a book", Chair => "a chair", Lamp => "a lamp",
        Clock => "a clock", Mirror => "a mirror", Key => "a key", Teacup => "a teacup",
        OakTree => "an oak tree", Cactus => "a cactus", Moss => "a patch of moss", Mushroom => "a mushroom",
        Sunflower => "a sunflower", Vine => "a climbing vine", Bamboo => "a stand of bamboo", Seed => "a seed",
        Water => "water", Fire => "fire", Wind => "the wind", Stone => "a stone",
        Light => "light", Ice => "ice", Cloud => "a cloud", Sand => "sand",
        Cat => "a cat", Crow => "a crow", Octopus => "an octopus", Wolf => "a wolf",
        Bee => "a bee", Whale => "a whale", Snail => "a snail", Owl => "an owl",
        Robot => "a robot", Satellite => "a satellite", Server => "a server humming in a data center",
        Typewriter => "a typewriter", Drone => "a drone", Radio => "a radio",
        Calculator => "a pocket calculator", Train => "a train",
        City => "a city", Island => "an island", Library => "a library", Cathedral => "a cathedral",
        Desert => "a desert", Forest => "a forest", Harbor => "a harbor", Ruin => "a ruin",
        Time => "time itself", Justice => "justice", Number => "a number", Silence => "silence",
        Memory => "memory", Chance => "chance", Love => "love", Entropy => "entropy",
    }
}
