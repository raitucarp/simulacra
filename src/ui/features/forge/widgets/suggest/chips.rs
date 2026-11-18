//! [`HintChips`]: example values under a field. Click uses one, Shift-click adds it to what is
//! typed (AI chips always replace). Below the wide layout the chips collapse to one row with "More"; AI suggestions draw
//! in the brand tint with a sparkle.

use super::super::ValueCallback;
use crate::state::forge::suggest::Joiner;
use crate::theme::ThemeTokens;
use crate::ui::features::forge::text::tr;
use crate::ui::icons::LucideIcon;
use crate::ui::responsive::LayoutClass;
use gpui_kit::component::Icon;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;

const MORE_KEY: &str = "suggest.ui.more";
const LESS_KEY: &str = "suggest.ui.less";
const EXPANDED_SUFFIX: &str = "expanded";
const TOGGLE_SUFFIX: &str = "toggle";

/// One chip: what it shows and the text it puts in the field.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChipSpec {
    pub label: String,
    pub value: String,
}

/// A row of example chips bound to one text value.
#[derive(IntoElement)]
pub struct HintChips {
    id: SharedString,
    chips: Vec<ChipSpec>,
    current: String,
    joiner: Joiner,
    ai: bool,
    theme: ThemeTokens,
    on_set: ValueCallback<String>,
}

impl HintChips {
    /// Chips for a field currently holding `current`; `on_set` gets the new full text.
    pub fn new(id: impl Into<SharedString>, chips: Vec<ChipSpec>, (current, joiner): (String, Joiner), theme: &ThemeTokens, on_set: ValueCallback<String>) -> Self {
        Self { id: id.into(), chips, current, joiner, ai: false, theme: theme.clone(), on_set }
    }

    /// Draws them as AI suggestions (brand tint, sparkle, never collapsed).
    pub fn ai(mut self) -> Self {
        self.ai = true;
        self
    }

    fn chip(&self, at: usize, chip: &ChipSpec) -> Stateful<Div> {
        let theme = &self.theme;
        let (border, bg, fg) = if self.ai {
            (theme.accent_brand_border, theme.accent_brand_subtle, theme.accent_brand)
        } else {
            (theme.border_subtle, theme.bg_card, theme.text_secondary)
        };
        let (value, current, joiner, on_set, ai) = (chip.value.clone(), self.current.clone(), self.joiner, self.on_set.clone(), self.ai);
        let hover = theme.bg_elevated;
        div()
            .id((self.id.clone(), at))
            .flex_shrink_0()
            .max_w(theme.suggest_chip_max_width)
            .flex()
            .items_center()
            .gap(theme.spacing_2xs)
            .px(theme.spacing_sm)
            .py(theme.spacing_3xs)
            .rounded(theme.radius_full)
            .border(theme.border_width_hairline)
            .border_color(border)
            .bg(bg)
            .text_size(theme.font_size_xs)
            .text_color(fg)
            .cursor_pointer()
            .hover(move |style| style.bg(hover))
            .when(self.ai, |chip| chip.child(Icon::new(LucideIcon::Sparkles).size(theme.font_size_xs)))
            .child(div().min_w_0().overflow_hidden().whitespace_nowrap().text_ellipsis().child(chip.label.clone()))
            .on_click(move |event: &ClickEvent, window, cx| {
                let text = if event.modifiers().shift && !ai { joiner.join(&current, &value) } else { value.clone() };
                on_set(text, window, cx);
            })
    }

    fn toggle(&self, expanded: &Entity<bool>, open: bool) -> Stateful<Div> {
        let theme = &self.theme;
        let state = expanded.clone();
        let (icon, key) = if open { (LucideIcon::ChevronUp, LESS_KEY) } else { (LucideIcon::ChevronDown, MORE_KEY) };
        div()
            .id(SharedString::from(format!("{}-{TOGGLE_SUFFIX}", self.id)))
            .flex_shrink_0()
            .flex()
            .items_center()
            .gap(theme.spacing_3xs)
            .px(theme.spacing_xs)
            .text_size(theme.font_size_xs)
            .text_color(theme.text_muted)
            .cursor_pointer()
            .child(Icon::new(icon).size(theme.font_size_xs))
            .child(tr(key))
            .on_click(move |_, _, cx| state.update(cx, |open, cx| {
                *open = !*open;
                cx.notify();
            }))
    }
}

impl RenderOnce for HintChips {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = self.theme.clone();
        let key = SharedString::from(format!("{}-{EXPANDED_SUFFIX}", self.id));
        let expanded = window.use_keyed_state(key, cx, |_, _| false);
        let open = *expanded.read(cx);
        let narrow = LayoutClass::of_window(window, &theme) != LayoutClass::Wide;
        let collapsed = narrow && !self.ai && !open && self.chips.len() > 1;
        let chips: Vec<Stateful<Div>> = self.chips.iter().enumerate().map(|(at, chip)| self.chip(at, chip)).collect();
        let toggle = (narrow && !self.ai && self.chips.len() > 1).then(|| self.toggle(&expanded, open));
        let row = div().flex().items_center().gap(theme.spacing_xs);
        let body = if collapsed {
            row.child(div().flex_1().min_w_0().flex().gap(theme.spacing_xs).overflow_hidden().children(chips)).children(toggle)
        } else {
            row.flex_wrap().children(chips).children(toggle)
        };
        body.into_any_element()
    }
}
