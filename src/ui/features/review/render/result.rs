//! A checked drawing: whether the structure matched exactly, precision and recall, the
//! colour legend of the overlay, the grade it earned and the button that moves on.

use super::super::panel::ReviewPanel;
use super::super::text::{grade_label, percent, tr};
use super::common::{caption, control_row, legend_item};
use super::grades::grade_color;
use crate::domain::review::connections::MapScore;
use crate::domain::review::quiz::piece_text;
use crate::domain::review::Grade;
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use rust_i18n::t;

const CONTINUE_ID: &str = "review-drill-continue";

fn headline(score: &MapScore, theme: &ThemeTokens) -> Div {
    let (icon, color, text) = if score.is_exact() {
        (LucideIcon::CircleCheck, theme.accent_success, tr("review.drill.exact"))
    } else {
        let text = t!("review.drill.partial", right = score.correct.len(), total = score.reference(), wrong = score.wrong.len()).to_string();
        (LucideIcon::CircleAlert, theme.accent_danger, text)
    };
    div()
        .flex()
        .items_center()
        .gap(theme.inline_gap)
        .child(Icon::new(icon).size(theme.icon_size_sm).text_color(color))
        .child(div().text_size(theme.font_size_sm).font_weight(FontWeight::SEMIBOLD).text_color(color).child(text))
}

fn legend(theme: &ThemeTokens) -> Div {
    control_row(theme)
        .child(legend_item(theme.accent_success, tr("review.drill.legend_right"), theme))
        .child(legend_item(theme.accent_danger, tr("review.drill.legend_wrong"), theme))
        .child(legend_item(theme.warning, tr("review.drill.legend_missing"), theme))
}

/// Characters of a point's title in a mistake line.
const NAME_CHARS: usize = 40;
/// Mistakes listed at most.
const MAX_MISTAKES: usize = 6;

/// One line per wrong or missing connection: "A — B: wrong direction", "A — B: missing".
fn mistakes(score: &MapScore, titles: &[String], theme: &ThemeTokens) -> Div {
    let name = |i: usize| piece_text(titles.get(i).map_or("", String::as_str), NAME_CHARS);
    let pair = |a: usize, b: usize| format!("{} — {}", name(a), name(b));
    let wrong = score.wrong.iter().map(|(edge, mistake)| (pair(edge.from, edge.to), tr(&format!("review.drill.mistake.{}", mistake.key())), theme.accent_danger));
    let missing = score.missing.iter().map(|edge| (pair(edge.from, edge.to), tr("review.drill.mistake.missing"), theme.warning));
    let rows = wrong.chain(missing).take(MAX_MISTAKES).map(|(pair, why, color)| {
        div().flex().gap(theme.inline_gap).text_size(theme.font_size_xs).child(div().min_w_0().truncate().text_color(theme.text_secondary).child(pair)).child(div().flex_none().text_color(color).child(why))
    });
    div().flex().flex_col().gap(theme.spacing_3xs).children(rows)
}

/// The result block under a checked map (`titles` name its points).
pub fn render_drill_result(score: &MapScore, grade: Grade, titles: &[String], theme: &ThemeTokens, cx: &mut Context<ReviewPanel>) -> Div {
    let measures = t!("review.drill.measures", precision = percent(score.precision()), recall = percent(score.recall())).to_string();
    let graded = div().text_size(theme.font_size_sm).font_weight(FontWeight::SEMIBOLD).text_color(grade_color(grade, theme)).child(t!("review.drill.graded", grade = grade_label(grade)).to_string());
    let next = Button::new(CONTINUE_ID).primary().small().icon(Icon::new(LucideIcon::ArrowRight)).label(tr("review.drill.continue"))
        .on_click(cx.listener(move |this, _, _, cx| this.grade_session(grade, cx)));
    div()
        .flex()
        .flex_col()
        .gap(theme.stack_gap)
        .child(headline(score, theme))
        .child(caption(measures, theme))
        .child(legend(theme))
        .when(!score.is_exact(), |block| block.child(mistakes(score, titles, theme)))
        .child(caption(tr(if score.is_exact() { "review.drill.why_exact" } else { "review.drill.why_again" }), theme))
        .child(control_row(theme).child(graded).child(div().flex_1()).child(next))
}
