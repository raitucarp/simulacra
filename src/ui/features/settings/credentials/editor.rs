//! The add / edit credential form: service chips, name, key and endpoint inputs.

use super::super::panes::common::{field_group, field_header, field_title};
use super::props::{CredentialEditorView, CredentialsPaneProps};
use crate::agent::credentials::CredentialKind;
use crate::theme::ThemeTokens;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::input::{Input, InputState};
use gpui_kit::component::Sizable;
use gpui_kit::{div, Div, Entity, IntoElement, ParentElement, Styled};
use rust_i18n::t;

const KIND_CHIP_PREFIX: &str = "settings-credential-kind-";
const SAVE_BUTTON_ID: &str = "settings-credential-save";
const CANCEL_BUTTON_ID: &str = "settings-credential-cancel";

/// The editor below the list.
pub(super) fn render_editor(theme: &ThemeTokens, editor: &CredentialEditorView, props: &CredentialsPaneProps) -> Div {
    let secret_hint = if editor.is_new { String::new() } else { t!("settings.credentials_secret_keep").to_string() };
    let (on_commit, on_cancel) = (props.on_commit.clone(), props.on_cancel.clone());
    div()
        .flex()
        .flex_col()
        .gap(theme.spacing_md)
        .pt(theme.spacing_sm)
        .border_t(theme.border_width_hairline)
        .border_color(theme.border)
        .child(render_kind_chips(theme, editor.kind, props))
        .child(input_field(theme, t!("settings.credentials_label").to_string(), String::new(), &editor.label))
        .child(input_field(theme, t!("settings.credentials_secret").to_string(), secret_hint, &editor.secret))
        .child(input_field(theme, t!("settings.credentials_endpoint").to_string(), String::new(), &editor.endpoint))
        .child(
            div()
                .flex()
                .justify_end()
                .gap(theme.spacing_sm)
                .child(
                    Button::new(CANCEL_BUTTON_ID)
                        .ghost()
                        .small()
                        .label(t!("settings.credentials_cancel").to_string())
                        .on_click(move |_, window, cx| on_cancel(window, cx)),
                )
                .child(
                    Button::new(SAVE_BUTTON_ID)
                        .primary()
                        .small()
                        .label(t!("settings.credentials_save").to_string())
                        .on_click(move |_, window, cx| on_commit(window, cx)),
                ),
        )
}

/// One chip per credential kind; the chosen one is highlighted.
fn render_kind_chips(theme: &ThemeTokens, chosen: CredentialKind, props: &CredentialsPaneProps) -> Div {
    let chips = CredentialKind::ALL.iter().map(|&kind| {
        let pick = props.on_pick_kind.clone();
        let chip = Button::new(format!("{KIND_CHIP_PREFIX}{}", kind.key())).xsmall().label(kind.label());
        let chip = if kind == chosen { chip.primary() } else { chip.outline() };
        chip.on_click(move |_, window, cx| pick(kind, window, cx)).into_any_element()
    });
    field_group(theme)
        .child(field_title(theme, t!("settings.credentials_kind").to_string()))
        .child(div().flex().flex_wrap().gap(theme.spacing_xs).children(chips))
}

/// A labelled single-line input.
fn input_field(theme: &ThemeTokens, title: String, hint: String, state: &Entity<InputState>) -> Div {
    field_group(theme)
        .child(field_header(theme, title, hint))
        .child(Input::new(state).w_full().h(theme.settings_input_height).text_size(theme.font_size_label))
}
