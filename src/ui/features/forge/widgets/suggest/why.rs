//! The "Why?" pill and its card: what makes a good value, the typical range, good starting
//! picks, examples, and what pairs or clashes — already translated by the caller.

use super::super::explainer::{CardBody, ExplainerButton};
use crate::theme::ThemeTokens;
use crate::ui::features::forge::text::tr;
use crate::ui::icons::LucideIcon;
use gpui_kit::*;
use std::rc::Rc;

const WHY_KEY: &str = "suggest.ui.why";
const BULLET: &str = "· ";

/// How a section of the card reads.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WhyTone {
    Plain,
    Good,
    Warn,
}

/// One headed section of lines.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhySection {
    pub heading: String,
    pub lines: Vec<String>,
    pub tone: WhyTone,
}

/// The whole card.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhyCard {
    pub title: String,
    pub sections: Vec<WhySection>,
}

fn section(section: &WhySection, theme: &ThemeTokens) -> Div {
    let color = match section.tone {
        WhyTone::Plain => theme.text_primary,
        WhyTone::Good => theme.accent_success,
        WhyTone::Warn => theme.warning,
    };
    let lines = section.lines.iter().map(|line| {
        let text = if section.lines.len() > 1 { format!("{BULLET}{line}") } else { line.clone() };
        div().text_size(theme.font_size_xs).text_color(color).line_height(relative(theme.line_height_body)).child(text)
    });
    div()
        .flex()
        .flex_col()
        .gap(theme.spacing_2xs)
        .child(div().text_size(theme.font_size_2xs).font_weight(FontWeight::BOLD).text_color(theme.text_muted).child(section.heading.clone()))
        .children(lines)
}

/// The card's body.
pub fn why_body(card: &WhyCard, theme: &ThemeTokens) -> Div {
    div()
        .w(theme.suggest_card_width)
        .p(theme.card_padding)
        .flex()
        .flex_col()
        .gap(theme.spacing_sm)
        .child(div().text_size(theme.font_size_sm).font_weight(FontWeight::BOLD).child(card.title.clone()))
        .children(card.sections.iter().filter(|part| !part.lines.is_empty()).map(|part| section(part, theme)))
}

/// A "Why?" pill opening `card` on hover (Enter / Space pins it, like the "?" explainers).
pub fn why_button(id: impl Into<SharedString>, card: WhyCard, theme: &ThemeTokens) -> ExplainerButton {
    let body: CardBody = Rc::new(move |theme: &ThemeTokens| why_body(&card, theme));
    ExplainerButton::custom(id, LucideIcon::Lightbulb, body, theme).label(tr(WHY_KEY))
}
