//! The footer: how changes are saved, the unsaved-AI-changes note, Close and (for the AI
//! provider draft) Save & apply.

use super::super::callbacks::SettingsActions;
use super::super::consts::{CANCEL_BUTTON_ID, SAVE_BUTTON_ID};
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::Icon;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use rust_i18n::t;

/// The footer bar; `show_save` while the AI provider draft is shown or unsaved.
pub(in crate::ui::features::settings) fn render_footer(theme: &ThemeTokens, actions: &SettingsActions, ai_dirty: bool, show_save: bool) -> Div {
    let (on_close, on_save) = (actions.on_close.clone(), actions.on_save.clone());
    let (note, note_color) = match (ai_dirty, show_save) {
        (true, _) => (t!("prefs.ui.ai_unsaved").to_string(), theme.warning),
        (false, true) => (t!("prefs.ui.ai_save_hint").to_string(), theme.text_muted),
        (false, false) => (t!("prefs.ui.autosaved").to_string(), theme.text_muted),
    };
    div()
        .flex_none()
        .h(theme.modal_footer_height)
        .px(theme.panel_padding)
        .border_t(theme.border_width_hairline)
        .border_color(theme.border)
        .bg(theme.bg_elevated)
        .flex()
        .items_center()
        .gap(theme.control_gap)
        .child(
            div()
                .flex_1()
                .min_w_0()
                .flex()
                .items_center()
                .gap(theme.inline_gap)
                .when(ai_dirty, |note| note.child(div().size(theme.dot_size_sm).rounded(theme.radius_full).bg(theme.warning)))
                .child(div().min_w_0().truncate().text_size(theme.font_size_hint).text_color(note_color).child(note)),
        )
        .child(Button::new(CANCEL_BUTTON_ID).ghost().label(t!("prefs.ui.close").to_string()).on_click(move |_, window, cx| on_close(window, cx)))
        .when(show_save || ai_dirty, |footer| {
            footer.child(
                Button::new(SAVE_BUTTON_ID)
                    .primary()
                    .icon(Icon::new(LucideIcon::Check).size(theme.icon_size_md))
                    .label(t!("settings.save_apply").to_string())
                    .on_click(move |_, window, cx| on_save(window, cx)),
            )
        })
}
