//! Building a palace as pure edits: a new palace, rooms added, removed and moved along the
//! route, things placed at a locus or taken away, cues and names written.

use super::{Locus, Palace, PalaceId, PlacedTarget, Placement, Room, RoomKind, ROOM_ID_PREFIX};
use crate::domain::learning::fresh_id;
use crate::domain::NetworkId;
use jiff::Timestamp;

/// Which way a room moves along the route.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Earlier,
    Later,
}

/// A new palace with one room of `kind`.
pub fn new_palace(id: PalaceId, network_id: NetworkId, name: &str, first: (RoomKind, &str), now: Timestamp) -> Palace {
    let palace = Palace { id, network_id, name: name.trim().to_string(), rooms: Vec::new(), created_at: now };
    with_room(palace, first.0, first.1)
}

/// A room of `kind` with its empty loci.
pub fn new_room(kind: RoomKind, name: &str) -> Room {
    let loci = (0..kind.slots().len()).map(|slot| Locus { slot, name: String::new(), placement: None }).collect();
    Room { id: fresh_id(ROOM_ID_PREFIX), name: name.trim().to_string(), kind, loci }
}

/// `palace` with a room of `kind` at the end of the route.
pub fn with_room(palace: Palace, kind: RoomKind, name: &str) -> Palace {
    let mut rooms = palace.rooms.clone();
    rooms.push(new_room(kind, name));
    Palace { rooms, ..palace }
}

/// `palace` without room `id`.
pub fn without_room(palace: Palace, id: &str) -> Palace {
    Palace { rooms: palace.rooms.iter().filter(|room| room.id != id).cloned().collect(), ..palace }
}

/// `palace` with room `id` one step earlier or later on the route (unchanged at the ends).
pub fn moved_room(palace: Palace, id: &str, direction: Direction) -> Palace {
    let Some(at) = palace.rooms.iter().position(|room| room.id == id) else { return palace };
    let to = match direction {
        Direction::Earlier => at.checked_sub(1),
        Direction::Later => Some(at + 1).filter(|to| *to < palace.rooms.len()),
    };
    let mut rooms = palace.rooms.clone();
    if let Some(to) = to {
        rooms.swap(at, to);
    }
    Palace { rooms, ..palace }
}

/// `palace` with locus `slot` of room `room` changed by `change`.
fn with_locus(palace: Palace, room: &str, slot: usize, change: impl Fn(Locus) -> Locus) -> Palace {
    let rooms = palace
        .rooms
        .iter()
        .map(|r| match r.id == room {
            true => Room { loci: r.loci.iter().map(|l| if l.slot == slot { change(l.clone()) } else { l.clone() }).collect(), ..r.clone() },
            false => r.clone(),
        })
        .collect();
    Palace { rooms, ..palace }
}

/// `target` placed at a locus (replacing what was there, keeping no cue).
pub fn placed(palace: Palace, room: &str, slot: usize, target: PlacedTarget) -> Palace {
    with_locus(palace, room, slot, |locus| Locus { placement: Some(Placement { target: target.clone(), cue: String::new(), image: None }), ..locus })
}

/// The locus emptied.
pub fn cleared(palace: Palace, room: &str, slot: usize) -> Palace {
    with_locus(palace, room, slot, |locus| Locus { placement: None, ..locus })
}

/// The placement's cue rewritten.
pub fn with_cue(palace: Palace, room: &str, slot: usize, cue: &str) -> Palace {
    with_locus(palace, room, slot, |locus| Locus { placement: locus.placement.map(|p| Placement { cue: cue.trim().to_string(), ..p }), ..locus })
}

/// The placement's picture set (or removed).
pub fn with_image(palace: Palace, room: &str, slot: usize, image: Option<String>) -> Palace {
    with_locus(palace, room, slot, |locus| Locus { placement: locus.placement.map(|p| Placement { image: image.clone(), ..p }), ..locus })
}

/// The locus renamed (blank: back to the landmark's name).
pub fn renamed_locus(palace: Palace, room: &str, slot: usize, name: &str) -> Palace {
    with_locus(palace, room, slot, |locus| Locus { name: name.trim().to_string(), ..locus })
}
