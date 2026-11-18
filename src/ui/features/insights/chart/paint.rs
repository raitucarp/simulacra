//! Canvas painting primitives shared by the charts: dots, batched strokes (split into paths
//! small enough for the tessellator's 16-bit indices), filled polygons and text labels.

use gpui_kit::{fill, font, point, px, size, App, Bounds, PathBuilder, Pixels, Rgba, SharedString, TextAlign, TextRun, TruncateFrom, Window};

/// Segments per stroked path (each takes a handful of vertices; a path holds 65 535).
const SEGMENTS_PER_PATH: usize = 2_000;
const DIAMETER: f32 = 2.0;
const ELLIPSIS: &str = "…";

pub fn to_point((x, y): (f32, f32)) -> gpui_kit::Point<Pixels> {
    point(px(x), px(y))
}

/// A filled dot, optionally ringed.
pub fn dot(window: &mut Window, (x, y): (f32, f32), radius: f32, colour: Rgba, ring: Option<(Pixels, Rgba)>) {
    let side = px(radius * DIAMETER);
    let mut quad = fill(Bounds::new(point(px(x - radius), px(y - radius)), size(side, side)), colour).corner_radii(px(radius));
    if let Some((width, ring_colour)) = ring {
        quad = quad.border_widths(width).border_color(ring_colour);
    }
    window.paint_quad(quad);
}

/// Strokes of one colour and width, flushed in bounded paths.
pub struct StrokeBatch {
    width: Pixels,
    colour: Rgba,
    builder: PathBuilder,
    segments: usize,
}

impl StrokeBatch {
    pub fn new(width: Pixels, colour: Rgba) -> Self {
        Self { width, colour, builder: PathBuilder::stroke(width), segments: 0 }
    }

    fn flush_if_full(&mut self, window: &mut Window) {
        if self.segments >= SEGMENTS_PER_PATH {
            self.flush(window);
        }
    }

    fn flush(&mut self, window: &mut Window) {
        let builder = std::mem::replace(&mut self.builder, PathBuilder::stroke(self.width));
        if self.segments > 0 {
            if let Ok(path) = builder.build() {
                window.paint_path(path, self.colour);
            }
        }
        self.segments = 0;
    }

    pub fn line(&mut self, a: (f32, f32), b: (f32, f32), window: &mut Window) {
        self.builder.move_to(to_point(a));
        self.builder.line_to(to_point(b));
        self.segments += 1;
        self.flush_if_full(window);
    }

    pub fn curve(&mut self, a: (f32, f32), b: (f32, f32), control: (f32, f32), window: &mut Window) {
        self.builder.move_to(to_point(a));
        self.builder.curve_to(to_point(b), to_point(control));
        self.segments += 1;
        self.flush_if_full(window);
    }

    /// Paints what is left.
    pub fn finish(mut self, window: &mut Window) {
        self.flush(window);
    }
}

/// A filled polygon.
pub fn polygon(window: &mut Window, points: &[(f32, f32)], colour: Rgba) {
    if points.len() < 3 {
        return;
    }
    let mut builder = PathBuilder::fill();
    let corners: Vec<_> = points.iter().map(|p| to_point(*p)).collect();
    builder.add_polygon(&corners, true);
    if let Ok(path) = builder.build() {
        window.paint_path(path, colour);
    }
}

/// Text style of a painted label.
#[derive(Clone)]
pub struct LabelStyle {
    pub family: SharedString,
    pub size: Pixels,
    pub line_height: Pixels,
    pub colour: Rgba,
    pub max_width: Pixels,
}

/// Paints `text` (cut to the style's width) with its top-left corner at `origin`; returns its
/// width in pixels.
pub fn label(window: &mut Window, cx: &mut App, text: &str, origin: (f32, f32), style: &LabelStyle) -> f32 {
    if text.is_empty() {
        return 0.0;
    }
    let run = TextRun { len: text.len(), font: font(style.family.clone()), color: style.colour.into(), background_color: None, underline: None, strikethrough: None };
    let mut wrapper = cx.text_system().line_wrapper(run.font.clone(), style.size);
    let runs = [run];
    let (text, runs) = wrapper.truncate_line(SharedString::from(text.to_string()), style.max_width, ELLIPSIS, &runs, TruncateFrom::End);
    let line = window.text_system().shape_line(text, style.size, &runs, None);
    let width = line.width().as_f32();
    let _ = line.paint(to_point(origin), style.line_height, TextAlign::Left, None, window, cx);
    width
}

/// Width `text` would take in `style` (cut to its maximum).
pub fn label_width(window: &mut Window, text: &str, style: &LabelStyle) -> f32 {
    let run = TextRun { len: text.len(), font: font(style.family.clone()), color: style.colour.into(), background_color: None, underline: None, strikethrough: None };
    let line = window.text_system().shape_line(SharedString::from(text.to_string()), style.size, &[run], None);
    line.width().as_f32().min(style.max_width.as_f32())
}
