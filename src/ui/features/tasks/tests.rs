use super::filters::due_filter_label;
use super::{render_tasks_page, TasksView};
use crate::domain::todo::tasks::{collect_tasks, task_contexts, DueFilter, TaskFilter};
use crate::state::AppState;
use crate::theme::ThemeTokens;
use std::rc::Rc;

#[test]
fn every_due_filter_is_translated() {
    for due in DueFilter::ALL {
        let label = due_filter_label(due);
        assert!(!label.starts_with("todo."), "{due:?}");
    }
    for key in ["todo.tasks.title", "todo.tasks.empty", "todo.tasks.any", "todo.tasks.show_done", "todo.tasks.filter.due", "todo.tasks.filter.priority", "todo.tasks.filter.context", "todo.tasks.in_post"] {
        assert_ne!(rust_i18n::t!(key, locale = "en"), key);
    }
}

#[test]
fn the_page_renders_with_rows_and_empty() {
    let mut state = AppState::new_in_memory();
    state.submit_post("Errands\n[ ] post the parcel ^fri !! &town\n  [x] find the tape").unwrap();
    let today = crate::time::local_now().date();
    let theme = ThemeTokens::default();
    for filter in [TaskFilter::default(), TaskFilter { due: DueFilter::NoDate, context: Some("nowhere".into()), ..TaskFilter::default() }] {
        let posts: Vec<_> = state.posts.iter().map(|post| (post, crate::time::local_date(post.created_at))).collect();
        let rows = collect_tasks(posts, &filter, today);
        let view = TasksView { rows, contexts: task_contexts(&state.posts), filter: &filter, today, on_event: Rc::new(|_, _, _| {}) };
        let _page = render_tasks_page(view, &state, &theme);
    }
}
