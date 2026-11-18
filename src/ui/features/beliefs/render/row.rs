//! One belief: the proposition and its credence (as a number and a bar), the sparkline of
//! its history, where it came from (a link) and when it last changed, and the controls: pin,
//! edit, forget, and the revision log.

use super::super::data::BeliefRow;
use super::super::events::BeliefsEvent;
use super::super::spark::sparkline;
use super::super::{BeliefEdit, BeliefsPanel};
use super::text::{credence_text, source_text, stamp, tr};
use crate::domain::beliefs::{BeliefAction, BeliefOrigin};
use crate::ui::features::arena::kit::{caption, control_row, icon_button, value_bar};
use crate::ui::icons::LucideIcon;
use gpui_kit::*;

const PIN_PREFIX: &str = "belief-pin-";
const EDIT_PREFIX: &str = "belief-edit-";
const FORGET_PREFIX: &str = "belief-forget-";
const LOG_PREFIX: &str = "belief-log-";
const SOURCE_PREFIX: &str = "belief-source-";

fn id(prefix: &str, index: usize) -> ElementId {
    ElementId::Name(format!("{prefix}{index}").into())
}

impl BeliefsPanel {
    fn row_controls(&self, index: usize, row: &BeliefRow, cx: &mut Context<Self>) -> Div {
        let belief = &row.belief;
        let act = |action: BeliefAction| {
            let entity = cx.entity().downgrade();
            move |_: &ClickEvent, _: &mut Window, cx: &mut App| {
                let _ = entity.update(cx, |_, cx| cx.emit(BeliefsEvent::Act(action.clone())));
            }
        };
        let (pin_icon, pin_tip) = if belief.pinned { (LucideIcon::PinOff, tr("beliefs.unpin")) } else { (LucideIcon::Pin, tr("beliefs.pin")) };
        let (edit_entity, log_entity) = (cx.entity().downgrade(), cx.entity().downgrade());
        let edit = BeliefEdit { id: belief.id.clone(), credence: belief.credence };
        let (proposition, log_id) = (belief.proposition.clone(), belief.id.clone());
        control_row(&self.theme)
            .child(icon_button(id(PIN_PREFIX, index), pin_icon, pin_tip).on_click(act(BeliefAction::TogglePin(belief.id.clone()))))
            .child(icon_button(id(EDIT_PREFIX, index), LucideIcon::Edit, tr("beliefs.edit")).on_click(move |_, window, cx| {
                let _ = edit_entity.update(cx, |panel, cx| panel.start_edit(edit.clone(), proposition.clone(), window, cx));
            }))
            .child(icon_button(id(FORGET_PREFIX, index), LucideIcon::Trash, tr("beliefs.forget")).on_click(act(BeliefAction::Forget(belief.id.clone()))))
            .child(icon_button(id(LOG_PREFIX, index), LucideIcon::ListTree, tr("beliefs.history")).on_click(move |_, _, cx| {
                let _ = log_entity.update(cx, |panel, cx| panel.toggle_history(log_id.clone(), cx));
            }))
    }

    fn source_link(&self, index: usize, row: &BeliefRow, cx: &mut Context<Self>) -> AnyElement {
        let theme = &self.theme;
        let source = row.belief.source.clone();
        let text = source_text(&source);
        if source.origin == BeliefOrigin::User || source.id.is_empty() {
            return caption(text, theme).into_any_element();
        }
        let entity = cx.entity().downgrade();
        div()
            .id(id(SOURCE_PREFIX, index))
            .cursor_pointer()
            .text_size(theme.font_size_xs)
            .text_color(theme.accent_brand)
            .hover(|style| style.underline())
            .on_click(move |_, _, cx| {
                let _ = entity.update(cx, |_, cx| cx.emit(BeliefsEvent::OpenSource(source.clone())));
            })
            .child(text)
            .into_any_element()
    }

    pub(super) fn render_row(&self, index: usize, row: &BeliefRow, cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        let belief = &row.belief;
        let editing = self.editing.as_ref().filter(|edit| edit.id == belief.id).cloned();
        let color = if belief.pinned { theme.accent_dialectic } else { theme.accent_agent };
        let text = div().flex_1().min_w_0().text_size(theme.font_size_sm).line_height(relative(theme.line_height_body)).text_color(theme.text_primary).child(belief.proposition.clone());
        let value = div().text_size(theme.font_size_sm).font_weight(FontWeight::SEMIBOLD).text_color(color).child(credence_text(belief.credence));
        let pinned = belief.pinned.then(|| gpui_kit::component::Icon::new(LucideIcon::Pin).size(theme.icon_size_xs).text_color(theme.accent_dialectic));
        let meta = div()
            .flex()
            .flex_wrap()
            .items_center()
            .gap(theme.control_gap)
            .child(value_bar(belief.credence.unit(), theme.belief_credence_width, color, theme))
            .child(sparkline(row.series.clone(), color, theme))
            .child(self.source_link(index, row, cx))
            .child(caption(stamp(belief.updated_at), theme));
        let body = match editing {
            Some(edit) => self.render_edit(&edit, cx),
            None => self.row_controls(index, row, cx),
        };
        let history = (self.expanded.as_ref() == Some(&belief.id)).then(|| self.render_history(row));
        let head = div().w_full().flex().items_start().gap(theme.control_gap).children(pinned).child(text).child(value);
        crate::ui::features::arena::kit::surface(theme).gap(theme.spacing_xs).child(head).child(meta).child(body).children(history)
    }
}
