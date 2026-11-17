use super::geometry::{control_point, faded, hit_test, node_radius};
use super::labels::{label_origin, room_for_labels};
use super::viewport::{wheel_factor, Fit, Frame, Viewport, ZOOM_MAX, ZOOM_MIN};
use crate::domain::graph::layout::LayoutPoint;
use crate::state::graph::GraphScene;
use crate::state::AppState;
use gpui_kit::rgba;

const FIT: Fit = Fit { padding: 20.0, extent: (1.0, 1.0) };
const TOLERANCE: f32 = 1e-3;

fn frame() -> Frame {
    Frame { x: 100.0, y: 50.0, width: 400.0, height: 300.0 }
}

fn close(a: (f32, f32), b: (f32, f32)) -> bool {
    (a.0 - b.0).abs() < TOLERANCE && (a.1 - b.1).abs() < TOLERANCE
}

#[test]
fn fitted_layout_is_centered_inside_the_padded_frame() {
    let view = Viewport::default();
    assert!(close(view.to_screen(&frame(), FIT, LayoutPoint { x: 0.5, y: 0.5 }), (300.0, 200.0)));
    let corner = view.to_screen(&frame(), FIT, LayoutPoint { x: 0.0, y: 0.0 });
    assert!(close(corner, (300.0 - 130.0, 200.0 - 130.0)), "{corner:?}");
}

#[test]
fn a_wide_node_box_fills_a_wide_canvas_and_a_lone_node_stays_small() {
    let view = Viewport::default();
    let wide = Frame { x: 0.0, y: 0.0, width: 900.0, height: 500.0 };
    let flat = Fit { padding: 20.0, extent: (1.0, 0.5) };
    let left = view.to_screen(&wide, flat, LayoutPoint { x: 0.0, y: 0.5 });
    let right = view.to_screen(&wide, flat, LayoutPoint { x: 1.0, y: 0.5 });
    assert!(close((left.0, right.0), (20.0, 880.0)), "{left:?} {right:?}");
    let lone = Fit { padding: 20.0, extent: (0.0, 0.0) };
    let edge = view.to_screen(&wide, lone, LayoutPoint { x: 0.6, y: 0.5 });
    assert!(edge.0.is_finite() && edge.0 < 900.0, "{edge:?}");
}

#[test]
fn screen_and_layout_coordinates_round_trip() {
    let view = Viewport { pan: (37.0, -12.0), zoom: 2.5 };
    let p = LayoutPoint { x: 0.2, y: 0.9 };
    let back = view.to_layout(&frame(), FIT, view.to_screen(&frame(), FIT, p));
    assert!(close((back.x, back.y), (p.x, p.y)));
}

#[test]
fn zooming_keeps_the_point_under_the_cursor_fixed_and_clamps() {
    let mut view = Viewport::default();
    let cursor = (180.0, 90.0);
    let anchor = view.to_layout(&frame(), FIT, cursor);
    view.zoom_at(&frame(), FIT, cursor, 3.0);
    assert!(close(view.to_screen(&frame(), FIT, anchor), cursor));
    view.zoom_at(&frame(), FIT, cursor, 1e6);
    assert_eq!(view.zoom, ZOOM_MAX);
    view.zoom_at(&frame(), FIT, cursor, 1e-9);
    assert_eq!(view.zoom, ZOOM_MIN);
    assert!(wheel_factor(120.0) > 1.0 && wheel_factor(-120.0) < 1.0);
}

#[test]
fn node_radius_and_colors_stay_in_range() {
    assert_eq!(node_radius(0, 10, (3.0, 9.0)), 3.0);
    assert_eq!(node_radius(10, 10, (3.0, 9.0)), 9.0);
    assert!(node_radius(5, 0, (3.0, 9.0)).is_finite());
    assert_eq!(faded(rgba(0xff0000ff), 0.5).a, 0.5);
}

#[test]
fn control_point_bends_perpendicular_to_the_edge() {
    let ctrl = control_point((0.0, 0.0), (10.0, 0.0), 0.2);
    assert!(close(ctrl, (5.0, 2.0)));
    assert!(close(control_point((3.0, 3.0), (3.0, 3.0), 0.2), (3.0, 3.0)));
}

#[test]
fn hit_test_finds_the_node_under_the_cursor() {
    let scene = GraphScene::of(&AppState::new_in_memory());
    if scene.model.node_count() == 0 {
        return;
    }
    let view = Viewport::default();
    let target = scene.layout.positions()[0];
    let at = view.to_screen(&frame(), FIT, target);
    let hit = hit_test(&scene, &view, &frame(), FIT, (3.0, 9.0), at).unwrap();
    let hit_at = view.to_screen(&frame(), FIT, scene.layout.positions()[hit]);
    assert!((hit_at.0 - at.0).hypot(hit_at.1 - at.1) < TOLERANCE + 9.0);
    assert!(hit_test(&scene, &view, &frame(), FIT, (3.0, 9.0), (-1e4, -1e4)).is_none());
}

#[test]
fn small_graphs_on_roomy_canvases_label_every_node() {
    let canvas = Frame { x: 0.0, y: 0.0, width: 800.0, height: 600.0 };
    assert!(room_for_labels(&canvas, 12, 200.0));
    assert!(!room_for_labels(&canvas, 400, 200.0));
    let mini = Frame { x: 0.0, y: 0.0, width: 280.0, height: 175.0 };
    assert!(!room_for_labels(&mini, 12, 200.0));
    assert!(room_for_labels(&canvas, 0, 200.0), "an empty graph never divides by zero");
}

#[test]
fn labels_near_an_edge_slide_inside_the_canvas() {
    let canvas = Frame { x: 0.0, y: 0.0, width: 400.0, height: 300.0 };
    assert_eq!(label_origin((200.0, 100.0), (80.0, 20.0), &canvas, 4.0), (160.0, 100.0));
    assert_eq!(label_origin((395.0, 100.0), (80.0, 20.0), &canvas, 4.0), (316.0, 100.0));
    assert_eq!(label_origin((2.0, 100.0), (80.0, 20.0), &canvas, 4.0), (4.0, 100.0));
    assert_eq!(label_origin((200.0, 295.0), (80.0, 20.0), &canvas, 4.0), (160.0, 276.0));
}
