//! N16: the lens fan view of a thread: every post fanned out through lenses, with its replies
//! side by side, one column per lens (the lens, the agent, the reply or that it is still
//! awaited); a click opens a reply. A locked post lists no fan, and a locked reply shows only
//! its placeholder.

use super::common::{empty_note, locked_note, scroll_column, section_heading};
use super::input::ViewInput;
use crate::domain::post::lens::Lens;
use crate::domain::views::rapid_log::one_line;
use crate::domain::{Post, PostId};
use crate::state::AppState;
use crate::ui::features::data::lens_name;
use gpui_kit::*;
use rust_i18n::t;

/// A reply in a fan column: its id and its text on one line (`None`: locked).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FanReply {
    pub id: PostId,
    pub text: Option<String>,
}

/// One lens of a fan.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FanColumn {
    pub lens: Lens,
    pub agent: String,
    pub reply: Option<FanReply>,
}

/// A post fanned out, and its lenses.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FanGroup {
    pub root: PostId,
    pub text: String,
    pub columns: Vec<FanColumn>,
}

fn reply(state: &AppState, id: &PostId) -> Option<FanReply> {
    let post = state.get_post(id)?;
    Some(FanReply { id: id.clone(), text: (!post.is_locked()).then(|| one_line(post)) })
}

/// The fans among `posts`, in their order.
pub fn fans(state: &AppState, posts: &[&Post]) -> Vec<FanGroup> {
    let group = |post: &&Post| {
        let lines = state.fan_lines(&post.id);
        let name = |agent: &crate::domain::ProfileId| state.user_profiles.iter().find(|p| &p.id == agent).map_or_else(String::new, |p| p.name.clone());
        let columns: Vec<FanColumn> = lines.iter().map(|line| FanColumn { lens: line.lens, agent: name(&line.agent), reply: line.reply.as_ref().and_then(|id| reply(state, id)) }).collect();
        (!columns.is_empty()).then(|| FanGroup { root: post.id.clone(), text: one_line(post), columns })
    };
    posts.iter().filter_map(group).collect()
}

pub(super) fn render_lens_fan(input: &ViewInput<'_>) -> AnyElement {
    let found = fans(input.state, &input.posts);
    if found.is_empty() {
        return empty_note("views.lens_fan.empty", input.theme);
    }
    scroll_column(input, "lens-fan").children(found.iter().map(|group| render_group(group, input))).into_any_element()
}

fn render_group(group: &FanGroup, input: &ViewInput<'_>) -> Div {
    let theme = input.theme;
    let root = div().min_w_0().truncate().text_size(theme.font_size_xs).text_color(theme.text_primary).child(group.text.clone());
    let columns = div().flex().items_start().gap(theme.inline_gap).children(group.columns.iter().map(|column| render_column(&group.root, column, input)));
    div().flex().flex_col().gap(theme.spacing_xs).child(section_heading(t!("views.lens_fan.heading").to_string(), theme)).child(root).child(columns)
}

fn render_column(root: &PostId, column: &FanColumn, input: &ViewInput<'_>) -> Div {
    let theme = input.theme;
    let head = div().text_size(theme.font_size_2xs).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_secondary).child(lens_name(column.lens));
    let agent = div().text_size(theme.font_size_2xs).text_color(theme.text_muted).child(column.agent.clone());
    let body = match &column.reply {
        None => div().text_size(theme.font_size_xs).text_color(theme.text_muted).child(t!("views.lens_fan.waiting").to_string()).into_any_element(),
        Some(found) => render_reply(root, column.lens, found, input).into_any_element(),
    };
    div().flex_1().min_w_0().pl(theme.spacing_xs).border_l(theme.border_width_threadline).border_color(theme.accent_dialectic).flex().flex_col().gap(theme.spacing_3xs).child(head).child(agent).child(body)
}

fn render_reply(root: &PostId, lens: Lens, found: &FanReply, input: &ViewInput<'_>) -> Stateful<Div> {
    let theme = input.theme;
    let text = match &found.text {
        None => locked_note(theme).into_any_element(),
        Some(text) => div().min_w_0().text_size(theme.font_size_xs).text_color(theme.text_primary).child(text.clone()).into_any_element(),
    };
    let hover = theme.hover_bg;
    div().id(input.id(format!("lens-fan-{root}-{}", lens.key()))).py(theme.spacing_3xs).rounded(theme.radius_sm).cursor_pointer().hover(move |r| r.bg(hover)).on_click(input.opener(&found.id)).child(text)
}
