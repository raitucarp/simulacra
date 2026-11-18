//! The focused session: progress, the card (prompt, then the answer), its source, and the
//! reveal button or the four grade buttons.

use super::super::events::ReviewEvent;
use super::super::panel::ReviewPanel;
use super::super::text::{kind_label, tr};
use super::common::{caption, control_row, progress_bar, surface};
use super::face::render_face;
use super::grades::render_grade_buttons;
use crate::domain::review::card::card_faces;
use crate::domain::review::{Card, Stage};
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
const SOURCE_ID: &str = "review-card-source";

impl ReviewPanel {
    pub(super) fn render_session(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &self.theme;
        let Some(card) = self.session_card() else { return div().child(self.render_summary(cx)) };
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
        let bottom = if revealed {
            let entity = cx.entity().downgrade();
            let on_grade = Rc::new(move |grade, _: &mut Window, cx: &mut App| {
                let _ = entity.update(cx, |panel, cx| panel.grade_session(grade, cx));
            });
            render_grade_buttons(self.session_intervals(), None, on_grade, theme).into_any_element()
        } else {
            self.reveal_row(cx).into_any_element()
        };
        div().w_full().max_w(theme.review_card_max_width).mx_auto().flex().flex_col().gap(theme.section_gap).child(top).child(self.render_card(card, revealed, cx)).child(bottom)
    }

    fn reveal_row(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &self.theme;
        let reveal = Button::new(REVEAL_ID).primary().icon(Icon::new(LucideIcon::Eye)).label(tr("review.session.reveal")).on_click(cx.listener(|this, _, _, cx| this.reveal(cx)));
        control_row(theme).justify_center().child(reveal).child(render_key_cap(tr("keymap.key.space"), theme)).child(caption(tr("review.session.recall_first"), theme))
    }

    /// The card surface: kind and tags, the prompt, the answer once revealed, the source.
    pub(super) fn render_card(&self, card: &Card, revealed: bool, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &self.theme;
        let faces = card_faces(&card.kind);
        let meta = std::iter::once(kind_label(card.kind.key())).chain(card.tags.iter().map(|tag| format!("#{tag}"))).collect::<Vec<_>>().join(" · ");
        surface(theme)
            .min_h(theme.review_card_min_height)
            .max_w(theme.review_card_max_width)
            .mx_auto()
            .p(theme.panel_padding)
            .gap(theme.section_gap)
            .child(caption(meta, theme))
            .child(render_face(&faces.prompt, theme.font_size_lg, theme.text_primary, theme))
            .when(revealed, |surface| {
                surface
                    .child(div().w_full().h(theme.border_width_hairline).bg(theme.border))
                    .child(render_face(&faces.answer, theme.font_size_lg, theme.text_primary, theme))
            })
            .child(div().flex_1())
            .child(self.render_source(card, cx))
    }

    /// The cited source as a link back to it (a sentence source is shown, not linked).
    fn render_source(&self, card: &Card, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &self.theme;
        let label = self.data.source_label(&card.source).unwrap_or_else(|| tr("review.source.note"));
        let linked = card.source.owner().is_some();
        let source = card.source.clone();
        div()
            .id(SOURCE_ID)
            .flex()
            .items_center()
            .gap(theme.inline_gap)
            .min_w_0()
            .text_size(theme.font_size_xs)
            .text_color(theme.text_muted)
            .when(linked, |chip| chip.cursor_pointer().hover(|chip| chip.text_color(theme.accent_brand)))
            .when(linked, |chip| chip.on_click(cx.listener(move |_, _, _, cx| cx.emit(ReviewEvent::OpenSource(source.clone())))))
            .child(Icon::new(LucideIcon::Quote).size(theme.icon_size_xs))
            .child(div().truncate().child(t!("review.source.from", source = label).to_string()))
    }
}
