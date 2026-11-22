//! L1: a web address written in the body, in the link colour. A click opens it in the system
//! browser (without selecting the card); hovering shows the whole address, which a long link
//! shortens on screen.

use super::inline::body_text;
use crate::theme::ThemeTokens;
use gpui_kit::component::tooltip::Tooltip;
use gpui_kit::{AnyElement, InteractiveElement, IntoElement, MouseButton, ParentElement, StatefulInteractiveElement, Styled};

/// Element id of a body link (suffixed with its item index).
const LINK_ID: &str = "body-link";
/// Scheme prefixes left off the shown text.
const SHOWN_SCHEMES: [&str; 2] = ["https://", "http://"];
/// Characters of an address shown before it is cut short.
const MAX_SHOWN_CHARS: usize = 40;
const ELLIPSIS: char = '…';

/// The text a link shows: without its scheme or a lone trailing slash, cut short when long.
pub(super) fn shown_url(url: &str) -> String {
    let bare = SHOWN_SCHEMES.iter().find_map(|scheme| url.get(..scheme.len()).filter(|head| head.eq_ignore_ascii_case(scheme)).map(|head| &url[head.len()..]));
    let bare = bare.unwrap_or(url);
    let bare = bare.strip_suffix('/').filter(|rest| !rest.contains('/')).unwrap_or(bare);
    match bare.char_indices().nth(MAX_SHOWN_CHARS) {
        Some((cut, _)) => format!("{}{ELLIPSIS}", &bare[..cut]),
        None => bare.to_string(),
    }
}

/// The link `url`; `index` keeps its id unique within the body.
pub(super) fn link_span(index: usize, url: &str, theme: &ThemeTokens) -> AnyElement {
    let (target, full) = (url.to_string(), url.to_string());
    body_text(theme)
        .id((LINK_ID, index))
        .min_w_0()
        .text_color(theme.link_text)
        .cursor_pointer()
        .hover(|style| style.underline())
        .tooltip(move |window, cx| Tooltip::new(full.clone()).build(window, cx))
        .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
        .on_click(move |_, _, cx| cx.open_url(&target))
        .child(shown_url(url))
        .into_any_element()
}
