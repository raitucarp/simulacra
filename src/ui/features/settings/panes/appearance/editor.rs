//! Custom palette editor: duplicate/reset/delete actions, one color picker per base color and
//! the live contrast badges.

use super::super::common::{card, muted_note, section, section_label, spaced_row};
use super::consts::{COLOR_I18N_PREFIX, DELETE_BUTTON_ID, DUPLICATE_BUTTON_ID, RESET_BUTTON_ID};
use super::contrast::{contrast_checks, render_check};
use super::handlers::{act, PanelHandle};
use super::AppearancePanel;
use crate::theme::color::to_hex;
use crate::theme::{PaletteColorKey, PaletteColors, ThemeTokens};
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::color_picker::{ColorPicker, ColorPickerState};
use gpui_kit::component::Sizable;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use rust_i18n::t;

/// The editor section.
pub(super) fn render(pane: &AppearancePanel, handle: &PanelHandle) -> Div {
    let theme = &pane.tokens;
    let palette = pane.settings.effective_palette();
    let editable = pane.settings.active_is_custom();
    let checks = contrast_checks(&palette, theme);
    let body = card(theme)
        .gap(theme.spacing_md)
        .child(actions(pane, handle, editable))
        .when(editable, |body| body.child(color_grid(&pane.pickers, &palette.colors, theme)))
        .child(section_label(theme, t!("appearance.contrast_title").to_string()))
        .children(checks.iter().map(|check| render_check(check, theme)));
    section(section_label(theme, t!("appearance.editor_title").to_string()), body, theme)
}

/// Status line plus Duplicate (always) and Reset / Delete (custom palettes only).
fn actions(pane: &AppearancePanel, handle: &PanelHandle, editable: bool) -> Div {
    let theme = &pane.tokens;
    let status = if editable {
        t!("appearance.editing", name = pane.settings.base_palette().name.as_str()).to_string()
    } else {
        t!("appearance.editor_locked").to_string()
    };
    let button = |id: &'static str, label: String, action: fn(&mut AppearancePanel, &mut Window, &mut Context<AppearancePanel>)| {
        let run = act(handle, action);
        Button::new(id).small().label(label).on_click(move |_, window, cx| run(window, cx))
    };
    // Duplicate is the way forward from a read-only theme; on a custom one it sits level with
    // Reset, and only Delete stands out, in red.
    let duplicate = button(DUPLICATE_BUTTON_ID, t!("appearance.duplicate").to_string(), AppearancePanel::duplicate);
    let buttons = div()
        .flex()
        .gap(theme.spacing_xs)
        .child(if editable { duplicate.outline() } else { duplicate.primary() })
        .when(editable, |row| {
            row.child(button(RESET_BUTTON_ID, t!("appearance.reset").to_string(), AppearancePanel::reset).outline())
                .child(button(DELETE_BUTTON_ID, t!("appearance.delete").to_string(), AppearancePanel::delete_active).danger())
        });
    spaced_row().gap(theme.spacing_md).child(div().flex_1().child(muted_note(theme, status))).child(buttons)
}

/// Columns of the color grid: two even halves of the card.
const COLOR_COLUMNS: u16 = 2;

/// Two-column grid of labelled color pickers with hex readouts, filling the card.
fn color_grid(pickers: &[(PaletteColorKey, Entity<ColorPickerState>)], colors: &PaletteColors, theme: &ThemeTokens) -> Div {
    let fields = pickers.iter().map(|(key, state)| {
        let label_key = format!("{COLOR_I18N_PREFIX}{}", key.id());
        let label = t!(label_key.as_str()).to_string();
        div()
            .w_full()
            .min_w_0()
            .flex()
            .items_center()
            .gap(theme.spacing_sm)
            .child(ColorPicker::new(state).small())
            .child(div().flex_1().truncate().text_size(theme.font_size_xs).text_color(theme.text_secondary).child(label))
            .child(
                div()
                    .w(theme.hex_readout_width)
                    .text_size(theme.font_size_xs)
                    .font_family(theme.font_family_mono.clone())
                    .text_color(theme.text_muted)
                    .child(to_hex(key.get(colors))),
            )
    });
    div().w_full().grid().grid_cols(COLOR_COLUMNS).gap_x(theme.section_gap).gap_y(theme.spacing_sm).children(fields)
}
