//! The scenario's variables: up to three, each a name with its two to four values typed in
//! one field below it (separated by `|`, the full width so no value is cut off), with buttons
//! to add or remove one.

use super::super::kit::{caption, control_row, ghost_button, line_field, section_label};
use super::super::panel::ArenaPanel;
use super::super::text::tr;
use crate::domain::arena::lab::{MAX_VALUES, MAX_VARIABLES, MIN_VALUES};
use crate::ui::icons::LucideIcon;
use gpui_kit::base::Disableable;
use gpui_kit::*;
use rust_i18n::t;

const ADD_ID: &str = "arena-lab-add-variable";
const REMOVE_ID: &str = "arena-lab-remove-variable";

impl ArenaPanel {
    fn variable_row(&self, index: usize) -> Div {
        let theme = &self.theme;
        let fields = &self.fields.variables[index];
        let name = div().w_full().child(line_field(&fields.name, theme));
        let values = div().w_full().child(line_field(&fields.values, theme));
        div().w_full().flex().flex_col().gap(theme.spacing_2xs).child(name).child(values)
    }

    fn step_variables(&self, id: &'static str, icon: LucideIcon, label: String, delta: i32, enabled: bool, cx: &mut Context<Self>) -> impl IntoElement {
        let entity = cx.entity().downgrade();
        ghost_button(id, icon, label).disabled(!enabled).on_click(move |_, _, cx| {
            let _ = entity.update(cx, |panel, cx| {
                panel.lab = panel.lab.with_variables(delta);
                cx.notify();
            });
        })
    }

    pub(super) fn render_variables(&self, cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        let count = self.lab.variables;
        let rows = (0..count.min(self.fields.variables.len())).map(|index| self.variable_row(index));
        let buttons = control_row(theme)
            .child(self.step_variables(ADD_ID, LucideIcon::Plus, tr("arena.lab.add_variable"), 1, count < MAX_VARIABLES, cx))
            .child(self.step_variables(REMOVE_ID, LucideIcon::Minus, tr("arena.lab.remove_variable"), -1, count > 1, cx));
        div()
            .flex()
            .flex_col()
            .gap(theme.spacing_xs)
            .child(section_label(tr("arena.lab.variables"), theme))
            .child(div().flex().flex_col().gap(theme.stack_gap).children(rows))
            .child(caption(t!("arena.lab.variables_hint", min = MIN_VALUES, max = MAX_VALUES).to_string(), theme))
            .child(buttons)
    }
}
