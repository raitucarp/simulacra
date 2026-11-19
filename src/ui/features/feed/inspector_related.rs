//! "Related thoughts" under the conversation: the posts nearest the focused post in meaning,
//! from other conversations, each focusing its thread when clicked.

use super::callbacks::PostCallbacks;
use crate::state::semantic::{HitTarget, SemanticHit};
use crate::theme::ThemeTokens;
use crate::ui::features::semantic::{hit_icon, render_scored_row};
use gpui_kit::*;
use rust_i18n::t;

/// Row index offset (keeps these row ids apart from other scored lists).
const ROW_INDEX_OFFSET: usize = 2_000;

/// The section, or nothing when no related post is known yet.
pub(super) fn render_related(hits: &[SemanticHit], callbacks: &PostCallbacks, theme: &ThemeTokens) -> Option<Div> {
    if hits.is_empty() {
        return None;
    }
    let heading = div().text_size(theme.font_size_xs).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_primary).child(t!("feed.related.title").to_string());
    let hint = div().text_size(theme.font_size_2xs).text_color(theme.text_muted).child(t!("feed.related.hint").to_string());
    let rows = hits.iter().enumerate().map(|(index, hit)| row(ROW_INDEX_OFFSET + index, hit, callbacks, theme));
    let section = div()
        .mt(theme.spacing_sm)
        .pt(theme.spacing_sm)
        .px(theme.spacing_sm)
        .border_t(theme.border_width_hairline)
        .border_color(theme.border_subtle)
        .flex()
        .flex_col()
        .gap(theme.spacing_3xs)
        .child(heading)
        .child(hint)
        .children(rows);
    Some(section)
}

fn row(index: usize, hit: &SemanticHit, callbacks: &PostCallbacks, theme: &ThemeTokens) -> AnyElement {
    let element = render_scored_row(index, hit_icon(&hit.target), &hit.title, &hit.detail, &hit.excerpt, hit.score, false, theme);
    match &hit.target {
        HitTarget::Post(id) => {
            let (id, on_select) = (id.clone(), callbacks.on_select.clone());
            element.on_click(move |_, window, cx| on_select(id.clone(), window, cx)).into_any_element()
        }
        HitTarget::Document(_) => element.into_any_element(),
    }
}
