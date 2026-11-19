//! The composer's hint row: the draft's atomicity score, its claim count and the first
//! findings, with "Split" (more than one claim) and a dismiss button. Muted on purpose: it
//! informs and never blocks posting.

use super::super::labels::{finding_text, tr};
use super::score_colour;
use crate::domain::thinking::atomic::AtomicReport;
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::*;
use rust_i18n::t;
use std::rc::Rc;

const SPLIT_ID: &str = "atomic-hint-split";
const DISMISS_ID: &str = "atomic-hint-dismiss";
/// Findings named in the row (the sheet lists them all).
const FINDINGS_SHOWN: usize = 2;

/// A hint row action.
pub type HintAction = Rc<dyn Fn(&mut Window, &mut App) + 'static>;

/// The row for `report`; `on_split` is offered when the draft holds several claims.
pub fn render_atomic_hint(report: &AtomicReport, on_split: HintAction, on_dismiss: HintAction, theme: &ThemeTokens) -> Div {
    let claims = report.claims.len();
    let headline = t!("thinking.atomic.hint", score = report.score, count = claims).to_string();
    let findings: Vec<String> = report.findings.iter().take(FINDINGS_SHOWN).map(finding_text).collect();
    let text = div()
        .flex_1()
        .min_w_0()
        .flex()
        .flex_col()
        .gap(theme.spacing_3xs)
        .child(div().text_size(theme.font_size_2xs).font_weight(FontWeight::SEMIBOLD).text_color(score_colour(report.score, theme)).child(headline))
        .children((!findings.is_empty()).then(|| div().text_size(theme.font_size_2xs).line_height(relative(theme.line_height_body)).text_color(theme.text_muted).child(findings.join(" · "))));
    let split = (claims > 1).then(|| Button::new(SPLIT_ID).ghost().xsmall().icon(Icon::new(LucideIcon::Split)).label(tr("thinking.atomic.split_short")).on_click(move |_, window, cx| on_split(window, cx)));
    let dismiss = Button::new(DISMISS_ID).ghost().xsmall().icon(Icon::new(LucideIcon::X)).tooltip(tr("thinking.atomic.dismiss")).on_click(move |_, window, cx| on_dismiss(window, cx));
    div()
        .w_full()
        .flex()
        .items_start()
        .gap(theme.inline_gap)
        .px(theme.spacing_sm)
        .py(theme.spacing_xs)
        .rounded(theme.radius_md)
        .bg(theme.hover_bg)
        .child(Icon::new(LucideIcon::Atom).size(theme.icon_size_sm).text_color(theme.text_muted))
        .child(text)
        .children(split)
        .child(dismiss)
}
