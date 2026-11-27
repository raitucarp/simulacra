//! The "Keyboard shortcuts" sheet: every section of the keymap with its rows, generated from
//! [`super::SHORTCUTS`]. Escape, the close button or the scrim dismiss it.

use super::caps::render_chord;
use super::contexts::SHEET_KEY_CONTEXT;
use super::groups::{sheet_rows, target_label, ShortcutGroup, SheetRow};
use crate::theme::ThemeTokens;
use crate::ui::atoms::ElevationExt;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::Icon;
use gpui_kit::*;
use rust_i18n::t;
use std::rc::Rc;

const BACKDROP_ID: &str = "shortcut-sheet-backdrop";
const CARD_ID: &str = "shortcut-sheet-card";
const CLOSE_ID: &str = "shortcut-sheet-close";
const BODY_ID: &str = "shortcut-sheet-body";
const TITLE_KEY: &str = "keymap.title";
const SUBTITLE_KEY: &str = "keymap.subtitle";

/// A shared no-argument callback.
pub type SheetAction = Rc<dyn Fn(&mut Window, &mut App)>;

/// The shortcut sheet overlay.
#[derive(IntoElement)]
pub struct ShortcutSheet {
    theme: ThemeTokens,
    focus: FocusHandle,
    on_close: SheetAction,
}

impl ShortcutSheet {
    pub fn new(theme: &ThemeTokens, focus: FocusHandle, on_close: SheetAction) -> Self {
        Self { theme: theme.clone(), focus, on_close }
    }
}

impl RenderOnce for ShortcutSheet {
    fn render(self, window: &mut Window, _: &mut App) -> impl IntoElement {
        let theme = &self.theme;
        // As wide and tall as the tokens allow, kept inside the window with the modal margin.
        let (room, inset) = (window.viewport_size(), theme.settings_modal_inset + theme.settings_modal_inset);
        let (on_scrim, on_button) = (self.on_close.clone(), self.on_close);
        let sections = ShortcutGroup::ALL.into_iter().map(|group| render_section(group, theme));
        let card = div()
            .id(CARD_ID)
            .w(theme.shortcut_sheet_width.min(room.width - inset))
            .max_h(theme.shortcut_sheet_max_height.min(room.height - inset))
            .flex()
            .flex_col()
            .rounded(theme.radius_lg)
            .bg(theme.bg_elevated)
            .border(theme.border_width_hairline)
            .border_color(theme.border)
            .elevation_shadow(theme)
            .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
            .child(render_header(theme, on_button))
            .child(div().id(BODY_ID).flex_1().min_h_0().overflow_y_scroll().p(theme.panel_padding).flex().flex_wrap().gap(theme.section_gap).children(sections));
        div()
            .id(BACKDROP_ID)
            .key_context(SHEET_KEY_CONTEXT)
            .track_focus(&self.focus)
            .absolute()
            .inset_0()
            .occlude()
            .bg(theme.bg_scrim)
            .flex()
            .items_center()
            .justify_center()
            .on_mouse_down(MouseButton::Left, move |_, window, cx| on_scrim(window, cx))
            .child(card)
    }
}

fn render_header(theme: &ThemeTokens, on_close: SheetAction) -> Div {
    let hover_bg = theme.hover_bg;
    let title = div()
        .flex()
        .flex_col()
        .gap(theme.spacing_2xs)
        .child(div().text_size(theme.font_size_md).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_primary).child(t!(TITLE_KEY).to_string()))
        .child(div().text_size(theme.font_size_xs).text_color(theme.text_muted).child(t!(SUBTITLE_KEY).to_string()));
    let close = div()
        .id(CLOSE_ID)
        .p(theme.spacing_xs)
        .rounded(theme.radius_sm)
        .cursor_pointer()
        .text_color(theme.text_muted)
        .hover(move |style| style.bg(hover_bg))
        .on_click(move |_, window, cx| on_close(window, cx))
        .child(Icon::new(LucideIcon::X));
    div()
        .flex()
        .items_center()
        .gap(theme.inline_gap)
        .px(theme.panel_padding)
        .py(theme.card_padding)
        .border_b(theme.border_width_hairline)
        .border_color(theme.border_subtle)
        .child(div().text_color(theme.accent_brand_text).child(Icon::new(LucideIcon::Keyboard)))
        .child(title.flex_1())
        .child(close)
}

fn render_section(group: ShortcutGroup, theme: &ThemeTokens) -> Div {
    let rows = sheet_rows(group).into_iter().map(|row| render_row(row, theme));
    div()
        .w(theme.shortcut_sheet_section_width)
        .flex()
        .flex_col()
        .gap(theme.spacing_xs)
        .child(div().text_size(theme.font_size_2xs).font_weight(FontWeight::BOLD).text_color(theme.text_muted).child(t!(group.title_key()).to_string()))
        .children(rows)
}

fn render_row(row: SheetRow, theme: &ThemeTokens) -> Div {
    let chords = row.chords.iter().map(|chord| render_chord(chord, theme));
    div()
        .flex()
        .items_center()
        .justify_between()
        .gap(theme.inline_gap)
        .py(theme.spacing_3xs)
        .child(div().min_w_0().truncate().text_size(theme.font_size_xs).text_color(theme.text_primary).child(target_label(row.target)))
        .child(div().flex().flex_none().gap(theme.inline_gap).children(chords))
}
