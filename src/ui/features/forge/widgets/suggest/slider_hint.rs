//! [`SliderHint`]: everything a slider shows about its suggestions — "Why?" and "Suggest" in
//! its header, the typical band and any AI values under its track.

use super::super::{ActionCallback, ValueCallback};
use super::band::typical_band;
use super::chips::{ChipSpec, HintChips};
use super::why::{why_button, WhyCard};
use crate::state::forge::suggest::{Joiner, RangeHint};
use crate::theme::ThemeTokens;
use crate::ui::features::forge::text::tr;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::Sizable;
use gpui_kit::*;

const SUGGEST_FIELD_KEY: &str = "suggest.ui.suggest_field";
const WHY_SUFFIX: &str = "why";
const ASK_SUFFIX: &str = "ask";
const AI_SUFFIX: &str = "ai";

/// The suggestions of one slider.
#[derive(Clone)]
pub struct SliderHint {
    pub band: RangeHint,
    pub why: WhyCard,
    /// "Suggest from context" for this slider, and whether it is running.
    pub ask: Option<(ActionCallback, bool)>,
    /// AI values waiting for this slider (the chip value is the value's index).
    pub ai: Vec<ChipSpec>,
    pub on_ai: ValueCallback<String>,
}

impl SliderHint {
    /// "Why?" and the sparkle, for the slider's header.
    pub fn header(&self, id: &str, theme: &ThemeTokens) -> Div {
        let why = why_button(format!("{id}-{WHY_SUFFIX}"), self.why.clone(), theme);
        let ask = self.ask.clone().map(|(ask, running)| {
            Button::new(SharedString::from(format!("{id}-{ASK_SUFFIX}")))
                .ghost()
                .xsmall()
                .icon(LucideIcon::Sparkles)
                .loading(running)
                .tooltip(tr(SUGGEST_FIELD_KEY))
                .on_click(move |_, w, cx| ask(w, cx))
        });
        div().flex().items_center().gap(theme.spacing_2xs).child(why).children(ask)
    }

    /// The typical band and the AI values, under the track.
    pub fn footer(&self, id: &str, theme: &ThemeTokens) -> Div {
        let ai = (!self.ai.is_empty()).then(|| {
            HintChips::new(format!("{id}-{AI_SUFFIX}"), self.ai.clone(), (String::new(), Joiner::Space), theme, self.on_ai.clone()).ai()
        });
        div().flex().flex_col().gap(theme.spacing_xs).child(typical_band(&self.band, theme)).children(ai)
    }
}
