//! Focus Thread & Epistemic Inspector pane: the conversation around the selected post.

use super::callbacks::PostCallbacks;
use super::consts::{INSPECTOR_SCROLL_ID, MAX_INSPECTOR_NESTED_REPLIES, MAX_INSPECTOR_REPLIES};
use super::inspector_header::{render_inspector_header, render_quick_reply_bar};
use super::inspector_outline::{render_outline_pane, InspectorCallback, InspectorMode, InspectorView};
use super::inspector_replies::conversation_children;
use super::params::ToggleRepliesCallback;
use crate::domain::Post;
use crate::state::{build_conversation, AppState, Conversation, ReplyLimits};
use crate::theme::ThemeTokens;
use gpui_kit::component::scroll::ScrollableElement;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::{div, Div, InteractiveElement, IntoElement, ParentElement, Styled};
use rust_i18n::t;
use std::rc::Rc;

/// Shared inputs of the inspector renderers.
pub(crate) struct InspectorContext<'a> {
    pub state: &'a AppState,
    pub theme: &'a ThemeTokens,
    pub callbacks: &'a PostCallbacks,
    /// Whether every reply is shown instead of the collapsed preview.
    pub expanded: bool,
    pub on_toggle_replies: Rc<ToggleRepliesCallback>,
    /// sb C2: cards or outline, the outline's memory and focus, and the commands they send.
    pub view: &'a InspectorView,
    pub on_inspector: InspectorCallback,
}

impl InspectorContext<'_> {
    /// Reply caps for the current expansion state.
    pub(crate) fn limits(&self) -> ReplyLimits {
        if self.expanded {
            ReplyLimits::UNLIMITED
        } else {
            ReplyLimits { direct: MAX_INSPECTOR_REPLIES, nested: MAX_INSPECTOR_NESTED_REPLIES }
        }
    }
}

/// Inspector pane: header, scrollable conversation, and quick reply bar.
pub(crate) fn render_inspector_pane(ctx: &InspectorContext<'_>, focused: Option<&Post>) -> Div {
    let theme = ctx.theme;
    let conversation = focused.map(|post| build_conversation(ctx.state, post, ctx.limits()));
    let replies_count = conversation.as_ref().map(|c| c.direct_total);
    let outline = focused.filter(|_| ctx.view.mode == InspectorMode::Outline);
    let body = match outline {
        Some(post) => render_outline_pane(ctx.state, post, ctx.view, &ctx.on_inspector, theme).into_any_element(),
        None => render_thread_scroll(ctx, conversation.as_ref()).into_any_element(),
    };
    div()
        .w_full()
        .h_full()
        .flex()
        .flex_col()
        .bg(theme.bg_app)
        .child(render_inspector_header(theme, replies_count, ctx.view.mode, &ctx.on_inspector))
        .child(body)
        .when_some(focused, |this, root| {
            this.child(render_quick_reply_bar(root, ctx.state.author_of(root), theme, ctx.callbacks.on_reply.clone()))
        })
}

/// Scrollable conversation (ancestors, focused post, replies) or the empty state.
fn render_thread_scroll(ctx: &InspectorContext<'_>, conversation: Option<&Conversation<'_>>) -> impl IntoElement {
    let theme = ctx.theme;
    let is_empty = conversation.is_none();
    div()
        .id(INSPECTOR_SCROLL_ID)
        .flex_1()
        .overflow_y_scrollbar()
        .flex()
        .flex_col()
        .when_some(conversation, |this, c| this.children(conversation_children(ctx, c)))
        .when(is_empty, |this| {
            this.p(theme.panel_padding).items_center().justify_center().child(
                div()
                    .text_size(theme.font_size_sm)
                    .text_color(theme.text_muted)
                    .child(t!("feed.select_to_inspect").to_string()),
            )
        })
}
