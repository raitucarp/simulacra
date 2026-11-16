//! Emblem → SVG template (theme placeholders, shared 256-unit portrait canvas).

use super::EmblemSpec;
use std::f32::consts::{FRAC_PI_2, TAU};
use std::fmt::Write;

/// Canvas side and centre (same canvas as portrait parts, so both crop identically).
pub const EMBLEM_CANVAS: f32 = 256.0;
const CENTER: f32 = EMBLEM_CANVAS / 2.0;
/// Halo disc, silhouette and wing-marker geometry.
const HALO_RADIUS: f32 = 120.0;
const HALO_OPACITY: f32 = 0.1;
const SHAPE_RADIUS: f32 = 80.0;
const SHAPE_FILL_OPACITY: f32 = 0.22;
const SHAPE_STROKE: f32 = 6.0;
/// Dash pattern of a perceiving (open-ended) silhouette.
const OPEN_DASH: &str = "14 10";
const WING_RADIUS: f32 = 116.0;
const WING_DOT: f32 = 7.0;
/// Angle between successive wing positions (nine positions around the circle).
const WING_SLOTS: f32 = 9.0;
/// Placeholder names of the background and ink colors.
const BACKGROUND_KEY: &str = "surface";
const INK_KEY: &str = "fg";

/// `{{key}}`: a theme color placeholder.
fn placeholder(key: &str) -> String {
    format!("{{{{{key}}}}}")
}

impl EmblemSpec {
    /// The emblem as an SVG template whose colors are `{{palette_key}}` placeholders.
    pub fn to_svg_template(&self) -> String {
        let primary = placeholder(self.primary.palette_key());
        let secondary = placeholder(self.secondary.palette_key());
        let ink = placeholder(INK_KEY);
        let mut out = String::new();
        let _ = write!(
            out,
            r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {EMBLEM_CANVAS} {EMBLEM_CANVAS}" fill="none" stroke-linecap="round" stroke-linejoin="round">"#
        );
        let _ = write!(out, r#"<rect width="{EMBLEM_CANVAS}" height="{EMBLEM_CANVAS}" fill="{}"/>"#, placeholder(BACKGROUND_KEY));
        let _ = write!(out, r#"<circle cx="{CENTER}" cy="{CENTER}" r="{HALO_RADIUS}" fill="{primary}" fill-opacity="{HALO_OPACITY}"/>"#);
        let dash = if self.glyph.judging { String::new() } else { format!(r#" stroke-dasharray="{OPEN_DASH}""#) };
        let outline = self.shape.outline(CENTER, CENTER, SHAPE_RADIUS);
        let _ = write!(
            out,
            r#"<path d="{outline}" fill="{primary}" fill-opacity="{SHAPE_FILL_OPACITY}" stroke="{primary}" stroke-width="{SHAPE_STROKE}"{dash}/>"#
        );
        out.push_str(&self.glyph.fragment(CENTER, &primary, &secondary, &ink));
        out.push_str(&self.wing_marker(&secondary));
        out.push_str("</svg>");
        out
    }

    /// Small disc orbiting at the wing's clock position (wing 9 at 12 o'clock).
    fn wing_marker(&self, fill: &str) -> String {
        let angle = TAU * self.wing as f32 / WING_SLOTS - FRAC_PI_2;
        let (x, y) = (CENTER + WING_RADIUS * angle.cos(), CENTER + WING_RADIUS * angle.sin());
        format!(r#"<circle cx="{x:.1}" cy="{y:.1}" r="{WING_DOT}" fill="{fill}"/>"#)
    }
}
