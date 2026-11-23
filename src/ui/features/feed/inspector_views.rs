//! The registry's list views in the thread inspector (epic W): the whole thread, in reading
//! order (the outline's order), shown as the chosen view; a click selects the post. The
//! calendar's month is remembered for the thread it was chosen in.

use super::inspector::InspectorContext;
use super::inspector_outline::{thread_outline, thread_root, InspectorCommand, OutlineNode};
use crate::domain::views::ViewScope;
use crate::domain::Post;
use crate::state::AppState;
use crate::ui::features::views::{render_list_view, OpenPost, PickMonth, ViewInput};
use gpui_kit::AnyElement;
use std::rc::Rc;

/// Every post of the thread `focused` belongs to, root first, in reading order.
pub(crate) fn thread_posts<'a>(state: &'a AppState, focused: &'a Post) -> Vec<&'a Post> {
    let root = thread_root(focused, |id| state.get_post(id));
    let tree = thread_outline(state, root);
    let mut ids = Vec::new();
    collect(&tree, &mut ids);
    ids.into_iter().filter_map(|id| state.get_post(id)).collect()
}

fn collect<'t>(node: &'t OutlineNode, out: &mut Vec<&'t crate::domain::PostId>) {
    out.push(&node.id);
    node.children.iter().for_each(|child| collect(child, out));
}

/// The chosen list view of the thread, or `None` when the inspector draws the view itself.
pub(crate) fn render_thread_view(ctx: &InspectorContext<'_>, focused: &Post) -> Option<AnyElement> {
    let root = thread_root(focused, |id| ctx.state.get_post(id)).id.clone();
    let select = ctx.callbacks.on_select.clone();
    let on_open: OpenPost = Rc::new(move |id, window, cx| select(id, window, cx));
    let (send, thread) = (ctx.on_inspector.clone(), root.clone());
    let on_month: PickMonth = Rc::new(move |month, window, cx| send(InspectorCommand::Month { root: thread.clone(), month }, window, cx));
    let month = ctx.view.month.as_ref().filter(|(owner, _)| *owner == root).map(|(_, month)| *month);
    let input = ViewInput { state: ctx.state, theme: ctx.theme, posts: thread_posts(ctx.state, focused), scheduled: Vec::new(), scope: ViewScope::Thread, on_open, month, on_month };
    render_list_view(ctx.view.mode, &input)
}
