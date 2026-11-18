//! The tutor's start screen: what to talk about (a post or thread, a vault document, a
//! typed concept), then the earlier dialogues to resume.

use super::super::super::panel::ReviewPanel;
use super::super::super::render::common::{block_title, caption, control_row, surface};
use super::super::super::text::tr;
use super::super::render::{chip, framed_input, section_label};
use crate::domain::learning::{TutorPhase, TutorSession, TutorSource};
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use rust_i18n::t;

const PICK_POST_ID: &str = "learning-tutor-pick-post";
const START_CONCEPT_ID: &str = "learning-tutor-start-concept";
const DOCUMENT_ID_PREFIX: &str = "learning-tutor-document-";
const SESSION_ID_PREFIX: &str = "learning-tutor-session-";
/// Documents offered as chips.
const DOCUMENT_CHIPS: usize = 8;
/// Earlier sessions listed.
const LISTED_SESSIONS: usize = 12;

impl ReviewPanel {
    pub(in crate::ui::features::review) fn render_tutor(&self, cx: &mut Context<Self>) -> AnyElement {
        match self.open_session() {
            Some(session) => self.render_dialogue(session, cx).into_any_element(),
            None => div().w_full().max_w(self.theme.review_card_max_width).flex().flex_col().gap(self.theme.section_gap).child(self.render_tutor_start(cx)).child(self.render_sessions(cx)).into_any_element(),
        }
    }

    fn render_tutor_start(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &self.theme;
        let pick = Button::new(PICK_POST_ID).ghost().small().icon(Icon::new(LucideIcon::MessageSquare)).label(tr("learning.tutor.pick_post")).on_click(cx.listener(|this, _, _, cx| this.pick_tutor_post(cx)));
        let start = Button::new(START_CONCEPT_ID).primary().small().icon(Icon::new(LucideIcon::ArrowRight)).label(tr("learning.tutor.start")).on_click(cx.listener(|this, _, window, cx| this.start_tutor_on_concept(window, cx)));
        let documents = self.data.learning.documents.iter().take(DOCUMENT_CHIPS).enumerate().map(|(index, (id, title))| {
            let source = TutorSource::Document { id: id.clone() };
            chip(ElementId::NamedInteger(DOCUMENT_ID_PREFIX.into(), index as u64), title.clone(), false, theme).on_click(cx.listener(move |this, _, _, cx| this.start_tutor(source.clone(), cx)))
        }).collect::<Vec<_>>();
        let has_documents = !self.data.learning.documents.is_empty();
        surface(theme)
            .child(block_title(LucideIcon::HelpCircle, theme.accent_dialectic, tr("learning.tutor.title"), theme))
            .child(caption(tr("learning.tutor.intro"), theme))
            .child(section_label(tr("learning.tutor.on_concept"), theme))
            .child(control_row(theme).child(framed_input(&self.learn_fields.concept, theme)).child(start))
            .child(section_label(tr("learning.tutor.on_post"), theme))
            .child(control_row(theme).child(pick))
            .when(has_documents, |card| card.child(section_label(tr("learning.tutor.on_document"), theme)).child(control_row(theme).children(documents)))
    }

    fn session_line(&self, session: &TutorSession) -> String {
        let state = match session.phase() {
            TutorPhase::Finished => tr("learning.tutor.state.finished"),
            _ => t!("learning.tutor.state.open", answers = session.answered()).to_string(),
        };
        let level = session.mastery().map(|level| tr(&format!("learning.bloom.{}", level.key())));
        std::iter::once(tr(&format!("learning.source.{}", session.source.key()))).chain(std::iter::once(state)).chain(level).collect::<Vec<_>>().join(" · ")
    }

    fn render_sessions(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &self.theme;
        let sessions = &self.data.learning.records.sessions;
        let rows = sessions.iter().take(LISTED_SESSIONS).enumerate().map(|(index, session)| {
            let id = session.id.clone();
            div()
                .id(ElementId::NamedInteger(SESSION_ID_PREFIX.into(), index as u64))
                .flex()
                .flex_col()
                .gap(theme.spacing_3xs)
                .py(theme.spacing_xs)
                .px(theme.spacing_xs)
                .rounded(theme.radius_sm)
                .border_t(theme.border_width_hairline)
                .border_color(theme.border_subtle)
                .cursor_pointer()
                .hover(|row| row.bg(theme.hover_bg))
                .on_click(cx.listener(move |this, _, _, cx| this.show_session(id.clone(), cx)))
                .child(div().truncate().text_size(theme.font_size_sm).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_primary).child(session.title.clone()))
                .child(caption(self.session_line(session), theme))
        }).collect::<Vec<_>>();
        let empty = sessions.is_empty().then(|| caption(tr("learning.tutor.no_sessions"), theme));
        surface(theme).gap(theme.spacing_xs).child(block_title(LucideIcon::RotateCcw, theme.accent_brand, tr("learning.tutor.sessions"), theme)).children(empty).children(rows)
    }
}
