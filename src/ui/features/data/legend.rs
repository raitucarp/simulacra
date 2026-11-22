//! The legend page (S9): the core symbols with their fixed meanings, then the custom symbols
//! the user's posts carry with the meaning the user gave each (editable, one at a time).

use super::events::{DataAction, DataEvent};
use super::sheet::{footer, heading, hint, sheet_frame, SheetParts};
use super::text::tr;
use crate::domain::symbols::{entry_key, signifier_key, SymbolArt, SymbolGlyph, SymbolRef};
use crate::theme::ThemeTokens;
use crate::ui::features::symbols::{art_label, glyph_label, render_glyph, render_symbol_art};
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::input::{Input, InputState};
use gpui_kit::component::Sizable;
use gpui_kit::*;
use std::rc::Rc;

const SHEET_ID: &str = "data-legend";
const CLOSE_ID: &str = "data-legend-close";
const EDIT_ID: &str = "data-legend-edit";
const SAVE_ID: &str = "data-legend-save";
const CANCEL_ID: &str = "data-legend-cancel";

/// What the legend shows.
#[derive(Clone)]
pub struct LegendView {
    /// Every custom symbol in use or in the key, with its meaning (empty: none yet).
    pub custom: Vec<(SymbolRef, String)>,
    /// The symbol whose meaning is being edited, and the field holding it.
    pub editing: Option<SymbolRef>,
    pub meaning: Entity<InputState>,
}

/// The fixed meaning of a core symbol.
fn core_meaning(art: SymbolArt) -> String {
    match art {
        SymbolArt::Entry(entry, _) => tr(&format!("symbols.meaning.{}", entry_key(entry))),
        SymbolArt::Signifier(signifier) => tr(&format!("symbols.meaning.{}", signifier_key(signifier))),
    }
}

/// The core symbols shown: each entry in its first state, then each signifier.
fn core_arts() -> Vec<SymbolArt> {
    crate::domain::symbols::core_choices().iter().map(crate::ui::features::composer::choice_glyph).filter_map(|glyph| match glyph {
        SymbolGlyph::Core(art) => Some(art),
        SymbolGlyph::Custom(_) => None,
    }).collect()
}

fn line(glyph: AnyElement, name: String, meaning: Div, theme: &ThemeTokens) -> Div {
    let name = div().w(theme.data_label_width).flex_none().text_size(theme.font_size_xs).text_color(theme.text_primary).child(name);
    div().flex().items_center().gap(theme.spacing_sm).py(theme.spacing_2xs).child(glyph).child(name).child(meaning.flex_1().min_w_0())
}

fn custom_row(index: usize, (symbol, meaning): &(SymbolRef, String), view: &LegendView, on_event: &DataAction, theme: &ThemeTokens) -> Div {
    let glyph = render_glyph(&SymbolGlyph::Custom(symbol.clone()), theme.icon_size_md, theme);
    let name = glyph_label(&SymbolGlyph::Custom(symbol.clone()));
    if view.editing.as_ref() == Some(symbol) {
        let (save, cancel) = (on_event.clone(), on_event.clone());
        let editor = div().flex().gap(theme.spacing_xs).child(div().flex_1().child(Input::new(&view.meaning).small()))
            .child(Button::new(SAVE_ID).primary().xsmall().label(tr("symbols.legend.save")).on_click(move |_, w, cx| save(DataEvent::SaveMeaning, w, cx)))
            .child(Button::new(CANCEL_ID).ghost().xsmall().label(tr("symbols.legend.cancel")).on_click(move |_, w, cx| cancel(DataEvent::CancelMeaning, w, cx)));
        return line(glyph, name, editor, theme);
    }
    let text = if meaning.is_empty() { tr("symbols.legend.no_meaning") } else { meaning.clone() };
    let (on_edit, edit) = (on_event.clone(), symbol.clone());
    let button = Button::new(SharedString::from(format!("{EDIT_ID}-{index}"))).ghost().xsmall().icon(LucideIcon::Pencil).tooltip(tr("symbols.legend.edit")).on_click(move |_, w, cx| on_edit(DataEvent::EditMeaning(edit.clone()), w, cx));
    let meaning = div().flex().items_center().gap(theme.spacing_xs).child(div().flex_1().text_size(theme.font_size_xs).text_color(theme.text_muted).child(text)).child(button);
    line(glyph, name, meaning, theme)
}

/// The overlay.
pub fn render_legend_sheet(view: &LegendView, on_event: DataAction, theme: &ThemeTokens) -> Stateful<Div> {
    let core = core_arts().into_iter().map(|art| line(render_symbol_art(art, theme.icon_size_md).into_any_element(), art_label(art), hint(core_meaning(art), theme), theme));
    let custom: Vec<Div> = view.custom.iter().enumerate().map(|(i, entry)| custom_row(i, entry, view, &on_event, theme)).collect();
    let custom_body = if custom.is_empty() { div().child(hint(tr("symbols.legend.no_custom"), theme)) } else { div().children(custom) };
    let body = div().flex().flex_col().gap(theme.spacing_sm)
        .child(heading(tr("symbols.legend.core"), theme)).children(core)
        .child(heading(tr("symbols.legend.custom"), theme)).child(hint(tr("symbols.legend.custom_hint"), theme)).child(custom_body);
    let (close, closing) = (on_event.clone(), on_event);
    let buttons = footer(theme).child(Button::new(CLOSE_ID).ghost().small().label(tr("data.close")).on_click(move |_, w, cx| close(DataEvent::CloseLegend, w, cx)));
    let parts = SheetParts { id: SHEET_ID, icon: LucideIcon::BookOpen, title: tr("symbols.legend.title"), subtitle: Some(tr("symbols.legend.subtitle")), body, footer: buttons, on_close: Rc::new(move |w, cx| closing(DataEvent::CloseLegend, w, cx)) };
    sheet_frame(parts, theme)
}
