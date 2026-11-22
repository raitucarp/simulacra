//! The composer's symbol picker state (S5): the symbols chosen for the thread's first post
//! (or the edited post), what the picker remembers, the search field and the icon library
//! it searches. Publishing remembers the chosen symbols as the most recent.

use super::super::symbol_picker::{choice_label, SymbolPickerView};
use super::ComposerModalPanel;
use crate::domain::icons::SymbolIndex;
use crate::domain::symbols::{core_choices, search_symbols, PostSymbols, SymbolChoice, SymbolMemory};
use gpui_kit::component::input::InputState;
use gpui_kit::*;
use rust_i18n::t;
use std::rc::Rc;

/// The picker's state.
pub(super) struct SymbolPicker {
    pub chosen: PostSymbols,
    pub memory: SymbolMemory,
    pub search: Entity<InputState>,
    pub searching: bool,
    /// The downloaded icons, searched by name (S15).
    pub library: Rc<SymbolIndex>,
}

impl SymbolPicker {
    pub fn new(window: &mut Window, cx: &mut Context<ComposerModalPanel>) -> Self {
        let placeholder = t!("symbols.picker.search_placeholder").to_string();
        let search = cx.new(|cx| InputState::new(window, cx).placeholder(placeholder));
        Self { chosen: PostSymbols::default(), memory: SymbolMemory::default(), search, searching: false, library: Rc::default() }
    }
}

impl ComposerModalPanel {
    /// Toggles `choice` on the chosen symbols.
    pub(super) fn pick_symbol(&mut self, choice: SymbolChoice, cx: &mut Context<Self>) {
        choice.toggle(&mut self.symbols.chosen);
        cx.notify();
    }

    /// Opens the search field (focused) or closes and empties it.
    pub(super) fn toggle_symbol_search(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.symbols.searching = !self.symbols.searching;
        let searching = self.symbols.searching;
        self.symbols.search.update(cx, |state, cx| match searching {
            true => state.focus(window, cx),
            false => state.set_value("", window, cx),
        });
        cx.notify();
    }

    /// What the picker remembers and the library it searches (from the shell).
    pub fn set_symbol_sources(&mut self, memory: SymbolMemory, library: Rc<SymbolIndex>) {
        self.symbols.memory = memory;
        self.symbols.library = library;
    }

    /// The picker's memory, with `used` remembered as the most recent.
    pub fn remember_symbols(&mut self, used: &PostSymbols) -> SymbolMemory {
        SymbolChoice::of(used).into_iter().rev().for_each(|choice| self.symbols.memory.used(choice));
        self.symbols.memory.clone()
    }

    /// Empties the picker (the memory and library stay).
    pub(super) fn reset_symbols(&mut self, window: &mut Window, cx: &mut App) {
        self.symbols.chosen = PostSymbols::default();
        self.symbols.searching = false;
        self.symbols.search.update(cx, |state, cx| state.set_value("", window, cx));
    }

    pub(super) fn symbol_view(&self, cx: &App) -> SymbolPickerView {
        let core = core_choices();
        let recent = self.symbols.memory.recent.iter().filter(|choice| !core.contains(choice)).cloned().collect();
        let query = self.symbols.search.read(cx).value().to_string();
        let results = if self.symbols.searching { search_symbols(&query, choice_label, &self.symbols.library.search(&query, None)) } else { Vec::new() };
        SymbolPickerView { chosen: self.symbols.chosen.clone(), recent, search: self.symbols.search.clone(), searching: self.symbols.searching, results }
    }
}
