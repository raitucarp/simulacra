//! Appearance pane: searchable theme gallery with live previews, the custom palette editor
//! (color pickers + contrast badges), typography/layout scales and JSON import/export.
//!
//! The pane is its own entity holding a working copy of [`ThemeSettings`]; every change is
//! emitted as [`AppearanceEvent::Changed`] and applied (persisted, derived, kit-synced) by the
//! root view, so edits are live.

mod consts;
mod contrast;
mod editor;
mod filter;
mod gallery;
mod handlers;
mod layout;
mod pickers;
mod preview_card;
mod transfer;

#[cfg(test)]
mod tests;

use super::common::section;
use crate::theme::{PaletteColorKey, ThemeSettings, ThemeTokens};
use filter::ModeFilter;
use gpui_kit::component::color_picker::ColorPickerState;
use gpui_kit::component::input::InputState;
use gpui_kit::*;
use rust_i18n::t;

/// Emitted whenever the user changes the theme from this pane.
pub enum AppearanceEvent {
    Changed(ThemeSettings),
}

/// Stateful Appearance pane.
pub struct AppearancePanel {
    settings: ThemeSettings,
    /// Tokens of `settings` (the pane's own chrome uses them, so it updates live).
    tokens: ThemeTokens,
    filter: ModeFilter,
    search: Entity<InputState>,
    pickers: Vec<(PaletteColorKey, Entity<ColorPickerState>)>,
    _subscriptions: Vec<Subscription>,
}

impl EventEmitter<AppearanceEvent> for AppearancePanel {}

impl AppearancePanel {
    /// Builds the pane around the persisted settings.
    pub fn new(settings: ThemeSettings, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let placeholder = t!("appearance.search_placeholder").to_string();
        let search = cx.new(|cx| InputState::new(window, cx).placeholder(placeholder));
        let (pickers, mut subscriptions) = pickers::build(&settings, window, cx);
        subscriptions.push(cx.subscribe(&search, |_, _, _: &gpui_kit::component::input::InputEvent, cx| cx.notify()));
        Self { tokens: settings.tokens(), settings, filter: ModeFilter::default(), search, pickers, _subscriptions: subscriptions }
    }

    /// Adopts settings changed elsewhere (no event is emitted).
    pub fn set_settings(&mut self, settings: ThemeSettings, window: &mut Window, cx: &mut Context<Self>) {
        if settings != self.settings {
            self.tokens = settings.tokens();
            self.settings = settings;
            self.sync_pickers(window, cx);
            cx.notify();
        }
    }

    /// Applies `edit` to a copy of the settings and, when it changed anything, adopts and
    /// emits the result.
    fn edit(&mut self, window: &mut Window, cx: &mut Context<Self>, edit: impl FnOnce(&mut ThemeSettings)) {
        let mut next = self.settings.clone();
        edit(&mut next);
        if next == self.settings {
            return;
        }
        self.set_settings(next.clone(), window, cx);
        cx.emit(AppearanceEvent::Changed(next));
    }
}

impl Render for AppearancePanel {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let entity = cx.entity().downgrade();
        let theme = &self.tokens;
        div()
            .flex()
            .flex_col()
            .gap(theme.spacing_xl)
            .child(section(gallery::header(self, &entity, theme), gallery::body(self, &entity, cx)))
            .child(editor::render(self, &entity))
            .child(layout::render(self, &entity))
            .child(transfer::render(self, &entity))
    }
}
