//! Pure SVG template handling: `{{token}}` substitution and raster-size scaling.

use crate::domain::portrait::{FormColor, FormSpec, HairColor, SkinTone};
use crate::theme::color::to_hex;
use crate::theme::{PaletteColorKey, PaletteColors};

/// Placeholder delimiters (`{{accent}}`).
pub const PLACEHOLDER_OPEN: &str = "{{";
pub const PLACEHOLDER_CLOSE: &str = "}}";
/// Non-theme portrait placeholders (see `assets/illustrations/STYLE.md`).
pub const SKIN_PLACEHOLDER: &str = "skin";
pub const HAIR_PLACEHOLDER: &str = "hair";
/// Non-theme form placeholders: `{{primary}}` / `{{secondary}}` (see STYLE.md, "Forms").
pub const PRIMARY_PLACEHOLDER: &str = FormColor::Primary.key();
pub const SECONDARY_PLACEHOLDER: &str = FormColor::Secondary.key();
/// Palette color substituted for unknown placeholders (a neutral that reads on every theme).
pub const FALLBACK_COLOR_KEY: PaletteColorKey = PaletteColorKey::FgMuted;
/// Root element prefix the raster size attributes are inserted after.
const SVG_ROOT: &str = "<svg";
const VIEW_BOX_ATTR: &str = "viewBox=\"";

/// Extra, non-palette placeholder values (portrait skin and hair, form colors), kept sorted so equal
/// sets hash equally.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct TemplateExtras(Vec<(&'static str, String)>);

impl TemplateExtras {
    /// No extras: palette placeholders only.
    pub fn none() -> Self {
        Self::default()
    }

    /// The `{{skin}}` / `{{hair}}` fills of a portrait.
    pub fn portrait(skin: SkinTone, hair: HairColor) -> Self {
        Self::none().with(SKIN_PLACEHOLDER, skin.hex()).with(HAIR_PLACEHOLDER, hair.hex())
    }

    /// The `{{primary}}` / `{{secondary}}` fills of a form.
    /// A kind without swatches leaves them unset, so they render in the neutral fallback.
    pub fn form(spec: &FormSpec) -> Self {
        [(PRIMARY_PLACEHOLDER, FormColor::Primary), (SECONDARY_PLACEHOLDER, FormColor::Secondary)]
            .into_iter()
            .filter(|(_, color)| !spec.hex(*color).is_empty())
            .fold(Self::none(), |extras, (key, color)| extras.with(key, spec.hex(color)))
    }

    /// These extras with `key` filled by `value` (replacing an earlier value).
    pub fn with(mut self, key: &'static str, value: impl Into<String>) -> Self {
        self.0.retain(|(k, _)| *k != key);
        self.0.push((key, value.into()));
        self.0.sort();
        self
    }

    /// The value of `key`, if set.
    pub fn get(&self, key: &str) -> Option<&str> {
        self.0.iter().find(|(k, _)| *k == key).map(|(_, v)| v.as_str())
    }
}

/// Hex text of the palette color named `key` (`"accent_2"`), if it is one.
pub fn palette_fill(colors: &PaletteColors, key: &str) -> Option<String> {
    PaletteColorKey::ALL.iter().find(|k| k.id() == key).map(|k| to_hex(k.get(colors)))
}

/// `template` with every `{{key}}` replaced: extras first, then palette colors, and anything
/// unknown (or malformed) by the neutral fallback, so no `{{` survives.
pub fn fill_template(template: &str, colors: &PaletteColors, extras: &TemplateExtras) -> String {
    let fallback = to_hex(FALLBACK_COLOR_KEY.get(colors));
    let mut out = String::with_capacity(template.len());
    let mut rest = template;
    while let Some(start) = rest.find(PLACEHOLDER_OPEN) {
        out.push_str(&rest[..start]);
        let after = &rest[start + PLACEHOLDER_OPEN.len()..];
        let Some(end) = after.find(PLACEHOLDER_CLOSE) else {
            out.push_str(&fallback);
            rest = after;
            continue;
        };
        let key = after[..end].trim();
        let value = extras.get(key).map(str::to_string).or_else(|| palette_fill(colors, key));
        out.push_str(&value.unwrap_or_else(|| fallback.clone()));
        rest = &after[end + PLACEHOLDER_CLOSE.len()..];
    }
    out.push_str(rest);
    out
}

/// Intrinsic widths an illustration is rasterized at. Snapping display sizes to these
/// buckets bounds how many variants of one template the cache (and gpui) keep.
pub const RASTER_WIDTHS: [u16; 6] = [32, 64, 128, 256, 512, 1024];

/// Smallest raster bucket at least `display` logical pixels wide (the largest bucket for
/// anything bigger). gpui rasterizes an SVG `img` at 2x its intrinsic size and ignores the
/// window's DPI, so this yields at least two device pixels per logical pixel.
pub fn raster_width(display: f32) -> u16 {
    let last = RASTER_WIDTHS[RASTER_WIDTHS.len() - 1];
    RASTER_WIDTHS.iter().copied().find(|w| f32::from(*w) >= display).unwrap_or(last)
}

/// `svg` with explicit `width`/`height` (`width` wide, viewBox aspect), which sets the
/// intrinsic size gpui rasterizes at. Unchanged for `None` or when the root has no viewBox.
pub fn with_raster_width(svg: &str, width: Option<u16>) -> String {
    let Some(((vw, vh), width)) = width.and_then(|w| view_box_size(svg).map(|size| (size, w))) else {
        return svg.to_string();
    };
    let Some(root) = svg.find(SVG_ROOT) else {
        return svg.to_string();
    };
    let at = root + SVG_ROOT.len();
    let width = f32::from(width);
    format!(r#"{} width="{width}" height="{}"{}"#, &svg[..at], width * vh / vw, &svg[at..])
}

/// Width and height of the root `viewBox="minx miny w h"`.
pub fn view_box_size(svg: &str) -> Option<(f32, f32)> {
    let start = svg.find(VIEW_BOX_ATTR)? + VIEW_BOX_ATTR.len();
    let len = svg[start..].find('"')?;
    let numbers: Vec<f32> = svg[start..start + len].split_whitespace().filter_map(|n| n.parse().ok()).collect();
    match numbers.as_slice() {
        [_, _, w, h] if *w > 0.0 && *h > 0.0 => Some((*w, *h)),
        _ => None,
    }
}
