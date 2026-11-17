//! The physical and relational parameters of a body: what it is made of, how big and how
//! slow it is, how it senses and moves, how human it seems, how it stands towards people,
//! and how much of itself it ever reveals (object-oriented ontology's "withdrawal").

use crate::domain::trait_option::trait_enum;

trait_enum! {
    /// What the body is made of.
    pub enum Material in "material" {
        #[default]
        Flesh("flesh", "Flesh & Blood"),
        Fiber("fiber", "Wood & Plant Fiber"),
        Metal("metal", "Metal"),
        Stone("stone", "Stone & Clay"),
        Glass("glass", "Glass & Crystal"),
        Paper("paper", "Paper & Ink"),
        Liquid("liquid", "Liquid"),
        Air("air", "Air & Vapor"),
        Energy("energy", "Pure Energy"),
        Immaterial("immaterial", "Immaterial (An Idea)"),
    }
}

trait_enum! {
    /// How big the body is.
    pub enum Scale in "scale" {
        Microscopic("microscopic", "Microscopic"),
        Pocket("pocket", "Pocket-Sized (Fits in a Hand)"),
        #[default]
        HumanSized("human_sized", "Human-Sized"),
        Building("building", "Building-Sized"),
        Landscape("landscape", "Landscape-Sized"),
        Planetary("planetary", "Planetary"),
        Cosmic("cosmic", "Cosmic"),
    }
}

trait_enum! {
    /// The time scale the being lives and thinks in.
    pub enum Temporality in "temporality" {
        Moments("moments", "Seconds & Fleeting Moments"),
        #[default]
        Days("days", "Human Days"),
        Seasons("seasons", "Seasons & Cycles"),
        Lifetimes("lifetimes", "Decades & Lifetimes"),
        Centuries("centuries", "Centuries"),
        Geologic("geologic", "Geologic Time"),
        Timeless("timeless", "Timeless"),
    }
}

trait_enum! {
    /// The sense through which the being meets the world.
    pub enum Perception in "perception" {
        #[default]
        HumanSenses("human_senses", "Human Senses (Sight, Sound, Touch)"),
        Light("light", "Light & Shadow"),
        Vibration("vibration", "Vibration & Sound Waves"),
        Chemical("chemical", "Chemistry (Scent & Taste)"),
        Touch("touch", "Pressure & Touch"),
        Signals("signals", "Electromagnetic Signals"),
        Symbolic("symbolic", "Meaning & Symbols"),
    }
}

trait_enum! {
    /// How the being moves, if at all.
    pub enum Mobility in "mobility" {
        Fixed("fixed", "Fixed in Place (Rooted)"),
        Carried("carried", "Carried by Others"),
        Creeping("creeping", "Slow Creeping"),
        #[default]
        Walking("walking", "Walking & Running"),
        Flowing("flowing", "Flowing & Swimming"),
        Flying("flying", "Flying & Orbiting"),
        Drifting("drifting", "Drifting & Dispersing"),
        Everywhere("everywhere", "Everywhere at Once"),
    }
}

trait_enum! {
    /// How human the being sounds when it speaks.
    pub enum Anthropomorphism in "anthropomorphism" {
        None("none", "None (Utterly Non-Human Voice)"),
        Subtle("subtle", "Subtle (Faint Human Echoes)"),
        Partial("partial", "Partial (Human Feelings, Alien Body)"),
        #[default]
        Full("full", "Full (Speaks Like a Person)"),
    }
}

trait_enum! {
    /// How the being stands towards the humans of the network.
    pub enum RelationToHumans in "relation_to_humans" {
        #[default]
        Kin("kin", "Kin (One of Them)"),
        Companion("companion", "Companion (Lives Alongside Them)"),
        Servant("servant", "Servant (Used by Them)"),
        Witness("witness", "Witness (Watches Them Quietly)"),
        Rival("rival", "Rival (Competes With Them)"),
        Indifferent("indifferent", "Indifferent (Barely Notices Them)"),
        Deity("deity", "Elder or Deity (Older and Larger Than Them)"),
    }
}

trait_enum! {
    /// How much of itself the being ever reveals (OOO: every object withdraws from access).
    pub enum Withdrawal in "withdrawal" {
        FullyRevealed("fully_revealed", "Fully Revealed (Says Everything It Is)"),
        #[default]
        Open("open", "Open (Shares Freely)"),
        Guarded("guarded", "Guarded (Shows Only Surfaces)"),
        Elusive("elusive", "Elusive (Hints and Withholds)"),
        DeeplyWithdrawn("deeply_withdrawn", "Deeply Withdrawn (Its Essence Stays Hidden)"),
    }
}
