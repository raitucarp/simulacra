//! The room types of a memory palace and their loci: each room scene
//! (`assets/illustrations/palace/<kind>.svg`, drawn by `_tools/palace_art.py`) has five
//! places along a walk from left to right, at the positions below on its 160 × 120 canvas.
//! The art draws a landmark at each position; keep both in step.

use crate::domain::document::keyed::keyed_enum;

/// The room scenes' canvas (SVG units).
pub const ROOM_CANVAS: (f32, f32) = (160.0, 120.0);
/// Folder of the room scenes under the asset root.
pub const ROOM_ART_DIR: &str = "illustrations/palace";
const ART_EXT: &str = "svg";

keyed_enum! {
    /// A kind of room, with its scene and its loci.
    RoomKind {
        Hall => "hall",
        Library => "library",
        Garden => "garden",
        Kitchen => "kitchen",
        Study => "study",
        Stairwell => "stairwell",
    }
}

/// One place in a room: its landmark (a locale key) and its position on the canvas.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LocusSlot {
    pub place: &'static str,
    pub x: f32,
    pub y: f32,
}

const fn slot(place: &'static str, x: f32, y: f32) -> LocusSlot {
    LocusSlot { place, x, y }
}

const HALL: [LocusSlot; 5] = [slot("door", 24.0, 62.0), slot("mirror", 54.0, 44.0), slot("chandelier", 80.0, 20.0), slot("bench", 108.0, 85.0), slot("clock", 136.0, 54.0)];
const LIBRARY: [LocusSlot; 5] = [slot("ladder", 24.0, 64.0), slot("shelf", 55.0, 40.0), slot("globe", 84.0, 80.0), slot("lamp", 112.0, 28.0), slot("armchair", 139.0, 77.0)];
const GARDEN: [LocusSlot; 5] = [slot("gate", 20.0, 72.0), slot("tree", 50.0, 34.0), slot("fountain", 82.0, 82.0), slot("bench", 114.0, 82.0), slot("birdhouse", 138.0, 42.0)];
const KITCHEN: [LocusSlot; 5] = [slot("stove", 25.0, 72.0), slot("shelf", 54.0, 32.0), slot("table", 82.0, 85.0), slot("sink", 114.0, 76.0), slot("window", 136.0, 36.0)];
const STUDY: [LocusSlot; 5] = [slot("bookcase", 23.0, 56.0), slot("painting", 54.0, 34.0), slot("desk", 84.0, 78.0), slot("plant", 114.0, 84.0), slot("window", 136.0, 40.0)];
const STAIRWELL: [LocusSlot; 5] = [slot("step", 24.0, 98.0), slot("banister", 54.0, 66.0), slot("portrait", 86.0, 31.0), slot("landing", 118.0, 43.0), slot("window", 140.0, 20.0)];

impl Default for RoomKind {
    fn default() -> Self {
        Self::Hall
    }
}

impl RoomKind {
    /// The room's loci, in walking order.
    pub fn slots(self) -> &'static [LocusSlot] {
        match self {
            Self::Hall => &HALL,
            Self::Library => &LIBRARY,
            Self::Garden => &GARDEN,
            Self::Kitchen => &KITCHEN,
            Self::Study => &STUDY,
            Self::Stairwell => &STAIRWELL,
        }
    }

    /// The scene's asset path.
    pub fn art_path(self) -> String {
        format!("{ROOM_ART_DIR}/{}.{ART_EXT}", self.key())
    }

    /// Position of locus `slot` as shares (0–1) of the scene's width and height.
    pub fn position(self, slot: usize) -> Option<(f32, f32)> {
        self.slots().get(slot).map(|s| (s.x / ROOM_CANVAS.0, s.y / ROOM_CANVAS.1))
    }
}
