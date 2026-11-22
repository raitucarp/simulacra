//! The Tasks page in the shell (D9): what it is narrowed to, and its events: filters change
//! in place, a box ticks the item in its post, and the line under it opens the post.

use super::binder::Binder;
use super::view::AppView;
use crate::domain::todo::tasks::TaskFilter;
use crate::state::NavigationTab;
use crate::ui::features::tasks::{render_tasks_page, TasksEvent, TasksView};
use gpui_kit::*;
use std::rc::Rc;

/// What the Tasks page keeps for the session.
#[derive(Debug, Clone, Default)]
pub struct TasksHost {
    pub filter: TaskFilter,
}

impl AppView {
    /// The Tasks page as it stands.
    pub(super) fn tasks_page(&self, bind: &Binder) -> AnyElement {
        let today = crate::time::local_now().date();
        let filter = &self.tasks.filter;
        let view = TasksView { rows: self.state.task_rows(filter, today), contexts: self.state.task_contexts(), filter, today, on_event: Rc::new(bind.with(Self::on_tasks_event)) };
        render_tasks_page(view, &self.state, &self.theme)
    }

    fn on_tasks_event(&mut self, event: TasksEvent, window: &mut Window, cx: &mut Context<Self>) {
        let filter = &mut self.tasks.filter;
        match event {
            TasksEvent::Due(due) => filter.due = due,
            TasksEvent::Priority(priority) => filter.priority = priority,
            TasksEvent::Context(context) => filter.context = context,
            TasksEvent::ToggleClosed => filter.show_closed = !filter.show_closed,
            TasksEvent::Tick(id, index) => self.toggle_todo(&id, index, window, cx),
            TasksEvent::OpenPost(id) => {
                self.select_nav(NavigationTab::Timeline, window, cx);
                self.select_post(id, window, cx);
            }
        }
        cx.notify();
    }
}
