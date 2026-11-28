//! The right-click menu of a vault document (M1), on its shelf card and its table row: the
//! entry page's actions (open, read, edit, mark read, learn, history, quote, delete), each
//! with the same handler and Delete with the same confirmation.

use crate::domain::document::record::ReadingStatus;
use crate::domain::GroundingDocument;
use crate::ui::features::vault::panel::VaultPanel;
use crate::ui::features::vault::screen::Screen;
use crate::ui::features::vault::VaultEvent;
use crate::ui::icons::LucideIcon;
use crate::ui::menus::{context_menu_of, MenuItem, MenuSection, MenuSpec, MenuText};
use gpui_kit::component::menu::PopupMenu;
use gpui_kit::component::button::ButtonVariant;
use gpui_kit::component::WindowExt;
use gpui_kit::*;
use rust_i18n::t;

/// Asks before deleting the document `id` titled `title`.
pub(super) fn confirm_delete(panel: WeakEntity<VaultPanel>, id: String, title: String, window: &mut Window, cx: &mut App) {
    window.open_alert_dialog(cx, move |alert, _, _| {
        let (panel, id) = (panel.clone(), id.clone());
        alert
            .confirm()
            .title(t!("vault.delete_title").to_string())
            .description(t!("vault.delete_desc", title = title.clone()).to_string())
            .ok_text(t!("vault.delete").to_string())
            .ok_variant(ButtonVariant::Danger)
            .cancel_text(t!("composer.cancel").to_string())
            .on_ok(move |_, _, cx| {
                let _ = panel.update(cx, |_, cx| cx.emit(VaultEvent::Delete(id.clone())));
                true
            })
    });
}

type PanelUpdate = fn(&mut VaultPanel, String, &mut Window, &mut Context<VaultPanel>);

fn panel_item(key: &'static str, icon: LucideIcon, panel: &WeakEntity<VaultPanel>, id: &str, run: PanelUpdate) -> MenuItem {
    let (panel, id) = (panel.clone(), id.to_string());
    MenuItem::new(format!("vault.doc.{key}"), MenuText::key(key)).icon(icon).on_click(move |window, cx| {
        let _ = panel.update(cx, |this, cx| run(this, id.clone(), window, cx));
    })
}

/// What the menu needs of a document (cards keep this, not the whole text).
#[derive(Clone)]
struct DocMenu {
    id: String,
    title: String,
    read: bool,
}

/// The right-click menu builder of `doc` in `panel`.
pub(super) fn doc_context_menu(doc: &GroundingDocument, panel: WeakEntity<VaultPanel>) -> impl Fn(PopupMenu, &mut Window, &mut Context<PopupMenu>) -> PopupMenu + 'static {
    let doc = DocMenu { id: doc.id.clone(), title: doc.title.clone(), read: doc.record.reading.status == ReadingStatus::Read };
    context_menu_of(move || doc_menu_spec(&doc, &panel))
}

/// The menu of `doc`.
fn doc_menu_spec(doc: &DocMenu, panel: &WeakEntity<VaultPanel>) -> MenuSpec {
    let (id, read) = (doc.id.as_str(), doc.read);
    let open = MenuSection::new()
        .entry(panel_item("data.saved.open", LucideIcon::ExternalLink, panel, id, |this, id, _, cx| this.go(Screen::Entry(id), cx)))
        .entry(panel_item("vault.read", LucideIcon::BookOpen, panel, id, |this, id, _, cx| this.go(Screen::Reader(id), cx)));
    let mark: PanelUpdate = if read { |this, id, _, cx| this.mark_read(id, false, cx) } else { |this, id, _, cx| this.mark_read(id, true, cx) };
    let edit = MenuSection::new()
        .entry(panel_item("vault.edit", LucideIcon::Edit, panel, id, |this, id, window, cx| this.open_editor(Some(id), window, cx)))
        .entry(panel_item(if read { "vault.mark_unread" } else { "vault.mark_read" }, LucideIcon::Check, panel, id, mark))
        .entry(panel_item("data.history.button", LucideIcon::RotateCcwClock, panel, id, |_, id, _, cx| cx.emit(VaultEvent::History(id))));
    let study = MenuSection::new()
        .entry(panel_item("vault.learn_passages", LucideIcon::GraduationCap, panel, id, |_, id, _, cx| cx.emit(VaultEvent::LearnPassages(id))))
        .entry(panel_item("vault.learn_connections", LucideIcon::Share2, panel, id, |_, id, _, cx| cx.emit(VaultEvent::LearnConnections(id))))
        .entry(panel_item("posts.pinned.button", LucideIcon::Quote, panel, id, |_, id, _, cx| cx.emit(VaultEvent::Quote(id))));
    let (weak, doc_id, title) = (panel.clone(), doc.id.clone(), doc.title.clone());
    let delete = MenuItem::new("vault.doc.delete", MenuText::key("vault.delete")).icon(LucideIcon::Trash).destructive().on_click(move |window, cx| confirm_delete(weak.clone(), doc_id.clone(), title.clone(), window, cx));
    MenuSpec::new().section(open).section(edit).section(study).section(MenuSection::new().entry(delete))
}
