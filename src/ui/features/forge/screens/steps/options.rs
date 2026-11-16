//! The right-hand options panel: Mnemo, any notice, then the current step's page.

use super::super::common::message;
use crate::state::forge::ForgeStep;
use crate::ui::features::forge::view::{ForgeHandle, ForgeView, Notice};
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::scroll::ScrollableElement;
use gpui_kit::component::Sizable;
use gpui_kit::*;

const OPTIONS_ID: &str = "forge-options";
const NOTICE_CLOSE_ID: &str = "forge-notice-close";

impl ForgeView {
    pub(super) fn options_panel(&self, handle: &ForgeHandle) -> impl IntoElement {
        let theme = &self.theme;
        div()
            .id(OPTIONS_ID)
            .w(theme.forge_options_width)
            .flex_shrink_0()
            .h_full()
            .overflow_y_scrollbar()
            .p(theme.spacing_lg)
            .flex()
            .flex_col()
            .gap(theme.spacing_lg)
            .bg(theme.bg_sidebar)
            .border_l(theme.border_width_hairline)
            .border_color(theme.border_subtle)
            .children(self.guide_panel(handle))
            .children(self.notice.as_ref().map(|notice| self.notice_box(notice, handle)))
            .child(self.step_page(handle))
    }

    fn step_page(&self, handle: &ForgeHandle) -> AnyElement {
        match self.draft.step() {
            ForgeStep::Origin => self.origin_page(handle),
            ForgeStep::Appearance => self.appearance_page(handle),
            ForgeStep::Voice => self.voice_page(handle),
            ForgeStep::DailyLife => self.daily_life_page(handle),
            ForgeStep::Relationships => self.relationships_page(handle),
            ForgeStep::Grimoire => self.grimoire_page(handle),
            ForgeStep::Review => self.review_page(handle),
            other => self.generic_page(other, handle),
        }
    }

    fn notice_box(&self, notice: &Notice, handle: &ForgeHandle) -> Div {
        let theme = &self.theme;
        let color = if notice.is_error { theme.accent_danger } else { theme.accent_success };
        let dismiss = handle.act(|this, _, cx| {
            this.notice = None;
            cx.notify();
        });
        div()
            .flex()
            .items_start()
            .gap(theme.spacing_sm)
            .p(theme.spacing_md)
            .rounded(theme.radius_lg)
            .border(theme.border_width_hairline)
            .border_color(color)
            .bg(theme.bg_card)
            .child(
                div()
                    .flex_1()
                    .flex()
                    .flex_col()
                    .gap(theme.spacing_2xs)
                    .child(div().text_size(theme.font_size_sm).text_color(color).child(notice.text.clone()))
                    .children(notice.details.iter().map(|line| message(line.clone(), theme.text_muted, theme))),
            )
            .child(Button::new(NOTICE_CLOSE_ID).ghost().xsmall().icon(LucideIcon::X).on_click(move |_, w, cx| dismiss(w, cx)))
    }
}
