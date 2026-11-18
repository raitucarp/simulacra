//! "Search concept" extras: the vault entries that mention the concept and the thoughts
//! nearest in meaning, each row opening its item.

use super::model::{ConceptHits, SheetEvent};
use super::sheet::SheetAction;
use crate::state::semantic::SemanticHit;
use crate::theme::ThemeTokens;
use crate::ui::features::semantic::{hit_icon, render_scored_row};
use gpui_kit::*;
use rust_i18n::t;

const VAULT_LIST_ID: &str = "post-actions-vault-hits";
const NEARBY_LIST_ID: &str = "post-actions-nearby-hits";
/// Row index offset of the second list (keeps row ids unique across both lists).
const NEARBY_INDEX_OFFSET: usize = 1_000;

/// Both lists, each under its heading.
pub(super) fn render_concept_hits(hits: &ConceptHits, on_event: &SheetAction, theme: &ThemeTokens) -> Div {
    let vault = section(VAULT_LIST_ID, t!("post_actions.concept_vault").to_string(), t!("post_actions.concept_no_vault").to_string(), &hits.vault, 0, on_event, theme);
    let nearby = section(NEARBY_LIST_ID, t!("post_actions.concept_nearby").to_string(), t!("post_actions.concept_no_nearby").to_string(), &hits.nearby, NEARBY_INDEX_OFFSET, on_event, theme);
    div().flex().flex_col().gap(theme.stack_gap).child(vault).child(nearby)
}

fn section(id: &'static str, title: String, empty: String, hits: &[SemanticHit], offset: usize, on_event: &SheetAction, theme: &ThemeTokens) -> Div {
    let heading = div().text_size(theme.font_size_xs).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_primary).child(title);
    let rows = hits.iter().enumerate().map(|(index, hit)| row(offset + index, hit, on_event.clone(), theme));
    let list = div().id(id).max_h(theme.post_actions_hits_max_height).overflow_y_scroll().flex().flex_col().children(rows);
    let none = hits.is_empty().then(|| div().text_size(theme.font_size_2xs).text_color(theme.text_muted).child(empty));
    div().flex().flex_col().gap(theme.spacing_3xs).child(heading).children(none).child(list)
}

fn row(index: usize, hit: &SemanticHit, on_event: SheetAction, theme: &ThemeTokens) -> AnyElement {
    let target = hit.target.clone();
    render_scored_row(index, hit_icon(&hit.target), &hit.title, &hit.detail, &hit.excerpt, hit.score, false, theme)
        .on_click(move |_, window, cx| on_event(SheetEvent::OpenHit(target.clone()), window, cx))
        .into_any_element()
}
