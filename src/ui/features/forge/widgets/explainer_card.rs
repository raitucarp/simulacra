//! The text of an explainer card ([`Explainer`]) and how it is laid out.

use crate::domain::TraitOption;
use crate::theme::ThemeTokens;
use crate::ui::features::forge::text::{field_label, tr, tr_opt, trait_description, trait_label};
use gpui_kit::*;

/// `forge.explain.{path}.{what|effect|example}`.
pub(super) const EXPLAIN_ROOT: &str = "forge.explain";
const WHAT: &str = "what";
const EFFECT: &str = "effect";
const EXAMPLE: &str = "example";
const WHAT_HEADING: &str = "forge.ui.explain_what";
const EFFECT_HEADING: &str = "forge.ui.explain_effect";
const EXAMPLE_HEADING: &str = "forge.ui.explain_example";

/// The text of one explainer card.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Explainer {
    pub title: String,
    pub what: String,
    pub effect: Option<String>,
    pub example: Option<String>,
}

impl Explainer {
    /// The explainer of the field with serde name `path`, when its texts exist.
    pub fn for_field(path: &str) -> Option<Self> {
        let part = |suffix: &str| tr_opt(&format!("{EXPLAIN_ROOT}.{path}.{suffix}"));
        Some(Self { title: field_label(path), what: part(WHAT)?, effect: part(EFFECT), example: part(EXAMPLE) })
    }

    /// The explainer of one trait option (its label and description), when described.
    pub fn for_option<T: TraitOption>(option: T) -> Option<Self> {
        Some(Self { title: trait_label(option), what: trait_description(option)?, effect: None, example: None })
    }
}

fn section(heading_key: &str, body: String, theme: &ThemeTokens) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(theme.spacing_2xs)
        .child(div().text_size(theme.font_size_2xs).font_weight(FontWeight::BOLD).text_color(theme.text_muted).child(tr(heading_key)))
        .child(div().text_size(theme.font_size_xs).text_color(theme.text_primary).line_height(relative(theme.line_height_body)).child(body))
}

pub(super) fn card(explainer: &Explainer, theme: &ThemeTokens) -> Div {
    div()
        .w(theme.forge_explainer_width)
        .p(theme.card_padding)
        .flex()
        .flex_col()
        .gap(theme.spacing_sm)
        .child(div().text_size(theme.font_size_sm).font_weight(FontWeight::BOLD).child(explainer.title.clone()))
        .child(section(WHAT_HEADING, explainer.what.clone(), theme))
        .children(explainer.effect.clone().map(|text| section(EFFECT_HEADING, text, theme)))
        .children(explainer.example.clone().map(|text| section(EXAMPLE_HEADING, text, theme)))
}
