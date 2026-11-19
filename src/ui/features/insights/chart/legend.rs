//! Legends whose entries toggle their series (click, or Enter / Space when focused), plus a
//! static swatch row.

use crate::theme::ThemeTokens;
use crate::ui::features::forge::widgets::KeyboardStop;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use std::rc::Rc;

/// A toggle on entry `index`.
pub type ToggleCallback = Rc<dyn Fn(usize, &mut Window, &mut App)>;

/// One legend entry.
#[derive(Debug, Clone, PartialEq)]
pub struct LegendEntry {
    pub colour: Rgba,
    pub label: String,
    /// A count or share shown after the label.
    pub value: Option<String>,
    pub hidden: bool,
}

fn swatch(colour: Rgba, hidden: bool, theme: &ThemeTokens) -> Div {
    let fill = if hidden { theme.bg_transparent } else { colour };
    div().flex_none().size(theme.insights_swatch_size).rounded(theme.radius_xs).bg(fill).border(theme.border_width_hairline).border_color(colour)
}

fn entry_body(entry: &LegendEntry, theme: &ThemeTokens) -> Vec<AnyElement> {
    let text = if entry.hidden { theme.text_muted } else { theme.text_secondary };
    let mut parts = vec![
        swatch(entry.colour, entry.hidden, theme).into_any_element(),
        div().text_size(theme.font_size_xs).text_color(text).when(entry.hidden, |d| d.line_through()).child(entry.label.clone()).into_any_element(),
    ];
    if let Some(value) = &entry.value {
        parts.push(div().text_size(theme.font_size_xs).text_color(theme.text_muted).child(value.clone()).into_any_element());
    }
    parts
}

/// A wrapping row of toggling entries.
pub fn legend(id: &str, entries: &[LegendEntry], on_toggle: Option<ToggleCallback>, theme: &ThemeTokens) -> Div {
    let items = entries.iter().enumerate().map(|(index, entry)| {
        let row = div()
            .id(ElementId::NamedInteger(SharedString::from(id.to_string()), index as u64))
            .flex()
            .items_center()
            .gap(theme.inline_gap)
            .px(theme.spacing_xs)
            .py(theme.spacing_3xs)
            .rounded(theme.radius_sm)
            .border(theme.border_width_hairline)
            .border_color(theme.bg_transparent)
            .children(entry_body(entry, theme));
        match on_toggle.clone() {
            Some(toggle) => {
                let activate = Rc::clone(&toggle);
                row.cursor_pointer()
                    .hover(|s| s.bg(theme.hover_bg))
                    .on_click(move |_, window, cx| toggle(index, window, cx))
                    .keyboard_stop(theme, Rc::new(move |window, cx| activate(index, window, cx)))
                    .into_any_element()
            }
            None => row.into_any_element(),
        }
    });
    div().flex().flex_wrap().items_center().gap(theme.control_gap).children(items)
}
