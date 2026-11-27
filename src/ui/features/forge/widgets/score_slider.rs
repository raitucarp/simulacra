//! [`ScoreSlider`]: the kit slider bound to a `ScoreField`, with the field name, a level name
//! (Novice…Legendary for intelligences, very low…very high otherwise) and the personality
//! point budget enforced: a budgeted score cannot be dragged past the points left (unless
//! the budget is switched off with [`ScoreSlider::budgeted`]). The field's "?" explainer sits
//! next to its name when it has one.
//!
//! The value is committed on release (one undo step per drag); while dragging, the label
//! follows the thumb.

use super::explainer::ExplainerButton;
use super::slider_slot::{keyboard_slider, keyed_slider, SliderRange};
use super::suggest::SliderHint;
use super::ValueCallback;
use crate::domain::agent_config::AgentConfig;
use crate::state::forge::attributes::{budget_remaining, BUDGET_FIELDS};
use crate::state::forge::fields::MAX_SCORE;
use crate::state::forge::ScoreField;
use crate::theme::ThemeTokens;
use crate::ui::features::forge::logic::score_level_key;
use crate::ui::features::forge::text::{field_label, tr};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use std::rc::Rc;

const SLIDER_ID_PREFIX: &str = "forge-score";
const NO_POINTS_KEY: &str = "forge.ui.no_points";
const KEYS_SUFFIX: &str = "keys";
/// One slider notch.
const SCORE_STEP: u16 = 1;

/// A labeled 0–100 slider for one score field.
#[derive(IntoElement)]
pub struct ScoreSlider {
    field: ScoreField,
    value: u8,
    ceiling: u8,
    disabled: bool,
    hint: Option<SliderHint>,
    theme: ThemeTokens,
    on_change: ValueCallback<u8>,
}

impl ScoreSlider {
    /// A slider showing `field` on `cfg`; `on_change` gets the released value.
    pub fn new(field: ScoreField, cfg: &AgentConfig, theme: &ThemeTokens, on_change: impl Fn(u8, &mut Window, &mut App) + 'static) -> Self {
        let value = *field.of(cfg);
        let ceiling = if BUDGET_FIELDS.contains(&field) {
            let room = budget_remaining(cfg).max(0);
            (i32::from(value) + room).min(i32::from(MAX_SCORE)) as u8
        } else {
            MAX_SCORE
        };
        Self { field, value, ceiling, disabled: false, hint: None, theme: theme.clone(), on_change: Rc::new(on_change) }
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Shows the slider's suggestions: "Why?" and "Suggest" by the name, the typical band and
    /// any AI values under the track.
    pub fn hint(mut self, hint: Option<SliderHint>) -> Self {
        self.hint = hint;
        self
    }

    /// Whether the balanced-build budget caps this slider (on by default); off frees every
    /// score up to the maximum.
    pub fn budgeted(mut self, budgeted: bool) -> Self {
        if !budgeted {
            self.ceiling = MAX_SCORE;
        }
        self
    }

    fn header(&self, shown: u8) -> Div {
        let theme = &self.theme;
        let level = div()
            .text_size(theme.font_size_xs)
            .text_color(theme.accent_brand_text)
            .font_weight(FontWeight::SEMIBOLD)
            .child(format!("{} · {shown}", tr(&score_level_key(self.field, shown))));
        let name = div()
            .flex()
            .items_center()
            .gap(theme.spacing_xs)
            .child(div().text_size(theme.font_size_sm).text_color(theme.text_primary).child(field_label(self.field.path())))
            .children(ExplainerButton::for_field(self.field.path(), theme))
            .children(self.hint.as_ref().map(|hint| hint.header(&format!("{SLIDER_ID_PREFIX}-{}", self.field.path()), theme)));
        div().flex().items_center().justify_between().child(name).child(level)
    }
}

impl RenderOnce for ScoreSlider {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let id = SharedString::from(format!("{SLIDER_ID_PREFIX}-{}", self.field.path()));
        let range = SliderRange { min: 0, max: u16::from(MAX_SCORE), step: SCORE_STEP, ceiling: u16::from(self.ceiling) };
        let on_change = self.on_change.clone();
        let on_release: ValueCallback<u16> = Rc::new(move |value, window, cx| on_change(value.min(u16::from(MAX_SCORE)) as u8, window, cx));
        let binding = keyed_slider(id.clone(), u16::from(self.value), range, on_release.clone(), window, cx);
        let shown = binding.dragging.map_or(self.value, |live| live.min(u16::from(MAX_SCORE)) as u8);
        let at_ceiling = self.ceiling <= self.value && self.ceiling < MAX_SCORE;
        let theme = &self.theme;
        div()
            .flex()
            .flex_col()
            .gap(theme.spacing_xs)
            .child(self.header(shown))
            .child(keyboard_slider(SharedString::from(format!("{id}-{KEYS_SUFFIX}")), &binding, (u16::from(self.value), range), self.disabled, theme, on_release))
            .children(self.hint.as_ref().map(|hint| hint.footer(&id, theme)))
            .when(at_ceiling, |col| {
                col.child(div().text_size(theme.font_size_2xs).text_color(theme.warning).child(tr(NO_POINTS_KEY)))
            })
    }
}
