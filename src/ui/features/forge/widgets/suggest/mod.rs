//! Suggestion widgets (sb D1): example chips under text fields ([`HintChips`]), the typical
//! band under sliders ([`typical_band`]), the "Suggested" badge on option cards and the
//! "Why?" card ([`why_button`]).

mod band;
mod chips;
mod why;

pub use band::{band_caption, band_fraction, typical_band};
pub use chips::{ChipSpec, HintChips};
pub use why::{why_body, why_button, WhyCard, WhySection, WhyTone};

use crate::theme::ThemeTokens;
use crate::ui::features::forge::text::tr;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::Icon;
use gpui_kit::*;

const SUGGESTED_KEY: &str = "suggest.ui.suggested";

/// The gold "Suggested" pill pinned to the top-left of an option card's art.
pub fn suggested_badge(theme: &ThemeTokens) -> Div {
    div()
        .absolute()
        .top(theme.spacing_xs)
        .left(theme.spacing_xs)
        .flex()
        .items_center()
        .gap(theme.spacing_3xs)
        .px(theme.spacing_xs)
        .py(theme.spacing_3xs)
        .rounded(theme.radius_full)
        .border(theme.border_width_hairline)
        .border_color(theme.accent_dialectic_border)
        .bg(theme.bg_elevated)
        .text_size(theme.font_size_2xs)
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(theme.accent_dialectic)
        .child(Icon::new(LucideIcon::Sparkles).size(theme.font_size_2xs))
        .child(tr(SUGGESTED_KEY))
}
