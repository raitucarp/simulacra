//! The weakest concepts ranked: the gap as a bar, the weakest dimension, and one-click
//! actions: ask the tutor, import a source, write a post, learn related posts.

use super::super::super::panel::ReviewPanel;
use super::super::super::render::common::{block_title, caption, control_row, progress_bar, surface};
use super::super::super::text::{percent, tr};
use super::super::events::LearningEvent;
use crate::domain::learning::{GapEntry, GapReport, TutorSource};
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use rust_i18n::t;

/// Concepts ranked at most.
pub const RANKED: usize = 8;
const ROW_ID_PREFIX: &str = "learning-gap-row-";
const TUTOR_ID_PREFIX: &str = "learning-gap-tutor-";
const IMPORT_ID_PREFIX: &str = "learning-gap-import-";
const WRITE_ID_PREFIX: &str = "learning-gap-write-";
const RELATED_ID_PREFIX: &str = "learning-gap-related-";

impl ReviewPanel {
    fn gap_actions(&self, index: usize, concept: &str, cx: &mut Context<Self>) -> Div {
        let button = |prefix: &'static str, icon: LucideIcon, label: &str| Button::new(ElementId::NamedInteger(prefix.into(), index as u64)).ghost().xsmall().icon(Icon::new(icon)).label(tr(label));
        let (tutor, import, write, related) = (concept.to_string(), concept.to_string(), concept.to_string(), concept.to_string());
        let ask = button(TUTOR_ID_PREFIX, LucideIcon::HelpCircle, "learning.gaps.action.tutor").on_click(cx.listener(move |this, _, _, cx| this.start_tutor(TutorSource::Concept { name: tutor.clone() }, cx)));
        let source = button(IMPORT_ID_PREFIX, LucideIcon::FileUp, "learning.gaps.action.import").on_click(cx.listener(move |this, _, _, cx| this.emit_learning(LearningEvent::ImportSource(import.clone()), cx)));
        let post = button(WRITE_ID_PREFIX, LucideIcon::Feather, "learning.gaps.action.write").on_click(cx.listener(move |this, _, _, cx| this.emit_learning(LearningEvent::WritePost(write.clone()), cx)));
        let learn = button(RELATED_ID_PREFIX, LucideIcon::GraduationCap, "learning.gaps.action.related").on_click(cx.listener(move |this, _, _, cx| this.emit_learning(LearningEvent::LearnRelated(related.clone()), cx)));
        control_row(&self.theme).child(ask).child(source).child(post).child(learn)
    }

    fn gap_row(&self, index: usize, entry: &GapEntry, cx: &mut Context<Self>) -> Stateful<Div> {
        let theme = &self.theme;
        let selected = self.learning.gaps_selected.as_deref() == Some(entry.concept.as_str()) || (self.learning.gaps_selected.is_none() && index == 0);
        let concept = entry.concept.clone();
        let weakest = tr(&format!("learning.gaps.dimension.{}", entry.weakest().key()));
        let line = t!("learning.gaps.row", gap = percent(entry.gap()), dimension = weakest).to_string();
        let name = div().flex_1().min_w_0().truncate().text_size(theme.font_size_sm).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_primary).child(entry.concept.clone());
        div()
            .id(ElementId::NamedInteger(ROW_ID_PREFIX.into(), index as u64))
            .flex()
            .flex_col()
            .gap(theme.spacing_xs)
            .p(theme.spacing_sm)
            .rounded(theme.radius_md)
            .border(theme.border_width_hairline)
            .border_color(if selected { theme.accent_danger } else { theme.border_subtle })
            .when(selected, |row| row.bg(theme.bg_elevated))
            .cursor_pointer()
            .on_click(cx.listener(move |this, _, _, cx| this.select_concept(concept.clone(), cx)))
            .child(div().flex().items_center().gap(theme.control_gap).child(name).child(div().w(theme.learning_rank_bar_width).flex_none().child(progress_bar(entry.gap(), theme.accent_danger, theme))))
            .child(caption(line, theme))
            .child(self.gap_actions(index, &entry.concept, cx))
    }

    pub(super) fn render_gap_list(&self, report: &GapReport, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &self.theme;
        let rows = report.weakest(RANKED).iter().enumerate().map(|(index, entry)| self.gap_row(index, entry, cx)).collect::<Vec<_>>();
        surface(theme).child(block_title(LucideIcon::ListOrdered, theme.accent_danger, tr("learning.gaps.weakest"), theme)).children(rows)
    }
}
