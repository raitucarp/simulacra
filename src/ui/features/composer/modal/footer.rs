//! Modal footer: the format toolbar and post options hint, and the primary post (or
//! "post all") button.

use super::super::consts::MODAL_SUBMIT_POST_BTN_ID;
use super::super::format::format_buttons;
use super::super::free_compose::free_button;
use super::super::preview::preview_button;
use super::super::schedule_picker::schedule_button;
use super::super::symbol_picker::symbol_picker;
use super::super::tint_picker::tint_picker;
use super::super::todo_help::todo_help_button;
use gpui_kit::prelude::FluentBuilder;
use super::ComposerModalView;
use gpui_kit::base::Disableable;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::Sizable;
use gpui_kit::*;
use rust_i18n::t;

/// Id scope of the modal's format buttons.
const MODAL_FORMAT_SCOPE: &str = "modal";

impl ComposerModalView {
    pub(super) fn footer(&self, can_post: bool) -> Div {
        let theme = &self.theme;
        let on_submit_btn = self.callbacks.on_submit.clone();
        let label = self.submit_label();
        div()
            .w_full()
            .px(theme.panel_padding)
            .py(theme.spacing_md)
            .border_t(theme.border_width_hairline)
            .border_color(theme.border)
            .flex()
            .items_center()
            .justify_between()
            .child(self.footer_tools())
            .child(
                Button::new(MODAL_SUBMIT_POST_BTN_ID)
                    .primary()
                    .small()
                    .disabled(!can_post)
                    .label(label.to_string())
                    .on_click(move |_, window, cx| {
                        (on_submit_btn)(window, cx);
                    }),
            )
    }

    /// The primary button's label: save, post (all) or, with a moment picked (Y3), schedule (all).
    fn submit_label(&self) -> std::borrow::Cow<'static, str> {
        let scheduling = self.schedule.as_ref().and_then(|view| view.choice).is_some_and(|choice| choice.is_ok());
        match (self.is_editing(), scheduling, self.drafts.len() > 1) {
            (true, _, _) => t!("data.edit.save"),
            (false, true, true) => t!("composer.schedule_all_button"),
            (false, true, false) => t!("composer.schedule_button"),
            (false, false, true) => t!("composer.post_all_button"),
            (false, false, false) => t!("composer.post_button"),
        }
    }

    /// The tint picker of the thread (sb C3), between the drafts and the footer.
    pub(super) fn tint_row(&self) -> Div {
        let theme = &self.theme;
        div()
            .w_full()
            .px(theme.panel_padding)
            .py(theme.spacing_xs)
            .border_t(theme.border_width_hairline)
            .border_color(theme.border_subtle)
            .child(tint_picker(self.tint, self.callbacks.on_pick_tint.clone(), theme))
    }

    /// The symbol picker of the first post (S5), under the tint row.
    pub(super) fn symbol_row(&self) -> Div {
        let theme = &self.theme;
        let picker = symbol_picker(&self.symbols, self.callbacks.on_pick_symbol.clone(), self.callbacks.on_toggle_symbol_search.clone(), theme);
        div().w_full().px(theme.panel_padding).py(theme.spacing_xs).border_t(theme.border_width_hairline).border_color(theme.border_subtle).child(picker)
    }

    /// The preview toggle, the format toolbar (acting on the last focused draft; hidden while
    /// previewing) and the audience hint.
    fn footer_tools(&self) -> Div {
        let theme = &self.theme;
        let hint = div().text_size(theme.font_size_xs).text_color(theme.text_muted).child(t!("composer.post_options").to_string());
        let toggle = preview_button(MODAL_FORMAT_SCOPE, self.preview, self.callbacks.on_toggle_preview.clone());
        div()
            .flex()
            .items_center()
            .gap(theme.control_gap)
            .child(toggle)
            .when(!self.preview, |row| row.child(format_buttons(MODAL_FORMAT_SCOPE, self.callbacks.on_format.clone(), theme)))
            .when(self.can_schedule(), |row| row.child(schedule_button(self.schedule.is_some(), self.callbacks.on_toggle_schedule.clone())))
            .when(!self.is_editing(), |row| row.child(free_button(self.free.is_some(), self.callbacks.on_toggle_free.clone()))) // T2
            .child(todo_help_button(self.todo_help, self.callbacks.on_toggle_todo_help.clone())) // D1
            .child(hint)
    }
}
