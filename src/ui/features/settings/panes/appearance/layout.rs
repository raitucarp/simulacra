//! Typography & layout: interface font family, text size, density and corner radius chips.

use super::super::common::{card, field_title, section, section_label};
use super::consts::{DENSITY_ID_PREFIX, FONT_FAMILIES, FONT_ID_PREFIX, I18N_JOINER, I18N_PREFIX, ID_SEPARATOR, RADIUS_ID_PREFIX, TYPE_SCALE_ID_PREFIX};
use super::handlers::{act, PanelHandle};
use super::AppearancePanel;
use crate::theme::{Density, RadiusScale, ThemeTokens, TypeScale};
use crate::ui::components::{chip_row, choice_chip};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use rust_i18n::t;

/// One labelled row of option chips.
struct ChipGroup<'a, T: Copy + PartialEq + 'static> {
    title: String,
    options: &'a [T],
    current: T,
    id_prefix: &'static str,
    key: fn(T) -> &'static str,
    /// i18n group of the option labels; `None` shows the key itself (font names).
    label_group: Option<&'static str>,
}

impl<T: Copy + PartialEq + 'static> ChipGroup<'_, T> {
    fn render(self, handle: &PanelHandle, theme: &ThemeTokens, pick: fn(&mut AppearancePanel, T, &mut Window, &mut Context<AppearancePanel>)) -> Div {
        let chips = self.options.iter().map(|&option| {
            let key = (self.key)(option);
            let label = match self.label_group {
                Some(group) => {
                    let label_key = format!("{I18N_PREFIX}{group}{I18N_JOINER}{key}");
                    t!(label_key.as_str()).to_string()
                }
                None => key.to_string(),
            };
            let id = SharedString::from(format!("{}{ID_SEPARATOR}{key}", self.id_prefix));
            choice_chip(id, &label, option == self.current, theme, act(handle, move |pane, window, cx| pick(pane, option, window, cx)))
        });
        // Title on the left and choices on the right, like every other settings row.
        div()
            .w_full()
            .flex()
            .items_center()
            .justify_between()
            .gap(theme.spacing_md)
            .py(theme.spacing_sm)
            .child(field_title(theme, self.title))
            .child(chip_row(theme).justify_end().children(chips))
    }
}

/// `rows` set off by hairlines, as in the cards of the settings rows.
fn divided(rows: Vec<Div>, theme: &ThemeTokens) -> Vec<Div> {
    let line = (theme.border_width_hairline, theme.border_subtle);
    rows.into_iter().enumerate().map(|(index, row)| row.when(index > 0, |row| row.border_t(line.0).border_color(line.1))).collect()
}

/// The typography & layout section.
pub(super) fn render(pane: &AppearancePanel, handle: &PanelHandle) -> Div {
    let theme = &pane.tokens;
    let s = &pane.settings;
    let font = FONT_FAMILIES.iter().copied().find(|f| *f == s.font_family).unwrap_or(FONT_FAMILIES[0]);
    let rows = vec![
        ChipGroup { title: t!("appearance.font_family").to_string(), options: &FONT_FAMILIES, current: font, id_prefix: FONT_ID_PREFIX, key: |f| f, label_group: None }
            .render(handle, theme, |pane, family, window, cx| pane.set_font(family, window, cx)),
        ChipGroup { title: t!("appearance.type_scale").to_string(), options: TypeScale::ALL, current: s.type_scale, id_prefix: TYPE_SCALE_ID_PREFIX, key: TypeScale::id, label_group: Some("type_scale") }
            .render(handle, theme, AppearancePanel::set_type_scale),
        ChipGroup { title: t!("appearance.density").to_string(), options: Density::ALL, current: s.density, id_prefix: DENSITY_ID_PREFIX, key: Density::id, label_group: Some("density") }
            .render(handle, theme, AppearancePanel::set_density),
        ChipGroup { title: t!("appearance.radius").to_string(), options: RadiusScale::ALL, current: s.radius_scale, id_prefix: RADIUS_ID_PREFIX, key: RadiusScale::id, label_group: Some("radius") }
            .render(handle, theme, AppearancePanel::set_radius),
        super::panels::render(theme).py(theme.spacing_sm),
    ];
    let body = card(theme).py(theme.spacing_xs).children(divided(rows, theme));
    section(section_label(theme, t!("appearance.layout_title").to_string()), body, theme)
}
