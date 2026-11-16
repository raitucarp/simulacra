//! Top bar: close, undo / redo, "Step n of N · title" with the tagline, the guide button,
//! Back / Next.

use super::super::common::muted;
use crate::ui::features::forge::text::{tr, tr_with};
use crate::ui::features::forge::view::{ForgeHandle, ForgeView};
use crate::ui::features::forge::widgets::KeyboardHint;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::Disableable;
use gpui_kit::*;

const CLOSE_ID: &str = "forge-close";
const UNDO_ID: &str = "forge-undo";
const REDO_ID: &str = "forge-redo";
const BACK_ID: &str = "forge-back";
const NEXT_ID: &str = "forge-next";
const CLOSE_KEY: &str = "forge.ui.close";
const UNDO_KEY: &str = "forge.ui.undo";
const REDO_KEY: &str = "forge.ui.redo";
const BACK_KEY: &str = "forge.ui.back";
const NEXT_KEY: &str = "forge.ui.next";
const STEP_OF_KEY: &str = "forge.ui.step_of";
const ARG_CURRENT: &str = "current";
const ARG_TOTAL: &str = "total";

impl ForgeView {
    pub(super) fn top_bar(&self, handle: &ForgeHandle) -> Div {
        let theme = &self.theme;
        div()
            .h(theme.forge_topbar_height)
            .flex_shrink_0()
            .flex()
            .items_center()
            .gap(theme.spacing_md)
            .px(theme.spacing_lg)
            .border_b(theme.border_width_hairline)
            .border_color(theme.border_subtle)
            .bg(theme.bg_sidebar)
            .child(self.history_buttons(handle))
            .child(self.step_title())
            .child(KeyboardHint::button(theme))
            .children(self.guide_show_button(handle))
            .child(self.nav_buttons(handle))
    }

    fn history_buttons(&self, handle: &ForgeHandle) -> Div {
        let (close, undo, redo) = (
            handle.act(ForgeView::close),
            handle.act(|this, _, cx| this.undo(cx)),
            handle.act(|this, _, cx| this.redo(cx)),
        );
        div()
            .flex()
            .items_center()
            .gap(self.theme.spacing_2xs)
            .child(Button::new(CLOSE_ID).ghost().icon(LucideIcon::X).tooltip(tr(CLOSE_KEY)).on_click(move |_, w, cx| close(w, cx)))
            .child(
                Button::new(UNDO_ID)
                    .ghost()
                    .icon(LucideIcon::Undo2)
                    .tooltip(tr(UNDO_KEY))
                    .disabled(!self.draft.can_undo())
                    .on_click(move |_, w, cx| undo(w, cx)),
            )
            .child(
                Button::new(REDO_ID)
                    .ghost()
                    .icon(LucideIcon::Redo2)
                    .tooltip(tr(REDO_KEY))
                    .disabled(!self.draft.can_redo())
                    .on_click(move |_, w, cx| redo(w, cx)),
            )
    }

    fn step_title(&self) -> Div {
        let theme = &self.theme;
        let step = self.draft.step();
        let steps = self.draft.steps();
        let current = steps.iter().position(|s| *s == step).unwrap_or_default() + 1;
        let counter = tr_with(STEP_OF_KEY, &[(ARG_CURRENT, current.to_string()), (ARG_TOTAL, steps.len().to_string())]);
        div()
            .flex_1()
            .min_w_0()
            .flex()
            .flex_col()
            .child(
                div()
                    .flex()
                    .items_baseline()
                    .gap(theme.spacing_sm)
                    .child(div().text_size(theme.font_size_2xs).font_weight(FontWeight::BOLD).text_color(theme.accent_brand).child(counter))
                    .child(div().text_size(theme.font_size_lg).font_weight(FontWeight::BOLD).child(tr(&step.label_key()))),
            )
            .child(muted(tr(&step.tagline_key()), theme).truncate())
    }

    fn nav_buttons(&self, handle: &ForgeHandle) -> Div {
        let step = self.draft.step();
        let kind = self.draft.kind();
        let (back, next) = (handle.act(ForgeView::back), handle.act(ForgeView::next));
        div()
            .flex()
            .items_center()
            .gap(self.theme.spacing_xs)
            .child(
                Button::new(BACK_ID)
                    .outline()
                    .icon(LucideIcon::ArrowLeft)
                    .label(tr(BACK_KEY))
                    .disabled(step.previous(kind).is_none())
                    .on_click(move |_, w, cx| back(w, cx)),
            )
            .children(step.next(kind).map(|_| {
                Button::new(NEXT_ID).primary().icon(LucideIcon::ArrowRight).label(tr(NEXT_KEY)).on_click(move |_, w, cx| next(w, cx))
            }))
    }
}
