use super::category::diagram::{object_at, places};
use super::radial::geometry::{dial, radar_axis_at, radar_point, rhythm_cell_at, sector};
use super::sets::venn::{circles, region_at};
use super::symmetry::tree::{place, tree_layout};
use super::Room;
use crate::ui::features::mind_graph::viewport::Frame;
use gpui_kit::px;

const SQUARE: Frame = Frame { x: 0.0, y: 0.0, width: 200.0, height: 200.0 };

#[test]
fn room_follows_the_measured_width() {
    let tile = px(150.0);
    assert_eq!(Room::of(px(400.0), tile), Room::Narrow);
    assert_eq!(Room::of(px(900.0), tile), Room::Medium);
    assert_eq!(Room::of(px(1300.0), tile), Room::Wide);
}

#[test]
fn rhythm_dial_hits_rings_and_hours() {
    let d = dial(&SQUARE);
    assert!(d.inner < d.outer && d.outer <= 100.0);
    let midnight_inner = (100.0, 100.0 - d.inner - d.ring / 2.0);
    assert_eq!(rhythm_cell_at(&SQUARE, midnight_inner), Some((0, 0)), "Monday, midnight, straight up");
    let six_outer = (100.0 + d.outer - d.ring / 2.0, 101.0);
    assert_eq!(rhythm_cell_at(&SQUARE, six_outer), Some((6, 6)), "Sunday at six is the outer ring, right");
    assert_eq!(rhythm_cell_at(&SQUARE, (100.0, 100.0)), None, "the hole is empty");
    assert_eq!(rhythm_cell_at(&SQUARE, (0.0, 0.0)), None);
    assert_eq!(sector(&d, 2, 3).len(), 10);
}

#[test]
fn radar_axes_and_points() {
    let top = radar_point(&SQUARE, 0, 100.0);
    assert!((top.0 - 100.0).abs() < 1e-3 && top.1 < 30.0);
    assert_eq!(radar_point(&SQUARE, 3, 0.0), (100.0, 100.0));
    assert_eq!(radar_axis_at(&SQUARE, (100.0, 40.0)), Some(0));
    assert_eq!(radar_axis_at(&SQUARE, (100.0, 160.0)), Some(3));
    assert_eq!(radar_axis_at(&SQUARE, (0.0, 0.0)), None);
}

#[test]
fn venn_regions_by_membership() {
    let frame = Frame { x: 0.0, y: 0.0, width: 400.0, height: 300.0 };
    let two = circles(&frame, 2);
    assert_eq!(two.len(), 2);
    assert!(two[0].0 .0 > two[1].0 .0, "the first circle sits right of the second");
    assert_eq!(region_at(&frame, 2, (200.0, 150.0)), Some(0b11), "the middle is in both");
    assert_eq!(region_at(&frame, 2, two[0].0), Some(0b01), "a centre lies only in its own circle");
    assert_eq!(region_at(&frame, 3, (200.0, 150.0)), Some(0b111));
    assert_eq!(region_at(&frame, 3, (1.0, 1.0)), None);
    let wide_radius = circles(&Frame { width: 120.0, ..frame }, 2)[0].1;
    assert!(wide_radius * 3.6 <= 120.0 + 1e-3, "circles shrink to fit a narrow frame");
}

#[test]
fn tree_layout_centres_parents_over_children() {
    let parents = [None, Some(0), Some(0), Some(1), Some(1)];
    let layout = tree_layout(&parents);
    assert_eq!(layout[0].1, 0.0);
    assert_eq!(layout[3].1, 1.0);
    assert!((layout[1].0 - (layout[3].0 + layout[4].0) / 2.0).abs() < 1e-6);
    assert!((layout[0].0 - (layout[1].0 + layout[2].0) / 2.0).abs() < 1e-6);
    let placed = place(&layout, &SQUARE, 10.0);
    assert!(placed.iter().all(|p| p.0 >= 10.0 && p.0 <= 190.0 && p.1 >= 10.0 && p.1 <= 190.0));
}

#[test]
fn diagram_objects_are_hit_near_their_place() {
    let frame = Frame { x: 0.0, y: 0.0, width: 600.0, height: 400.0 };
    let at = places(&frame);
    assert_eq!(object_at(&frame, at[2], 5.0), Some(2));
    assert_eq!(object_at(&frame, (1.0, 1.0), 5.0), None);
}
