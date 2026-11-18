//! The keyed state behind a [`ForgeTextInput`](super::ForgeTextInput): the kit input entity,
//! the latest callbacks and the subscription forwarding its events.

use super::{ActionCallback, ValueCallback};
use gpui_kit::base::input::{InputBaseState, InputEvent, InputModeKind};
use gpui_kit::*;

/// Keyed state of one field: the input entity, the latest callbacks and its subscription.
pub(super) struct Slot<M: InputModeKind> {
    input: Entity<InputBaseState<M>>,
    on_change: Option<ValueCallback<String>>,
    on_blur: Option<ActionCallback>,
    _subscription: Subscription,
}

impl<M: InputModeKind> Slot<M> {
    pub(super) fn new(input: Entity<InputBaseState<M>>, window: &mut Window, cx: &mut Context<Self>) -> Self {
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
pub(super) fn bind<M: InputModeKind>(
    slot: &Entity<Slot<M>>,
    (value, on_change, on_blur): (&str, &ValueCallback<String>, &Option<ActionCallback>),
    window: &mut Window,
    cx: &mut App,
) -> Entity<InputBaseState<M>> {
    let input = slot.update(cx, |slot, _| {
        slot.on_change = Some(on_change.clone());
        slot.on_blur = on_blur.clone();
        slot.input.clone()
    });
    if input.read(cx).value().as_ref() != value {
        let value = value.to_string();
        input.update(cx, |state, cx| state.set_value(value, window, cx));
    }
    input
}
