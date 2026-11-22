//! The symbol picker of the composer (S5): the core set (task, event, note and the three
//! signifiers) as toggles, the symbols used most recently, and a search over symbol names
//! (the core set's and the downloaded icon library's). Chosen symbols are ringed.

use crate::domain::symbols::{core_choices, PostSymbols, SymbolArt, SymbolChoice, SymbolGlyph};
use crate::theme::ThemeTokens;
use crate::ui::features::symbols::{glyph_label, render_glyph};
use crate::ui::icons::LucideIcon;
use gpui_kit::component::input::{Input, InputState};
use gpui_kit::component::tooltip::Tooltip;
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use rust_i18n::t;
use std::rc::Rc;

/// Toggles a symbol on the thread's first post (or the edited post).
pub type SymbolCallback = Rc<dyn Fn(SymbolChoice, &mut Window, &mut App) + 'static>;

/// Element id prefixes of the choices (suffixed with their place) and of the search toggle.
const CHOICE_ID: &str = "composer-symbol";
const SEARCH_ID: &str = "composer-symbol-search";

/// What the picker shows.
#[derive(Clone)]
pub struct SymbolPickerView {
    pub chosen: PostSymbols,
    /// Recently used symbols outside the core set, newest first.
    pub recent: Vec<SymbolChoice>,
    pub search: Entity<InputState>,
    /// The search field is open; `results` match its text.
    pub searching: bool,
    pub results: Vec<SymbolChoice>,
}

/// The glyph a choice is drawn with.
pub fn choice_glyph(choice: &SymbolChoice) -> SymbolGlyph {
    match choice {
        SymbolChoice::Entry(entry) => SymbolGlyph::Core(SymbolArt::Entry(*entry, crate::domain::symbols::initial_state(*entry))),
        SymbolChoice::Signifier(signifier) => SymbolGlyph::Core(SymbolArt::Signifier(*signifier)),
        SymbolChoice::Custom(symbol) => SymbolGlyph::Custom(symbol.clone()),
    }
}

/// The translated name of a choice.
pub fn choice_label(choice: &SymbolChoice) -> String {
    glyph_label(&choice_glyph(choice))
}

/// The labelled rows: core set, recent symbols and the search toggle, then (while searching)
/// the field and its results.
pub fn symbol_picker(view: &SymbolPickerView, on_pick: SymbolCallback, on_search: Rc<dyn Fn(&mut Window, &mut App)>, theme: &ThemeTokens) -> Div {
    let label = div().text_size(theme.font_size_xs).text_color(theme.text_muted).child(t!("symbols.picker.label").to_string());
    let core = core_choices().into_iter().enumerate().map(|(i, choice)| chip(&choice, i, &view.chosen, on_pick.clone(), theme));
    let offset = core_choices().len();
    let recent = view.recent.iter().enumerate().map(|(i, choice)| chip(choice, offset + i, &view.chosen, on_pick.clone(), theme));
    let toggle = div().id(SEARCH_ID).cursor_pointer().text_color(theme.text_muted).child(Icon::new(LucideIcon::Search).size(theme.icon_size_sm))
        .tooltip(|window, cx| Tooltip::new(t!("symbols.picker.search").to_string()).build(window, cx))
        .on_click(move |_, window, cx| on_search(window, cx));
    let row = div().flex().flex_wrap().items_center().gap(theme.control_gap).child(label).children(core).children(recent).child(toggle);
    div().flex().flex_col().gap(theme.stack_gap).child(row).when(view.searching, |this| this.child(search_rows(view, on_pick, theme)))
}

fn search_rows(view: &SymbolPickerView, on_pick: SymbolCallback, theme: &ThemeTokens) -> Div {
    let offset = core_choices().len() + view.recent.len();
    let results = view.results.iter().enumerate().map(|(i, choice)| chip(choice, offset + i, &view.chosen, on_pick.clone(), theme));
    let empty = view.results.is_empty().then(|| div().text_size(theme.font_size_xs).text_color(theme.text_muted).child(t!("symbols.picker.no_results").to_string()));
    div().flex().flex_col().gap(theme.stack_gap).child(Input::new(&view.search).small().cleanable(true)).child(div().flex().flex_wrap().gap(theme.control_gap).children(results).children(empty))
}

fn chip(choice: &SymbolChoice, place: usize, chosen: &PostSymbols, on_pick: SymbolCallback, theme: &ThemeTokens) -> Stateful<Div> {
    let selected = choice.is_chosen(chosen);
    let (name, picked) = (choice_label(choice), choice.clone());
    div()
        .id(SharedString::from(format!("{CHOICE_ID}-{place}")))
        .flex_none()
        .rounded(theme.radius_full)
        .cursor_pointer()
        .border(if selected { theme.border_width_threadline } else { theme.border_width_hairline })
        .border_color(if selected { theme.border_focus } else { theme.bg_transparent })
        .child(render_glyph(&choice_glyph(choice), theme.icon_size_md, theme))
        .tooltip(move |window, cx| Tooltip::new(name.clone()).build(window, cx))
        .on_click(move |_, window, cx| on_pick(picked.clone(), window, cx))
}

#[cfg(test)]
mod tests {
    use super::{choice_glyph, choice_label};
    use crate::domain::symbols::{core_choices, EntrySymbol, SymbolArt, SymbolChoice, SymbolGlyph, SymbolRef};
    use rust_i18n::t;

    #[test]
    fn every_core_choice_and_the_picker_texts_are_translated() {
        for choice in core_choices() {
            let label = choice_label(&choice);
            assert!(!label.is_empty() && !label.contains("symbols."), "{choice:?}");
        }
        for key in ["symbols.picker.label", "symbols.picker.search", "symbols.picker.search_placeholder", "symbols.picker.no_results"] {
            assert_ne!(t!(key, locale = "en"), key);
        }
        assert_eq!(choice_label(&SymbolChoice::Custom(SymbolRef::new("fi", "zoom-out"))), "zoom out");
        assert_eq!(choice_glyph(&SymbolChoice::Entry(EntrySymbol::Note)), SymbolGlyph::Core(SymbolArt::Entry(EntrySymbol::Note, None)));
    }
}
