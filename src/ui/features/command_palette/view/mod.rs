//! [`CommandPalette`]: the overlay entity. It owns the search field, a snapshot of the items
//! taken when it opened, and the ranked results; it emits [`PaletteEvent`]s that the shell
//! dispatches, so it never touches app state itself.

mod actions;
mod footer;
mod render;
mod rows;
mod semantic;

pub use actions::{ConfirmSelection, DismissPalette, NextGroup, PreviousGroup, SelectNext, SelectPrevious};

use super::model::{
    locate, next_group_start, parse_query, previous_group_start, rank, result_count, stepped, PaletteHistory,
    PaletteItem, PaletteMode, PaletteQuery, PaletteTarget, RankedItem, ResultGroup, SemanticPhase,
};
use crate::theme::ThemeTokens;
use gpui_kit::component::input::{InputEvent, InputState, Position};
use gpui_kit::*;
use rust_i18n::t;

/// What the palette asks the shell to do.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PaletteEvent {
    /// Open or run this target (the palette should then close).
    Run(PaletteTarget),
    /// Close without running anything.
    Dismiss,
    /// Search posts and documents by the meaning of this text (semantic mode); the shell
    /// answers with [`CommandPalette::show_semantic_results`].
    SemanticQuery(String),
}

/// The open command palette.
pub struct CommandPalette {
    input: Entity<InputState>,
    items: Vec<PaletteItem>,
    history: PaletteHistory,
    query: PaletteQuery,
    groups: Vec<ResultGroup>,
    selected: usize,
    /// Semantic mode: the pending query or its scored rows.
    semantic: SemanticPhase,
    theme: ThemeTokens,
    scroll: ScrollHandle,
    _subscription: Subscription,
}

impl EventEmitter<PaletteEvent> for CommandPalette {}

impl CommandPalette {
    /// Opens over `items`, focused, in `mode` (command mode starts the query with `>`,
    /// semantic mode with `?`).
    pub fn new(items: Vec<PaletteItem>, history: PaletteHistory, mode: PaletteMode, theme: ThemeTokens, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let placeholder = t!("palette.placeholder").to_string();
        let initial = mode.initial_text();
        let caret = Position::new(0, initial.chars().count() as u32);
        let input = cx.new(|cx| InputState::new(window, cx).placeholder(placeholder).default_value(initial));
        // Focuses the field with the caret after the `>` prefix.
        input.update(cx, |input, cx| input.set_cursor_position(caret, window, cx));
        let subscription = cx.subscribe(&input, |this, _, event: &InputEvent, cx| {
            if matches!(event, InputEvent::Change) {
                this.refresh(cx);
            }
        });
        let (query, semantic, scroll) = (parse_query(""), SemanticPhase::Prompt, ScrollHandle::new());
        let mut palette = Self { input, items, history, query, groups: Vec::new(), selected: 0, semantic, theme, scroll, _subscription: subscription };
        palette.refresh(cx);
        palette
    }

    /// Re-ranks the results for the current text and selects the best one.
    fn refresh(&mut self, cx: &mut Context<Self>) {
        self.query = parse_query(&self.input.read(cx).value());
        self.groups = match self.query.mode {
            PaletteMode::Semantic => self.semantic_groups(cx),
            PaletteMode::Everything | PaletteMode::Commands => rank(&self.items, &self.query, &self.history),
        };
        self.selected = 0;
        self.scroll.scroll_to_item(0);
        cx.notify();
    }

    /// Selects flat result `index` and keeps it in view.
    fn select(&mut self, index: usize, cx: &mut Context<Self>) {
        self.selected = index;
        if let Some((group, _)) = locate(&self.groups, index) {
            // Each group's heading is one child of the list before its rows.
            self.scroll.scroll_to_item(index + group + 1);
        }
        cx.notify();
    }

    fn select_previous(&mut self, _: &SelectPrevious, _: &mut Window, cx: &mut Context<Self>) {
        self.select(stepped(self.selected, -1, result_count(&self.groups)), cx);
    }

    fn select_next(&mut self, _: &SelectNext, _: &mut Window, cx: &mut Context<Self>) {
        self.select(stepped(self.selected, 1, result_count(&self.groups)), cx);
    }

    fn next_group(&mut self, _: &NextGroup, _: &mut Window, cx: &mut Context<Self>) {
        self.select(next_group_start(&self.groups, self.selected), cx);
    }

    fn previous_group(&mut self, _: &PreviousGroup, _: &mut Window, cx: &mut Context<Self>) {
        self.select(previous_group_start(&self.groups, self.selected), cx);
    }

    fn confirm(&mut self, _: &ConfirmSelection, _: &mut Window, cx: &mut Context<Self>) {
        self.run(self.selected, cx);
    }

    fn dismiss(&mut self, _: &DismissPalette, _: &mut Window, cx: &mut Context<Self>) {
        cx.emit(PaletteEvent::Dismiss);
    }

    /// The item behind a ranked result (a scored row in semantic mode).
    fn item_at(&self, ranked: &RankedItem) -> &PaletteItem {
        match self.query.mode {
            PaletteMode::Semantic => &self.semantic.rows()[ranked.index].item,
            PaletteMode::Everything | PaletteMode::Commands => &self.items[ranked.index],
        }
    }

    /// Runs flat result `index` (Enter or a click).
    fn run(&mut self, index: usize, cx: &mut Context<Self>) {
        let target = locate(&self.groups, index).map(|(group, item)| self.item_at(&self.groups[group].items[item]).target.clone());
        if let Some(target) = target {
            cx.emit(PaletteEvent::Run(target));
        }
    }
}
