//! Memory palaces (the method of loci): a palace is a route through rooms; each room has
//! ordered places (loci); a review item or a post is placed at a locus with an optional vivid
//! image cue. Walking the palace visits the loci in order: recall what is placed, reveal,
//! grade. A walk counts as a review of the placed items (mode `palace` in the log).
//!
//! - [`rooms`]: room kinds, their scenes and loci positions (data).
//! - [`edit`]: building a palace as pure edits.
//! - [`walk`]: the route, walk planning (all loci or only what is due) and the walk itself.

pub mod edit;
pub mod rooms;
pub mod walk;

pub use rooms::{LocusSlot, RoomKind};
pub use walk::{plan_walk, route, Walk, WalkScope, WalkStop};

use crate::domain::review::ItemId;
use crate::domain::NetworkId;
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

/// Prefix of palace ids.
pub const PALACE_ID_PREFIX: &str = "palace";
/// Prefix of room ids.
pub const ROOM_ID_PREFIX: &str = "room";

/// A palace's identifier (the `learning_palaces.id` column).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PalaceId(pub String);

impl PalaceId {
    pub fn generate() -> Self {
        Self(super::fresh_id(PALACE_ID_PREFIX))
    }
}

/// What a locus holds.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum PlacedTarget {
    Item { id: ItemId },
    Post { id: String },
}

/// A placement with its memory cue.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Placement {
    pub target: PlacedTarget,
    /// A vivid image to picture at the place ("a giant clock melting over the door").
    #[serde(default)]
    pub cue: String,
    /// A picture of the cue (a stored image path).
    #[serde(default)]
    pub image: Option<String>,
}

/// One place of a room.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Locus {
    /// Index into the room kind's slots.
    pub slot: usize,
    /// The learner's own name for it (blank: the landmark's name).
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub placement: Option<Placement>,
}

/// A room of the palace.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Room {
    pub id: String,
    pub name: String,
    pub kind: RoomKind,
    pub loci: Vec<Locus>,
}

/// A memory palace.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Palace {
    pub id: PalaceId,
    pub network_id: NetworkId,
    pub name: String,
    pub rooms: Vec<Room>,
    pub created_at: Timestamp,
}

impl Palace {
    /// Loci with something placed.
    pub fn placed(&self) -> usize {
        self.rooms.iter().flat_map(|room| &room.loci).filter(|locus| locus.placement.is_some()).count()
    }

    /// Every locus.
    pub fn loci(&self) -> usize {
        self.rooms.iter().map(|room| room.loci.len()).sum()
    }

    pub fn room(&self, id: &str) -> Option<&Room> {
        self.rooms.iter().find(|room| room.id == id)
    }
}
