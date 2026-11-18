//! Suggestions on the option pages (sb D1): a caption with "?", "Why?" and "Suggest" for any
//! parameter, example chips under text fields, typical bands for sliders, "Suggested" picks
//! for option grids, AI values under their field, and the step's suggestion bar.

mod ai_chips;
mod step_bar;
mod why_card;

use super::super::common::message;
use crate::state::forge::suggest::ai::SuggestScope;
use crate::state::forge::suggest::{picks_for, range_hint, suggestions_for, FieldSuggestions, Param, SuggestContext};
use crate::state::forge::{DraftEdit, TextField};
use crate::ui::features::forge::text::{field_label, tr};
use crate::ui::features::forge::view::{ForgeHandle, ForgeView};
use crate::ui::features::forge::widgets::suggest::{why_button, ChipSpec, HintChips, SliderHint};
use crate::ui::features::forge::widgets::ExplainerButton;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::Sizable;
use gpui_kit::*;

const ID_PREFIX: &str = "forge-hint";
const SUGGEST_FIELD_KEY: &str = "suggest.ui.suggest_field";

impl ForgeView {
    /// Starting picks of `param` (the "Suggested" badges of its grid).
    pub(super) fn picks(&self, param: Param) -> Vec<&'static str> {
        picks_for(param, &SuggestContext::of(&self.draft))
    }

    /// The suggestions of a slider over `param` titled `label`: band, "Why?", "Suggest", AI values.
    pub(super) fn slider_hint(&self, param: Param, label: &str, handle: &ForgeHandle) -> Option<SliderHint> {
        let band = range_hint(param, &SuggestContext::of(&self.draft))?;
        let ask = param.ai_capable().then(|| {
            let ask = handle.act(move |this, _, cx| this.request_suggestions(SuggestScope::Field(param), cx));
            (ask, self.suggest.is_running(SuggestScope::Field(param)))
        });
        let why = self.why_card(param, Some(label.to_string()));
        Some(SliderHint { band, why, ask, ai: self.ai_chip_specs(param), on_ai: self.ai_accept(param, handle) })
    }

    /// "Why?" plus, when the AI may suggest `param`, a sparkle asking it for values.
    pub(super) fn hint_actions(&self, param: Param, handle: &ForgeHandle) -> Div {
        let theme = &self.theme;
        let why = why_button(format!("{ID_PREFIX}-why-{}", param.path()), self.why_card(param, None), theme);
        let running = self.suggest.is_running(SuggestScope::Field(param));
        let ask = handle.act(move |this, _, cx| this.request_suggestions(SuggestScope::Field(param), cx));
        let sparkle = param.ai_capable().then(|| {
            Button::new(SharedString::from(format!("{ID_PREFIX}-ai-{}", param.path())))
                .ghost()
                .xsmall()
                .icon(LucideIcon::Sparkles)
                .loading(running)
                .tooltip(tr(SUGGEST_FIELD_KEY))
                .on_click(move |_, w, cx| ask(w, cx))
        });
        div().flex().items_center().gap(theme.spacing_2xs).child(why).children(sparkle)
    }

    /// Caption of `param`: its name and "?" explainer, then "Why?" and "Suggest".
    pub(super) fn hint_caption(&self, param: Param, handle: &ForgeHandle) -> Div {
        let theme = &self.theme;
        let name = div()
            .text_size(theme.font_size_xs)
            .font_weight(FontWeight::SEMIBOLD)
            .text_color(theme.text_secondary)
            .child(field_label(param.path()));
        div()
            .flex()
            .items_center()
            .gap(theme.spacing_xs)
            .child(name)
            .children(ExplainerButton::for_field(param.path(), theme))
            .child(div().flex_1())
            .child(self.hint_actions(param, handle))
    }

    /// `control` under the hint caption of `param`, its AI values and any clash it is in.
    pub(super) fn hinted(&self, param: Param, control: impl IntoElement, handle: &ForgeHandle) -> Div {
        let theme = &self.theme;
        let clashes = self.clash_lines(param).into_iter().map(|line| message(line, theme.warning, theme));
        div()
            .flex()
            .flex_col()
            .gap(theme.spacing_xs)
            .child(self.hint_caption(param, handle))
            .child(control)
            .children(self.ai_chips(param, handle))
            .children(clashes)
    }

    /// Example chips under text `field` (click uses one, Shift-click adds it).
    pub(super) fn text_examples(&self, field: TextField, handle: &ForgeHandle) -> Option<HintChips> {
        let FieldSuggestions::Text(hints) = suggestions_for(Param::Text(field), &SuggestContext::of(&self.draft)) else { return None };
        let chips = hints.examples.iter().map(|example| ChipSpec { label: why_card::text(&example.label), value: why_card::text(&example.value) }).collect();
        let current = field.get(self.draft.profile()).clone();
        let set = handle.with(move |this, text: String, _, cx| {
            this.draft.seal_history();
            this.apply_edit(DraftEdit::Text(field, text), cx);
            this.draft.seal_history();
        });
        Some(HintChips::new(format!("{ID_PREFIX}-ex-{}", field.path()), chips, (current, hints.joiner), &self.theme, set))
    }
}
