//! Conversation cards: ancestor chain → focused post → reply branches → "show more" toggle.

use super::callbacks::CardMode;
use super::consts::{MAX_INSPECTOR_NESTED_REPLIES, MAX_INSPECTOR_REPLIES, TOGGLE_REPLIES_BUTTON_ID};
use super::inspector::InspectorContext;
use super::inspector_grounding::render_grounding_callout;
use crate::domain::Post;
use crate::state::{Conversation, ReplyBranch};
use crate::ui::features::post::{render_extras_detail, ThreadlineStyle};
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::{div, AnyElement, IntoElement, ParentElement, Styled};
use crate::i18n::t_count;
use rust_i18n::t;

/// Threadline style from whether the card connects upward and downward.
fn style_for(has_top: bool, has_bottom: bool) -> ThreadlineStyle {
    match (has_top, has_bottom) {
        (false, false) => ThreadlineStyle::None,
        (false, true) => ThreadlineStyle::ParentHasReplies,
        (true, true) => ThreadlineStyle::MiddleReply,
        (true, false) => ThreadlineStyle::LastReply,
    }
}

fn card(ctx: &InspectorContext<'_>, post: &Post, style: ThreadlineStyle) -> AnyElement {
    let mode = CardMode { is_focused: false, style, selectable: true };
    ctx.callbacks.render_post(post, ctx.state, ctx.theme, mode)
}

/// Every card of the conversation, in reading order.
pub(crate) fn conversation_children(ctx: &InspectorContext<'_>, conversation: &Conversation<'_>) -> Vec<AnyElement> {
    let mut children: Vec<AnyElement> = conversation
        .ancestors
        .iter()
        .enumerate()
        .map(|(idx, ancestor)| card(ctx, ancestor, style_for(idx > 0, true)))
        .collect();
    children.push(focused_card(ctx, conversation));
    if conversation.branches.is_empty() {
        children.push(render_grounding_callout(ctx.theme).into_any_element());
    }
    for branch in &conversation.branches {
        children.extend(branch_cards(ctx, branch));
    }
    if let Some(toggle) = replies_toggle(ctx, conversation) {
        children.push(toggle);
    }
    children
}

/// The inspected post, elevated, joined to its ancestor chain.
fn focused_card(ctx: &InspectorContext<'_>, conversation: &Conversation<'_>) -> AnyElement {
    let style = style_for(!conversation.ancestors.is_empty(), false);
    let mode = CardMode { is_focused: true, style, selectable: false };
    div()
        .w_full()
        .bg(ctx.theme.bg_elevated)
        .child(ctx.callbacks.render_post(conversation.focused, ctx.state, ctx.theme, mode))
        .children(render_extras_detail(&conversation.focused.extras, ctx.theme, ctx.callbacks.links.on_source.as_ref()))
        .into_any_element()
}

/// A direct reply threaded to its own (one level) replies.
fn branch_cards(ctx: &InspectorContext<'_>, branch: &ReplyBranch<'_>) -> Vec<AnyElement> {
    let nested_len = branch.nested.len();
    let mut cards = vec![card(ctx, branch.reply, style_for(false, nested_len > 0))];
    cards.extend(
        branch
            .nested
            .iter()
            .enumerate()
            .map(|(idx, nested)| card(ctx, nested, style_for(true, idx + 1 < nested_len))),
    );
    cards
}

/// "Show N more replies" while collapsed, "Show fewer replies" once expanded (when it matters).
fn replies_toggle(ctx: &InspectorContext<'_>, conversation: &Conversation<'_>) -> Option<AnyElement> {
    let label = if conversation.hidden > 0 {
        t_count("feed.show_more_replies", conversation.hidden)
    } else if ctx.expanded && collapsible(conversation) {
        t!("feed.show_fewer_replies").to_string()
    } else {
        return None;
    };
    let icon = if ctx.expanded { LucideIcon::ChevronUp } else { LucideIcon::ChevronDown };
    let on_toggle = ctx.on_toggle_replies.clone();
    let button = Button::new(TOGGLE_REPLIES_BUTTON_ID)
        .ghost()
        .small()
        .icon(Icon::new(icon))
        .label(label)
        .on_click(move |_, window, cx| on_toggle(window, cx));
    let theme = ctx.theme;
    Some(div().w_full().px(theme.panel_padding).py(theme.spacing_sm).child(button).into_any_element())
}

/// Whether the collapsed preview would hide any of the replies shown now.
fn collapsible(conversation: &Conversation<'_>) -> bool {
    conversation.direct_total > MAX_INSPECTOR_REPLIES
        || conversation.branches.iter().any(|b| b.nested.len() > MAX_INSPECTOR_NESTED_REPLIES)
}
