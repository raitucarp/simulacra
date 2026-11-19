//! Form controls of the editor: labelled fields with their problem, and choice chips.

use crate::domain::document::intake::{FieldProblem, RecordField};
use crate::theme::ThemeTokens;
use crate::ui::components::{chip_row, choice_chip};
use crate::ui::features::vault::editor::draft::split_list;
use crate::ui::features::vault::editor::fields::TextField;
use crate::ui::features::vault::panel::VaultPanel;
use crate::ui::features::vault::text::problem_message;
use gpui_kit::component::input::Input;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;

/// The record field a text field is validated as.
fn validated_as(field: TextField) -> Option<RecordField> {
    match field {
        TextField::Title => Some(RecordField::Title),
        TextField::Date => Some(RecordField::Date),
        TextField::Language => Some(RecordField::Language),
        TextField::Identifier => Some(RecordField::Identifier),
        TextField::Pages => Some(RecordField::Pages),
        _ => None,
    }
}

/// A label above `control`, an optional hint, and the problem in the danger color.
pub fn labelled(label: String, hint: Option<String>, problem: Option<FieldProblem>, control: impl IntoElement, theme: &ThemeTokens) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(theme.spacing_xs)
        .flex_1()
        .min_w_0()
        .child(div().text_size(theme.font_size_xs).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_secondary).child(label))
        .child(control)
        .when_some(hint, |f, hint| f.child(div().text_size(theme.font_size_2xs).text_color(theme.text_muted).child(hint)))
        .when_some(problem, |f, p| f.child(div().text_size(theme.font_size_xs).text_color(theme.accent_danger).child(problem_message(p))))
}

/// An inset frame around an input, outlined in the danger color while it has a problem.
pub fn frame(control: impl IntoElement, invalid: bool, theme: &ThemeTokens) -> Div {
    div()
        .w_full()
        .rounded(theme.radius_md)
        .border(theme.border_width_hairline)
        .border_color(if invalid { theme.accent_danger } else { theme.border })
        .bg(theme.input_bg)
        .px(theme.spacing_sm)
        .child(control)
}

impl VaultPanel {
    /// The labelled input of `field` (with `hint` below it).
    pub(super) fn text_field(&self, field: TextField, hint: Option<String>) -> Div {
        let theme = &self.theme;
        let problem = validated_as(field).and_then(|f| self.editor.problem(f));
        let input = frame(Input::new(self.editor.input(field)).bordered(false), problem.is_some(), theme);
        labelled(field.label(), hint, problem, input, theme)
    }

    /// A row of chips choosing one of `options` (choosing the chosen one clears it when
    /// `optional`).
    pub(super) fn choice_row<T: Copy + PartialEq + 'static>(
        &self,
        id: &str,
        options: &[T],
        current: Option<T>,
        label: impl Fn(T) -> String,
        set: impl Fn(&mut crate::ui::features::vault::editor::draft::EditorDraft, Option<T>) + Clone + 'static,
        cx: &mut Context<Self>,
    ) -> Div {
        let theme = &self.theme;
        let panel = cx.entity().downgrade();
        chip_row(theme).children(options.iter().enumerate().map(|(at, option)| {
            let (option, selected, panel, set) = (*option, current == Some(*option), panel.clone(), set.clone());
            choice_chip(ElementId::Name(format!("{id}-{at}").into()), &label(option), selected, theme, move |_, cx| {
                let set = set.clone();
                let value = if selected { None } else { Some(option) };
                let _ = panel.update(cx, |this, cx| this.update_choices(|draft| set(draft, value), cx));
            })
        }))
    }

    /// Chips of known terms for a list field: chosen ones are highlighted, a click adds or
    /// removes the term (recognition instead of recall).
    pub(super) fn term_chips(&self, field: TextField, terms: Vec<String>, cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        let chosen: Vec<String> = split_list(&self.editor.input(field).read(cx).value()).iter().map(|t| t.to_lowercase()).collect();
        let panel = cx.entity().downgrade();
        chip_row(theme).children(terms.into_iter().enumerate().map(|(at, term)| {
            let selected = chosen.contains(&term.to_lowercase());
            let (panel, label) = (panel.clone(), term.clone());
            choice_chip(ElementId::Name(format!("vault-term-{}-{at}", field.key()).into()), &term, selected, theme, move |window, cx| {
                let _ = panel.update(cx, |this, cx| this.toggle_term(field, &label, window, cx));
            })
        }))
    }
}
