//! A (stacked) column chart that fills its container: one equal slot per column across the
//! measured width, bars centred in their slot up to the token width, a value axis with nice
//! ticks and gridlines, thinned category labels that never overlap, a tooltip and an optional
//! click per column.

use super::scale::{BandScale, LinearScale};
use super::ticks::{compact, count_ticks, nice_ticks};
use crate::theme::ThemeTokens;
use crate::ui::components::measured_width;
use gpui_kit::component::tooltip::Tooltip;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use std::rc::Rc;

/// Value-axis steps aimed for.
const TICK_COUNT: usize = 4;

/// A click on column `index`.
pub type ColumnClick = Rc<dyn Fn(usize, &mut Window, &mut App)>;

/// One column: stacked segments (bottom first) and its tooltip.
#[derive(Debug, Clone, PartialEq)]
pub struct Column {
    pub segments: Vec<(f64, Rgba)>,
    pub tip: String,
}

impl Column {
    pub fn total(&self) -> f64 {
        self.segments.iter().map(|(v, _)| v).sum()
    }
}

/// Indices of the labels that fit: every `step`-th column so that labels of `label_width`
/// do not overlap within `width`.
pub fn label_stride(count: usize, width: f32, label_width: f32) -> usize {
    let band = BandScale { count, length: width, gap: 0.0 }.bandwidth().max(f32::EPSILON);
    ((label_width / band).ceil() as usize).max(1)
}

#[derive(IntoElement)]
pub struct ColumnChart {
    pub id: SharedString,
    pub columns: Vec<Column>,
    pub labels: Vec<String>,
    pub height: Pixels,
    pub on_click: Option<ColumnClick>,
    pub theme: ThemeTokens,
}

impl ColumnChart {
    fn column(&self, index: usize, column: &Column, scale: LinearScale) -> Stateful<Div> {
        let theme = &self.theme;
        let tip = column.tip.clone();
        let segments = column.segments.iter().rev().filter(|(v, _)| *v > 0.0).map(|(value, colour)| {
            div().w_full().h(relative(scale.share(*value))).bg(*colour)
        });
        let bar = div().h_full().w_full().max_w(theme.insights_bar_max_width).flex().flex_col().justify_end().rounded_t(theme.radius_xs).overflow_hidden().children(segments);
        let click = self.on_click.clone();
        div()
            .id(ElementId::NamedInteger(self.id.clone(), index as u64))
            .flex_1()
            .min_w_0()
            .h_full()
            .flex()
            .justify_center()
            .hover(|slot| slot.bg(theme.hover_bg))
            .tooltip(move |window, cx| Tooltip::new(tip.clone()).build(window, cx))
            .when_some(click, |slot, click| slot.cursor_pointer().on_click(move |_, window, cx| click(index, window, cx)))
            .child(bar)
    }

    fn axis(&self, ticks: &[f64], scale: LinearScale) -> Div {
        let theme = &self.theme;
        let marks = ticks.iter().map(|&tick| {
            let label = div().text_size(theme.font_size_2xs).line_height(theme.font_size_2xs).text_color(theme.text_muted).child(compact(tick));
            div().absolute().right_0().bottom(relative(scale.share(tick))).child(label)
        });
        div().relative().flex_none().w(theme.insights_axis_width).h(self.height).children(marks)
    }

    fn grid<'a>(&self, ticks: &'a [f64], scale: LinearScale) -> impl Iterator<Item = Div> + 'a {
        let theme = self.theme.clone();
        ticks.iter().copied().map(move |tick| {
            div().absolute().left_0().right_0().bottom(relative(scale.share(tick))).h(theme.border_width_hairline).bg(theme.border_subtle)
        })
    }

    /// Labels under every `stride`-th column, each spanning its `stride` columns so it is
    /// never cut to one narrow slot; a last label with fewer columns left than that is left out.
    fn label_row(&self, stride: usize) -> Div {
        let theme = &self.theme;
        let count = self.columns.len().max(1);
        let cells = (0..self.columns.len()).step_by(stride.max(1)).map(|i| {
            let span = stride.min(count - i) as f32 / count as f32;
            let text = if stride > 1 && count - i < stride { String::new() } else { self.labels.get(i).cloned().unwrap_or_default() };
            let cell = div().w(relative(span)).flex_none().overflow_hidden().whitespace_nowrap().text_size(theme.font_size_2xs).text_color(theme.text_muted);
            if stride == 1 { cell.flex().justify_center().child(text) } else { cell.child(text) }
        });
        div().flex().pl(theme.insights_axis_width + theme.spacing_xs).children(cells)
    }
}

impl RenderOnce for ColumnChart {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let (measured, probe) = measured_width(self.id.clone(), window, cx);
        let theme = self.theme.clone();
        let max = self.columns.iter().map(Column::total).fold(0.0, f64::max);
        let counts = self.columns.iter().flat_map(|c| &c.segments).all(|(value, _)| value.fract() == 0.0);
        let ticks = if counts { count_ticks(max, TICK_COUNT) } else { nice_ticks(max, TICK_COUNT) };
        let top = ticks.last().copied().unwrap_or(1.0);
        let scale = LinearScale::new((0.0, top), (0.0, 1.0));
        let plot_width = measured.map_or(theme.insights_tile_width, |w| w - theme.insights_axis_width).as_f32();
        let stride = label_stride(self.columns.len(), plot_width, theme.insights_axis_width.as_f32() * 2.0);
        let columns: Vec<Stateful<Div>> = self.columns.iter().enumerate().map(|(i, c)| self.column(i, c, scale)).collect();
        let plot = div().relative().flex_1().min_w_0().h(self.height).flex().gap(theme.insights_bar_gap).items_end().children(self.grid(&ticks, scale)).children(columns);
        div()
            .relative()
            .w_full()
            .flex()
            .flex_col()
            .gap(theme.spacing_xs)
            .child(probe)
            .child(div().flex().gap(theme.spacing_xs).child(self.axis(&ticks, scale)).child(plot))
            .child(self.label_row(stride))
    }
}
