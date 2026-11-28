//! The choice an action needs: audience chips, locale chips or concept chips, each with a
//! field for a typed value (a custom reader, any language, any concept).

use super::labels::audience_label;
use super::model::{ActionChoice, ActionSheet, SheetEvent};
use super::sheet::{SheetAction, SheetInputs};
use crate::domain::post_actions::{ActionInput, Audience, AudiencePreset, TargetLanguage};
use crate::i18n::Language;
use crate::theme::ThemeTokens;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::input::Input;
use gpui_kit::component::Sizable;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use rust_i18n::t;

const CHIP_ID_PREFIX: &str = "post-action-choice-";
const APPLY_ID: &str = "post-action-apply";

/// The chips and the typed field, or nothing when the action needs no choice.
pub(super) fn render_choices(inputs: &SheetInputs<'_>) -> Option<Div> {
    let (sheet, theme) = (inputs.sheet, inputs.theme);
    let chips: Vec<(ActionChoice, String, bool)> = match sheet.request.kind.input() {
        ActionInput::None => return None,
        ActionInput::Audience => audience_chips(sheet),
        ActionInput::Language => language_chips(sheet),
        ActionInput::Concept => sheet.concepts.iter().map(|c| (ActionChoice::Concept(c.clone()), c.clone(), sheet.request.concept == *c)).collect(),
    };
    let empty = chips.is_empty().then(|| div().text_size(theme.font_size_2xs).text_color(theme.text_muted).child(t!("post_actions.concept_none").to_string()));
    let row = div()
        .flex()
        .flex_wrap()
        .gap(theme.control_gap)
        .children(chips.into_iter().enumerate().map(|(index, (choice, label, selected))| chip(index, choice, label, selected, inputs.on_event.clone(), theme)));
    Some(div().flex().flex_col().gap(theme.stack_gap).child(row).children(empty).child(typed_row(inputs)))
}

fn audience_chips(sheet: &ActionSheet) -> Vec<(ActionChoice, String, bool)> {
    AudiencePreset::ALL
        .iter()
        .map(|preset| (ActionChoice::Audience(*preset), audience_label(*preset), sheet.request.audience == Audience::Preset(*preset)))
        .collect()
}

fn language_chips(sheet: &ActionSheet) -> Vec<(ActionChoice, String, bool)> {
    Language::ALL
        .iter()
        .map(|language| (ActionChoice::Language(*language), language.display_label().into_owned(), sheet.request.language == TargetLanguage::Locale(*language)))
        .collect()
}

fn chip(index: usize, choice: ActionChoice, label: String, selected: bool, on_event: SheetAction, theme: &ThemeTokens) -> Stateful<Div> {
    let hover = theme.hover_bg;
    div()
        .id(SharedString::from(format!("{CHIP_ID_PREFIX}{index}")))
        .px(theme.spacing_sm)
        .py(theme.spacing_3xs)
        .rounded(theme.radius_full)
        .border(theme.border_width_hairline)
        .cursor_pointer()
        .text_size(theme.font_size_2xs)
        .map(|chip| match selected {
            true => chip.bg(theme.accent_brand_subtle).border_color(theme.accent_brand_border).text_color(theme.text_primary),
            false => chip.border_color(theme.border_subtle).text_color(theme.text_secondary).hover(move |s| s.bg(hover)),
        })
        .on_click(move |_, window, cx| on_event(SheetEvent::Choose(choice.clone()), window, cx))
        .child(label)
}

/// The field for a typed value and its "use" button (Enter works too).
fn typed_row(inputs: &SheetInputs<'_>) -> Div {
    let on_event = inputs.on_event.clone();
    let apply = Button::new(APPLY_ID)
        .ghost()
        .small()
        .label(t!("post_actions.apply").to_string())
        .on_click(move |_, window, cx| on_event(SheetEvent::ApplyTyped, window, cx));
    div()
        .flex()
        .items_center()
        .gap(inputs.theme.control_gap)
        .child(div().flex_1().min_w_0().child(Input::new(inputs.input).small()))
        .child(apply)
}
