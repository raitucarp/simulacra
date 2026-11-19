use super::{at, day, item_on_post, network};
use crate::domain::learning::palace::edit::{cleared, moved_room, new_palace, placed, renamed_locus, with_cue, with_image, with_room, without_room, Direction};
use crate::domain::learning::palace::rooms::ROOM_CANVAS;
use crate::domain::learning::palace::{plan_walk, route, Palace, PalaceId, PlacedTarget, RoomKind, Walk, WalkScope};
use crate::domain::review::{Grade, ItemId, ReviewItem, ReviewMode};
use crate::domain::review::schedule::Phase;

fn palace() -> Palace {
    let palace = new_palace(PalaceId("palace_1".into()), network(), " Home ", (RoomKind::Hall, "Entrance"), at(0));
    with_room(palace, RoomKind::Library, "Books")
}

fn item(id: &str) -> PlacedTarget {
    PlacedTarget::Item { id: ItemId::new(id) }
}

#[test]
fn every_room_kind_has_five_loci_inside_its_scene() {
    for kind in RoomKind::ALL {
        let slots = kind.slots();
        assert_eq!(slots.len(), 5, "{}", kind.key());
        assert!(slots.iter().all(|s| s.x > 0.0 && s.x < ROOM_CANVAS.0 && s.y > 0.0 && s.y < ROOM_CANVAS.1));
        assert!(slots.windows(2).all(|w| w[0].x < w[1].x), "{} loci walk left to right", kind.key());
        assert_eq!(kind.art_path(), format!("illustrations/palace/{}.svg", kind.key()));
        assert!(std::path::Path::new("assets").join(kind.art_path()).is_file(), "the scene of {} is generated", kind.key());
        let (x, y) = kind.position(0).unwrap();
        assert!((0.0..=1.0).contains(&x) && (0.0..=1.0).contains(&y));
        assert!(kind.position(5).is_none());
    }
}

#[test]
fn a_palace_is_built_with_pure_edits() {
    let palace = palace();
    assert_eq!(palace.name, "Home");
    assert_eq!(palace.rooms.len(), 2);
    assert_eq!(palace.loci(), 10);
    let (hall, library) = (palace.rooms[0].id.clone(), palace.rooms[1].id.clone());
    let palace = placed(palace, &hall, 2, item("i1"));
    let palace = with_image(with_cue(palace, &hall, 2, " a clock melting "), &hall, 2, Some("cue.png".into()));
    let placement = palace.rooms[0].loci[2].placement.clone().unwrap();
    assert_eq!((placement.cue.as_str(), placement.image.as_deref()), ("a clock melting", Some("cue.png")));
    assert_eq!(placed(palace.clone(), &hall, 2, item("i2")).rooms[0].loci[2].placement.as_ref().unwrap().cue, "", "a new placement starts without a cue");
    let palace = renamed_locus(palace, &hall, 0, "Front door");
    assert_eq!(palace.rooms[0].loci[0].name, "Front door");
    assert_eq!(palace.placed(), 1);
    assert_eq!(cleared(palace.clone(), &hall, 2).placed(), 0);
    let moved = moved_room(palace.clone(), &library, Direction::Earlier);
    assert_eq!(moved.rooms[0].id, library);
    assert_eq!(moved_room(moved.clone(), &library, Direction::Earlier), moved, "the first room stays first");
    assert_eq!(moved_room(palace.clone(), &library, Direction::Later), palace, "the last room stays last");
    assert_eq!(without_room(palace, &hall).rooms.len(), 1);
}

#[test]
fn the_route_visits_rooms_then_loci_in_order() {
    let palace = palace();
    let (hall, library) = (palace.rooms[0].id.clone(), palace.rooms[1].id.clone());
    let palace = placed(placed(placed(palace, &library, 0, item("c")), &hall, 3, item("b")), &hall, 1, item("a"));
    let order: Vec<(usize, usize)> = route(&palace).into_iter().map(|(r, l, _)| (r, l)).collect();
    assert_eq!(order, vec![(0, 1), (0, 3), (1, 0)]);
    let swapped = moved_room(palace, &library, Direction::Earlier);
    assert_eq!(route(&swapped)[0].2, item("c"), "moving a room changes the walk");
}

#[test]
fn a_due_walk_keeps_only_due_or_new_items() {
    let palace = palace();
    let hall = palace.rooms[0].id.clone();
    let palace = placed(placed(placed(palace, &hall, 0, item("due")), &hall, 1, item("later")), &hall, 2, PlacedTarget::Post { id: "p".into() });
    let lookup = |target: &PlacedTarget| -> Option<ReviewItem> {
        let PlacedTarget::Item { id } = target else { return None };
        let mut item = item_on_post(id.as_str(), "p", "Q?");
        item.schedule.phase = Phase::Review;
        item.schedule.due = if id.as_str() == "due" { day(1) } else { day(20) };
        Some(item)
    };
    let all = plan_walk(&palace, WalkScope::All, &lookup, day(2));
    assert_eq!(all.len(), 3);
    assert_eq!(all[2].item, None, "a post without an item is walked but not graded");
    let due = plan_walk(&palace, WalkScope::Due, &lookup, day(2));
    assert_eq!(due.iter().map(|s| s.item.clone()).collect::<Vec<_>>(), vec![Some(ItemId::new("due"))]);
}

#[test]
fn a_walk_reveals_grades_and_answers_in_palace_mode() {
    let palace = palace();
    let hall = palace.rooms[0].id.clone();
    let palace = placed(placed(palace, &hall, 0, item("i1")), &hall, 1, PlacedTarget::Post { id: "p".into() });
    let stops = plan_walk(&palace, WalkScope::All, &|t: &PlacedTarget| matches!(t, PlacedTarget::Item { .. }).then(|| item_on_post("i1", "p", "Q?")), day(0));
    let walk = Walk::new(palace.id.clone(), stops);
    let (unchanged, none) = walk.clone().grade(Grade::Good);
    assert_eq!((unchanged, none), (walk.clone(), None), "nothing is graded before the reveal");
    let (walk, answer) = walk.reveal().grade(Grade::Good);
    let answer = answer.expect("the item is reviewed");
    assert_eq!((answer.item_id, answer.mode), (ItemId::new("i1"), ReviewMode::Palace));
    assert_eq!(walk.position, 1);
    let (walk, answer) = walk.reveal().grade(Grade::Again);
    assert!(answer.is_none(), "the post has no item to review");
    assert!(walk.is_finished());
    assert_eq!(walk.recalled_share(), Some(0.5));
    assert!(!walk.clone().reveal().revealed, "a finished walk shows nothing");
}
