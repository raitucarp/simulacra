//! [`ExplainerButton`]: a small "?" next to a setting that opens a hover card saying what it
//! is, how it changes posts and an example, in plain language.

use crate::domain::TraitOption;
use crate::theme::ThemeTokens;
use crate::ui::features::forge::text::{field_label, tr, tr_opt, trait_description, trait_label};
use crate::ui::icons::LucideIcon;
use gpui_kit::component::hover_card::HoverCard;
use gpui_kit::component::Icon;
use gpui_kit::*;

/// `forge.explain.{path}.{what|effect|example}`.
const EXPLAIN_ROOT: &str = "forge.explain";
const WHAT: &str = "what";
const EFFECT: &str = "effect";
const EXAMPLE: &str = "example";
const WHAT_HEADING: &str = "forge.ui.explain_what";
const EFFECT_HEADING: &str = "forge.ui.explain_effect";
const EXAMPLE_HEADING: &str = "forge.ui.explain_example";
const TRIGGER_SUFFIX: &str = "trigger";

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

/// A "?" icon showing an [`Explainer`] on hover.
#[derive(IntoElement)]
pub struct ExplainerButton {
    id: SharedString,
    explainer: Explainer,
    theme: ThemeTokens,
}

impl ExplainerButton {
    pub fn new(id: impl Into<SharedString>, explainer: Explainer, theme: &ThemeTokens) -> Self {
        Self { id: id.into(), explainer, theme: theme.clone() }
    }

    /// A button for field `path`, or `None` when the field has no explainer texts.
    pub fn for_field(path: &str, theme: &ThemeTokens) -> Option<Self> {
        Explainer::for_field(path).map(|explainer| Self::new(SharedString::from(format!("{EXPLAIN_ROOT}-{path}")), explainer, theme))
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

fn card(explainer: &Explainer, theme: &ThemeTokens) -> Div {
    div()
        .w(theme.forge_explainer_width)
        .p(theme.spacing_md)
        .flex()
        .flex_col()
        .gap(theme.spacing_sm)
        .child(div().text_size(theme.font_size_sm).font_weight(FontWeight::BOLD).child(explainer.title.clone()))
        .child(section(WHAT_HEADING, explainer.what.clone(), theme))
        .children(explainer.effect.clone().map(|text| section(EFFECT_HEADING, text, theme)))
        .children(explainer.example.clone().map(|text| section(EXAMPLE_HEADING, text, theme)))
}

impl RenderOnce for ExplainerButton {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        let theme = self.theme.clone();
        let trigger = div()
            .id(SharedString::from(format!("{}-{TRIGGER_SUFFIX}", self.id)))
            .flex_shrink_0()
            .cursor_pointer()
            .text_color(theme.text_muted)
            .child(Icon::new(LucideIcon::HelpCircle).size(theme.font_size_sm));
        let explainer = self.explainer;
        HoverCard::new(self.id).trigger(trigger).content(move |_, _, _| card(&explainer, &theme))
    }
}
