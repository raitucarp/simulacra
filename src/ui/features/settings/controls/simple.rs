//! Small controls: the switch, a text input, the locked "Off" badge, a theme scale readout
//! and the link to the category hosting a pane.

use super::super::catalog::{SettingRow, ThemeScale};
use super::super::row::{element_id_of, RowContext};
use super::super::tabs::SettingsTab;
use crate::domain::preferences::{EmbeddingField, ImageField, SettingField, SettingValue};
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::Button;
use gpui_kit::component::input::Input;
use gpui_kit::component::switch::Switch;
use gpui_kit::component::{Disableable, Icon, Sizable};
use gpui_kit::*;
use super::super::text::tr;
use rust_i18n::t;

/// The theme option label key (`appearance.density_cozy`).
const THEME_I18N_PREFIX: &str = "appearance.";
const THEME_I18N_JOINER: &str = "_";

/// An on/off switch.
pub(super) fn toggle(ctx: &RowContext, row: SettingRow, field: SettingField, on: bool, enabled: bool) -> AnyElement {
    let on_edit = ctx.actions.on_edit.clone();
    Switch::new(element_id_of(row, "switch"))
        .checked(on)
        .color(ctx.theme.accent_brand)
        .disabled(!enabled)
        .on_click(move |checked, window, cx| on_edit(field, SettingValue::Toggle(*checked), window, cx))
        .into_any_element()
}

/// The single-line input of a text preference.
pub(super) fn text_input(ctx: &RowContext, field: SettingField) -> AnyElement {
    let state = match field {
        SettingField::Images(ImageField::StylePrefix) => &ctx.inputs.style_prefix,
        SettingField::Embeddings(EmbeddingField::Endpoint) => &ctx.inputs.embedding_endpoint,
        _ => &ctx.inputs.embedding_model,
    };
    Input::new(state)
        .w(ctx.theme.settings_text_control_width)
        .h(ctx.theme.settings_input_height)
        .text_size(ctx.theme.font_size_label)
        .into_any_element()
}

/// A lock and "Off": shown, never editable.
pub(super) fn locked(ctx: &RowContext) -> AnyElement {
    let theme = ctx.theme;
    div()
        .flex()
        .items_center()
        .gap(theme.inline_gap)
        .px(theme.spacing_sm)
        .py(theme.spacing_2xs)
        .rounded(theme.radius_full)
        .border(theme.border_width_hairline)
        .border_color(theme.accent_success_border)
        .bg(theme.accent_success_subtle)
        .child(Icon::new(LucideIcon::Lock).size(theme.icon_size_xs).text_color(theme.accent_success))
        .child(div().text_size(theme.font_size_hint).text_color(theme.accent_success).child(t!("prefs.ui.always_off").to_string()))
        .into_any_element()
}

/// The current theme scale and a button showing the Appearance pane that edits it.
pub(super) fn theme_readout(ctx: &RowContext, row: SettingRow, scale: ThemeScale) -> AnyElement {
    let (group, id) = match scale {
        ThemeScale::Density => ("density", ctx.data.density.id()),
        ThemeScale::FontSize => ("type_scale", ctx.data.type_scale.id()),
    };
    let label = tr(&format!("{THEME_I18N_PREFIX}{group}{THEME_I18N_JOINER}{id}"));
    div()
        .flex()
        .items_center()
        .gap(ctx.theme.control_gap)
        .child(div().text_size(ctx.theme.font_size_hint).text_color(ctx.theme.text_secondary).child(label))
        .child(go_to(ctx, row, SettingsTab::Appearance))
        .into_any_element()
}

/// "Show": opens `tab` (clearing the search).
pub(super) fn go_to(ctx: &RowContext, row: SettingRow, tab: SettingsTab) -> AnyElement {
    let on_select = ctx.actions.on_select.clone();
    Button::new(element_id_of(row, "show"))
        .outline()
        .small()
        .icon(Icon::new(LucideIcon::ArrowRight).size(ctx.theme.icon_size_sm))
        .label(t!("prefs.ui.show_in", section = tab.title()).to_string())
        .on_click(move |_, window, cx| on_select(tab, window, cx))
        .into_any_element()
}
