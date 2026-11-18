//! Every card of the network, next due first: its prompt, kind, due date, tags and source,
//! with suspend / resume and delete (after a confirmation).

use super::super::events::ReviewEvent;
use super::super::panel::ReviewPanel;
use super::super::text::{due_label, kind_label, tr};
use super::common::{caption, surface};
use crate::domain::review::card::card_faces;
use crate::domain::review::{Card, CardId};
use crate::i18n::t_count;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariant, ButtonVariants};
use gpui_kit::component::{Icon, Sizable, WindowExt};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use rust_i18n::t;

const SUSPEND_ID_PREFIX: &str = "review-card-suspend-";
const DELETE_ID_PREFIX: &str = "review-card-delete-";
/// Most cards listed at once.
const MAX_LISTED: usize = 400;
/// Lines of a prompt shown in the list.
const PROMPT_LINES: usize = 2;

impl ReviewPanel {
    pub(super) fn render_cards(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &self.theme;
        if self.data.cards.is_empty() {
            return div().child(self.render_empty_deck(cx));
        }
        let mut cards: Vec<&Card> = self.data.cards.iter().collect();
        cards.sort_by_key(|card| (card.suspended, card.schedule.is_new(), card.schedule.due));
        let rows = cards.into_iter().take(MAX_LISTED).enumerate().map(|(index, card)| self.card_row(index, card, cx));
        div().child(surface(theme).gap(theme.spacing_xs).child(caption(t_count("review.cards.count", self.data.cards.len()), theme)).children(rows))
    }

    fn card_row(&self, index: usize, card: &Card, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &self.theme;
        let prompt = card_faces(&card.kind).prompt_text();
        let source = self.data.source_label(&card.source).unwrap_or_else(|| tr("review.source.note"));
        let meta = std::iter::once(kind_label(card.kind.key()))
            .chain(std::iter::once(if card.suspended { tr("review.cards.suspended") } else { due_label(&card.schedule, self.data.today) }))
            .chain(card.tags.iter().map(|tag| format!("#{tag}")))
            .chain(std::iter::once(source))
            .collect::<Vec<_>>()
            .join(" · ");
        let (id, suspended) = (card.id.clone(), card.suspended);
        let toggle = Button::new(ElementId::NamedInteger(SUSPEND_ID_PREFIX.into(), index as u64))
            .ghost()
            .xsmall()
            .icon(Icon::new(if suspended { LucideIcon::Play } else { LucideIcon::Pause }))
            .tooltip(tr(if suspended { "review.cards.resume" } else { "review.cards.suspend" }))
            .on_click(cx.listener(move |_, _, _, cx| cx.emit(ReviewEvent::SetSuspended(id.clone(), !suspended))));
        div()
            .flex()
            .items_center()
            .gap(theme.control_gap)
            .py(theme.spacing_xs)
            .border_t(theme.border_width_hairline)
            .border_color(theme.border_subtle)
            .when(suspended, |row| row.opacity(theme.vault_muted_opacity))
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .flex()
                    .flex_col()
                    .gap(theme.spacing_3xs)
                    .child(div().text_size(theme.font_size_sm).text_color(theme.text_primary).line_clamp(PROMPT_LINES).child(prompt))
                    .child(div().truncate().text_size(theme.font_size_2xs).text_color(theme.text_muted).child(meta)),
            )
            .child(toggle)
            .child(self.delete_button(index, card.id.clone(), cx))
    }

    fn delete_button(&self, index: usize, id: CardId, cx: &mut Context<Self>) -> impl IntoElement {
        let panel = cx.entity().downgrade();
        Button::new(ElementId::NamedInteger(DELETE_ID_PREFIX.into(), index as u64)).ghost().xsmall().icon(Icon::new(LucideIcon::Trash)).tooltip(tr("review.cards.delete")).on_click(
            move |_, window, cx| {
                let (panel, id) = (panel.clone(), id.clone());
                window.open_alert_dialog(cx, move |alert, _, _| {
                    let (panel, id) = (panel.clone(), id.clone());
                    alert
                        .confirm()
                        .title(tr("review.cards.delete_title"))
                        .description(t!("review.cards.delete_desc").to_string())
                        .ok_text(tr("review.cards.delete"))
                        .ok_variant(ButtonVariant::Danger)
                        .cancel_text(t!("composer.cancel").to_string())
                        .on_ok(move |_, _, cx| {
                            let _ = panel.update(cx, |_, cx| cx.emit(ReviewEvent::DeleteCard(id.clone())));
                            true
                        })
                });
            },
        )
    }
}
