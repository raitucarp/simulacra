//! The disclosed sections of the form: publication details, classification, abstract.

use super::super::common::card;
use super::controls::frame;
use super::pair;
use crate::domain::document::record::{Discipline, DocumentForm, IdentifierScheme, Period};
use crate::ui::features::vault::editor::fields::TextField;
use crate::ui::features::vault::editor::EditorSection;
use crate::ui::features::vault::panel::VaultPanel;
use crate::ui::features::vault::text::{discipline_label, form_label, period_label, scheme_label};
use crate::ui::icons::LucideIcon;
use gpui_kit::component::input::Textarea;
use gpui_kit::component::Icon;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use crate::ui::features::vault::text::tr;
use rust_i18n::t;

impl VaultPanel {
    pub(super) fn render_sections(&self, compact: bool, cx: &mut Context<Self>) -> Vec<AnyElement> {
        EditorSection::ALL.into_iter().map(|section| self.render_section(section, compact, cx)).collect()
    }

    fn render_section(&self, section: EditorSection, compact: bool, cx: &mut Context<Self>) -> AnyElement {
        let theme = &self.theme;
        let open = self.editor.open.contains(&section);
        let header = div()
            .id(ElementId::Name(format!("vault-section-{}", section.key()).into()))
            .flex()
            .items_center()
            .gap(theme.inline_gap)
            .cursor_pointer()
            .on_click(cx.listener(move |this, _, _, cx| this.toggle_section(section, cx)))
            .child(Icon::new(if open { LucideIcon::ChevronDown } else { LucideIcon::ChevronRight }).size(theme.icon_size_sm).text_color(theme.text_muted))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .child(div().text_size(theme.font_size_sm).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_primary).child(tr(format!("vault.section.{}", section.key()))))
                    .child(div().text_size(theme.font_size_xs).text_color(theme.text_muted).child(tr(format!("vault.section.{}_hint", section.key())))),
            );
        let body = open.then(|| match section {
            EditorSection::Publication => self.render_publication(compact, cx),
            EditorSection::Classification => self.render_classification(compact, cx),
            EditorSection::Abstract => frame(Textarea::new(&self.editor.description).bordered(false), false, theme),
        });
        card(theme).child(header).when_some(body, |c, body| c.child(body)).into_any_element()
    }

    fn render_publication(&self, compact: bool, cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        let schemes = self.choice_row("vault-scheme", IdentifierScheme::ALL, self.editor.choices.scheme, scheme_label, |d, s| d.scheme = s, cx);
        div()
            .flex()
            .flex_col()
            .gap(theme.stack_gap)
            .child(self.text_field(TextField::Subtitle, None))
            .child(pair(compact, theme).child(self.text_field(TextField::Publisher, None)).child(self.text_field(TextField::Language, Some(t!("vault.hint.language_note").to_string()))))
            .child(div().flex().flex_col().gap(theme.spacing_xs).child(schemes).child(self.text_field(TextField::Identifier, None)))
            .child(pair(compact, theme).child(self.text_field(TextField::Pages, None)).child(self.text_field(TextField::Rights, None)))
    }

    fn render_classification(&self, compact: bool, cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        let choices = &self.editor.choices;
        let disciplines = self.choice_row("vault-discipline", Discipline::ALL, choices.discipline, discipline_label, |d, v| d.discipline = v, cx);
        let forms = self.choice_row("vault-form", DocumentForm::ALL, choices.form, form_label, |d, v| d.form = v, cx);
        let periods = self.choice_row("vault-period", Period::ALL, choices.period, period_label, |d, v| d.period = v, cx);
        let labelled = |key: &str, row: Div| super::controls::labelled(tr(format!("vault.field.{key}")), None, None, row, theme);
        let known: Vec<String> = self.data.library.subjects.iter().map(|s| s.label.clone()).collect();
        let shelves: Vec<String> = self.data.library.collections_of(&self.data.network_id).iter().map(|c| c.name.clone()).collect();
        div()
            .flex()
            .flex_col()
            .gap(theme.stack_gap)
            .child(self.text_field(TextField::Subjects, Some(t!("vault.hint.subjects_note").to_string())))
            .child(self.term_chips(TextField::Subjects, known, cx))
            .child(self.text_field(TextField::Collections, None))
            .child(self.term_chips(TextField::Collections, shelves, cx))
            .child(labelled("discipline", disciplines))
            .child(labelled("form", forms))
            .child(pair(compact, theme).child(labelled("period", periods)).child(self.text_field(TextField::Place, None)))
    }
}
