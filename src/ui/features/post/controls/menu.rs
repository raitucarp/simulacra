//! The post card's menu (M1): the ghost "…" button that opens it, the right-click menu on the
//! card that shows the same spec, and Delete with its confirmation.

use super::super::consts::OVERFLOW_BTN_ID_PREFIX;
use super::handlers::{PostAction, PostCardHandlers};
use super::post_menu::{post_menu_spec, PostMenuSource};
use crate::domain::Post;
use crate::ui::icons::LucideIcon;
use crate::ui::menus::{context_menu_of, post_ids as ids, render_menu, MenuItem, MenuText};
use gpui_kit::component::button::{Button, ButtonVariant, ButtonVariants};
use gpui_kit::component::menu::{ContextMenu, ContextMenuExt, DropdownMenu};
use gpui_kit::component::{Icon, Sizable, WindowExt};
use gpui_kit::{Anchor, App, InteractiveElement, IntoElement, ParentElement, Styled, Window};
use rust_i18n::t;

/// Ghost "…" button opening the post's menu.
pub(crate) fn render_overflow_menu(post: &Post, handlers: &PostCardHandlers) -> impl IntoElement {
    let source = PostMenuSource::of(post, handlers);
    Button::new(format!("{OVERFLOW_BTN_ID_PREFIX}{}", post.id))
        .ghost()
        .xsmall()
        .icon(Icon::new(LucideIcon::Ellipsis))
        .tooltip(t!("post.menu_tooltip").to_string())
        .dropdown_menu_with_anchor(Anchor::TopRight, move |menu, window, cx| render_menu(menu, &post_menu_spec(&source), window, cx))
}

/// `card` with the post menu on right-click (and long press).
pub(crate) fn with_post_context_menu<E: InteractiveElement + ParentElement + Styled>(card: E, post: &Post, handlers: &PostCardHandlers) -> ContextMenu<E> {
    let source = PostMenuSource::of(post, handlers);
    card.context_menu(context_menu_of(move || post_menu_spec(&source)))
}

/// "Delete" (red), asking for confirmation first.
pub(super) fn delete_item(on_delete: PostAction) -> MenuItem {
    MenuItem::new(ids::DELETE, MenuText::key("post.menu_delete")).icon(LucideIcon::Trash).destructive().on_click(move |window, cx| confirm_delete(on_delete.clone(), window, cx))
}

/// Asks for confirmation before running `on_delete`.
fn confirm_delete(on_delete: PostAction, window: &mut Window, cx: &mut App) {
    window.open_alert_dialog(cx, move |alert, _, _| {
        let on_delete = on_delete.clone();
        alert
            .confirm()
            .title(t!("post.delete_title").to_string())
            .description(t!("post.delete_desc").to_string())
            .ok_text(t!("post.menu_delete").to_string())
            .ok_variant(ButtonVariant::Danger)
            .cancel_text(t!("composer.cancel").to_string())
            .on_ok(move |_, window, cx| {
                on_delete(window, cx);
                true
            })
    });
}

/// N1: the embed of `post` pinned to the version it shows, ready to paste into another post.
pub(crate) fn embed_code(post: &Post) -> String {
    crate::domain::post::embed_markup(post.id.as_str(), Some(post.extras.head.as_ref().map_or_else(Default::default, |head| head.version)))
}
