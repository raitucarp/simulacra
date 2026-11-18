//! [`ForgeTextInput`]: a text field bound to a draft value. The kit input entity lives in
//! keyed element state (created the first time the field renders, dropped when it stops
//! rendering), and is re-synced whenever the bound value changes from outside (undo, class).

use super::{ActionCallback, ValueCallback};
use crate::theme::ThemeTokens;
use super::text_input_slot::{bind, Slot};
use gpui_kit::component::input::{Input, InputState, Textarea, TextareaState};
use gpui_kit::*;
use std::rc::Rc;

/// A bordered, themed text field showing `value` and reporting every edit.
#[derive(IntoElement)]
pub struct ForgeTextInput {
    id: ElementId,
    value: String,
    placeholder: SharedString,
    rows: Option<(usize, usize)>,
    invalid: bool,
    theme: ThemeTokens,
    on_change: ValueCallback<String>,
    on_blur: Option<ActionCallback>,
    /// Shown under the field (example chips).
    footer: Option<AnyElement>,
}

impl ForgeTextInput {
    /// A single-line field.
    pub fn new(
        id: impl Into<ElementId>,
        value: impl Into<String>,
        theme: &ThemeTokens,
        on_change: impl Fn(String, &mut Window, &mut App) + 'static,
    ) -> Self {
        Self {
            id: id.into(),
            value: value.into(),
            placeholder: SharedString::default(),
            rows: None,
            invalid: false,
            theme: theme.clone(),
            on_change: Rc::new(on_change),
            on_blur: None,
            footer: None,
        }
    }

    /// Makes it a multi-line area growing from `min` to `max` rows.
    pub fn multiline(mut self, min: usize, max: usize) -> Self {
        self.rows = Some((min, max));
        self
    }

    /// Hint shown while empty (applied when the field is first created).
    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// Draws the frame in the danger color.
    pub fn invalid(mut self, invalid: bool) -> Self {
        self.invalid = invalid;
        self
    }

    /// Shows `footer` under the field (e.g. example chips).
    pub fn footer(mut self, footer: impl IntoElement) -> Self {
        self.footer = Some(footer.into_any_element());
        self
    }

    /// Called when the field loses focus (e.g. to end an undo run).
    pub fn on_blur(mut self, on_blur: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_blur = Some(Rc::new(on_blur));
        self
    }
}

impl RenderOnce for ForgeTextInput {
    fn render(mut self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let footer = self.footer.take();
        let gap = self.theme.spacing_xs;
        let field = self.field(window, cx);
        match footer {
            Some(footer) => div().flex().flex_col().gap(gap).child(field).child(footer),
            None => field,
        }
    }
}

impl ForgeTextInput {
    /// The framed input or text area.
    fn field(&self, window: &mut Window, cx: &mut App) -> Div {
        let theme = &self.theme;
        let border = if self.invalid { theme.accent_danger } else { theme.border };
        let frame = div()
            .w_full()
            .rounded(theme.radius_md)
            .border(theme.border_width_hairline)
            .border_color(border)
            .bg(theme.input_bg)
            .px(theme.spacing_xs)
            .py(theme.spacing_2xs);
        let placeholder = self.placeholder.clone();
        match self.rows {
            None => {
                let slot = window.use_keyed_state(self.id.clone(), cx, |window, cx| {
                    let input = cx.new(|cx| InputState::new(window, cx).placeholder(placeholder));
                    Slot::new(input, window, cx)
                });
                let input = bind(&slot, (&self.value, &self.on_change, &self.on_blur), window, cx);
                frame.child(Input::new(&input).bordered(false))
            }
            Some((min, max)) => {
                let slot = window.use_keyed_state(self.id.clone(), cx, |window, cx| {
                    let area = cx.new(|cx| TextareaState::new(window, cx).placeholder(placeholder).auto_grow(min, max));
                    Slot::new(area, window, cx)
                });
                let area = bind(&slot, (&self.value, &self.on_change, &self.on_blur), window, cx);
                frame.child(Textarea::new(&area).bordered(false))
            }
        }
    }
}
