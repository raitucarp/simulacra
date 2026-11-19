//! Pointer use of the lane chart: hover shows a cell's tooltip, a drag across the plot draws
//! a brush and zooms into the periods it covers, a click picks (or drops) the concept of the
//! row under the pointer. A move without the button held cancels a brush that left the chart.

use crate::ui::features::insights::chart::lanes::{LaneLayout, PeriodRange};
use crate::ui::features::insights::panel::InsightsPanel;
use crate::ui::features::mind_graph::viewport::Frame;
use gpui_kit::*;

fn x_y(at: Point<Pixels>) -> (f32, f32) {
    (at.x.as_f32(), at.y.as_f32())
}

impl InsightsPanel {
    /// The chart's layout in `frame`.
    pub(super) fn lane_layout(&self, frame: Frame, range: PeriodRange, rows: usize) -> LaneLayout {
        let theme = &self.theme;
        LaneLayout { frame, label_width: theme.insight_lane_label_width.as_f32(), lane_height: theme.insight_lane_height.as_f32(), gap: theme.insight_cell_gap.as_f32(), range, rows }
    }

    /// The layout painted last and the lanes of its rows (the row count follows the frame).
    fn painted(&self) -> Option<(LaneLayout, Vec<usize>)> {
        let timeline = &self.lenses.report.as_ref()?.timeline;
        let frame = self.lenses.timeline.frame.get()?;
        let theme = &self.theme;
        let rows = ((frame.height - theme.insight_axis_height.as_f32()) / theme.insight_lane_height.as_f32()).round().max(0.0) as usize;
        let range = self.timeline_range(timeline);
        let lanes = timeline.active_lanes(range, rows);
        Some((self.lane_layout(frame, range, lanes.len()), lanes))
    }

    /// The chart's container, wired to the pointer.
    pub(super) fn timeline_surface(&self, canvas: AnyElement, height: Pixels, cx: &mut Context<Self>) -> Stateful<Div> {
        div()
            .id("insight-timeline-chart")
            .relative()
            .w_full()
            .h(height)
            .cursor_crosshair()
            .on_mouse_down(MouseButton::Left, cx.listener(|panel, event: &MouseDownEvent, _, cx| panel.timeline_press(x_y(event.position), cx)))
            .on_mouse_move(cx.listener(|panel, event: &MouseMoveEvent, _, cx| panel.timeline_move(x_y(event.position), event.pressed_button == Some(MouseButton::Left), cx)))
            .on_mouse_up(MouseButton::Left, cx.listener(|panel, event: &MouseUpEvent, _, cx| panel.timeline_release(x_y(event.position), cx)))
            .on_hover(cx.listener(|panel, hovered: &bool, _, cx| if !hovered { panel.timeline_leave(cx) }))
            .child(canvas)
    }

    fn timeline_press(&mut self, (x, _): (f32, f32), cx: &mut Context<Self>) {
        if self.painted().is_some_and(|(layout, _)| layout.over_plot(x)) {
            self.lenses.timeline.brush = Some((x, x));
            cx.notify();
        }
    }

    fn timeline_move(&mut self, at: (f32, f32), pressed: bool, cx: &mut Context<Self>) {
        let ui = &mut self.lenses.timeline;
        match (ui.brush, pressed) {
            (Some((start, _)), true) => ui.brush = Some((start, at.0)),
            (Some(_), false) => ui.brush = None,
            (None, _) => {}
        }
        let hover = self.painted().and_then(|(layout, lanes)| layout.row_at(at.1).filter(|_| layout.over_plot(at.0)).and_then(|row| lanes.get(row).map(|&lane| (lane, layout.period_at(at.0)))));
        (self.lenses.timeline.hover, self.pointer) = (hover, at);
        cx.notify();
    }

    fn timeline_release(&mut self, (x, y): (f32, f32), cx: &mut Context<Self>) {
        let Some((start, _)) = self.lenses.timeline.brush.take() else { return };
        let Some((layout, lanes)) = self.painted() else { return };
        if (x - start).abs() >= self.theme.insight_brush_min_width.as_f32() {
            self.lenses.timeline.zoom = Some(layout.brush_range(start, x));
            self.lenses.timeline.hover = None;
        } else if let Some(&lane) = layout.row_at(y).and_then(|row| lanes.get(row)) {
            self.toggle_lane(lane, cx);
        }
        cx.notify();
    }

    fn timeline_leave(&mut self, cx: &mut Context<Self>) {
        if self.lenses.timeline.hover.take().is_some() {
            cx.notify();
        }
    }

    /// Picks `lane`'s concept, or drops it when it is already picked.
    pub(super) fn toggle_lane(&mut self, lane: usize, cx: &mut Context<Self>) {
        let selected = &mut self.lenses.timeline.selected;
        *selected = if *selected == Some(lane) { None } else { Some(lane) };
        cx.notify();
    }

    /// Back to the whole span.
    pub(super) fn reset_timeline_zoom(&mut self, cx: &mut Context<Self>) {
        self.lenses.timeline.zoom = None;
        cx.notify();
    }
}
