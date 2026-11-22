//! The Tasks page (D9): every to-do of the user's posts in one list, soonest first, with
//! filters for due day, priority and context. A box ticks the item in its post; the line
//! under it opens the post.

mod filters;
#[cfg(test)]
mod tests;

use crate::domain::todo::tasks::{DueFilter, TaskFilter, TaskRow};
use crate::domain::PostId;
use crate::state::AppState;
use crate::theme::ThemeTokens;
use crate::ui::features::post::{render_task_row, BodyLinks};
use gpui_kit::*;
use jiff::civil::Date;
use rust_i18n::t;
use std::rc::Rc;

const ROOT_ID: &str = "tasks-root";
const OPEN_ID: &str = "tasks-open-post";

/// Everything the Tasks page asks the shell to do.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TasksEvent {
    Due(DueFilter),
    Priority(Option<u8>),
    Context(Option<String>),
    ToggleClosed,
    /// Ticks the to-do at this index of the post.
    Tick(PostId, usize),
    OpenPost(PostId),
}

pub type TasksAction = Rc<dyn Fn(TasksEvent, &mut Window, &mut App) + 'static>;

/// What the page shows.
pub struct TasksView<'a> {
    pub rows: Vec<TaskRow>,
    pub contexts: Vec<String>,
    pub filter: &'a TaskFilter,
    pub today: Date,
    pub on_event: TasksAction,
}

fn open_line(row: &TaskRow, state: &AppState, on_event: &TasksAction, theme: &ThemeTokens) -> Stateful<Div> {
    let first = state.get_post(&row.post_id).and_then(|post| post.content.lines().find(|line| !line.trim().is_empty())).unwrap_or_default().trim().to_string();
    let (on_event, id, hover) = (on_event.clone(), row.post_id.clone(), theme.text_secondary);
    div()
        .id(SharedString::from(format!("{OPEN_ID}-{}-{}", row.post_id, row.entry.index)))
        .pl(theme.todo_indent)
        .truncate()
        .cursor_pointer()
        .text_size(theme.font_size_xs)
        .text_color(theme.text_muted)
        .hover(move |style| style.text_color(hover))
        .child(t!("todo.tasks.in_post", post = first).to_string())
        .on_click(move |_, window, cx| on_event(TasksEvent::OpenPost(id.clone()), window, cx))
}

fn task_card(row: &TaskRow, state: &AppState, on_event: &TasksAction, theme: &ThemeTokens) -> Div {
    let (tick, id) = (on_event.clone(), row.post_id.clone());
    let toggle: crate::ui::features::post::TodoToggle = Rc::new(move |index, window, cx| tick(TasksEvent::Tick(id.clone(), index), window, cx));
    let body = render_task_row(&row.entry, row.post_id.as_str(), row.written, state, theme, &BodyLinks::default(), Some(toggle));
    div().w_full().flex().flex_col().gap(theme.spacing_3xs).py(theme.spacing_xs).border_b(theme.border_width_hairline).border_color(theme.border_subtle).child(body).child(open_line(row, state, on_event, theme))
}

/// The page: a header with the count, the filters, then the list (or why it is empty).
pub fn render_tasks_page(view: TasksView<'_>, state: &AppState, theme: &ThemeTokens) -> AnyElement {
    let title = div().text_size(theme.font_size_xl).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_primary).child(t!("todo.tasks.title").to_string());
    let count = div().text_size(theme.font_size_sm).text_color(theme.text_muted).child(crate::i18n::t_count("todo.tasks.count", view.rows.len()));
    let header = div().flex().items_baseline().gap(theme.control_gap).child(title).child(count);
    let list = if view.rows.is_empty() {
        div().py(theme.section_gap).text_size(theme.font_size_sm).text_color(theme.text_muted).child(t!("todo.tasks.empty").to_string())
    } else {
        div().flex().flex_col().children(view.rows.iter().map(|row| task_card(row, state, &view.on_event, theme)))
    };
    let column = div().w_full().max_w(theme.tasks_page_max_width).flex().flex_col().gap(theme.section_gap).child(header).child(filters::render_filters(&view, theme)).child(list);
    div().id(ROOT_ID).size_full().overflow_y_scroll().bg(theme.bg_app).flex().justify_center().items_start().px(theme.panel_padding).py(theme.panel_padding).child(column).into_any_element()
}
