//! Appearance mode section: Dark / Light / System chips and the palette used in each mode.

use super::super::common::{card, field_title, muted_note, section, section_label, spaced_row};
use super::handlers::{act, PanelHandle};
use super::AppearancePanel;
use crate::theme::{AppearanceMode, PaletteMode, ThemeRegistry, ThemeSettings, ThemeTokens};
use crate::ui::components::{chip_row, choice_chip};
use crate::ui::features::title_bar::mode_label;
use gpui_kit::component::button::Button;
use gpui_kit::component::menu::{DropdownMenu, PopupMenuItem};
use gpui_kit::component::Sizable;
use gpui_kit::*;
use rust_i18n::t;

const MODE_CHIP_ID_PREFIX: &str = "appearance-mode-";
const PREFERRED_MENU_ID_PREFIX: &str = "appearance-preferred-";

/// `(id, name)` of every palette of `mode`: the bundled ones, then the user's own.
fn palettes_of(settings: &ThemeSettings, mode: PaletteMode) -> Vec<(String, String)> {
    let bundled = ThemeRegistry::global().by_mode(mode);
    let custom = settings.custom.iter().filter(|palette| palette.mode == mode);
    bundled.chain(custom).map(|palette| (palette.id.clone(), palette.name.clone())).collect()
}

/// Translation key of the "<mode> theme" field title.
const fn preferred_title_key(mode: PaletteMode) -> &'static str {
    match mode {
        PaletteMode::Dark => "appearance.preferred_dark",
        PaletteMode::Light => "appearance.preferred_light",
    }
}

/// Field title and a menu button naming the preferred palette of `mode`.
fn preferred_row(pane: &AppearancePanel, handle: &PanelHandle, mode: PaletteMode) -> Div {
    let (settings, theme) = (&pane.settings, &pane.tokens);
    let current = settings.preferred_id(mode).to_string();
    let name = settings.palette(&current).map(|palette| palette.name.clone()).unwrap_or_default();
    let options = palettes_of(settings, mode);
    let handle = handle.clone();
    let menu = Button::new(SharedString::from(format!("{PREFERRED_MENU_ID_PREFIX}{}", mode.id())))
        .outline()
        .small()
        .label(name)
        // The caret says it is a menu, as on every other settings menu button.
        .dropdown_caret(true)
        .dropdown_menu(move |mut menu, _, _| {
            for (id, label) in &options {
                let checked = *id == current;
                let id = id.clone();
                let pick = act(&handle, move |pane, window, cx| pane.set_preferred(mode, &id, window, cx));
                menu = menu.item(PopupMenuItem::new(label.clone()).checked(checked).on_click(move |_, window, cx| pick(window, cx)));
            }
            menu.scrollable(true)
        });
    spaced_row().gap(theme.control_gap).child(field_title(theme, t!(preferred_title_key(mode)).to_string())).child(menu)
}

/// Mode chips, a hint and both preferred-palette rows.
pub(super) fn render(pane: &AppearancePanel, handle: &PanelHandle) -> Div {
    let theme: &ThemeTokens = &pane.tokens;
    let current = pane.settings.appearance;
    let chips = AppearanceMode::ALL.iter().map(|&mode| {
        let id = SharedString::from(format!("{MODE_CHIP_ID_PREFIX}{}", mode.id()));
        choice_chip(id, &mode_label(mode), mode == current, theme, act(handle, move |pane, window, cx| pane.set_mode(mode, window, cx)))
    });
    let body = card(theme)
        .gap(theme.stack_gap)
        .child(chip_row(theme).children(chips))
        .child(muted_note(theme, t!("appearance.mode_hint").to_string()))
        .children(PaletteMode::ALL.map(|mode| preferred_row(pane, handle, mode)));
    section(section_label(theme, t!("appearance.mode_title").to_string()), body, theme)
}

impl AppearancePanel {
    /// Switches between Dark, Light and following the system.
    fn set_mode(&mut self, mode: AppearanceMode, window: &mut Window, cx: &mut Context<Self>) {
        let system = PaletteMode::of_window(window.appearance());
        self.edit(window, cx, |s| s.set_appearance(mode, system));
    }

    /// Picks the palette shown in `mode`.
    fn set_preferred(&mut self, mode: PaletteMode, id: &str, window: &mut Window, cx: &mut Context<Self>) {
        let system = PaletteMode::of_window(window.appearance());
        self.edit(window, cx, |s| s.set_preferred(mode, id, system));
    }
}
