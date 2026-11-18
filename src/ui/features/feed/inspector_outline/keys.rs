//! The outline's keyboard: arrows move and fold, Enter opens, Home and End jump. Bound in the
//! outline's own key context, so the keys mean nothing elsewhere.

use super::memory::{OutlineCommand, OutlineKey};
use super::row::RowAction;
use gpui_kit::{actions, App, InteractiveElement, KeyBinding, Stateful, Div};

actions!(inspector_outline, [OutlineUp, OutlineDown, OutlineFold, OutlineUnfold, OutlineOpen, OutlineFirst, OutlineLast]);

/// Key context of the focused outline.
pub const OUTLINE_KEY_CONTEXT: &str = "InspectorOutline";

/// Registers the outline's bindings (once, at startup).
pub fn init_key_bindings(cx: &mut App) {
    let context = Some(OUTLINE_KEY_CONTEXT);
    cx.bind_keys([
        KeyBinding::new("up", OutlineUp, context),
        KeyBinding::new("down", OutlineDown, context),
        KeyBinding::new("left", OutlineFold, context),
        KeyBinding::new("right", OutlineUnfold, context),
        KeyBinding::new("enter", OutlineOpen, context),
        KeyBinding::new("home", OutlineFirst, context),
        KeyBinding::new("end", OutlineLast, context),
    ]);
}

/// `list` answering the outline keys by sending them through `send`.
pub(super) fn with_outline_keys(list: Stateful<Div>, send: RowAction) -> Stateful<Div> {
    let key = move |key: OutlineKey| {
        let send = send.clone();
        move |window: &mut gpui_kit::Window, cx: &mut App| send(OutlineCommand::Key(key), window, cx)
    };
    let (up, down, fold, unfold) = (key(OutlineKey::Up), key(OutlineKey::Down), key(OutlineKey::Left), key(OutlineKey::Right));
    let (open, first, last) = (key(OutlineKey::Enter), key(OutlineKey::Home), key(OutlineKey::End));
    list.key_context(OUTLINE_KEY_CONTEXT)
        .on_action(move |_: &OutlineUp, window, cx| up(window, cx))
        .on_action(move |_: &OutlineDown, window, cx| down(window, cx))
        .on_action(move |_: &OutlineFold, window, cx| fold(window, cx))
        .on_action(move |_: &OutlineUnfold, window, cx| unfold(window, cx))
        .on_action(move |_: &OutlineOpen, window, cx| open(window, cx))
        .on_action(move |_: &OutlineFirst, window, cx| first(window, cx))
        .on_action(move |_: &OutlineLast, window, cx| last(window, cx))
}
