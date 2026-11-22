//! The control on the right of a setting row, one module per kind.

mod action;
mod choice;
mod labels;
mod number;
pub(super) mod path;
mod simple;

use super::catalog::{EmbeddedPane, SettingRow};
use super::row::RowContext;
use super::tabs::SettingsTab;
use crate::domain::preferences::{FieldKind, SettingField, SettingValue};
use gpui_kit::{div, AnyElement, IntoElement};

/// The control of `row` (`enabled == false` renders it inert).
pub(super) fn render_control(ctx: &RowContext, row: SettingRow, enabled: bool) -> AnyElement {
    match row {
        SettingRow::Field(field) => field_control(ctx, row, field, enabled),
        SettingRow::Theme(scale) => simple::theme_readout(ctx, row, scale),
        SettingRow::Action(command) => action::render(ctx, row, command),
        SettingRow::Path(folder) => path::render(ctx, row, folder),
        SettingRow::Embedded(pane) => simple::go_to(ctx, row, home_tab(pane)),
    }
}

/// The editor of a typed preference.
fn field_control(ctx: &RowContext, row: SettingRow, field: SettingField, enabled: bool) -> AnyElement {
    match (field.kind(), field.get(&ctx.data.prefs)) {
        (FieldKind::Toggle, SettingValue::Toggle(on)) => simple::toggle(ctx, row, field, on, enabled),
        (FieldKind::Choice(keys), SettingValue::Choice(current)) => choice::render(ctx, row, field, keys, current, enabled),
        (FieldKind::Number(spec), SettingValue::Number(value)) => number::render(ctx, row, field, spec, value, enabled),
        (FieldKind::Persona, SettingValue::Text(id)) => choice::persona(ctx, row, field, &id),
        (FieldKind::Text, _) => simple::text_input(ctx, field),
        (FieldKind::Locked, _) => simple::locked(ctx),
        _ => div().into_any_element(),
    }
}

/// The category that hosts a pane (search results link to it).
const fn home_tab(pane: EmbeddedPane) -> SettingsTab {
    match pane {
        EmbeddedPane::AiProvider | EmbeddedPane::Credentials => SettingsTab::AiIntelligence,
        EmbeddedPane::SchedulerStatus => SettingsTab::Agents,
        EmbeddedPane::Themes => SettingsTab::Appearance,
        EmbeddedPane::SymbolSets => SettingsTab::Symbols,
        EmbeddedPane::Licences => SettingsTab::Licences,
    }
}
