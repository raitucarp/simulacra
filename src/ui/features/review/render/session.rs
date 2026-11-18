//! The focused session: progress, the item (its prompt face, then its answer face) or the
//! drill of a connection map, and the reveal button or the four grade buttons.

use super::super::events::ReviewEvent;
use super::super::panel::ReviewPanel;
use super::super::text::{kind_label, tr};
use super::common::{caption, control_row, progress_bar, surface};
use super::face::{render_face, FaceRole};
use super::grades::render_grade_buttons;
use crate::domain::review::{ReviewItem, Stage};
use crate::ui::icons::LucideIcon;
use crate::ui::keymap::render_key_cap;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use rust_i18n::t;
use std::rc::Rc;

const END_ID: &str = "review-session-end";
const REVEAL_ID: &str = "review-session-reveal";
const SOURCE_ID: &str = "review-item-source";

impl ReviewPanel {
    pub(super) fn render_session(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &self.theme;
        let Some(item) = self.session_item() else { return div().child(self.render_summary(cx)) };
        let (done, planned) = (self.session.done(), self.session.planned.max(1));
        let end = Button::new(END_ID).ghost().xsmall().icon(Icon::new(LucideIcon::X)).label(tr("review.session.end")).on_click(cx.listener(|this, _, _, cx| this.end_session(cx)));
        let top = div()
            .flex()
            .items_center()
            .gap(theme.control_gap)
            .child(caption(t!("review.session.progress", done = done, total = planned, left = self.session.queue.len()).to_string(), theme).flex_none())
            .child(div().flex_1().child(progress_bar(done as f64 / planned as f64, theme.accent_brand, theme)))
            .child(end);
        let revealed = self.session.stage == Stage::Revealed;
        let body = match &self.drill {
            Some(run) => self.render_drill(item, run, cx).into_any_element(),
            None => self.render_item(item, revealed, cx).into_any_element(),
        };
        let bottom = match (revealed, self.drill.is_some()) {
            (_, true) => div().into_any_element(),
            (true, false) => {
                let entity = cx.entity().downgrade();
                let on_grade = Rc::new(move |grade, _: &mut Window, cx: &mut App| {
                    let _ = entity.update(cx, |panel, cx| panel.grade_session(grade, cx));
                });
                render_grade_buttons(self.session_intervals(), None, on_grade, theme).into_any_element()
            }
            (false, false) => self.reveal_row(cx).into_any_element(),
        };
        let width = if self.drill.is_some() { theme.review_page_max_width } else { theme.review_card_max_width };
        div().w_full().max_w(width).mx_auto().flex().flex_col().gap(theme.section_gap).child(top).child(body).child(bottom)
    }

    fn reveal_row(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &self.theme;
        let reveal = Button::new(REVEAL_ID).primary().icon(Icon::new(LucideIcon::Eye)).label(tr("review.session.reveal")).on_click(cx.listener(|this, _, _, cx| this.reveal(cx)));
        control_row(theme).justify_center().child(reveal).child(render_key_cap(tr("keymap.key.space"), theme)).child(caption(tr("review.session.recall_first"), theme))
    }

    /// The item surface: kind and tags, the prompt face, the answer face once revealed, and
    /// a link back to the source.
    pub(super) fn render_item(&self, item: &ReviewItem, revealed: bool, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &self.theme;
        let faces = item.faces();
        let meta = std::iter::once(kind_label(item.subject.key())).chain(item.tags.iter().map(|tag| format!("#{tag}"))).collect::<Vec<_>>().join(" · ");
        surface(theme)
            .min_h(theme.review_card_min_height)
            .max_w(theme.review_card_max_width)
            .mx_auto()
            .p(theme.panel_padding)
            .gap(theme.section_gap)
            .child(caption(meta, theme))
            .child(render_face(&faces.prompt, FaceRole::Prompt, &self.data, theme))
            .when(revealed, |surface| {
                surface
                    .child(div().w_full().h(theme.border_width_hairline).bg(theme.border))
                    .child(render_face(&faces.answer, FaceRole::Answer, &self.data, theme))
            })
            .child(div().flex_1())
            .child(self.render_source_link(item, cx))
    }

    /// "Open the post" / "Open the document" under an item.
    fn render_source_link(&self, item: &ReviewItem, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &self.theme;
        let Some(anchor) = item.subject.anchor().cloned() else { return div().id(SOURCE_ID) };
        let label = match &anchor {
            crate::domain::review::Anchor::Post { .. } => tr("review.source.open_post"),
            crate::domain::review::Anchor::Passage { .. } => tr("review.source.open_document"),
        };
        div()
            .id(SOURCE_ID)
            .flex()
            .items_center()
            .gap(theme.inline_gap)
            .text_size(theme.font_size_xs)
            .text_color(theme.text_muted)
            .cursor_pointer()
            .hover(|chip| chip.text_color(theme.accent_brand))
            .on_click(cx.listener(move |_, _, _, cx| cx.emit(ReviewEvent::OpenAnchor(anchor.clone()))))
            .child(Icon::new(LucideIcon::ExternalLink).size(theme.icon_size_xs))
            .child(label)
    }
}
