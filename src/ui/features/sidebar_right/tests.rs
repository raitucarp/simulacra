use super::heatmap_geometry::GridGeometry;
use crate::domain::activity::{MonthLabel, HEATMAP_WEEKS};
use gpui_kit::{px, Pixels};

const GAP: Pixels = px(1.0);
const RANGE: (Pixels, Pixels) = (px(8.0), px(14.0));

fn width_of(geometry: GridGeometry) -> Pixels {
    geometry.cell * geometry.weeks as f32 + GAP * (geometry.weeks as f32 - 1.0)
}

#[test]
fn a_wide_grid_shows_the_whole_year_with_capped_cells() {
    let geometry = GridGeometry::fit(px(800.0), GAP, RANGE);
    assert_eq!(geometry.weeks, HEATMAP_WEEKS);
    assert_eq!(geometry.cell, px(14.0));
    let roomy = GridGeometry::fit(px(560.0), GAP, RANGE);
    assert_eq!(roomy.weeks, HEATMAP_WEEKS);
    assert!(roomy.cell > px(8.0) && roomy.cell < px(14.0), "{roomy:?}");
    assert!(width_of(roomy) <= px(560.0) + px(0.01));
}

#[test]
fn a_narrow_grid_keeps_the_minimum_cell_and_drops_old_weeks() {
    for width in [272.0, 224.0, 120.0, 9.0] {
        let geometry = GridGeometry::fit(px(width), GAP, RANGE);
        assert!(geometry.weeks < HEATMAP_WEEKS && geometry.weeks >= 1, "{width}: {geometry:?}");
        assert!(geometry.cell >= px(8.0) && geometry.cell <= px(14.0), "{width}: {geometry:?}");
        assert!(width_of(geometry) <= px(width) + px(0.01), "{width}: {geometry:?}");
        assert_eq!(geometry.first_week() + geometry.weeks, HEATMAP_WEEKS);
    }
    assert_eq!(GridGeometry::fit(px(-5.0), GAP, RANGE).weeks, 1, "a collapsed sidebar still lays out");
}

#[test]
fn month_labels_never_overlap_and_the_later_month_wins() {
    let geometry = GridGeometry::fit(px(400.0), GAP, RANGE);
    let labels: Vec<MonthLabel> = [(10, 11), (11, 12), (15, 1), (19, 2), (20, 3), (52, 4)]
        .map(|(week, month)| MonthLabel { week: week + geometry.first_week() - 10, month })
        .into_iter()
        .filter(|label| label.week < HEATMAP_WEEKS)
        .collect();
    let placed = geometry.place_labels(&labels, px(26.0));
    let months: Vec<i8> = placed.iter().map(|label| label.month).collect();
    assert!(!months.contains(&11), "the crowded leading month is dropped: {months:?}");
    assert!(months.contains(&12) && months.contains(&1));
    for pair in placed.windows(2) {
        let end = geometry.step() * pair[0].column as f32 + pair[0].width;
        assert!(end <= geometry.step() * pair[1].column as f32, "{placed:?}");
        assert!(pair[0].width >= px(26.0) - geometry.step(), "{placed:?}");
    }
    assert!(placed.iter().all(|label| label.column < geometry.weeks));
}
