//! The card's quiet "edited" mark (V8): the version a post shows, opening its history.

use super::events::DataEvent;
use super::post_menu::DataControls;
use super::text::tr;
use crate::theme::ThemeTokens;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;

const EDITED_ID_PREFIX: &str = "post-edited-";

/// The quiet version badge of an edited post (V8), opening its history; `None` before any edit.
pub fn render_edited_mark(controls: &DataControls, theme: &ThemeTokens) -> Option<AnyElement> {
    let text = controls.edited_mark()?;
    let (on_event, post) = (controls.on_event.clone(), controls.post_id.clone());
    let hover = theme.text_secondary;
    let mark = div()
        .id(SharedString::from(format!("{EDITED_ID_PREFIX}{post}")))
        .flex_none()
        .cursor_pointer()
        .font_family(theme.font_family_mono.clone())
        .text_size(theme.font_size_2xs)
        .text_color(theme.text_muted)
        .hover(move |style| style.text_color(hover))
        .when(controls.edited_tooltip(), |mark| mark.tooltip(move |window, cx| gpui_kit::component::tooltip::Tooltip::new(tr("data.edited_tooltip")).build(window, cx)))
        .on_click(move |_, window, cx| on_event(DataEvent::OpenPostHistory(post.clone()), window, cx))
        .child(text);
    Some(mark.into_any_element())
}
