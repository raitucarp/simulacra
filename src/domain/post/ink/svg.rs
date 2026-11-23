//! N12: a drawing as an SVG document: one round-capped path per stroke over the canvas's view
//! box. Inks come from `colour_of` (the theme's colours when exporting); the text ink can be
//! `currentColor`, so the file follows whatever shows it.

use super::{Ink, InkDrawing, Stroke};
use std::fmt::Write;

/// The text ink as an SVG paint that follows the page it is shown on.
pub const SVG_TEXT_INK: &str = "currentColor";

/// The path data of `stroke`: `M x y L x y …` (a lone point is a dot).
fn path_data(stroke: &Stroke) -> String {
    let mut data = String::new();
    for (index, (x, y)) in stroke.pairs().enumerate() {
        let _ = write!(data, "{}{x} {y} ", if index == 0 { "M" } else { "L" });
    }
    if stroke.points.len() == 2 {
        data.push_str("l0 0");
    }
    data.trim_end().to_string()
}

/// `drawing` as a standalone SVG document, each ink painted as `colour_of` says.
pub fn to_svg(drawing: &InkDrawing, colour_of: impl Fn(Ink) -> String) -> String {
    let (width, height) = (drawing.width, drawing.height);
    let mut svg = format!("<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {width} {height}\" width=\"{width}\" height=\"{height}\" fill=\"none\" stroke-linecap=\"round\" stroke-linejoin=\"round\">\n");
    for stroke in drawing.strokes.iter().filter(|stroke| stroke.points.len() >= 2) {
        let _ = writeln!(svg, "  <path d=\"{}\" stroke=\"{}\" stroke-width=\"{}\"/>", path_data(stroke), colour_of(stroke.ink), stroke.width);
    }
    svg.push_str("</svg>\n");
    svg
}
