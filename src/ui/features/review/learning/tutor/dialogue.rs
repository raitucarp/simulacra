//! A dialogue in progress: the ladder, each question as a tutor bubble (its level and move),
//! each answer as the learner's bubble with its assessment, then the answer box, the request
//! status or the summary.

use super::super::super::panel::ReviewPanel;
use super::super::super::render::common::{caption, control_row, surface};
use super::super::super::text::tr;
use super::super::events::LearningEvent;
use super::super::render::bullet;
use super::ladder::{level_label, render_ladder, verdict_color};
use crate::domain::learning::{Exchange, TutorPhase, TutorSession};
use crate::ui::icons::LucideIcon;
use crate::ui::keymap::render_key_cap;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::input::Textarea;
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use crate::i18n::t_count;
use rust_i18n::t;

const BACK_ID: &str = "learning-tutor-back";
const SEND_ID: &str = "learning-tutor-send";
const FINISH_ID: &str = "learning-tutor-finish";
const RETRY_ID: &str = "learning-tutor-retry";
const CANCEL_ID: &str = "learning-tutor-cancel";
const DELETE_ID: &str = "learning-tutor-delete";
const EARLIER_ID: &str = "learning-tutor-earlier";
/// Exchanges shown before "earlier questions" is opened.
const LATEST_EXCHANGES: usize = 2;

impl ReviewPanel {
    pub(super) fn render_dialogue(&self, session: &TutorSession, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &self.theme;
        let back = Button::new(BACK_ID).ghost().xsmall().icon(Icon::new(LucideIcon::ArrowLeft)).label(tr("learning.tutor.all_sessions")).on_click(cx.listener(|this, _, _, cx| {
            this.learning.tutor_open = None;
            cx.notify();
        }));
        let id = session.id.clone();
        let delete = Button::new(DELETE_ID).ghost().xsmall().icon(Icon::new(LucideIcon::Trash)).tooltip(tr("learning.tutor.delete")).on_click(cx.listener(move |this, _, _, cx| {
            this.learning.tutor_open = None;
            this.emit_learning(LearningEvent::DeleteSession(id.clone()), cx);
        }));
        let aimed = (session.phase() == TutorPhase::AwaitingAnswer).then(|| session.exchanges.last().map(|e| e.question.level)).flatten();
        let head = surface(theme)
            .child(control_row(theme).child(back).child(div().flex_1()).child(delete))
            .child(div().font_family(theme.font_family_display.clone()).text_size(theme.font_size_lg).font_weight(FontWeight::BOLD).text_color(theme.text_primary).child(session.title.clone()))
            .child(caption(tr(&format!("learning.source.{}", session.source.key())), theme))
            .child(render_ladder(session.mastery(), aimed, theme));
        let hidden = if self.learning.tutor_all_exchanges { 0 } else { session.exchanges.len().saturating_sub(LATEST_EXCHANGES) };
        let earlier = (hidden > 0).then(|| {
            Button::new(EARLIER_ID).ghost().xsmall().icon(Icon::new(LucideIcon::ChevronUp)).label(t_count("learning.tutor.earlier", hidden)).on_click(cx.listener(|this, _, _, cx| {
                this.learning.tutor_all_exchanges = true;
                cx.notify();
            }))
        });
        let exchanges = session.exchanges.iter().skip(hidden).map(|exchange| self.render_exchange(exchange));
        let below = match session.phase() {
            TutorPhase::Finished => self.render_summary_card(session, cx).into_any_element(),
            TutorPhase::AwaitingAnswer if !self.data.learning.tutor.is_running(&session.id) => self.render_answer_box(session, cx).into_any_element(),
            _ => self.render_request_status(session, cx).into_any_element(),
        };
        let column = div().w_full().max_w(theme.review_card_max_width).flex().flex_col().gap(theme.section_gap).child(head);
        match session.phase() == TutorPhase::Finished {
            true => column.child(below).children(earlier).children(exchanges),
            false => column.children(earlier).children(exchanges).child(below),
        }
    }

    fn bubble(&self, text: String, meta: String, mine: bool) -> Div {
        let theme = &self.theme;
        let body = div()
            .max_w(theme.learning_bubble_max_width)
            .flex()
            .flex_col()
            .gap(theme.spacing_3xs)
            .p(theme.card_padding)
            .rounded(theme.radius_lg)
            .when(mine, |b| b.bg(theme.accent_brand_subtle).border(theme.border_width_hairline).border_color(theme.accent_brand_border))
            .when(!mine, |b| b.bg(theme.accent_dialectic_subtle).border_l(theme.border_width_threadline).border_color(theme.accent_dialectic))
            .child(div().text_size(theme.font_size_2xs).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_muted).child(meta))
            .child(div().text_size(theme.font_size_sm).line_height(relative(theme.line_height_reading)).text_color(theme.text_primary).child(text));
        div().w_full().flex().when(mine, |row| row.justify_end()).child(body)
    }

    fn render_exchange(&self, exchange: &Exchange) -> Div {
        let theme = &self.theme;
        let question = &exchange.question;
        let meta = format!("{} · {}", level_label(question.level), tr(&format!("learning.move.{}", question.kind.key())));
        let mut column = div().w_full().flex().flex_col().gap(theme.stack_gap).child(self.bubble(question.text.clone(), meta, false));
        if let Some(answer) = &exchange.answer {
            column = column.child(self.bubble(answer.clone(), tr("learning.tutor.you"), true));
        }
        if let Some(assessment) = &exchange.assessment {
            let color = verdict_color(assessment.verdict, theme);
            let verdict = div().text_size(theme.font_size_xs).font_weight(FontWeight::SEMIBOLD).text_color(color).child(tr(&format!("learning.verdict.{}", assessment.verdict.key())));
            let reached = assessment.reached.map(|level| caption(t!("learning.tutor.reached", level = level_label(level)).to_string(), theme));
            let right = assessment.right.iter().map(|point| bullet(point.clone(), theme.accent_success, theme));
            let wrong = assessment.misconceptions.iter().map(|point| bullet(point.clone(), theme.accent_danger, theme));
            column = column.child(div().pl(theme.card_padding).flex().flex_col().gap(theme.spacing_xs).child(control_row(theme).child(verdict).children(reached)).children(right).children(wrong));
        }
        column
    }

    fn render_answer_box(&self, session: &TutorSession, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &self.theme;
        let field = div().w_full().min_h(theme.learning_answer_min_height).rounded(theme.radius_md).border(theme.border_width_hairline).border_color(theme.border).bg(theme.input_bg).p(theme.spacing_sm).child(Textarea::new(&self.learn_fields.answer).bordered(false));
        let send = Button::new(SEND_ID).primary().small().icon(Icon::new(LucideIcon::Send)).label(tr("learning.tutor.send")).on_click(cx.listener(|this, _, window, cx| this.send_answer(window, cx)));
        let id = session.id.clone();
        let finish = Button::new(FINISH_ID).ghost().small().icon(Icon::new(LucideIcon::Check)).label(tr("learning.tutor.finish")).on_click(cx.listener(move |this, _, _, cx| this.emit_learning(LearningEvent::CloseTutor(id.clone()), cx)));
        let can_finish = session.answered() > 0;
        let keys = control_row(theme).child(render_key_cap(tr("keymap.key.ctrl"), theme)).child(render_key_cap(tr("keymap.key.enter"), theme));
        surface(theme).child(caption(tr("learning.tutor.answer_tip"), theme)).child(field).child(control_row(theme).child(send).child(keys).when(can_finish, |row| row.child(div().flex_1()).child(finish)))
    }

    fn render_request_status(&self, session: &TutorSession, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &self.theme;
        let id = session.id.clone();
        let retry = |label: &str| Button::new(RETRY_ID).primary().small().icon(Icon::new(LucideIcon::RefreshCw)).label(tr(label));
        let status = &self.data.learning.tutor;
        match (status.failure(&session.id), status.is_running(&session.id)) {
            (Some(message), _) => {
                let retry = retry("learning.tutor.retry").on_click(cx.listener(move |this, _, _, cx| this.emit_learning(LearningEvent::RetryTutor(id.clone()), cx)));
                surface(theme).child(div().text_size(theme.font_size_sm).text_color(theme.accent_danger).child(message.to_string())).child(control_row(theme).child(retry))
            }
            (None, true) => {
                let cancel = Button::new(CANCEL_ID).ghost().xsmall().icon(Icon::new(LucideIcon::X)).label(tr("learning.tutor.cancel")).on_click(cx.listener(|this, _, _, cx| this.emit_learning(LearningEvent::CancelTutor, cx)));
                let key = if session.phase() == TutorPhase::NeedsSummary { "learning.tutor.summarising" } else { "learning.tutor.thinking" };
                surface(theme).child(control_row(theme).child(Icon::new(LucideIcon::Sparkles).size(theme.icon_size_sm).text_color(theme.accent_dialectic)).child(caption(tr(key), theme)).child(div().flex_1()).child(cancel))
            }
            (None, false) => {
                let resume = retry("learning.tutor.resume").on_click(cx.listener(move |this, _, _, cx| this.emit_learning(LearningEvent::RetryTutor(id.clone()), cx)));
                surface(theme).child(caption(tr("learning.tutor.paused"), theme)).child(control_row(theme).child(resume))
            }
        }
    }
}
