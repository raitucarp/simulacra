//! N19: the "Changed my mind" view: every post in view that changed the user's mind, newest
//! first, with what they used to think (and the post it revises) beside what they think now;
//! a click opens it. A locked post is never listed, nor shown as revised.

use super::common::{empty_note, scroll_column, section_heading};
use super::input::ViewInput;
use crate::domain::views::rapid_log::one_line;
use crate::domain::{Post, PostId};
use crate::state::AppState;
use gpui_kit::*;
use rust_i18n::t;

/// One change of mind as listed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MindRow {
    pub post_id: PostId,
    /// What the user used to think, then the revised post on one line (either may be empty).
    pub was: String,
    pub revised: Option<String>,
    pub now: String,
}

/// The changes of mind among `posts`, newest first.
pub fn mind_rows(state: &AppState, posts: &[&Post]) -> Vec<MindRow> {
    let mut rows: Vec<(&Post, MindRow)> = posts
        .iter()
        .filter(|post| !post.is_locked())
        .filter_map(|post| {
            let change = post.extras.mind_change.as_deref()?;
            let revised = change.revises.as_ref().and_then(|id| state.get_post(id)).filter(|old| !old.is_locked()).map(one_line);
            Some((*post, MindRow { post_id: post.id.clone(), was: change.was.clone(), revised, now: one_line(post) }))
        })
        .collect();
    rows.sort_by_key(|(post, _)| std::cmp::Reverse(post.created_at));
    rows.into_iter().map(|(_, row)| row).collect()
}

pub(super) fn render_mind_changes(input: &ViewInput<'_>) -> AnyElement {
    let rows = mind_rows(input.state, &input.posts);
    if rows.is_empty() {
        return empty_note("views.mind_changes.empty", input.theme);
    }
    scroll_column(input, "mind-changes").children(rows.iter().map(|row| render_row(row, input))).into_any_element()
}

/// The revised post is quoted, not read: a couple of lines, quieter than what was thought.
const REVISED_LINES: usize = 2;

fn side(label: &str, lines: Vec<String>, input: &ViewInput<'_>) -> Div {
    let theme = input.theme;
    let texts = lines.into_iter().filter(|line| !line.is_empty()).map(|line| div().min_w_0().text_size(theme.font_size_xs).text_color(theme.text_primary).child(line));
    div().flex_1().min_w_0().flex().flex_col().gap(theme.spacing_3xs).child(section_heading(t!(label).to_string(), theme)).children(texts)
}

fn revised_quote(row: &MindRow, input: &ViewInput<'_>) -> Option<Div> {
    let theme = input.theme;
    let text = row.revised.clone().filter(|text| !text.is_empty())?;
    Some(div().min_w_0().line_clamp(REVISED_LINES).text_ellipsis().text_size(theme.font_size_xs).text_color(theme.text_muted).child(t!("posts.mind.revises", post = text).to_string()))
}

fn render_row(row: &MindRow, input: &ViewInput<'_>) -> Stateful<Div> {
    let theme = input.theme;
    let before = side("views.mind_changes.before", vec![row.was.clone()], input).children(revised_quote(row, input));
    let hover = theme.hover_bg;
    div()
        .id(input.id(format!("mind-change-{}", row.post_id)))
        .flex()
        .items_start()
        .gap(theme.inline_gap)
        .p(theme.spacing_xs)
        .rounded(theme.radius_sm)
        .border_l(theme.border_width_threadline)
        .border_color(theme.accent_dialectic)
        .cursor_pointer()
        .hover(move |r| r.bg(hover))
        .on_click(input.opener(&row.post_id))
        .child(before)
        .child(side("views.mind_changes.now", vec![row.now.clone()], input))
}
