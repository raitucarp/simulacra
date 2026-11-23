//! N12: painting an ink drawing. The strokes are scaled to fit the element (keeping the
//! canvas's shape, centred), each in its ink's theme colour and a width scaled alike; a lone
//! point is a dot. The drawing sheet's pad and the post card both paint through here.

use crate::domain::post::ink::{Ink, InkDrawing};
use crate::theme::ThemeTokens;
use crate::ui::features::insights::chart::paint::{dot, StrokeBatch};
use gpui_kit::*;

const HALF: f32 = 0.5;

/// The theme colour of `ink`.
pub fn ink_colour(ink: Ink, theme: &ThemeTokens) -> Rgba {
    match ink {
        Ink::Text => theme.text_primary,
        Ink::Accent => theme.accent_brand,
        Ink::Danger => theme.accent_danger,
        Ink::Success => theme.accent_success,
    }
}

/// `#rrggbb` of `colour` (for the saved SVG).
pub fn hex(colour: Rgba) -> String {
    let channel = |value: f32| (value.clamp(0.0, 1.0) * 255.0).round() as u8;
    format!("#{:02x}{:02x}{:02x}", channel(colour.r), channel(colour.g), channel(colour.b))
}

/// Where the canvas lands in `bounds`: its scale and top-left corner.
pub fn placement(bounds: Bounds<Pixels>, width: u16, height: u16) -> (f32, f32, f32) {
    let (w, h) = (bounds.size.width.as_f32(), bounds.size.height.as_f32());
    let scale = (w / f32::from(width.max(1))).min(h / f32::from(height.max(1)));
    let left = bounds.origin.x.as_f32() + (w - scale * f32::from(width)) * HALF;
    let top = bounds.origin.y.as_f32() + (h - scale * f32::from(height)) * HALF;
    (scale, left, top)
}

fn paint(bounds: Bounds<Pixels>, drawing: &InkDrawing, theme: &ThemeTokens, window: &mut Window) {
    let (scale, left, top) = placement(bounds, drawing.width, drawing.height);
    let at = |(x, y): (i16, i16)| (left + f32::from(x) * scale, top + f32::from(y) * scale);
    for stroke in &drawing.strokes {
        let (colour, width) = (ink_colour(stroke.ink, theme), f32::from(stroke.width) * scale);
        let points: Vec<(f32, f32)> = stroke.pairs().map(at).collect();
        if let [single] = points.as_slice() {
            dot(window, *single, width * HALF, colour, None);
            continue;
        }
        let mut batch = StrokeBatch::new(px(width), colour);
        for pair in points.windows(2) {
            batch.line(pair[0], pair[1], window);
        }
        batch.finish(window);
    }
}

/// An element painting `drawing` over its whole size.
pub fn ink_figure(drawing: InkDrawing, theme: &ThemeTokens) -> impl IntoElement {
    let theme = theme.clone();
    canvas(|_, _, _| {}, move |bounds, (), window, _| paint(bounds, &drawing, &theme, window)).size_full()
}
