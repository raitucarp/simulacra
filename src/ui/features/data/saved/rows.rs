//! One shelf row: the grip to drag it, the item's kind, title and byline, its note (or the
//! note editor), and open / remove. Bookmarks have neither grip nor note.

use super::super::events::{DataAction, DataEvent};
use super::super::text::tr;
use super::{SavedItemView, SavedView};
use crate::domain::data::saved::ItemRef;
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::input::Input;
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;

const ROW_ID: &str = "saved-row";
const OPEN_ID: &str = "saved-open";
const REMOVE_ID: &str = "saved-remove";
const NOTE_ID: &str = "saved-note";
const SAVE_NOTE_ID: &str = "saved-note-save";
const CANCEL_NOTE_ID: &str = "saved-note-cancel";

/// The row being dragged and its preview under the pointer.
#[derive(Clone)]
pub struct DraggedItem {
    pub index: usize,
    pub title: String,
    pub theme: ThemeTokens,
}

impl Render for DraggedItem {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        let theme = &self.theme;
        div().max_w(theme.saved_chip_max_width).px(theme.spacing_sm).py(theme.spacing_xs).rounded(theme.radius_md).bg(theme.bg_elevated).border(theme.border_width_hairline).border_color(theme.accent_brand).text_size(theme.font_size_2xs).text_color(theme.text_primary).truncate().child(self.title.clone())
    }
}

fn send(on_event: &DataAction, event: DataEvent) -> impl Fn(&ClickEvent, &mut Window, &mut App) + 'static {
    let on_event = on_event.clone();
    move |_, window, cx| on_event(event.clone(), window, cx)
}

fn note_block(view: &SavedView, index: usize, item: &SavedItemView, on_event: &DataAction, theme: &ThemeTokens) -> Div {
    if view.editing_note.as_ref() == Some(&item.item) {
        let buttons = div().flex().justify_end().gap(theme.control_gap)
            .child(Button::new((CANCEL_NOTE_ID, index)).ghost().xsmall().label(tr("data.cancel")).on_click(send(on_event, DataEvent::CancelNote)))
            .child(Button::new((SAVE_NOTE_ID, index)).primary().xsmall().label(tr("data.saved.save_note")).on_click(send(on_event, DataEvent::SaveNote)));
        return div().flex().flex_col().gap(theme.spacing_xs).child(div().min_h(theme.saved_note_min_height).child(Input::new(&view.note).small())).child(buttons);
    }
    let (text, colour) = if item.note.is_empty() { (tr("data.saved.add_note"), theme.text_muted) } else { (item.note.clone(), theme.text_secondary) };
    let hover = theme.text_primary;
    div().child(
        div()
            .id((NOTE_ID, index))
            .cursor_pointer()
            .text_size(theme.font_size_xs)
            .line_height(relative(theme.line_height_body))
            .text_color(colour)
            .when(item.note.is_empty(), |d| d.italic())
            .hover(move |s| s.text_color(hover))
            .on_click(send(on_event, DataEvent::EditNote(item.item.clone())))
            .child(text),
    )
}

/// The row of `item`, at `index` on the shelf.
pub(super) fn item_row(view: &SavedView, index: usize, item: &SavedItemView, on_event: &DataAction, theme: &ThemeTokens) -> Stateful<Div> {
    let icon = match item.item {
        ItemRef::Post(_) => LucideIcon::MessageSquare,
        ItemRef::Document(_) => LucideIcon::FileText,
    };
    let grip = view.reorderable.then(|| div().w(theme.saved_grip_width).flex_none().flex().justify_center().pt(theme.spacing_2xs).text_color(theme.text_muted).cursor_grab().child(Icon::new(LucideIcon::GripVertical).size(theme.icon_size_xs)));
    let texts = div()
        .flex_1()
        .min_w_0()
        .flex()
        .flex_col()
        .gap(theme.spacing_2xs)
        .child(div().flex().items_center().gap(theme.inline_gap).child(Icon::new(icon).size(theme.icon_size_xs).text_color(theme.accent_brand)).child(div().min_w_0().truncate().text_size(theme.font_size_sm).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_primary).child(item.title.clone())))
        .child(div().truncate().text_size(theme.font_size_2xs).text_color(theme.text_muted).child(item.subtitle.clone()))
        .children(view.reorderable.then(|| note_block(view, index, item, on_event, theme)));
    let actions = div().flex_none().flex().gap(theme.spacing_2xs)
        .child(Button::new((OPEN_ID, index)).ghost().xsmall().icon(Icon::new(LucideIcon::ExternalLink)).tooltip(tr("data.saved.open")).on_click(send(on_event, DataEvent::OpenItem(item.item.clone()))))
        .child(Button::new((REMOVE_ID, index)).ghost().xsmall().icon(Icon::new(LucideIcon::X)).tooltip(tr("data.saved.remove")).on_click(send(on_event, DataEvent::RemoveItem(item.item.clone()))));
    let row = div()
        .id((ROW_ID, index))
        .w_full()
        .flex()
        .items_start()
        .gap(theme.inline_gap)
        .p(theme.card_padding)
        .rounded(theme.radius_md)
        .bg(theme.bg_card)
        .border(theme.border_width_hairline)
        .border_color(theme.border_subtle)
        .children(grip)
        .child(texts)
        .child(actions);
    if !view.reorderable {
        return row;
    }
    let (dragged, tint, drop) = (DraggedItem { index, title: item.title.clone(), theme: theme.clone() }, theme.accent_brand_subtle, on_event.clone());
    row.on_drag(dragged, |dragged, _, _, cx| cx.new(|_| dragged.clone()))
        .drag_over::<DraggedItem>(move |row, _, _, _| row.bg(tint))
        .on_drop(move |dragged: &DraggedItem, window, cx| drop(DataEvent::MoveItem { from: dragged.index, to: index }, window, cx))
}
