//! Spoilers: hidden behind an opaque fill until clicked, or until Enter / Space on the
//! focused spoiler (each hidden spoiler is a tab stop), then shown for the rest of the
//! session. Reveals live in memory only: a restart hides every spoiler again.
//!
//! A spoiler is keyed by the post it sits in (the body's scope), its position among the
//! post's spoilers and its text, so revealing it in the feed also reveals it in the thread
//! view, and editing the post hides the changed spoiler again.

use crate::theme::ThemeTokens;
use crate::ui::features::forge::widgets::{ActionCallback, KeyboardStop};
use gpui_kit::component::tooltip::Tooltip;
use gpui_kit::{
    div, relative, AnyElement, InteractiveElement, IntoElement, MouseButton, ParentElement, StatefulInteractiveElement, Styled,
};
use rust_i18n::t;
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

/// Element id of a hidden spoiler (suffixed with its item index in the body).
const SPOILER_ID: &str = "post-spoiler";

/// Which spoiler of which post.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct SpoilerKey {
    /// The post the body belongs to (empty for bodies outside a post).
    pub scope: String,
    /// Position among the body's spoilers.
    pub ordinal: usize,
    /// Visible text of the spoiler.
    pub text: String,
}

thread_local! {
    /// Spoilers revealed this session (the UI runs on one thread).
    static REVEALED: RefCell<HashSet<SpoilerKey>> = RefCell::new(HashSet::new());
}

/// Whether the reader has revealed `key` this session.
pub(crate) fn is_revealed(key: &SpoilerKey) -> bool {
    REVEALED.with(|revealed| revealed.borrow().contains(key))
}

/// Reveals `key` for the rest of the session.
pub(crate) fn reveal(key: SpoilerKey) {
    REVEALED.with(|revealed| revealed.borrow_mut().insert(key));
}

/// A hidden spoiler: its text drawn in the fill color on the fill, so it keeps its size.
pub(super) fn hidden_spoiler(index: usize, key: SpoilerKey, theme: &ThemeTokens) -> AnyElement {
    let text = key.text.clone();
    let on_reveal: ActionCallback = Rc::new(move |window, _| {
        reveal(key.clone());
        window.refresh();
    });
    let on_click = on_reveal.clone();
    div()
        .id((SPOILER_ID, index))
        .min_w_0()
        .px(theme.spacing_3xs)
        .rounded(theme.radius_xs)
        .bg(theme.spoiler_fill)
        .text_color(theme.spoiler_fill)
        .font_family(theme.font_family_body.clone())
        .text_size(theme.font_size_md)
        .line_height(relative(theme.line_height_body))
        .cursor_pointer()
        .keyboard_stop(theme, on_reveal)
        .tooltip(|window, cx| Tooltip::new(t!("post.spoiler_reveal").to_string()).build(window, cx))
        .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
        .on_click(move |_, window, cx| {
            cx.stop_propagation();
            on_click(window, cx);
        })
        .child(text)
        .into_any_element()
}

/// A revealed spoiler: its items on a faint tint, so it still reads as one.
pub(super) fn revealed_spoiler(children: Vec<AnyElement>, theme: &ThemeTokens) -> AnyElement {
    div()
        .min_w_0()
        .px(theme.spacing_3xs)
        .rounded(theme.radius_xs)
        .bg(theme.spoiler_revealed_bg)
        .flex()
        .flex_wrap()
        .items_center()
        .gap_x(theme.inline_gap)
        .children(children)
        .into_any_element()
}
