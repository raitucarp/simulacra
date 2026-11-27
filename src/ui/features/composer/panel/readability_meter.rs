//! sb F3: the readability meter in a draft's meter row, on one line: a small bar filled to the
//! draft's ease (0–100, in its band's colour) and the band ("Very easy"); on hover the ease,
//! the formula with its value (and the grade level for English) and its source; and a dismiss
//! button. Muted on purpose, and it never pushes the composer down when it appears (Q12).

use super::super::modal::ModalActionCallback;
use crate::domain::readability::Readability;
use crate::theme::ThemeTokens;
use crate::ui::features::insights::readability_style::{band_colour, band_label, ease_text, formula_source, score_detail};
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::tooltip::Tooltip;
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::*;
use rust_i18n::t;

const ROW_ID: &str = "readability-meter";
const DISMISS_ID: &str = "readability-meter-dismiss";
/// Top of the ease scale.
const EASE_MAX: f64 = 100.0;

/// The meter of draft `draft`, with its dismiss action.
#[derive(Clone)]
pub struct ReadabilityMeterView {
    pub draft: usize,
    pub score: Readability,
    pub on_dismiss: ModalActionCallback,
}

/// The bar: a track filled to `ease` of 100.
fn bar(ease: f64, colour: Rgba, theme: &ThemeTokens) -> Div {
    let fill = div().h_full().w(relative((ease / EASE_MAX).clamp(0.0, 1.0) as f32)).rounded(theme.radius_full).bg(colour);
    div().flex_none().w(theme.insight_meter_width).h(theme.insight_meter_height).rounded(theme.radius_full).bg(theme.border_subtle).child(fill)
}

/// The row for `view`.
pub fn render_readability_meter(view: &ReadabilityMeterView, theme: &ThemeTokens) -> Stateful<Div> {
    let score = &view.score;
    let colour = band_colour(score.band, theme);
    let headline = t!("insight.meter.headline", band = band_label(score.band), ease = ease_text(score.ease)).to_string();
    // The formula and its source sit in the tooltip: the meter row is shared with the
    // atomic-note hint and the character count, and the band says enough at a glance.
    let source = format!("{headline}
{}
{}", score_detail(score), t!("insight.meter.source", source = formula_source(score.formula)));
    let on_dismiss = view.on_dismiss.clone();
    let dismiss = Button::new(DISMISS_ID).ghost().xsmall().icon(Icon::new(LucideIcon::X)).tooltip(t!("insight.meter.dismiss").to_string()).on_click(move |_, window, cx| on_dismiss(window, cx));
    let text = div().flex_none().text_size(theme.font_size_2xs).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_secondary).child(band_label(score.band));
    div()
        .id(ROW_ID)
        .flex_none()
        .flex()
        .items_center()
        .gap(theme.inline_gap)
        .tooltip(move |window, cx| Tooltip::new(source.clone()).build(window, cx))
        .child(Icon::new(LucideIcon::BookOpen).size(theme.icon_size_sm).text_color(theme.text_muted))
        .child(bar(score.ease, colour, theme))
        .child(text)
        .child(dismiss)
}
