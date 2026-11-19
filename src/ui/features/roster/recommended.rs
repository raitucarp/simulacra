//! The Recommended tab of a profile's overview: the posts closest in meaning to what the
//! persona or agent writes and answers (from the post embeddings), each opening its thread.

use crate::domain::{PostId, Profile};
use crate::state::semantic::{HitTarget, SemanticHit};
use crate::theme::ThemeTokens;
use crate::ui::features::semantic::{hit_icon, render_scored_row};
use gpui_kit::*;
use rust_i18n::t;
use std::rc::Rc;

/// Opens a recommended post in the timeline.
pub type OpenPostCallback = Rc<dyn Fn(PostId, &mut Window, &mut App)>;

/// The recommendations of `profile` in a reading-width column, or why there are none yet.
pub(super) fn recommended_column(profile: &Profile, hits: &[SemanticHit], on_open: &OpenPostCallback, theme: &ThemeTokens) -> Div {
    let heading = div().text_size(theme.font_size_sm).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_primary).child(t!("roster.recommended.title", name = profile.name).to_string());
    let hint_key = if hits.is_empty() { "roster.recommended.empty" } else { "roster.recommended.hint" };
    let hint = div().text_size(theme.font_size_xs).text_color(theme.text_muted).child(t!(hint_key, name = profile.name).to_string());
    let rows = hits.iter().enumerate().map(|(index, hit)| row(index, hit, on_open, theme));
    div().w_full().max_w(theme.reader_measure_width).flex().flex_col().gap(theme.spacing_xs).child(heading).child(hint).children(rows)
}

fn row(index: usize, hit: &SemanticHit, on_open: &OpenPostCallback, theme: &ThemeTokens) -> AnyElement {
    let element = render_scored_row(index, hit_icon(&hit.target), &hit.title, &hit.detail, &hit.excerpt, hit.score, false, theme);
    match &hit.target {
        HitTarget::Post(id) => {
            let (id, on_open) = (id.clone(), on_open.clone());
            element.on_click(move |_, window, cx| on_open(id.clone(), window, cx)).into_any_element()
        }
        HitTarget::Document(_) => element.into_any_element(),
    }
}
