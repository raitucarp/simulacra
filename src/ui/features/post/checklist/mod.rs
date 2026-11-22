//! The checklist of a post (D5): a body with to-do lines renders each of them as a row (its
//! box, its text, and chips for priority, due day, repeat, contexts, estimate and delegate),
//! nested by indentation, between the plain runs of text around them. An item past its due
//! day that is still to do shows its due chip in the danger colour.

mod chips;
mod row;
#[cfg(test)]
mod tests;

use super::body::{render_body_tokens, BodyLinks};
use super::lookup::PostLookup;
use crate::domain::post::tokenize_post_body;
use crate::domain::todo::entries::{body_parts, todo_entries, BodyPart, TodoEntry};
use crate::domain::Post;
use crate::theme::ThemeTokens;
use crate::domain::todo::state::TodoState;
use crate::ui::features::data::{DataControls, DataEvent};
use gpui_kit::{div, AnyElement, App, IntoElement, ParentElement, Styled, Window};
use std::rc::Rc;
use jiff::civil::Date;
use row::{todo_row, RowContext};

pub use chips::due_label as todo_due_label;

/// The text a checklist is drawn from.
pub(crate) struct ChecklistSource<'a> {
    pub content: &'a str,
    /// The post it belongs to (element ids).
    pub post_id: &'a str,
    /// The day it was written, which due phrases count from.
    pub written: Date,
    /// The boxes ticked so far (D6).
    pub ticks: &'a TodoState,
    /// Ticks a box; `None` leaves the boxes inert (posts the user did not write, previews).
    pub toggle: Option<TodoToggle>,
}

/// Clicks the box of the to-do at this index (D6).
pub type TodoToggle = Rc<dyn Fn(usize, &mut Window, &mut App) + 'static>;

/// The box toggle of a card: only on the user's own posts, through their data events.
pub(crate) fn todo_toggle(data: Option<&DataControls>) -> Option<TodoToggle> {
    let data = data.filter(|data| data.own)?;
    let (on_event, id) = (data.on_event.clone(), data.post_id.clone());
    Some(Rc::new(move |index, window, cx| on_event(DataEvent::ToggleTodo(id.clone(), index), window, cx)))
}

/// The checklist of `source`; `None` when it has no to-dos.
pub(crate) fn render_checklist<L: PostLookup>(source: &ChecklistSource<'_>, lookup: L, theme: &ThemeTokens, links: &BodyLinks) -> Option<AnyElement> {
    let entries = todo_entries(source.content, source.ticks);
    if entries.is_empty() {
        return None;
    }
    let context = RowContext { post_id: source.post_id.to_string(), written: source.written, today: crate::time::local_now().date(), theme, toggle: source.toggle.clone() };
    let parts: Vec<AnyElement> = body_parts(source.content)
        .into_iter()
        .map(|part| match part {
            BodyPart::Text(text) => render_body_tokens(&tokenize_post_body(&text), lookup, theme, links),
            BodyPart::Item(index) => entries.get(index).map_or_else(|| div().into_any_element(), |entry| todo_row(entry, lookup, links, &context)),
        })
        .collect();
    Some(div().w_full().min_w_0().flex().flex_col().gap(theme.spacing_2xs).children(parts).into_any_element())
}

/// The body of `post`: its checklist (ticked by `toggle`) when it has to-dos, else its cached
/// body tokens.
pub fn render_post_content<L: PostLookup>(post: &Post, lookup: L, theme: &ThemeTokens, links: &BodyLinks, toggle: Option<TodoToggle>) -> AnyElement {
    if post.is_locked() {
        return super::locked::render_locked_placeholder(None, theme); // X4: quotes and previews never open it
    }
    let source = ChecklistSource { content: &post.content, post_id: post.id.as_str(), written: crate::time::local_date(post.created_at), ticks: &post.extras.todo_state, toggle };
    render_checklist(&source, lookup, theme, links).unwrap_or_else(|| render_body_tokens(&lookup.body_tokens(post), lookup, theme, links))
}

/// One to-do outside its post (the Tasks page, D9): its row unindented, with its chips.
pub fn render_task_row<L: PostLookup>(entry: &TodoEntry, post_id: &str, written: Date, lookup: L, theme: &ThemeTokens, links: &BodyLinks, toggle: Option<TodoToggle>) -> AnyElement {
    let context = RowContext { post_id: post_id.to_string(), written, today: crate::time::local_now().date(), theme, toggle };
    todo_row(&TodoEntry { depth: 0, ..entry.clone() }, lookup, links, &context)
}
