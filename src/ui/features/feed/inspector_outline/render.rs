//! The outline pane: a toolbar (collapse all, expand all, the keyboard hint) over the
//! focusable, scrollable list of bullets of the whole thread.

use super::keys::with_outline_keys;
use super::memory::OutlineCommand;
use super::mode::{InspectorCallback, InspectorCommand, InspectorView};
use super::row::{render_row, RowAction, RowMarks};
use super::tree::{build_outline, thread_root, visible_rows, OutlineNode};
use crate::domain::{Post, PostId};
use crate::state::AppState;
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::*;
use rust_i18n::t;
use std::cmp::Reverse;
use std::rc::Rc;

const LIST_ID: &str = "inspector-outline";
const COLLAPSE_ALL_ID: &str = "outline-collapse-all";
const EXPAND_ALL_ID: &str = "outline-expand-all";

/// The outline of the thread `focused` belongs to.
pub(crate) fn render_outline_pane(state: &AppState, focused: &Post, view: &InspectorView, on_command: &InspectorCallback, theme: &ThemeTokens) -> Div {
    let root = thread_root(focused, |id| state.get_post(id));
    let tree = thread_outline(state, root);
    let thread = view.outlines.thread(&root.id);
    let cursor = thread.cursor.clone().filter(|id| tree.contains(id)).unwrap_or_else(|| focused.id.clone());
    let root_id = root.id.clone();
    let send: RowAction = {
        let (on_command, root) = (on_command.clone(), root_id.clone());
        Rc::new(move |command, window, cx| on_command(InspectorCommand::Outline { root: root.clone(), command }, window, cx))
    };
    let rows = visible_rows(&tree, &thread.collapsed).into_iter().map(|row| {
        let marks = RowMarks { is_cursor: row.id == cursor, is_inspected: row.id == focused.id };
        let (focus, send) = (view.focus.clone(), send.clone());
        let on_row: RowAction = Rc::new(move |command, window, cx| {
            window.focus(&focus, cx);
            send(command, window, cx);
        });
        render_row(&row, marks, on_row, theme)
    });
    let list = with_outline_keys(div().id(LIST_ID).track_focus(&view.focus), send.clone())
        .flex_1()
        .min_h_0()
        .overflow_y_scroll()
        .px(theme.panel_padding_compact)
        .py(theme.spacing_xs)
        .flex()
        .flex_col()
        .children(rows);
    div().flex_1().min_h_0().flex().flex_col().child(render_toolbar(send, theme)).child(list)
}

/// The outline tree under `root`: replies oldest first, authors by display name.
pub(crate) fn thread_outline(state: &AppState, root: &Post) -> OutlineNode {
    build_outline(root, &|id: &PostId| chronological_replies(state, id), &|post: &Post| state.author_of(post).name.clone())
}

/// Direct replies of `id`, oldest first (ties: earlier insertion first), like the cards.
fn chronological_replies<'a>(state: &'a AppState, id: &PostId) -> Vec<&'a Post> {
    let mut replies = state.replies_for_post(id);
    replies.sort_by_key(|post| (post.created_at, Reverse(state.post_position(&post.id).unwrap_or_default())));
    replies
}

fn render_toolbar(send: RowAction, theme: &ThemeTokens) -> Div {
    let button = |id: &'static str, icon: LucideIcon, label: String, command: OutlineCommand| {
        let send = send.clone();
        Button::new(id).ghost().xsmall().icon(Icon::new(icon)).label(label).on_click(move |_, window, cx| send(command.clone(), window, cx))
    };
    div()
        .flex()
        .items_center()
        .gap(theme.control_gap)
        .px(theme.panel_padding)
        .py(theme.spacing_2xs)
        .border_b(theme.border_width_hairline)
        .border_color(theme.border_subtle)
        .child(button(COLLAPSE_ALL_ID, LucideIcon::ChevronsDownUp, t!("feed.outline.collapse_all").to_string(), OutlineCommand::CollapseAll))
        .child(button(EXPAND_ALL_ID, LucideIcon::ChevronsUpDown, t!("feed.outline.expand_all").to_string(), OutlineCommand::ExpandAll))
        .child(div().flex_1())
        .child(div().truncate().text_size(theme.font_size_2xs).text_color(theme.text_muted).child(t!("feed.outline.keys_hint").to_string()))
}
