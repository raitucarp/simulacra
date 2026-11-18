//! [`NumberSlider`]: a labeled kit slider over any whole-number range (age, photo zoom),
//! with a formatter for the shown value. Reports the value on release.

use super::slider_slot::{keyboard_slider, keyed_slider, SliderRange};
use super::suggest::{typical_band, why_button, WhyCard};
use crate::state::forge::suggest::RangeHint;
use super::ValueCallback;
use crate::theme::ThemeTokens;
use gpui_kit::*;
use std::rc::Rc;

const KEYS_SUFFIX: &str = "keys";

/// Formats the value shown next to the label.
pub type ValueFormat = Rc<dyn Fn(u16) -> String>;

/// A slider row: label on the left, formatted value on the right, slider below.
#[derive(IntoElement)]
pub struct NumberSlider {
    id: SharedString,
    label: String,
    value: u16,
    range: SliderRange,
    format: ValueFormat,
    hint: Option<(RangeHint, WhyCard)>,
    theme: ThemeTokens,
    on_change: ValueCallback<u16>,
}

impl NumberSlider {
    /// A slider for `value` in `min..=max` moving by `step`.
    pub fn new(
        id: impl Into<SharedString>,
        label: impl Into<String>,
        value: u16,
        (min, max, step): (u16, u16, u16),
        theme: &ThemeTokens,
        on_change: impl Fn(u16, &mut Window, &mut App) + 'static,
    ) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            value,
            range: SliderRange { min, max, step, ceiling: max },
            format: Rc::new(|value| value.to_string()),
            hint: None,
            theme: theme.clone(),
            on_change: Rc::new(on_change),
        }
    }

    /// Draws the typical band of `hint` under the track and a "Why?" pill opening `why`.
    pub fn hint(mut self, hint: Option<(RangeHint, WhyCard)>) -> Self {
        self.hint = hint;
        self
    }

    /// How the value is written next to the label.
    pub fn format(mut self, format: impl Fn(u16) -> String + 'static) -> Self {
        self.format = Rc::new(format);
        self
    }
}

impl RenderOnce for NumberSlider {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let binding = keyed_slider(self.id.clone(), self.value, self.range, self.on_change.clone(), window, cx);
        let shown = (self.format)(binding.dragging.unwrap_or(self.value));
        let theme = &self.theme;
        let why = self.hint.as_ref().map(|(_, card)| why_button(format!("{}-why", self.id), card.clone(), theme));
        let name = div().flex().items_center().gap(theme.spacing_xs).child(div().text_size(theme.font_size_sm).text_color(theme.text_primary).child(self.label)).children(why);
        let header = div()
            .flex()
            .items_center()
            .justify_between()
            .child(name)
            .child(div().text_size(theme.font_size_xs).font_weight(FontWeight::SEMIBOLD).text_color(theme.accent_brand).child(shown));
        let keys_id = SharedString::from(format!("{}-{KEYS_SUFFIX}", self.id));
        let slider = keyboard_slider(keys_id, &binding, (self.value, self.range), false, theme, self.on_change.clone());
        let band = self.hint.as_ref().map(|(hint, _)| typical_band(hint, theme));
        div().flex().flex_col().gap(theme.spacing_xs).child(header).child(slider).children(band)
    }
}
