//! The passphrase strength meter: four segments filled by level, coloured from danger to
//! success, with the level's name.

use super::super::text::tr;
use crate::domain::data::snapshot::Strength;
use crate::theme::ThemeTokens;
use gpui_kit::*;

/// Segments of the meter.
const SEGMENTS: usize = 4;

fn colour(strength: Strength, theme: &ThemeTokens) -> Rgba {
    match strength {
        Strength::Empty => theme.border,
        Strength::Weak => theme.accent_danger,
        Strength::Fair => theme.warning,
        Strength::Strong | Strength::VeryStrong => theme.accent_success,
    }
}

/// The meter and its label.
pub(crate) fn strength_meter(strength: Strength, theme: &ThemeTokens) -> Div {
    let fill = colour(strength, theme);
    let segments = (1..=SEGMENTS).map(|level| div().flex_1().h(theme.strength_meter_height).rounded(theme.radius_full).bg(if level <= strength.level() { fill } else { theme.border_subtle }));
    let label = tr(&format!("data.snapshot.strength.{}", strength.key()));
    div()
        .flex()
        .items_center()
        .gap(theme.control_gap)
        .child(div().flex_1().flex().gap(theme.spacing_2xs).children(segments))
        .child(div().w(theme.data_label_width).flex_none().text_size(theme.font_size_2xs).font_weight(FontWeight::SEMIBOLD).text_color(fill).child(label))
}
