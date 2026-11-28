//! The Journal tab: the editor while an entry is open; otherwise new-entry buttons, the
//! search, the heat strip with streaks (told gently), and the entries found.

use super::super::panel::{JournalFields, JournalPanel};
use super::super::text::{day_label, tr};
use super::common::{caption, control_row, ghost_button, line_field, primary_button, surface};
use super::confirm::DeleteKind;
use crate::domain::journal::search::{excerpt_around, search_entries};
use crate::domain::journal::templates::Template;
use crate::domain::journal::JournalEntry;
use crate::ui::icons::LucideIcon;
use crate::ui::menus::{render_menu, MenuItem, MenuSection, MenuSpec, MenuText};
use crate::ui::responsive::LayoutClass;
use gpui_kit::component::menu::ContextMenuExt;
use gpui_kit::*;
use rust_i18n::t;

const NEW_ID: &str = "journal-new";
const TEMPLATE_ID: &str = "journal-new-template";
const EDIT_ID: &str = "journal-entry-edit";
const POST_ID: &str = "journal-entry-post";
const DELETE_ID: &str = "journal-entry-delete";
const ROW_ID: &str = "journal-entry-row";
/// Characters on each side of a search hit in a result's excerpt.
const EXCERPT_RADIUS: usize = 90;

impl JournalPanel {
    pub(super) fn render_journal(&self, class: LayoutClass, cx: &mut Context<Self>) -> AnyElement {
        if self.editor.is_some() {
            return self.render_editor(cx).into_any_element();
        }
        let theme = &self.theme;
        if self.data.book.entries.is_empty() {
            return div().w_full().flex().flex_col().gap(theme.section_gap).child(self.new_entry_row(cx)).child(self.journal_empty(cx)).into_any_element();
        }
        let query = JournalFields::text(&self.fields.search, cx);
        let found = search_entries(&self.data.book.entries, &query);
        let count = caption(t!("journal.search.count", count = found.len()).to_string(), theme);
        let rows: Vec<AnyElement> = found.iter().enumerate().map(|(i, entry)| self.entry_row(i, entry, &query, cx)).collect();
        let none = rows.is_empty().then(|| caption(tr("journal.search.none"), theme));
        div()
            .w_full()
            .flex()
            .flex_col()
            .gap(theme.section_gap)
            .child(self.new_entry_row(cx))
            .child(self.render_heat(class))
            .child(surface(theme).child(line_field(&self.fields.search, theme)).child(count).children(none).children(rows))
            .into_any_element()
    }

    fn new_entry_row(&self, cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        let new = primary_button(NEW_ID, LucideIcon::Plus, tr("journal.new_entry")).on_click(cx.listener(|this, _, window, cx| this.new_entry(None, false, window, cx)));
        let templates = Template::ALL.into_iter().map(|template| {
            ghost_button((TEMPLATE_ID, template as usize), LucideIcon::Layers, tr(&format!("journal.template.{}", template.key())))
                .on_click(cx.listener(move |this, _, window, cx| this.new_entry(Some(template), false, window, cx)))
        });
        control_row(theme).child(new).children(templates)
    }

    /// M1: an entry's right-click menu: the row's own buttons (edit, turn into a post, delete
    /// after the same confirmation).
    fn entry_menu(&self, entry: &JournalEntry, cx: &mut Context<Self>) -> MenuSpec {
        let panel = cx.entity().downgrade();
        let (edit_panel, post_panel, edit_id, post_id) = (panel.clone(), panel, entry.id.clone(), entry.id.clone());
        let confirm = self.confirm_delete(DeleteKind::Entry, entry.id.clone(), cx);
        let edit = MenuItem::new("journal.entry.edit", MenuText::key("journal.entry.edit")).icon(LucideIcon::NotebookPen).on_click(move |window, cx| {
            let _ = edit_panel.update(cx, |this, cx| this.edit_entry(&edit_id, window, cx));
        });
        let post = MenuItem::new("journal.entry.into_post", MenuText::key("journal.entry.into_post")).icon(LucideIcon::Send).on_click(move |_, cx| {
            let _ = post_panel.update(cx, |this, cx| this.entry_into_post(&post_id, cx));
        });
        let delete = MenuItem::new("journal.entry.delete", MenuText::key("journal.entry.delete")).icon(LucideIcon::Trash).destructive().on_click(move |window, cx| confirm(&ClickEvent::default(), window, cx));
        MenuSpec::new().section(MenuSection::new().entry(edit).entry(post)).section(MenuSection::new().entry(delete))
    }

    fn entry_row(&self, index: usize, entry: &JournalEntry, query: &str, cx: &mut Context<Self>) -> AnyElement {
        let theme = &self.theme;
        let (edit_id, post_id, delete_id) = (entry.id.clone(), entry.id.clone(), entry.id.clone());
        let edit = ghost_button((EDIT_ID, index), LucideIcon::NotebookPen, tr("journal.entry.edit")).on_click(cx.listener(move |this, _, window, cx| this.edit_entry(&edit_id, window, cx)));
        let post = ghost_button((POST_ID, index), LucideIcon::Send, tr("journal.entry.into_post")).on_click(cx.listener(move |this, _, _, cx| this.entry_into_post(&post_id, cx)));
        let delete = ghost_button((DELETE_ID, index), LucideIcon::Trash, tr("journal.entry.delete")).on_click(self.confirm_delete(DeleteKind::Entry, delete_id, cx));
        let meta = format!("{}{}{}", day_label(entry.day), crate::i18n::COUNT_SEPARATOR, t!("journal.entry.words", count = entry.word_count()));
        let menu = self.entry_menu(entry, cx);
        div()
            .id((ROW_ID, index))
            .w_full()
            .pt(theme.stack_gap)
            .border_t(theme.border_width_hairline)
            .border_color(theme.border_subtle)
            .flex()
            .flex_col()
            .gap(theme.spacing_xs)
            .child(div().text_size(theme.font_size_sm).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_primary).child(entry.title()))
            .child(caption(meta, theme))
            .child(div().text_size(theme.font_size_xs).text_color(theme.text_secondary).child(excerpt_around(entry, query, EXCERPT_RADIUS)))
            .child(control_row(theme).child(edit).child(post).child(delete))
            .context_menu(move |popup, window, cx| render_menu(popup, &menu, window, cx))
            .into_any_element()
    }
}
