//! Focus Thread & Epistemic Inspector pane (replaces the redundant second timeline).

use super::callbacks::{CardMode, PostCallbacks};
use super::consts::{INSPECTOR_SCROLL_ID, MAX_INSPECTOR_REPLIES};
use super::inspector_header::{
    render_grounding_callout, render_inspector_header, render_quick_reply_bar,
};
use crate::domain::Post;
use crate::state::AppState;
use crate::theme::ThemeTokens;
use crate::ui::features::post::ThreadlineStyle;
use gpui_kit::component::scroll::ScrollableElement;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::{div, AnyElement, Div, InteractiveElement, IntoElement, ParentElement, Styled};
use rust_i18n::t;

/// Shared inputs of the inspector renderers.
pub(crate) struct InspectorContext<'a> {
    pub state: &'a AppState,
    pub theme: &'a ThemeTokens,
    pub callbacks: &'a PostCallbacks,
}

/// Inspector pane: header, scrollable focused thread, and quick reply bar.
pub(crate) fn render_inspector_pane(ctx: &InspectorContext<'_>, focused: Option<&Post>) -> Div {
    let theme = ctx.theme;
    let thread = focused.map(|root| (root, render_thread_replies(ctx, root)));
    let replies_count = thread.as_ref().map(|(_, (_, total))| *total);
    div()
        .w_full()
        .h_full()
        .flex()
        .flex_col()
        .bg(theme.bg_app)
        .child(render_inspector_header(theme, replies_count))
        .child(render_thread_scroll(ctx, thread.map(|(root, (replies, _))| (root, replies))))
        .when_some(focused, |this, root| {
            this.child(render_quick_reply_bar(root, theme, ctx.callbacks.on_reply.clone()))
        })
}

/// Renders up to [`MAX_INSPECTOR_REPLIES`] replies of `root`; returns them with the total count.
fn render_thread_replies(ctx: &InspectorContext<'_>, root: &Post) -> (Vec<AnyElement>, usize) {
    // Retrieve direct or linked replies in O(1) time via precomputed index
    let replies_list = ctx.state.replies_for_post(&root.id);
    let total = replies_list.len();
    let rendered_len = total.min(MAX_INSPECTOR_REPLIES);
    let elements = replies_list
        .iter()
        .take(MAX_INSPECTOR_REPLIES)
        .enumerate()
        .map(|(idx, reply)| {
            let style = if idx + 1 < rendered_len {
                ThreadlineStyle::MiddleReply
            } else {
                ThreadlineStyle::LastReply
            };
            let mode = CardMode { is_focused: false, style, selectable: true };
            ctx.callbacks.render_post(reply, ctx.state, ctx.theme, mode)
        })
        .collect();
    (elements, total)
}

/// Scrollable thread body: inspected root, connected replies or grounding insight, or empty state.
fn render_thread_scroll(
    ctx: &InspectorContext<'_>,
    thread: Option<(&Post, Vec<AnyElement>)>,
) -> impl IntoElement {
    let theme = ctx.theme;
    let is_empty = thread.is_none();
    div()
        .id(INSPECTOR_SCROLL_ID)
        .flex_1()
        .overflow_y_scrollbar()
        .flex()
        .flex_col()
        .when_some(thread, |this, (root, replies)| this.children(render_thread_children(ctx, root, replies)))
        .when(is_empty, |this| {
            this.p(theme.spacing_2xl).items_center().justify_center().child(
                div()
                    .text_size(theme.font_size_sm)
                    .text_color(theme.text_muted)
                    .child(t!("feed.select_to_inspect").to_string()),
            )
        })
}

fn render_thread_children(ctx: &InspectorContext<'_>, root: &Post, replies: Vec<AnyElement>) -> Vec<AnyElement> {
    let has_replies = !replies.is_empty();
    let style = if has_replies { ThreadlineStyle::ParentHasReplies } else { ThreadlineStyle::None };
    let mode = CardMode { is_focused: true, style, selectable: false };
    let root_card = div()
        .w_full()
        .bg(ctx.theme.bg_elevated)
        .child(ctx.callbacks.render_post(root, ctx.state, ctx.theme, mode));
    let mut children = vec![root_card.into_any_element()];
    if has_replies {
        children.extend(replies);
    } else {
        children.push(render_grounding_callout(ctx.theme).into_any_element());
    }
    children
}
