//! Free recall: type the answer, compare it with the item's (a post or passage is compared
//! in your own words), then grade yourself (the comparison only suggests a grade).

use super::super::panel::ReviewPanel;
use super::super::text::tr;
use super::common::{caption, control_row, surface};
use super::face::{render_face, FaceRole};
use super::grades::{grade_color, render_grade_buttons};
use crate::domain::review::quiz::{QuizKind, RecallRun, RecallVerdict};
use crate::domain::review::schedule::preview;
use crate::domain::review::{Grade, ReviewItem};
use crate::ui::icons::LucideIcon;
use crate::ui::keymap::render_key_cap;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::input::Input;
use gpui_kit::component::Icon;
use gpui_kit::*;
use rust_i18n::t;
use std::rc::Rc;

const CHECK_ID: &str = "review-recall-check";

impl ReviewPanel {
    pub(super) fn render_recall(&self, run: &RecallRun, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &self.theme;
        let total = run.items.len();
        let Some(item) = run.current().and_then(|id| self.data.item(id)) else {
            let recalled = run.grades.iter().filter(|grade| grade.is_recall()).count();
            return self.render_quiz_done(QuizKind::Recall, t!("review.recall.score", right = recalled, total = total).to_string(), cx);
        };
        let progress = t!("review.recall.progress", index = run.index + 1, total = total).to_string();
        let below = match &run.checked {
            None => self.recall_input(cx).into_any_element(),
            Some((typed, verdict)) => self.recall_result(item, typed, *verdict, cx).into_any_element(),
        };
        div()
            .w_full()
            .max_w(theme.review_card_max_width)
            .mx_auto()
            .flex()
            .flex_col()
            .gap(theme.section_gap)
            .child(self.quiz_top(progress, run.index as f64 / total.max(1) as f64, cx))
            .child(self.render_item(item, false, cx))
            .child(below)
    }

    fn recall_input(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &self.theme;
        let field = div()
            .flex_1()
            .min_w_0()
            .rounded(theme.radius_md)
            .border(theme.border_width_hairline)
            .border_color(theme.border)
            .bg(theme.input_bg)
            .px(theme.spacing_sm)
            .child(Input::new(&self.fields.answer).bordered(false));
        let check = Button::new(CHECK_ID).primary().icon(Icon::new(LucideIcon::Check)).label(tr("review.recall.check")).on_click(cx.listener(|this, _, window, cx| this.check_recall(window, cx)));
        div()
            .flex()
            .flex_col()
            .gap(theme.spacing_xs)
            .child(div().flex().items_center().gap(theme.control_gap).child(field).child(check))
            .child(control_row(theme).child(render_key_cap(tr("keymap.key.enter"), theme)).child(caption(tr("review.recall.hint"), theme)))
    }

    fn recall_result(&self, item: &ReviewItem, typed: &str, verdict: RecallVerdict, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &self.theme;
        let suggested = verdict.suggested_grade();
        let verdict_line = div()
            .flex()
            .items_center()
            .gap(theme.inline_gap)
            .child(Icon::new(if verdict == RecallVerdict::Wrong { LucideIcon::CircleX } else { LucideIcon::CircleCheck }).size(theme.icon_size_sm).text_color(grade_color(suggested, theme)))
            .child(div().text_size(theme.font_size_sm).font_weight(FontWeight::SEMIBOLD).text_color(grade_color(suggested, theme)).child(tr(&format!("review.recall.verdict.{}", verdict.key()))));
        let typed = if typed.is_empty() { tr("review.recall.blank") } else { typed.to_string() };
        let entity = cx.entity().downgrade();
        let on_grade = Rc::new(move |grade: Grade, window: &mut Window, cx: &mut App| {
            let _ = entity.update(cx, |panel, cx| panel.grade_recall(grade, window, cx));
        });
        let intervals = preview(&item.schedule, self.data.today, &self.data.policy);
        surface(theme)
            .child(verdict_line)
            .child(caption(t!("review.recall.you_typed", answer = typed).to_string(), theme))
            .child(render_face(&item.faces().answer, FaceRole::Answer, &self.data, theme))
            .child(caption(tr("review.recall.grade_yourself"), theme))
            .child(render_grade_buttons(Some(intervals), Some(suggested), on_grade, theme))
    }
}
