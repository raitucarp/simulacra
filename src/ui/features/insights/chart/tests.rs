use super::cache::GeometryCache;
use super::columns::label_stride;
use super::cull::{label_budget, place_labels, point_visible, segment_visible, too_short};
use super::pan_zoom::PanZoom;
use super::palette::{categorical, contrast, mix, sequential, series, MIN_MARK_CONTRAST};
use super::projection::{fit_of, project};
use super::scale::{BandScale, LinearScale, RadialScale};
use super::spatial::SpatialGrid;
use super::ticks::{compact, nice_ticks, percent};
use super::tooltip::placement;
use crate::theme::ThemeTokens;
use crate::ui::features::mind_graph::viewport::{Frame, Viewport};
use std::cell::Cell;

const FRAME: Frame = Frame { x: 100.0, y: 50.0, width: 400.0, height: 300.0 };

#[test]
fn linear_and_band_scales() {
    let s = LinearScale::new((0.0, 10.0), (0.0, 200.0));
    assert_eq!(s.share(2.5), 50.0);
    assert_eq!(s.share(20.0), 200.0);
    assert_eq!(LinearScale::new((3.0, 3.0), (0.0, 1.0)).share(3.0), 0.0);
    let band = BandScale { count: 4, length: 100.0, gap: 4.0 };
    assert_eq!(band.bandwidth(), 22.0);
    assert_eq!(BandScale { count: 0, length: 10.0, gap: 4.0 }.bandwidth(), 10.0);
}

#[test]
fn radial_scale_starts_at_twelve_and_turns_clockwise() {
    let r = RadialScale { centre: (0.0, 0.0), slots: 4 };
    let (x, y) = r.point(0.0, 10.0);
    assert!(x.abs() < 1e-4 && (y + 10.0).abs() < 1e-4, "slot 0 is straight up");
    assert_eq!(r.locate((10.0, 0.1)).0, 1, "three o'clock is the second quarter");
    assert_eq!(r.locate((-0.1, -10.0)).0, 3);
    assert!((r.locate((3.0, 4.0)).1 - 5.0).abs() < 1e-4);
}

#[test]
fn ticks_and_labels() {
    assert_eq!(nice_ticks(9.0, 4), vec![0.0, 5.0, 10.0]);
    assert_eq!(nice_ticks(37.0, 4), vec![0.0, 10.0, 20.0, 30.0, 40.0]);
    assert_eq!(nice_ticks(0.0, 4), vec![0.0, 1.0]);
    assert_eq!(compact(12.0), "12");
    assert_eq!(compact(1_500.0), "1.5k");
    assert_eq!(compact(25_000.0), "25k");
    assert_eq!(compact(2_000_000.0), "2M");
    assert_eq!(compact(0.126), "0.13");
    assert_eq!(compact(3.27), "3.3");
    assert_eq!(percent(0.256), "26%");
    assert_eq!(label_stride(90, 600.0, 60.0), 9);
    assert_eq!(label_stride(5, 600.0, 60.0), 1);
}

#[test]
fn palette_comes_from_tokens_in_both_modes() {
    let registry = crate::theme::ThemeRegistry::global();
    for palette in registry.all() {
        let theme: ThemeTokens = crate::theme::derive::derive_default_tokens(palette);
        let colours = categorical(&theme);
        let opaque = |c: gpui_kit::Rgba| gpui_kit::Rgba { a: 1.0, ..c };
        let faint = colours.iter().filter(|c| contrast(opaque(**c), opaque(theme.bg_card)) < MIN_MARK_CONTRAST - 0.05).count();
        assert_eq!(faint, 0, "palette {} has marks below 3:1 on its cards", palette.id);
        assert_eq!(series(8, &theme), colours[0], "series wrap");
        assert!(colours.iter().all(|c| c.a > 0.0));
        assert_eq!(sequential(0.0, &theme), theme.border_subtle);
        assert!(sequential(1.0, &theme).a >= sequential(0.3, &theme).a);
    }
    let (a, b) = (gpui_kit::Rgba { r: 0.0, g: 0.0, b: 0.0, a: 1.0 }, gpui_kit::Rgba { r: 1.0, g: 1.0, b: 1.0, a: 1.0 });
    assert_eq!(mix(a, b, 0.5).r, 0.5);
}

#[test]
fn culling_and_label_placement() {
    let rect = (0.0, 0.0, 100.0, 100.0);
    assert!(point_visible((105.0, 50.0), 6.0, rect));
    assert!(!point_visible((120.0, 50.0), 6.0, rect));
    assert!(segment_visible((-10.0, 50.0), (110.0, 50.0), rect), "crossing segments stay");
    assert!(!segment_visible((120.0, 0.0), (150.0, 90.0), rect));
    assert!(too_short((0.0, 0.0), (0.5, 0.5), 1.0));
    let candidates = [(7, (10.0, 10.0, 40.0, 10.0)), (8, (20.0, 12.0, 40.0, 10.0)), (9, (10.0, 40.0, 40.0, 10.0)), (10, (90.0, 90.0, 40.0, 10.0))];
    assert_eq!(place_labels(&candidates, rect, 10), vec![7, 9], "overlapping and clipped labels are dropped");
    assert_eq!(place_labels(&candidates, rect, 1), vec![7]);
    assert_eq!(label_budget(rect, (10.0, 10.0), 1.0), 100);
}

#[test]
fn spatial_grid_finds_the_nearest_within_reach() {
    let points: Vec<(f32, f32)> = (0..1000).map(|i| ((i % 40) as f32 * 10.0, (i / 40) as f32 * 10.0)).collect();
    let grid = SpatialGrid::new(&points, 16.0);
    assert_eq!(grid.nearest(&points, (21.0, 9.0), |_| 5.0), Some(42));
    assert_eq!(grid.nearest(&points, (25.0, 5.0), |_| 2.0), None);
    assert_eq!(SpatialGrid::new(&[], 8.0).nearest(&[], (0.0, 0.0), |_| 9.0), None);
}

#[test]
fn geometry_cache_rebuilds_only_on_a_new_key() {
    let cache: GeometryCache<(u64, u32), Vec<u32>> = GeometryCache::default();
    let builds = Cell::new(0);
    let build = || {
        builds.set(builds.get() + 1);
        vec![1, 2]
    };
    cache.get(&(1, 10), build);
    cache.get(&(1, 10), || unreachable!("same key"));
    assert_eq!(builds.get(), 1);
    cache.get(&(2, 10), || vec![3]);
    assert_eq!(*cache.last().unwrap(), vec![3]);
    cache.clear();
    assert!(cache.last().is_none());
}

#[test]
fn projection_fills_the_frame_and_pan_zoom_moves_it() {
    let positions = [[0.0, 0.25], [1.0, 0.75]];
    let fit = fit_of(&positions, 10.0);
    let projected = project(&positions, &FRAME, &Viewport::default(), fit, 16.0);
    assert!((projected.points[0].0 - 110.0).abs() < 1e-3, "{:?}", projected.points);
    assert!((projected.points[1].0 - 490.0).abs() < 1e-3);
    let mut pan = PanZoom::default();
    pan.press((0.0, 0.0));
    assert!(!pan.drag((1.0, 1.0)), "a tiny move is still a click");
    assert!(pan.drag((20.0, 0.0)));
    assert!(!pan.release(), "a drag is not a click");
    assert_eq!(pan.view.pan, (20.0, 0.0));
    pan.wheel(&FRAME, fit, (300.0, 200.0), 500.0);
    assert!(pan.view.zoom > 1.0);
    pan.reset();
    assert!(pan.is_reset());
}

#[test]
fn tooltips_flip_near_the_edges() {
    assert_eq!(placement(&FRAME, (150.0, 100.0), (100.0, 40.0)), (62.0, 62.0));
    let (left, top) = placement(&FRAME, (480.0, 330.0), (100.0, 40.0));
    assert_eq!((left, top), (268.0, 228.0));
}
