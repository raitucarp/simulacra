//! The composer's atomic-note hint: the draft's atomicity score, its claim count and its first
//! finding on one line, with "Split" (more than one claim) and a dismiss button. Muted on
//! purpose: it informs and never blocks posting.

use super::super::labels::{finding_text, tr};
use super::score_colour;
use crate::domain::thinking::atomic::{AtomicReport, Finding};
use crate::i18n::plural_key;
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::tooltip::Tooltip;
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use rust_i18n::t;
use std::rc::Rc;

const LABEL_ID: &str = "atomic-hint";
const SPLIT_ID: &str = "atomic-hint-split";
const DISMISS_ID: &str = "atomic-hint-dismiss";
/// Findings named in the row (the sheet lists them all).
const FINDINGS_SHOWN: usize = 2;

/// A hint row action.
pub type HintAction = Rc<dyn Fn(&mut Window, &mut App) + 'static>;

/// The chip for `report`, on one line in the draft's meter row (which is always there, so the
/// hint never pushes the composer down when it appears, Q12): the score and claim count in the
/// score's colour, the first finding the count does not already say (cut to fit, the first ones
/// on hover), "Split" when the draft holds several claims, and a dismiss button.
pub fn render_atomic_hint(report: &AtomicReport, on_split: HintAction, on_dismiss: HintAction, theme: &ThemeTokens) -> Div {
    let claims = report.claims.len();
    let key = plural_key("thinking.atomic.hint", claims);
    let headline = t!(key.as_str(), score = report.score, count = claims).to_string();
    let findings: Vec<String> = report.findings.iter().take(FINDINGS_SHOWN).map(finding_text).collect();
    let first = report.findings.iter().find(|finding| !matches!(finding, Finding::MultipleClaims { .. })).map(finding_text);
    let all = findings.join("
");
    let label = div()
        .id(LABEL_ID)
        .flex_1()
        .min_w_0()
        .overflow_hidden()
        .flex()
        .items_center()
        .gap(theme.spacing_xs)
        .text_size(theme.font_size_2xs)
        .child(Icon::new(LucideIcon::Atom).size(theme.icon_size_sm).text_color(theme.text_muted))
        .child(div().flex_none().font_weight(FontWeight::SEMIBOLD).text_color(score_colour(report.score, theme)).child(headline))
        .children(first.map(|finding| div().min_w_0().truncate().text_color(theme.text_muted).child(finding)))
        .when(!findings.is_empty(), |label| label.tooltip(move |window, cx| Tooltip::new(all.clone()).build(window, cx)));
    let split = (claims > 1).then(|| Button::new(SPLIT_ID).ghost().xsmall().icon(Icon::new(LucideIcon::Split)).label(tr("thinking.atomic.split_short")).on_click(move |_, window, cx| on_split(window, cx)));
    let dismiss = Button::new(DISMISS_ID).ghost().xsmall().icon(Icon::new(LucideIcon::X)).tooltip(tr("thinking.atomic.dismiss")).on_click(move |_, window, cx| on_dismiss(window, cx));
    // Its own width first (it wraps to a line of its own in a wrapping row), shrinking only when
    // even that line is too narrow; then the finding is cut, never the headline under "Split".
    div().flex_auto().min_w_0().flex().items_center().gap(theme.spacing_3xs).child(label).children(split).child(dismiss)
}
