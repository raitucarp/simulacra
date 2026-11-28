//! Every item of the network, next due first: its prompt (or the map's title), kind, due
//! date, tags and source, with edit, suspend / resume and delete (after a confirmation).
//! Items whose post was deleted are listed last as retired.

use super::super::events::ReviewEvent;
use super::super::panel::ReviewPanel;
use super::super::text::{due_label, kind_label, tr};
use super::common::{caption, surface};
use crate::domain::review::{ItemId, ReviewItem};
use crate::i18n::t_count;
use crate::ui::icons::LucideIcon;
use crate::ui::menus::{render_menu, MenuItem, MenuSection, MenuSpec, MenuText};
use gpui_kit::component::menu::ContextMenuExt;
use gpui_kit::component::button::{Button, ButtonVariant, ButtonVariants};
use gpui_kit::component::{Icon, Sizable, WindowExt};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use rust_i18n::t;

const EDIT_ID_PREFIX: &str = "review-item-edit-";
const SUSPEND_ID_PREFIX: &str = "review-item-suspend-";
const DELETE_ID_PREFIX: &str = "review-item-delete-";
const ROW_ID_PREFIX: &str = "review-item-row-";
/// Most items listed at once.
const MAX_LISTED: usize = 400;
/// Lines of a prompt shown in the list.
const PROMPT_LINES: usize = 2;

impl ReviewPanel {
    pub(super) fn render_items(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &self.theme;
        if self.data.items.is_empty() {
            return div().child(self.render_empty_deck(cx));
        }
        let mut items: Vec<&ReviewItem> = self.data.items.iter().collect();
        items.sort_by_key(|item| (item.retired, item.suspended, item.schedule.is_new(), item.schedule.due));
        let rows = items.into_iter().take(MAX_LISTED).enumerate().map(|(index, item)| self.item_row(index, item, cx));
        div().child(surface(theme).gap(theme.spacing_xs).child(caption(t_count("review.items.count", self.data.items.len()), theme)).children(rows))
    }

    /// The line an item is listed under.
    fn item_headline(&self, item: &ReviewItem) -> String {
        match item.subject.map() {
            Some(map) => map.title.clone(),
            None => self.data.prompt_text(item),
        }
    }

    fn item_meta(&self, item: &ReviewItem) -> String {
        let state = match (item.retired, item.suspended) {
            (true, _) => tr("review.items.retired"),
            (_, true) => tr("review.items.suspended"),
            _ => due_label(&item.schedule, self.data.today),
        };
        std::iter::once(kind_label(item.subject.key())).chain(std::iter::once(state)).chain(item.tags.iter().map(|tag| format!("#{tag}"))).collect::<Vec<_>>().join(" · ")
    }

    fn item_row(&self, index: usize, item: &ReviewItem, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &self.theme;
        let (id, suspended, is_map) = (item.id.clone(), item.suspended, item.subject.map().is_some());
        let edit_id = item.id.clone();
        let edit = Button::new(ElementId::NamedInteger(EDIT_ID_PREFIX.into(), index as u64)).ghost().xsmall().icon(Icon::new(LucideIcon::Edit)).tooltip(tr("review.items.edit"))
            .on_click(cx.listener(move |this, _, window, cx| if is_map { this.edit_map(&edit_id, window, cx) } else { cx.emit(ReviewEvent::EditItem(edit_id.clone())) }));
        let toggle = Button::new(ElementId::NamedInteger(SUSPEND_ID_PREFIX.into(), index as u64))
            .ghost()
            .xsmall()
            .icon(Icon::new(if suspended { LucideIcon::Play } else { LucideIcon::Pause }))
            .tooltip(tr(if suspended { "review.items.resume" } else { "review.items.suspend" }))
            .on_click(cx.listener(move |_, _, _, cx| cx.emit(ReviewEvent::SetSuspended(id.clone(), !suspended))));
        let menu = self.item_menu(item, cx);
        div()
            .flex()
            .items_center()
            .gap(theme.control_gap)
            .py(theme.spacing_xs)
            .border_t(theme.border_width_hairline)
            .border_color(theme.border_subtle)
            .when(suspended || item.retired, |row| row.opacity(theme.vault_muted_opacity))
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .flex()
                    .flex_col()
                    .gap(theme.spacing_3xs)
                    .child(div().text_size(theme.font_size_sm).text_color(theme.text_primary).line_clamp(PROMPT_LINES).text_ellipsis().child(self.item_headline(item)))
                    .child(div().truncate().text_size(theme.font_size_2xs).text_color(theme.text_muted).child(self.item_meta(item))),
            )
            .when(!item.retired, |row| row.child(edit).child(toggle))
            .child(self.delete_button(index, item.id.clone(), cx))
            .id(ElementId::NamedInteger(ROW_ID_PREFIX.into(), index as u64))
            .context_menu(move |popup, window, cx| render_menu(popup, &menu, window, cx))
    }

    fn delete_button(&self, index: usize, id: ItemId, cx: &mut Context<Self>) -> impl IntoElement {
        let panel = cx.entity().downgrade();
        Button::new(ElementId::NamedInteger(DELETE_ID_PREFIX.into(), index as u64)).ghost().xsmall().icon(Icon::new(LucideIcon::Trash)).tooltip(tr("review.items.delete")).on_click(move |_, window, cx| confirm_delete(panel.clone(), id.clone(), window, cx))
    }

    /// M1: an item's right-click menu: the row's own buttons (edit, suspend or resume, stop
    /// learning after the same confirmation).
    fn item_menu(&self, item: &ReviewItem, cx: &mut Context<Self>) -> MenuSpec {
        let panel = cx.entity().downgrade();
        let (id, suspended, is_map) = (item.id.clone(), item.suspended, item.subject.map().is_some());
        let (edit_panel, edit_id, toggle_panel, toggle_id, delete_id) = (panel.clone(), id.clone(), panel.clone(), id.clone(), id);
        let edit = MenuItem::new("review.item.edit", MenuText::key("review.items.edit")).icon(LucideIcon::Edit).on_click(move |window, cx| {
            let _ = edit_panel.update(cx, |this, cx| if is_map { this.edit_map(&edit_id, window, cx) } else { cx.emit(ReviewEvent::EditItem(edit_id.clone())) });
        });
        let toggle_key = if suspended { "review.items.resume" } else { "review.items.suspend" };
        let toggle = MenuItem::new("review.item.suspend", MenuText::key(toggle_key)).icon(if suspended { LucideIcon::Play } else { LucideIcon::Pause }).on_click(move |_, cx| {
            let _ = toggle_panel.update(cx, |_, cx| cx.emit(ReviewEvent::SetSuspended(toggle_id.clone(), !suspended)));
        });
        let delete = MenuItem::new("review.item.delete", MenuText::key("review.items.delete")).icon(LucideIcon::Trash).destructive().on_click(move |window, cx| confirm_delete(panel.clone(), delete_id.clone(), window, cx));
        let active = MenuSection::new().entry_opt((!item.retired).then_some(edit)).entry_opt((!item.retired).then_some(toggle));
        MenuSpec::new().section(active).section(MenuSection::new().entry(delete))
    }
}

/// Asks before deleting the item `id` (it stops being learnt).
fn confirm_delete(panel: WeakEntity<ReviewPanel>, id: ItemId, window: &mut Window, cx: &mut App) {
    window.open_alert_dialog(cx, move |alert, _, _| {
        let (panel, id) = (panel.clone(), id.clone());
        alert
            .confirm()
            .title(tr("review.items.delete_title"))
            .description(t!("review.items.delete_desc").to_string())
            .ok_text(tr("review.items.delete"))
            .ok_variant(ButtonVariant::Danger)
            .cancel_text(t!("composer.cancel").to_string())
            .on_ok(move |_, _, cx| {
                let _ = panel.update(cx, |_, cx| cx.emit(ReviewEvent::DeleteItem(id.clone())));
                true
            })
    });
}
