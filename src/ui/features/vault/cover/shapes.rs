//! SVG primitives of the cover motifs (palette placeholders or `#000` / `#fff` shading).

use std::f32::consts::TAU;

pub const WHITE: &str = "#fff";
pub const BLACK: &str = "#000";

pub fn n(value: f32) -> String {
    format!("{:.1}", value)
}

pub fn paint(color: &str) -> String {
    if color.starts_with('#') { color.to_string() } else { format!("{{{{{color}}}}}") }
}

pub fn rect(x: f32, y: f32, w: f32, h: f32, color: &str, opacity: f32) -> String {
    format!(r#"<rect x="{}" y="{}" width="{}" height="{}" rx="1.5" fill="{}" fill-opacity="{}"/>"#, n(x), n(y), n(w), n(h), paint(color), n(opacity))
}

pub fn circle(cx: f32, cy: f32, r: f32, stroke: &str, width: f32, opacity: f32) -> String {
    format!(r#"<circle cx="{}" cy="{}" r="{}" stroke="{}" stroke-width="{}" stroke-opacity="{}"/>"#, n(cx), n(cy), n(r), paint(stroke), n(width), n(opacity))
}

pub fn polygon(cx: f32, cy: f32, r: f32, sides: u64, turn: f32, fill: &str) -> String {
    let points: Vec<String> = (0..sides)
        .map(|i| {
            let angle = turn + TAU * i as f32 / sides as f32;
            format!("{},{}", n(cx + r * angle.cos()), n(cy + r * angle.sin()))
        })
        .collect();
    format!(r#"<polygon points="{}" fill="{}"/>"#, points.join(" "), paint(fill))
}
