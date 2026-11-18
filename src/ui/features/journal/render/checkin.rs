//! The quick check-in: the affect grid beside energy (1–5), sleep, tags, a note, the tick
//! "I'd like pointers to support" (its text shows only when ticked), and Save.

use super::super::panel::JournalPanel;
use super::super::text::{region_label, tr};
use super::common::{block_title, caption, control_row, labelled, line_field, primary_button, surface};
use crate::domain::journal::mood::{ENERGY_MAX, ENERGY_MIN};
use crate::ui::icons::LucideIcon;
use crate::ui::responsive::LayoutClass;
use gpui_kit::component::checkbox::Checkbox;
use gpui_kit::component::Disableable;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;

const ENERGY_ID: &str = "journal-energy";
const SUPPORT_ID: &str = "journal-support";
const SAVE_ID: &str = "journal-checkin-save";

impl JournalPanel {
    pub(super) fn render_checkin(&self, class: LayoutClass, cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        let feeling = self.draft.affect.map(|a| region_label(a.region())).unwrap_or_else(|| tr("journal.checkin.pick"));
        let side = div()
            .w_full()
            .flex_1()
            .min_w_0()
            .flex()
            .flex_col()
            .gap(theme.stack_gap)
            .child(div().text_size(theme.font_size_md).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_primary).child(feeling))
            .child(labelled(tr("journal.checkin.energy"), self.energy_buttons(cx), theme))
            .child(labelled(tr("journal.checkin.sleep"), line_field(&self.fields.sleep, theme), theme))
            .child(labelled(tr("journal.checkin.tags"), line_field(&self.fields.tags, theme), theme))
            .child(labelled(tr("journal.checkin.note"), line_field(&self.fields.note, theme), theme))
            .child(self.support_tick(cx))
            .children(self.draft.wants_support.then(|| self.support_text()))
            .child(control_row(theme).child(self.save_checkin_button(cx)));
        let body = div().w_full().flex().gap(theme.section_gap).when(class.is_compact(), |b| b.flex_col().items_center()).child(self.render_affect_grid(cx)).child(side);
        surface(theme)
            .child(block_title(LucideIcon::HeartPulse, theme.accent_brand, tr("journal.checkin.title"), theme))
            .child(caption(tr("journal.checkin.hint"), theme))
            .child(body)
    }

    fn energy_buttons(&self, cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        let buttons = (ENERGY_MIN..=ENERGY_MAX).map(|level| {
            let active = self.draft.energy == level;
            div()
                .id((ENERGY_ID, level as usize))
                .size(theme.journal_energy_button_size)
                .flex()
                .items_center()
                .justify_center()
                .rounded(theme.radius_md)
                .border(theme.border_width_hairline)
                .border_color(if active { theme.accent_brand } else { theme.border_subtle })
                .bg(if active { theme.accent_brand_subtle } else { theme.bg_app })
                .text_size(theme.font_size_sm)
                .text_color(if active { theme.text_primary } else { theme.text_secondary })
                .cursor_pointer()
                .hover(|b| b.bg(theme.hover_bg))
                .on_click(cx.listener(move |this, _, _, cx| this.set_energy(level, cx)))
                .child(level.to_string())
        });
        div().flex().items_center().gap(theme.spacing_xs).children(buttons)
    }

    fn support_tick(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &self.theme;
        div()
            .id(SUPPORT_ID)
            .flex()
            .items_center()
            .gap(theme.inline_gap)
            .cursor_pointer()
            .on_click(cx.listener(|this, _, _, cx| this.toggle_support(cx)))
            .child(Checkbox::new((SUPPORT_ID, 0usize)).checked(self.draft.wants_support))
            .child(div().text_size(theme.font_size_xs).text_color(theme.text_secondary).child(tr("journal.checkin.support")))
    }

    /// A gentle pointer to help, shown only because the user asked for it.
    pub(super) fn support_text(&self) -> Div {
        let theme = &self.theme;
        div()
            .p(theme.card_padding)
            .rounded(theme.radius_md)
            .bg(theme.accent_success_subtle)
            .border(theme.border_width_hairline)
            .border_color(theme.accent_success_border)
            .flex()
            .items_start()
            .gap(theme.inline_gap)
            .child(gpui_kit::component::Icon::new(LucideIcon::HandHeart).size(theme.icon_size_sm).text_color(theme.accent_success))
            .child(caption(tr("journal.support.text"), theme).flex_1().min_w_0().text_color(theme.text_primary))
    }

    fn save_checkin_button(&self, cx: &mut Context<Self>) -> impl IntoElement {
        primary_button(SAVE_ID, LucideIcon::Check, tr("journal.checkin.save"))
            .disabled(self.draft.affect.is_none())
            .on_click(cx.listener(|this, _, window, cx| this.save_checkin(window, cx)))
    }
}
