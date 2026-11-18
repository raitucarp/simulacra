//! The Saved page: header with "New collection" and "Learn this collection", the shelf
//! chips, then the shelf's rows (or its empty state).

use super::super::events::{DataAction, DataEvent, SavedTarget};
use super::super::text::tr;
use super::rows::item_row;
use super::SavedView;
use crate::theme::ThemeTokens;
use crate::ui::components::{art_state, ArtState};
use crate::ui::icons::LucideIcon;
use crate::ui::illustration::{EmptyKey, Illustration};
use gpui_kit::base::Disableable;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::input::Input;
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use rust_i18n::t;

const PAGE_ID: &str = "saved-page";
const CHIP_ID_PREFIX: &str = "saved-chip-";
const CREATE_ID: &str = "saved-create";
const LEARN_ID: &str = "saved-learn";

fn send(on_event: &DataAction, event: DataEvent) -> impl Fn(&ClickEvent, &mut Window, &mut App) + 'static {
    let on_event = on_event.clone();
    move |_, window, cx| on_event(event.clone(), window, cx)
}

fn header(view: &SavedView, on_event: &DataAction, theme: &ThemeTokens) -> Div {
    let titles = div()
        .flex_1()
        .min_w_0()
        .flex()
        .flex_col()
        .gap(theme.spacing_3xs)
        .child(div().font_family(theme.font_family_display.clone()).text_size(theme.font_size_xl).font_weight(FontWeight::BOLD).text_color(theme.text_primary).child(tr("data.saved.title")))
        .child(div().text_size(theme.font_size_xs).text_color(theme.text_muted).child(tr("data.saved.subtitle")));
    let create = div()
        .flex()
        .items_center()
        .gap(theme.control_gap)
        .child(div().w(theme.saved_chip_max_width).child(Input::new(&view.new_collection).small()))
        .child(Button::new(CREATE_ID).outline().small().icon(Icon::new(LucideIcon::FolderPlus)).label(tr("data.saved.create")).on_click(send(on_event, DataEvent::CreateCollection)));
    let learn = Button::new(LEARN_ID).ghost().small().icon(Icon::new(LucideIcon::GraduationCap)).label(tr("data.saved.learn")).disabled(view.items.is_empty()).on_click(send(on_event, DataEvent::LearnShelf));
    div().flex().flex_wrap().items_center().gap(theme.control_gap).child(titles).child(create).child(learn)
}

fn chips(view: &SavedView, on_event: &DataAction, theme: &ThemeTokens) -> Div {
    let chips = view.chips.iter().enumerate().map(|(index, chip)| {
        let active = chip.target == view.target;
        let icon = if chip.target == SavedTarget::Bookmarks { LucideIcon::Bookmark } else { LucideIcon::Folder };
        let hover = theme.hover_bg;
        div()
            .id(ElementId::NamedInteger(CHIP_ID_PREFIX.into(), index as u64))
            .max_w(theme.saved_chip_max_width)
            .flex()
            .items_center()
            .gap(theme.inline_gap)
            .px(theme.spacing_sm)
            .py(theme.spacing_xs)
            .rounded(theme.radius_full)
            .border(theme.border_width_hairline)
            .cursor_pointer()
            .text_size(theme.font_size_xs)
            .when(active, |c| c.bg(theme.accent_brand_subtle).border_color(theme.accent_brand).text_color(theme.text_primary))
            .when(!active, |c| c.bg(theme.bg_card).border_color(theme.border_subtle).text_color(theme.text_secondary).hover(move |h| h.bg(hover)))
            .on_click(send(on_event, DataEvent::SelectShelf(chip.target.clone())))
            .child(Icon::new(icon).size(theme.icon_size_xs))
            .child(div().min_w_0().truncate().child(chip.label.clone()))
            .child(div().flex_none().font_family(theme.font_family_mono.clone()).text_size(theme.font_size_2xs).text_color(theme.text_muted).child(chip.count.to_string()))
    });
    div().flex().flex_wrap().gap(theme.control_gap).children(chips)
}

fn empty(view: &SavedView, theme: &ThemeTokens) -> Div {
    let (title, hint) = match view.target {
        SavedTarget::Bookmarks => (tr("data.saved.no_bookmarks"), tr("data.saved.no_bookmarks_hint")),
        SavedTarget::Collection(_) => (tr("data.saved.empty_collection"), tr("data.saved.empty_collection_hint")),
    };
    div().py(theme.section_gap).flex().justify_center().child(art_state(ArtState::titled(Illustration::empty(EmptyKey::NoResults).width(theme.art_empty_width), title, hint), theme))
}

/// The page.
pub fn render_saved_page(view: &SavedView, on_event: DataAction, theme: &ThemeTokens) -> Stateful<Div> {
    let count = t!("data.saved.count", count = view.items.len()).to_string();
    let rows = view.items.iter().enumerate().map(|(index, item)| item_row(view, index, item, &on_event, theme));
    let list = if view.items.is_empty() { empty(view, theme) } else { div().flex().flex_col().gap(theme.spacing_xs).children(rows) };
    let hint = view.reorderable.then(|| div().text_size(theme.font_size_2xs).text_color(theme.text_muted).child(tr("data.saved.drag_hint")));
    let column = div()
        .w_full()
        .max_w(theme.saved_page_max_width)
        .flex()
        .flex_col()
        .gap(theme.section_gap)
        .child(header(view, &on_event, theme))
        .child(chips(view, &on_event, theme))
        .child(div().flex().items_center().justify_between().gap(theme.control_gap).child(div().text_size(theme.font_size_2xs).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_muted).child(count)).children(hint))
        .child(list);
    div().id(PAGE_ID).size_full().overflow_y_scroll().bg(theme.bg_app).flex().flex_col().items_center().p(theme.panel_padding).child(column)
}
