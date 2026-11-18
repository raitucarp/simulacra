//! The dialogue's summary: what was shown, what was missing, and review questions for the
//! gaps, ticked by default, made into review items anchored to the source. A typed concept
//! has no source to anchor to: it offers to write a post on it instead.

use super::super::super::panel::ReviewPanel;
use super::super::super::render::common::{block_title, caption, control_row, surface};
use super::super::super::text::tr;
use super::super::events::LearningEvent;
use super::super::render::{bullet, section_label};
use super::ladder::level_label;
use crate::domain::learning::{TutorSession, TutorSource};
use crate::i18n::t_count;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::{Disableable, Icon, Sizable};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use rust_i18n::t;

const GAP_ID_PREFIX: &str = "learning-tutor-gap-";
const CREATE_ID: &str = "learning-tutor-create-items";
const WRITE_ID: &str = "learning-tutor-write";

impl ReviewPanel {
    pub(super) fn render_summary_card(&self, session: &TutorSession, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &self.theme;
        let Some(summary) = &session.summary else { return surface(theme) };
        let mastery = summary.mastery.map_or_else(|| tr("learning.tutor.mastery_none"), |level| t!("learning.tutor.mastery", level = level_label(level)).to_string());
        let shown = summary.shown.iter().map(|point| bullet(point.clone(), theme.accent_success, theme));
        let missing = summary.missing.iter().map(|point| bullet(point.clone(), theme.warning, theme));
        surface(theme)
            .child(block_title(LucideIcon::ListChecks, theme.accent_dialectic, tr("learning.tutor.summary"), theme))
            .child(caption(mastery, theme))
            .when(!summary.shown.is_empty(), |card| card.child(section_label(tr("learning.tutor.shown"), theme)).children(shown))
            .when(!summary.missing.is_empty(), |card| card.child(section_label(tr("learning.tutor.missing"), theme)).children(missing))
            .child(self.render_gap_offer(session, cx))
    }

    fn render_gap_offer(&self, session: &TutorSession, cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        let questions = session.summary.as_ref().map(|summary| summary.review.clone()).unwrap_or_default();
        if let TutorSource::Concept { name } = &session.source {
            let concept = name.clone();
            let write = Button::new(WRITE_ID).ghost().small().icon(Icon::new(LucideIcon::Feather)).label(tr("learning.gaps.action.write")).on_click(cx.listener(move |this, _, _, cx| this.emit_learning(LearningEvent::WritePost(concept.clone()), cx)));
            return div().flex().flex_col().gap(theme.stack_gap).child(caption(tr("learning.tutor.concept_no_items"), theme)).child(control_row(theme).child(write));
        }
        if session.items_created > 0 {
            return div().child(caption(t_count("learning.tutor.items_made", session.items_created), theme));
        }
        let rows = questions.iter().enumerate().map(|(index, question)| {
            let ticked = !self.learning.tutor_unticked.contains(&index);
            let icon = if ticked { LucideIcon::CircleCheck } else { LucideIcon::Minus };
            div()
                .id(ElementId::NamedInteger(GAP_ID_PREFIX.into(), index as u64))
                .flex()
                .items_start()
                .gap(theme.inline_gap)
                .cursor_pointer()
                .on_click(cx.listener(move |this, _, _, cx| this.toggle_gap(index, cx)))
                .child(Icon::new(icon).size(theme.icon_size_sm).text_color(if ticked { theme.accent_success } else { theme.text_muted }))
                .child(div().flex_1().min_w_0().text_size(theme.font_size_sm).text_color(theme.text_primary).child(question.clone()))
        }).collect::<Vec<_>>();
        let ticked = (0..questions.len()).filter(|i| !self.learning.tutor_unticked.contains(i)).count();
        let create = Button::new(CREATE_ID).primary().small().icon(Icon::new(LucideIcon::GraduationCap)).label(t_count("learning.tutor.create_items", ticked)).disabled(ticked == 0)
            .on_click(cx.listener(|this, _, _, cx| this.create_gap_items(cx)));
        div().flex().flex_col().gap(theme.stack_gap).when(!questions.is_empty(), |offer| {
            offer.child(section_label(tr("learning.tutor.review_offer"), theme)).child(caption(tr("learning.tutor.review_hint"), theme)).children(rows).child(control_row(theme).child(create))
        })
    }
}
