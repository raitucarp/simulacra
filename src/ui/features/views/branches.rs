//! N6: the what-if view of a thread: every post the thread forks at, with its lines side by
//! side (the main line first, then each branch), each post on one line; a click opens it. A
//! locked post shows only its placeholder, and a locked branch only the placeholder label.

use super::common::{empty_note, locked_note, scroll_column, section_heading};
use super::input::ViewInput;
use crate::domain::views::rapid_log::one_line;
use crate::domain::{Post, PostId};
use crate::state::AppState;
use gpui_kit::*;
use rust_i18n::t;

/// One post of a line: its text on one line (`None`: locked).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LineRow {
    pub id: PostId,
    pub text: Option<String>,
}

/// One line under a fork: its label (`None`: the main line) and its posts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LineColumn {
    pub label: Option<String>,
    pub rows: Vec<LineRow>,
}

/// A post the thread forks at, and its lines.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fork {
    pub at: LineRow,
    pub columns: Vec<LineColumn>,
}

fn row(state: &AppState, id: &PostId) -> Option<LineRow> {
    let post = state.get_post(id)?;
    Some(LineRow { id: id.clone(), text: (!post.is_locked()).then(|| one_line(post)) })
}

/// The forks among `posts`, in their order.
pub fn forks(state: &AppState, posts: &[&Post]) -> Vec<Fork> {
    posts
        .iter()
        .filter_map(|post| {
            let lines = state.branch_lines(&post.id);
            let columns = lines.into_iter().map(|line| LineColumn { label: line.label, rows: line.posts.iter().filter_map(|id| row(state, id)).collect() }).collect::<Vec<_>>();
            if columns.is_empty() {
                return None;
            }
            Some(Fork { at: row(state, &post.id)?, columns })
        })
        .collect()
}

pub(super) fn render_branches(input: &ViewInput<'_>) -> AnyElement {
    let found = forks(input.state, &input.posts);
    if found.is_empty() {
        return empty_note("views.branches.empty", input.theme);
    }
    scroll_column(input, "branches").children(found.iter().map(|fork| render_fork(fork, input))).into_any_element()
}

fn render_fork(fork: &Fork, input: &ViewInput<'_>) -> Div {
    let theme = input.theme;
    let columns = div().flex().items_start().gap(theme.inline_gap).children(fork.columns.iter().enumerate().map(|(at, column)| render_column(&fork.at.id, at, column, input)));
    div().flex().flex_col().gap(theme.spacing_xs).child(section_heading(t!("views.branches.forked_at").to_string(), theme)).child(render_row(&fork.at, "fork", input)).child(columns)
}

fn render_column(fork: &PostId, at: usize, column: &LineColumn, input: &ViewInput<'_>) -> Div {
    let theme = input.theme;
    let title = match &column.label {
        None => t!("posts.branch.main").to_string(),
        Some(label) => crate::ui::features::post::what_if_text(label),
    };
    let edge = if column.label.is_some() { theme.accent_dialectic } else { theme.border_subtle };
    let head = div().text_size(theme.font_size_2xs).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_secondary).child(title);
    let rows = column.rows.iter().map(|line_row| render_row(line_row, &format!("{fork}-{at}"), input));
    div().flex_1().min_w_0().pl(theme.spacing_xs).border_l(theme.border_width_threadline).border_color(edge).flex().flex_col().gap(theme.spacing_3xs).child(head).children(rows)
}

fn render_row(line_row: &LineRow, part: &str, input: &ViewInput<'_>) -> Stateful<Div> {
    let theme = input.theme;
    let text = match &line_row.text {
        None => locked_note(theme).into_any_element(),
        Some(text) => div().min_w_0().truncate().text_size(theme.font_size_xs).text_color(theme.text_primary).child(text.clone()).into_any_element(),
    };
    let hover = theme.hover_bg;
    div()
        .id(input.id(format!("branch-{part}-{}", line_row.id)))
        .px(theme.spacing_xs)
        .py(theme.spacing_3xs)
        .rounded(theme.radius_sm)
        .cursor_pointer()
        .hover(move |r| r.bg(hover))
        .on_click(input.opener(&line_row.id))
        .child(text)
}
