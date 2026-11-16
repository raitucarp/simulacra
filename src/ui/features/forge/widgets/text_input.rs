//! [`ForgeTextInput`]: a text field bound to a draft value. The kit input entity lives in
//! keyed element state (created the first time the field renders, dropped when it stops
//! rendering), and is re-synced whenever the bound value changes from outside (undo, class).

use super::{ActionCallback, ValueCallback};
use crate::theme::ThemeTokens;
use gpui_kit::base::input::{InputBaseState, InputEvent, InputModeKind};
use gpui_kit::component::input::{Input, InputState, Textarea, TextareaState};
use gpui_kit::*;
use std::rc::Rc;

/// Keyed state of one field: the input entity, the latest callbacks and its subscription.
struct Slot<M: InputModeKind> {
    input: Entity<InputBaseState<M>>,
    on_change: Option<ValueCallback<String>>,
    on_blur: Option<ActionCallback>,
    _subscription: Subscription,
}

impl<M: InputModeKind> Slot<M> {
    fn new(input: Entity<InputBaseState<M>>, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let subscription = cx.subscribe_in(&input, window, |slot: &mut Self, input, event, window, cx| match event {
            InputEvent::Change => {
                let text = input.read(cx).value().to_string();
                if let Some(on_change) = slot.on_change.clone() {
                    on_change(text, window, cx);
                }
            }
            InputEvent::Blur => {
                if let Some(on_blur) = slot.on_blur.clone() {
                    on_blur(window, cx);
                }
            }
            _ => {}
        });
        Self { input, on_change: None, on_blur: None, _subscription: subscription }
    }
}

/// Installs this frame's callbacks and pushes `value` into the input when it differs.
fn bind<M: InputModeKind>(slot: &Entity<Slot<M>>, field: &ForgeTextInput, window: &mut Window, cx: &mut App) -> Entity<InputBaseState<M>> {
    let input = slot.update(cx, |slot, _| {
        slot.on_change = Some(field.on_change.clone());
        slot.on_blur = field.on_blur.clone();
        slot.input.clone()
    });
    if input.read(cx).value().as_ref() != field.value.as_str() {
        let value = field.value.clone();
        input.update(cx, |state, cx| state.set_value(value, window, cx));
    }
    input
}

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

    /// Called when the field loses focus (e.g. to end an undo run).
    pub fn on_blur(mut self, on_blur: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_blur = Some(Rc::new(on_blur));
        self
    }
}

impl RenderOnce for ForgeTextInput {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
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
                let input = bind(&slot, &self, window, cx);
                frame.child(Input::new(&input).bordered(false))
            }
            Some((min, max)) => {
                let slot = window.use_keyed_state(self.id.clone(), cx, |window, cx| {
                    let area = cx.new(|cx| TextareaState::new(window, cx).placeholder(placeholder).auto_grow(min, max));
                    Slot::new(area, window, cx)
                });
                let area = bind(&slot, &self, window, cx);
                frame.child(Textarea::new(&area).bordered(false))
            }
        }
    }
}
