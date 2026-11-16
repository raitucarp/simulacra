//! Popular tags (trending topics) list shared by the unified and legacy right sidebars.

use std::rc::Rc;

use crate::state::AppState;
use crate::theme::ThemeTokens;
use super::topic_item::render_topic_item;
use gpui_kit::base::StyledExt;
use gpui_kit::*;
use rust_i18n::t;

/// Builds one clickable row per trending topic, ranked from 1.
pub(super) fn trending_elements<F>(
    state: &AppState,
    on_topic: Rc<F>,
    theme: &ThemeTokens,
) -> Vec<AnyElement>
where
    F: Fn(&str, &mut Window, &mut App) + ?Sized + 'static,
{
    state
        .trending
        .iter()
        .enumerate()
        .map(|(idx, topic)| {
            let cb = on_topic.clone();
            render_topic_item(topic, idx + 1, theme, move |tag, w, cx| {
                cb(tag, w, cx);
            })
            .into_any_element()
        })
        .collect()
}

/// Muted "Popular tags" caption.
pub(super) fn render_popular_tags_title(theme: &ThemeTokens) -> Div {
    div()
        .text_size(theme.font_size_label)
        .font_bold()
        .text_color(theme.text_muted)
        .child(t!("sidebar.popular_tags").to_string())
}

/// Vertical list holding the trending rows.
pub(super) fn render_trending_list(elements: Vec<AnyElement>, theme: &ThemeTokens) -> Div {
    div().flex().flex_col().gap(theme.spacing_xs).children(elements)
}
