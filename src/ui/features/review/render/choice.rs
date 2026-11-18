//! Multiple choice: rate your confidence, pick an option (1–4), see the right one, move on.

use super::super::panel::ReviewPanel;
use super::super::text::tr;
use super::common::{caption, control_row};
use crate::domain::review::quiz::{ChoiceQuestion, ChoiceRun, QuizKind};
use crate::domain::review::Confidence;
use crate::ui::components::choice_chip;
use crate::ui::icons::LucideIcon;
use crate::ui::keymap::render_key_cap;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::Icon;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use rust_i18n::t;

const OPTION_ID_PREFIX: &str = "review-option-";
const CONFIDENCE_ID_PREFIX: &str = "review-confidence-";
const NEXT_ID: &str = "review-choice-next";

impl ReviewPanel {
    pub(super) fn render_choice(&self, run: &ChoiceRun, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &self.theme;
        let total = run.questions.len();
        let Some(question) = run.current() else {
            let score = t!("review.choice.score", right = run.right, total = total).to_string();
            return self.render_quiz_done(QuizKind::Choice, score, cx);
        };
        let Some(item) = self.data.item(&question.item_id) else { return div() };
        let progress = t!("review.choice.progress", index = run.index + 1, total = total, right = run.right).to_string();
        div()
            .w_full()
            .max_w(theme.review_card_max_width)
            .mx_auto()
            .flex()
            .flex_col()
            .gap(theme.section_gap)
            .child(self.quiz_top(progress, run.index as f64 / total.max(1) as f64, cx))
            .child(self.render_item(item, false, cx))
            .child(self.confidence_row(run, cx))
            .child(div().flex().flex_col().gap(theme.spacing_xs).children(question.options.iter().enumerate().map(|(i, option)| self.option_row(question, run.picked, i, option, cx))))
            .when(run.picked.is_some(), |screen| screen.child(self.choice_feedback(question, run.picked, cx)))
    }

    fn confidence_row(&self, run: &ChoiceRun, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &self.theme;
        let chips = Confidence::ALL.into_iter().map(|confidence| {
            let entity = cx.entity().downgrade();
            let label = tr(&format!("review.confidence.{}", confidence.key()));
            choice_chip(ElementId::Name(format!("{CONFIDENCE_ID_PREFIX}{}", confidence.key()).into()), &label, run.confidence == Some(confidence), theme, move |_, cx| {
                let _ = entity.update(cx, |panel, cx| panel.set_confidence(confidence, cx));
            })
        });
        control_row(theme).child(caption(tr("review.confidence.ask"), theme)).children(chips)
    }

    fn option_row(&self, question: &ChoiceQuestion, picked: Option<usize>, index: usize, text: &str, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &self.theme;
        let (is_right, is_picked) = (index == question.correct, picked == Some(index));
        let border = match picked {
            Some(_) if is_right => theme.accent_success,
            Some(_) if is_picked => theme.accent_danger,
            _ => theme.border,
        };
        div()
            .id(ElementId::NamedInteger(OPTION_ID_PREFIX.into(), index as u64))
            .flex()
            .items_center()
            .gap(theme.inline_gap)
            .p(theme.card_padding)
            .rounded(theme.radius_md)
            .border(theme.border_width_hairline)
            .border_color(border)
            .bg(if picked.is_some() && is_right { theme.accent_success_subtle } else { theme.bg_card })
            .when(picked.is_none(), |row| row.cursor_pointer().hover(|row| row.bg(theme.hover_bg)))
            .on_click(cx.listener(move |this, _, _, cx| this.pick_choice(index, cx)))
            .child(render_key_cap((index + 1).to_string(), theme))
            .child(div().flex_1().min_w_0().text_size(theme.font_size_sm).text_color(theme.text_primary).child(text.to_string()))
            .when(picked.is_some() && (is_right || is_picked), |row| {
                let (icon, color) = if is_right { (LucideIcon::CircleCheck, theme.accent_success) } else { (LucideIcon::CircleX, theme.accent_danger) };
                row.child(Icon::new(icon).size(theme.icon_size_sm).text_color(color))
            })
    }

    fn choice_feedback(&self, question: &ChoiceQuestion, picked: Option<usize>, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &self.theme;
        let key = if picked == Some(question.correct) { "review.choice.right" } else { "review.choice.wrong" };
        let next = Button::new(NEXT_ID).primary().icon(Icon::new(LucideIcon::ArrowRight)).label(tr("review.choice.next")).on_click(cx.listener(|this, _, _, cx| this.next_choice(cx)));
        control_row(theme).child(caption(tr(key), theme)).child(div().flex_1()).child(next).child(render_key_cap(tr("keymap.key.space"), theme))
    }
}
