use super::*;

/// A 400 × 100 frame at (10, 20): 100 px of labels, 30 periods, rows 20 px high.
fn layout(range: PeriodRange) -> LaneLayout {
    LaneLayout { frame: Frame { x: 10.0, y: 20.0, width: 400.0, height: 100.0 }, label_width: 100.0, lane_height: 20.0, gap: 0.0, range, rows: 3 }
}

#[test]
fn cells_sit_in_their_row_and_column() {
    let full = layout((0, 29));
    assert_eq!(full.periods(), 30);
    assert_eq!(full.period_width(), 10.0);
    assert_eq!(full.cell(0, 0), Some((110.0, 20.0, 10.0, 20.0)));
    assert_eq!(full.cell(2, 29), Some((400.0, 60.0, 10.0, 20.0)));
    assert_eq!(full.cell(3, 0), None, "no such row");
}

#[test]
fn zooming_culls_periods_outside_the_range() {
    let zoomed = layout((10, 19));
    assert_eq!(zoomed.period_width(), 30.0);
    assert_eq!(zoomed.cell(0, 9), None);
    assert_eq!(zoomed.cell(0, 20), None);
    assert_eq!(zoomed.cell(0, 10).map(|r| r.0), Some(110.0));
    let counts = [1u32; 30];
    let cells = zoomed.cells(&[&counts]);
    assert_eq!(cells.len(), 10);
    assert!(cells.iter().all(|c| (10..=19).contains(&c.period)));
}

#[test]
fn shares_are_relative_to_the_row_peak_in_range() {
    let counts = [0, 2, 4, 0, 1];
    let cells = layout((0, 4)).cells(&[&counts]);
    assert_eq!(cells.iter().map(|c| (c.period, c.share)).collect::<Vec<_>>(), vec![(1, 0.5), (2, 1.0), (4, 0.25)]);
}

#[test]
fn hit_testing_and_brushing() {
    let full = layout((0, 29));
    assert_eq!(full.row_at(25.0), Some(0));
    assert_eq!(full.row_at(79.0), Some(2));
    assert_eq!(full.row_at(80.0), None, "below the rows: the axis");
    assert_eq!(full.row_at(10.0), None);
    assert_eq!(full.period_at(115.0), 0);
    assert_eq!(full.period_at(5000.0), 29, "clamped");
    assert_eq!(full.period_at(0.0), 0);
    assert_eq!(full.brush_range(300.0, 150.0), (4, 19));
    assert!(full.over_plot(110.0) && !full.over_plot(60.0));
}

#[test]
fn axis_labels_never_overlap() {
    let full = layout((0, 29));
    assert_eq!(full.axis_periods(35.0), vec![0, 4, 8, 12, 16, 20, 24, 28]);
    assert_eq!(layout((3, 5)).axis_periods(35.0), vec![3, 4, 5], "wide columns label every period");
}
