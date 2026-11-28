//! Reference renders for making packs (`illustration-refs`, `tools/illustrations/`): each
//! Classic template filled by the app's own filler with the default (dark) palette, and the
//! persona placeholders of layered families filled with the key colours, so a generated
//! layer comes back in colours the runtime can tint.

use crate::domain::packs::tint::{PRIMARY_KEY, SECONDARY_KEY};
use crate::theme::{PaletteColors, ThemeRegistry};
use crate::ui::assets::template_text;
use crate::ui::illustration::template::{HAIR_PLACEHOLDER, PRIMARY_PLACEHOLDER, SECONDARY_PLACEHOLDER, SKIN_PLACEHOLDER};
use crate::ui::illustration::{fill_template, TemplateExtras};

/// `#rrggbb` of an RGB colour.
fn hex([r, g, b]: [u8; 3]) -> String {
    format!("#{r:02x}{g:02x}{b:02x}")
}

/// The persona placeholders in key colours: grey for skin, hair and a form's primary, cyan
/// for its secondary.
pub fn key_colour_extras() -> TemplateExtras {
    let (grey, cyan) = (hex(PRIMARY_KEY), hex(SECONDARY_KEY));
    TemplateExtras::none().with(SKIN_PLACEHOLDER, grey.clone()).with(HAIR_PLACEHOLDER, grey.clone()).with(PRIMARY_PLACEHOLDER, grey).with(SECONDARY_PLACEHOLDER, cyan)
}

/// Colours of the bundled default palette (a dark one).
pub fn reference_palette() -> PaletteColors {
    ThemeRegistry::global().default_palette().colors
}

/// The filled SVG of the Classic template at `path`, ready to render.
pub fn reference_svg(path: &str) -> Option<String> {
    Some(fill_template(&template_text(path)?, &reference_palette(), &key_colour_extras()))
}
