//! A number stepper: minus, the readout in its unit, plus. Each button moves one step and
//! is disabled at its end of the range.

use super::super::catalog::SettingRow;
use super::super::row::{element_id_of, RowContext};
use super::labels::number_readout;
use crate::domain::preferences::{NumberSpec, SettingField, SettingValue};
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::{Disableable, Icon, Sizable};
use gpui_kit::*;
use rust_i18n::t;

/// One step down, one step up.
const STEPS: [(i64, &str, LucideIcon); 2] = [(-1, "down", LucideIcon::Minus), (1, "up", LucideIcon::Plus)];

/// The stepper of a numeric preference.
pub(super) fn render(ctx: &RowContext, row: SettingRow, field: SettingField, spec: NumberSpec, value: i64, enabled: bool) -> AnyElement {
    let theme = ctx.theme;
    let [down, up] = STEPS.map(|(steps, part, icon)| {
        let next = spec.bounds.stepped(value, steps);
        let on_edit = ctx.actions.on_edit.clone();
        let tooltip = if steps < 0 { t!("prefs.ui.decrease") } else { t!("prefs.ui.increase") };
        Button::new(element_id_of(row, part))
            .ghost()
            .xsmall()
            .icon(Icon::new(icon).size(theme.icon_size_sm))
            .tooltip(tooltip.to_string())
            .disabled(!enabled || next == value)
            .on_click(move |_, window, cx| on_edit(field, SettingValue::Number(next), window, cx))
    });
    let readout = div()
        .min_w(theme.settings_number_readout_width)
        .flex()
        .justify_center()
        .text_size(theme.font_size_label)
        .font_weight(FontWeight::MEDIUM)
        .text_color(theme.text_primary)
        .child(number_readout(spec, value, &ctx.data.prefs));
    div()
        .flex()
        .items_center()
        .gap(theme.spacing_2xs)
        .p(theme.spacing_2xs)
        .rounded(theme.radius_md)
        .border(theme.border_width_hairline)
        .border_color(theme.border)
        .bg(theme.input_bg)
        .child(down)
        .child(readout)
        .child(up)
        .into_any_element()
}
