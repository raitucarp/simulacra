//! The credentials list: one row per stored key with edit / remove actions.

use super::super::panes::common::{card, field_title, muted_note, section, section_label, spaced_row};
use super::editor::render_editor;
use super::props::{CredentialIdAction, CredentialRowView, CredentialsPaneProps};
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::{div, Div, FontWeight, IntoElement, ParentElement, Styled};
use rust_i18n::t;

const ADD_BUTTON_ID: &str = "settings-credential-add";
const EDIT_BUTTON_PREFIX: &str = "settings-credential-edit-";
const REMOVE_BUTTON_PREFIX: &str = "settings-credential-remove-";

/// The CREDENTIALS section: header with the add button, the list, and the open editor.
pub fn render_credentials_section(theme: &ThemeTokens, props: CredentialsPaneProps) -> Div {
    let on_add = props.on_add.clone();
    let add = Button::new(ADD_BUTTON_ID)
        .ghost()
        .small()
        .icon(Icon::new(LucideIcon::Plus).size(theme.icon_size_md))
        .label(t!("settings.credentials_add").to_string())
        .on_click(move |_, window, cx| on_add(window, cx));
    let mut body = card(theme).gap(theme.spacing_sm).child(muted_note(theme, t!("settings.credentials_note")));
    if props.rows.is_empty() {
        body = body.child(muted_note(theme, t!("settings.credentials_empty")));
    }
    for row in &props.rows {
        body = body.child(render_row(theme, row, &props.on_edit, &props.on_remove));
    }
    if let Some(editor) = &props.editor {
        body = body.child(render_editor(theme, editor, &props));
    }
    section(spaced_row().child(section_label(theme, t!("settings.credentials_title"))).child(add), body, theme)
}

/// One credential: kind, name, masked key, endpoint and its actions.
fn render_row(theme: &ThemeTokens, row: &CredentialRowView, on_edit: &CredentialIdAction, on_remove: &CredentialIdAction) -> Div {
    let (edit_id, remove_id) = (row.id.clone(), row.id.clone());
    let (on_edit, on_remove) = (on_edit.clone(), on_remove.clone());
    let details = format!("{} · {}", row.kind.label(), row.masked_secret);
    let mut info = div().flex().flex_col().gap(theme.spacing_2xs).child(field_title(theme, row.label.clone())).child(
        div().text_size(theme.font_size_hint).font_weight(FontWeight::NORMAL).text_color(theme.text_secondary).child(details),
    );
    if !row.endpoint.trim().is_empty() {
        info = info.child(muted_note(theme, row.endpoint.clone()));
    }
    let actions = div()
        .flex()
        .gap(theme.spacing_xs)
        .child(
            Button::new(format!("{EDIT_BUTTON_PREFIX}{}", row.id))
                .ghost()
                .xsmall()
                .icon(LucideIcon::Edit)
                .tooltip(t!("settings.credentials_edit").to_string())
                .on_click(move |_, window, cx| on_edit(edit_id.clone(), window, cx)),
        )
        .child(
            Button::new(format!("{REMOVE_BUTTON_PREFIX}{}", row.id))
                .ghost()
                .xsmall()
                .icon(LucideIcon::Trash)
                .tooltip(t!("settings.credentials_remove").to_string())
                .on_click(move |_, window, cx| on_remove(remove_id.clone(), window, cx)),
        );
    spaced_row()
        .py(theme.spacing_xs)
        .border_t(theme.border_width_hairline)
        .border_color(theme.border)
        .child(info)
        .child(actions.into_any_element())
}
