//! Format shortcuts of the composers: one action per mark, bound in the kit's `Input`
//! context by the central keymap (Ctrl/Cmd+B, I, U, E and Shift+S).
//!
//! Only the composer text areas handle these actions. In any other text field the action
//! goes unhandled and gpui tries the next binding, so Ctrl/Cmd+B there still reaches the
//! shell (toggle sidebar).

use super::format::{apply_format, FormatCallback};
use crate::domain::post::format::FormatMark;
use gpui_kit::component::input::TextareaState;
use gpui_kit::{actions, Action, Entity, InteractiveElement, Stateful, Div};
use std::rc::Rc;

actions!(simulacra_format, [FormatBold, FormatItalic, FormatUnderline, FormatCode, FormatSpoiler]);

/// The action a mark's shortcut dispatches.
pub fn format_action(mark: FormatMark) -> Box<dyn Action> {
    match mark {
        FormatMark::Bold => Box::new(FormatBold),
        FormatMark::Italic => Box::new(FormatItalic),
        FormatMark::Underline => Box::new(FormatUnderline),
        FormatMark::Code => Box::new(FormatCode),
        FormatMark::Spoiler => Box::new(FormatSpoiler),
    }
}

/// Formats `input` when one of the format actions reaches `element` (its text area wrapper).
pub(super) fn handle_format_actions(element: Stateful<Div>, input: &Entity<TextareaState>) -> Stateful<Div> {
    let input = input.clone();
    let on_format: FormatCallback = Rc::new(move |mark, window, cx| apply_format(&input, mark, window, cx));
    let handler = |mark: FormatMark| {
        let on_format = on_format.clone();
        move |window: &mut gpui_kit::Window, cx: &mut gpui_kit::App| on_format(mark, window, cx)
    };
    let (bold, italic, underline) = (handler(FormatMark::Bold), handler(FormatMark::Italic), handler(FormatMark::Underline));
    let (code, spoiler) = (handler(FormatMark::Code), handler(FormatMark::Spoiler));
    element
        .on_action(move |_: &FormatBold, window, cx| bold(window, cx))
        .on_action(move |_: &FormatItalic, window, cx| italic(window, cx))
        .on_action(move |_: &FormatUnderline, window, cx| underline(window, cx))
        .on_action(move |_: &FormatCode, window, cx| code(window, cx))
        .on_action(move |_: &FormatSpoiler, window, cx| spoiler(window, cx))
}
