//! The filter rows of the Tasks page: due day, priority, context and "Show done".

use super::{TasksAction, TasksEvent, TasksView};
use crate::domain::todo::tasks::DueFilter;
use crate::theme::ThemeTokens;
use crate::ui::components::chip::{choice_chip, chip_row};
use crate::ui::features::data::tr;
use gpui_kit::*;
use rust_i18n::t;

const DUE_ID: &str = "tasks-due";
const PRIORITY_ID: &str = "tasks-priority";
const CONTEXT_ID: &str = "tasks-context";
const CLOSED_ID: &str = "tasks-closed";
/// The priorities offered, as their `!` count.
const PRIORITIES: [u8; 3] = [1, 2, 3];
/// The `!` a priority chip repeats.
const PRIORITY_MARK: &str = "!";

/// The label of a due filter.
pub fn due_filter_label(due: DueFilter) -> String {
    tr(&format!("todo.tasks.due.{}", due.key()))
}

fn chip(id: String, label: &str, selected: bool, event: TasksEvent, on_event: &TasksAction, theme: &ThemeTokens) -> Stateful<Div> {
    let on_event = on_event.clone();
    choice_chip(SharedString::from(id), label, selected, theme, move |window, cx| on_event(event.clone(), window, cx))
}

fn labelled(label: String, chips: Div, theme: &ThemeTokens) -> Div {
    let name = div().flex_none().w(theme.tasks_filter_label_width).text_size(theme.font_size_xs).text_color(theme.text_muted).child(label);
    div().flex().items_center().gap(theme.control_gap).child(name).child(chips)
}

fn due_row(view: &TasksView<'_>, theme: &ThemeTokens) -> Div {
    let chips = DueFilter::ALL.map(|due| chip(format!("{DUE_ID}-{}", due.key()), &due_filter_label(due), view.filter.due == due, TasksEvent::Due(due), &view.on_event, theme));
    labelled(t!("todo.tasks.filter.due").to_string(), chip_row(theme).children(chips), theme)
}

fn priority_row(view: &TasksView<'_>, theme: &ThemeTokens) -> Div {
    let any = chip(format!("{PRIORITY_ID}-any"), &t!("todo.tasks.any"), view.filter.priority.is_none(), TasksEvent::Priority(None), &view.on_event, theme);
    let levels = PRIORITIES.map(|level| chip(format!("{PRIORITY_ID}-{level}"), &PRIORITY_MARK.repeat(usize::from(level)), view.filter.priority == Some(level), TasksEvent::Priority(Some(level)), &view.on_event, theme));
    labelled(t!("todo.tasks.filter.priority").to_string(), chip_row(theme).child(any).children(levels), theme)
}

fn context_row(view: &TasksView<'_>, theme: &ThemeTokens) -> Div {
    let selected = view.filter.context.as_deref();
    let any = chip(format!("{CONTEXT_ID}-any"), &t!("todo.tasks.any"), selected.is_none(), TasksEvent::Context(None), &view.on_event, theme);
    let contexts = view.contexts.iter().enumerate().map(|(n, name)| {
        let is_selected = selected.is_some_and(|wanted| wanted.eq_ignore_ascii_case(name));
        chip(format!("{CONTEXT_ID}-{n}"), name, is_selected, TasksEvent::Context(Some(name.clone())), &view.on_event, theme)
    });
    labelled(t!("todo.tasks.filter.context").to_string(), chip_row(theme).child(any).children(contexts), theme)
}

/// Every filter row; the context row only once some to-do has a context.
pub(super) fn render_filters(view: &TasksView<'_>, theme: &ThemeTokens) -> Div {
    let closed = chip(CLOSED_ID.to_string(), &t!("todo.tasks.show_done"), view.filter.show_closed, TasksEvent::ToggleClosed, &view.on_event, theme);
    div()
        .flex()
        .flex_col()
        .gap(theme.stack_gap)
        .child(due_row(view, theme))
        .child(priority_row(view, theme))
        .children((!view.contexts.is_empty()).then(|| context_row(view, theme)))
        .child(labelled(String::new(), chip_row(theme).child(closed), theme))
}
