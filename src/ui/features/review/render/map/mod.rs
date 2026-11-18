//! The connection map canvas: point chips placed at fractions of the container (measured
//! like the statistics charts, so the map fills whatever width it gets) over a painted
//! layer of connections, with arrowheads for one-way ones and their relation names.
//!
//! - `chips`: what a point says (a post's author and words, a passage's document, a concept).
//! - `geometry`: chip boxes and connection ends in pixels (pure).
//! - `paint`: the connection strokes and arrowheads.

pub mod chips;
mod geometry;
mod paint;

pub use chips::{point_chip, point_name, PointChip};

use crate::theme::ThemeTokens;
use crate::ui::components::measured_width;
use geometry::{chip_origin, Frame};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use paint::paint_strokes;
use std::rc::Rc;

/// A click on point `index`.
pub type PointAction = Rc<dyn Fn(usize, &mut Window, &mut App) + 'static>;

/// One connection to draw.
#[derive(Debug, Clone, PartialEq)]
pub struct Stroke {
    pub from: usize,
    pub to: usize,
    pub color: Rgba,
    /// Drawn as a curve (missing connections, so they stand apart from drawn ones).
    pub curved: bool,
    pub arrow: bool,
    pub label: Option<String>,
}

/// A map to draw.
#[derive(IntoElement)]
pub struct MapCanvas {
    pub id: SharedString,
    pub chips: Vec<PointChip>,
    pub positions: Vec<(f32, f32)>,
    pub strokes: Vec<Stroke>,
    /// The point a connection is being drawn from.
    pub pending: Option<usize>,
    pub on_point: Option<PointAction>,
    pub theme: ThemeTokens,
}

impl MapCanvas {
    fn chip(&self, index: usize, chip: &PointChip, frame: &Frame) -> Stateful<Div> {
        let theme = &self.theme;
        let (x, y) = chip_origin(frame, self.positions.get(index).copied().unwrap_or_default());
        let pending = self.pending == Some(index);
        let on_point = self.on_point.clone();
        chips::render_chip(chip, ElementId::NamedInteger(self.id.clone(), index as u64), pending, theme)
            .absolute()
            .left(px(x))
            .top(px(y))
            .when_some(on_point, |chip, action| chip.cursor_pointer().on_click(move |_, window, cx| action(index, window, cx)))
    }

    fn label(&self, stroke: &Stroke, frame: &Frame) -> Option<Div> {
        let theme = &self.theme;
        let text = stroke.label.clone()?;
        let (x, y) = geometry::label_origin(frame, &self.positions, stroke)?;
        Some(
            div()
                .absolute()
                .left(px(x))
                .top(px(y))
                .w(theme.review_map_label_width)
                .flex()
                .justify_center()
                .child(div().max_w_full().truncate().px(theme.spacing_xs).rounded(theme.radius_full).bg(theme.bg_elevated).border(theme.border_width_hairline).border_color(stroke.color).text_size(theme.font_size_2xs).text_color(theme.text_secondary).child(text)),
        )
    }
}

impl RenderOnce for MapCanvas {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let (measured, probe) = measured_width(ElementId::Name(format!("{}-width", self.id).into()), window, cx);
        let theme = self.theme.clone();
        let frame = Frame::new(measured.unwrap_or(theme.review_card_max_width), &theme);
        let chips: Vec<Stateful<Div>> = self.chips.iter().enumerate().map(|(i, chip)| self.chip(i, chip, &frame)).collect();
        let labels: Vec<Div> = self.strokes.iter().filter_map(|stroke| self.label(stroke, &frame)).collect();
        let (strokes, positions, paint_theme) = (Rc::new(self.strokes), Rc::new(self.positions), theme.clone());
        let layer = canvas(|_, _, _| {}, move |bounds, (), window, _| paint_strokes(&strokes, &positions, bounds, &paint_theme, window)).absolute().top_0().left_0().size_full();
        div()
            .relative()
            .w_full()
            .h(theme.review_map_height)
            .rounded(theme.radius_md)
            .border(theme.border_width_hairline)
            .border_color(theme.border_subtle)
            .bg(theme.bg_app)
            .overflow_hidden()
            .child(probe)
            .child(layer)
            .children(labels)
            .children(chips)
    }
}
