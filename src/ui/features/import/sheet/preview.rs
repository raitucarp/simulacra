//! The virtualized preview of the units: one fixed-height row per unit with its checkbox,
//! position and first line of text.

use super::super::labels::position_label;
use super::{ImportSheet, SheetHandler};
use crate::import::Unit;
use crate::theme::ThemeTokens;
use gpui_kit::component::checkbox::Checkbox;
use gpui_kit::*;
use rust_i18n::t;
use std::rc::Rc;

const LIST_ID: &str = "import-units";
const ROW_ID: &str = "import-unit";
const CHECK_ID: &str = "import-unit-check";
/// Line breaks inside a unit (verse) are shown as this in its one-line preview.
const LINE_BREAK_MARK: &str = " / ";

/// One preview row; a click anywhere on it (the checkbox included) toggles the unit.
fn unit_row(theme: &ThemeTokens, index: usize, unit: &Unit, label: String, checked: bool, toggle: SheetHandler<usize>) -> Stateful<Div> {
    let hover = theme.hover_bg;
    let text_color = if checked { theme.text_primary } else { theme.text_muted };
    let text = div()
        .flex_1()
        .min_w_0()
        .flex()
        .flex_col()
        .child(div().text_size(theme.font_size_micro).text_color(theme.text_muted).truncate().child(label))
        .child(div().text_size(theme.font_size_sm).text_color(text_color).truncate().child(unit.text.replace('\n', LINE_BREAK_MARK)));
    div()
        .id((ROW_ID, index))
        .w_full()
        .h(theme.import_preview_row_height)
        .px(theme.card_padding)
        .flex()
        .items_center()
        .gap(theme.control_gap)
        .border_b(theme.border_width_hairline)
        .border_color(theme.border_subtle)
        .cursor_pointer()
        .hover(move |s| s.bg(hover))
        .on_click(move |_, w, cx| toggle(index, w, cx))
        .child(Checkbox::new((CHECK_ID, index)).checked(checked))
        .child(text)
}

impl ImportSheet {
    pub(super) fn preview(&self) -> AnyElement {
        let theme = self.theme.clone();
        if self.session.units.is_empty() {
            let empty = div().p(theme.card_padding).flex().justify_center();
            return empty.child(crate::ui::components::art_state(crate::ui::components::ArtState::hint(crate::ui::illustration::Illustration::empty(crate::ui::illustration::EmptyKey::ImportNothing).width(theme.art_notice_width), t!("import.no_units").to_string()), &theme)).into_any_element();
        }
        let (units, selected, titles) = (self.session.units.clone(), Rc::new(self.session.selected.clone()), Rc::new(self.session.chapter_titles()));
        let toggle = self.actions.on_toggle.clone();
        let rows = move |range: std::ops::Range<usize>, _: &mut Window, _: &mut App| {
            range
                .map(|index| {
                    let unit = &units[index];
                    let label = position_label(unit.position, titles.get(unit.position.chapter).and_then(|title| title.as_deref()));
                    let checked = selected.get(index).copied().unwrap_or(false);
                    unit_row(&theme, index, unit, label, checked, toggle.clone())
                })
                .collect::<Vec<_>>()
        };
        // Full width, so each row's text is cut with an ellipsis instead of running under the edge.
        let list = uniform_list(LIST_ID, self.session.units.len(), rows).w_full().h(self.theme.import_preview_height);
        div()
            .rounded(self.theme.radius_md)
            .border(self.theme.border_width_hairline)
            .border_color(self.theme.border_subtle)
            .bg(self.theme.bg_card)
            .overflow_hidden()
            .child(list)
            .into_any_element()
    }
}
