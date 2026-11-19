//! The add / edit form: opening it, choices, sections and saving.

use super::{EditorSubmission, VaultPanel};
use crate::domain::document::record::CatalogueRecord;
use crate::ui::features::vault::editor::draft::{split_list, EditorDraft};
use crate::ui::features::vault::editor::fields::TextField;
use crate::ui::features::vault::editor::EditorSection;
use crate::ui::features::vault::screen::Screen;
use crate::ui::features::vault::VaultEvent;
use gpui_kit::{Context, Window};

impl VaultPanel {
    /// Opens the form empty (`None`) or filled with document `id`.
    pub fn open_editor(&mut self, id: Option<String>, window: &mut Window, cx: &mut Context<Self>) {
        let draft = id
            .as_deref()
            .and_then(|id| self.data.document(id))
            .map(|doc| EditorDraft::of_document(doc, &self.data.library))
            .unwrap_or_default();
        self.editor.load(draft, window, cx);
        self.go(Screen::Editor(id), cx);
    }

    /// Changes the non-text values of the form.
    pub(in crate::ui::features::vault) fn update_choices(&mut self, change: impl FnOnce(&mut EditorDraft), cx: &mut Context<Self>) {
        change(&mut self.editor.choices);
        cx.notify();
    }

    pub(in crate::ui::features::vault) fn toggle_section(&mut self, section: EditorSection, cx: &mut Context<Self>) {
        if !self.editor.open.remove(&section) {
            self.editor.open.insert(section);
        }
        cx.notify();
    }

    /// The record the edited document has now (empty for a new one).
    fn editing_base(&self) -> (Option<String>, CatalogueRecord) {
        match &self.nav.current {
            Screen::Editor(Some(id)) => (Some(id.clone()), self.data.document(id).map(|d| d.record.clone()).unwrap_or_default()),
            _ => (None, CatalogueRecord::default()),
        }
    }

    /// Validates the form and asks the store to save it; problems stay on the form.
    pub(in crate::ui::features::vault) fn save_editor(&mut self, cx: &mut Context<Self>) {
        let draft = self.editor.read(cx);
        let (editing, base) = self.editing_base();
        let problems = draft.problems(&base);
        if !problems.is_empty() {
            self.editor.show_problems(problems);
            cx.notify();
            return;
        }
        let submission = EditorSubmission {
            editing,
            title: draft.title.trim().to_string(),
            content: draft.content.clone(),
            record: draft.record(&base),
            subject_labels: draft.subject_labels(),
            collection_names: draft.collection_names(),
        };
        cx.emit(VaultEvent::Save(submission));
    }

    /// Shows the entry of a saved document, replacing the form in the history.
    pub fn saved(&mut self, id: &str, cx: &mut Context<Self>) {
        self.nav = std::mem::take(&mut self.nav).back();
        self.go(Screen::Entry(id.to_string()), cx);
    }

    /// Shows problems the store found while saving.
    pub fn save_failed(&mut self, problems: Vec<crate::domain::document::intake::FieldProblem>, cx: &mut Context<Self>) {
        self.editor.show_problems(problems);
        cx.notify();
    }

    /// Adds `term` to the list in `field`, or removes it when it is there already.
    pub(in crate::ui::features::vault) fn toggle_term(&mut self, field: TextField, term: &str, window: &mut Window, cx: &mut Context<Self>) {
        let input = self.editor.input(field).clone();
        let mut terms = split_list(&input.read(cx).value());
        let before = terms.len();
        terms.retain(|t| !t.eq_ignore_ascii_case(term) && t.to_lowercase() != term.to_lowercase());
        if terms.len() == before {
            terms.push(term.to_string());
        }
        input.update(cx, |input, cx| input.set_value(terms.join(", "), window, cx));
        cx.notify();
    }
}
