//! The palette preview: art, description and "What this palette sets", grouped by step, plus
//! "Begin as <palette>". The change rows are shared with the step palette strip.

use super::super::common::{heading, muted};
use super::gallery::palette_art;
use crate::domain::{Palette, TraitOption};
use crate::state::forge::{preview_palette, ChangeValue, FieldChange, ForgeStep, TraitValue, TraitVisitor};
use crate::theme::ThemeTokens;
use crate::ui::features::forge::text::{set_labels, tr, tr_count, tr_with, trait_label};
use crate::ui::features::forge::view::{ForgeHandle, ForgeView};
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::*;

/// Changes listed per step before "+N more".
const ROWS_PER_STEP: usize = 3;
const BEGIN_ID: &str = "forge-begin-as";
const BEGIN_KEY: &str = "forge.ui.start.begin_as";
const CHANGES_KEY: &str = "forge.palette.ui.what_sets";
const NO_CHANGES_KEY: &str = "forge.palette.ui.no_changes";
const MORE_KEY: &str = "forge.ui.start.more_changes";
const ON_KEY: &str = "forge.ui.on";
const OFF_KEY: &str = "forge.ui.off";
const ARG_CLASS: &str = "class";
const NONE_KEY: &str = "forge.ui.multi.none";
const LIST_SEPARATOR: &str = ", ";
const ARROW: &str = "→";
/// Longest value shown in a change row before it is cut with an ellipsis.
const MAX_VALUE_CHARS: usize = 32;
const ELLIPSIS: char = '…';

/// `text` cut to [`MAX_VALUE_CHARS`] characters.
fn shortened(text: String) -> String {
    if text.chars().count() <= MAX_VALUE_CHARS {
        return text;
    }
    let mut cut: String = text.chars().take(MAX_VALUE_CHARS).collect();
    cut.push(ELLIPSIS);
    cut
}

/// Display name of a trait value at its concrete option type.
struct LabelOf;

impl TraitVisitor for LabelOf {
    type Output = String;

    fn visit<T: TraitOption>(self, current: T, _: fn(T) -> TraitValue) -> String {
        trait_label(current)
    }
}

fn value_text(value: &ChangeValue) -> String {
    shortened(match value {
        ChangeValue::Text(text) => text.clone(),
        ChangeValue::Number(number) => number.to_string(),
        ChangeValue::Flag(flag) => tr(if *flag { ON_KEY } else { OFF_KEY }),
        ChangeValue::Trait(value) => value.visit(LabelOf),
        ChangeValue::Set(value) => set_labels(value),
        ChangeValue::Key(key) => tr(key),
        ChangeValue::Keys(keys) if keys.is_empty() => tr(NONE_KEY),
        ChangeValue::Keys(keys) => keys.iter().map(|key| tr(key)).collect::<Vec<_>>().join(LIST_SEPARATOR),
    })
}

fn change_row(change: &FieldChange, theme: &ThemeTokens) -> Div {
    div()
        .flex()
        .flex_wrap()
        .gap(theme.spacing_xs)
        .text_size(theme.font_size_xs)
        .child(div().text_color(theme.text_secondary).child(tr(&change.label_key)))
        .child(div().text_color(theme.text_muted).line_through().child(value_text(&change.before)))
        .child(div().text_color(theme.text_muted).child(ARROW))
        .child(div().text_color(theme.accent_brand_text).child(value_text(&change.after)))
}

/// The changes of one step under its name ("+N more" past the first few).
pub(in crate::ui::features::forge::screens) fn step_group(step: ForgeStep, changes: &[&FieldChange], theme: &ThemeTokens) -> Div {
    let more = changes.len().saturating_sub(ROWS_PER_STEP);
    div()
        .flex()
        .flex_col()
        .gap(theme.spacing_2xs)
        .child(div().text_size(theme.font_size_xs).font_weight(FontWeight::BOLD).child(tr(&step.label_key())))
        .children(changes.iter().take(ROWS_PER_STEP).map(|change| change_row(change, theme)))
        .children((more > 0).then(|| muted(tr_count(MORE_KEY, more, &[]), theme)))
}

impl ForgeView {
    pub(super) fn palette_preview(&self, palette: &'static Palette, handle: &ForgeHandle) -> Div {
        let theme = &self.theme;
        let name = tr(&palette.label_key());
        let begin = handle.act(move |this, window, cx| this.begin_as(palette, window, cx));
        div()
            .flex()
            .flex_col()
            .gap(theme.spacing_md)
            .child(div().flex().justify_center().child(palette_art(palette).width(theme.forge_class_card_width)))
            .child(heading(name.clone(), theme))
            .child(muted(tr(&palette.description_key()), theme))
            .child(
                Button::new(BEGIN_ID)
                    .primary()
                    .icon(LucideIcon::ArrowRight)
                    .label(tr_with(BEGIN_KEY, &[(ARG_CLASS, name)]))
                    .on_click(move |_, w, cx| begin(w, cx)),
            )
            .child(self.what_changes(palette))
    }

    fn what_changes(&self, palette: &Palette) -> Div {
        let theme = &self.theme;
        let changes = preview_palette(&self.draft, palette);
        let groups: Vec<Div> = self
            .draft
            .steps()
            .iter()
            .filter_map(|&step| {
                let in_step: Vec<&FieldChange> = changes.iter().filter(|change| change.step == step).collect();
                (!in_step.is_empty()).then(|| step_group(step, &in_step, theme))
            })
            .collect();
        let body = if groups.is_empty() { vec![muted(tr(NO_CHANGES_KEY), theme)] } else { groups };
        div().flex().flex_col().gap(theme.spacing_sm).child(heading(tr(CHANGES_KEY), theme)).children(body)
    }
}
