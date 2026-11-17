//! An editable list of HTTP headers (custom requests, custom image requests, custom tools):
//! one name / value row each, a remove button per row and "Add header".

use super::kit::engine_id;
use crate::domain::engine::{HeaderPair, API_KEY_PLACEHOLDER};
use crate::domain::EngineConfig;
use crate::state::forge::EngineSection;
use crate::ui::features::forge::text::tr;
use crate::ui::features::forge::view::{ForgeHandle, ForgeView};
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::Sizable;
use gpui_kit::*;

const ADD_KEY: &str = "forge.ui.engine.header_add";
const NAME_KEY: &str = "forge.ui.engine.header_name";
const VALUE_KEY: &str = "forge.ui.engine.header_value";
const EMPTY_KEY: &str = "forge.ui.engine.headers_empty";
const KEY_HINT_KEY: &str = "forge.ui.engine.headers_key_hint";

/// Finds the header list being edited (`index` picks a custom tool, else unused).
pub(super) type HeaderAccess = fn(&mut EngineConfig, usize) -> Option<&mut Vec<HeaderPair>>;

impl ForgeView {
    /// Rows of `headers`, which `access` finds again on the engine copy of each edit.
    pub(super) fn headers_editor(&self, name: &str, headers: &[HeaderPair], (access, index): (HeaderAccess, usize), handle: (EngineSection, &ForgeHandle)) -> Div {
        let theme = &self.theme;
        let rows: Vec<Div> = headers.iter().enumerate().map(|(row, header)| self.header_row(name, (row, header), (access, index), handle)).collect();
        let empty = rows.is_empty().then(|| div().text_size(theme.font_size_xs).text_color(theme.text_muted).child(tr(EMPTY_KEY)));
        let add = handle.1.engine(move |e: &mut EngineConfig, ()| {
            if let Some(list) = access(e, index) {
                list.push(HeaderPair::default());
            }
        });
        let button = Button::new(engine_id(format!("{name}-header-add"))).outline().xsmall().icon(LucideIcon::Plus).label(tr(ADD_KEY));
        let hint = div().text_size(theme.font_size_2xs).text_color(theme.text_muted).child(format!("{} {API_KEY_PLACEHOLDER}", tr(KEY_HINT_KEY)));
        div()
            .flex()
            .flex_col()
            .gap(theme.spacing_xs)
            .children(rows)
            .children(empty)
            .child(div().flex().items_center().gap(theme.spacing_sm).child(button.on_click(move |_, w, cx| add((), w, cx))).child(hint))
    }

    fn header_row(&self, name: &str, (row, header): (usize, &HeaderPair), (access, index): (HeaderAccess, usize), (section, handle): (EngineSection, &ForgeHandle)) -> Div {
        let theme = &self.theme;
        let key = self.engine_input(format!("{name}-header-{row}-name"), &header.name, (section, handle), move |e, text| {
            if let Some(pair) = access(e, index).and_then(|list| list.get_mut(row)) {
                pair.name = text;
            }
        });
        let value = self.engine_input(format!("{name}-header-{row}-value"), &header.value, (section, handle), move |e, text| {
            if let Some(pair) = access(e, index).and_then(|list| list.get_mut(row)) {
                pair.value = text;
            }
        });
        let remove = handle.engine(move |e: &mut EngineConfig, ()| {
            if let Some(list) = access(e, index).filter(|list| row < list.len()) {
                list.remove(row);
            }
        });
        let button = Button::new(engine_id(format!("{name}-header-{row}-remove"))).ghost().xsmall().icon(LucideIcon::X);
        div()
            .flex()
            .items_center()
            .gap(theme.spacing_xs)
            .child(div().flex_1().min_w_0().child(key.placeholder(tr(NAME_KEY))))
            .child(div().flex_1().min_w_0().child(value.placeholder(tr(VALUE_KEY))))
            .child(button.on_click(move |_, w, cx| remove((), w, cx)))
    }
}
