//! "…" overflow menu of a post card: copy, reply, quote, find similar, learn (a submenu)
//! and (own thoughts) delete.

use super::super::consts::OVERFLOW_BTN_ID_PREFIX;
use super::actions_menu::with_actions_submenu;
use super::handlers::{PostAction, PostCardHandlers};
use super::learn::with_learn_submenu;
use super::share::{copy_with_toast, post_permalink};
use crate::domain::Post;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariant, ButtonVariants};
use gpui_kit::component::menu::{DropdownMenu, PopupMenu, PopupMenuItem};
use gpui_kit::component::{Icon, Sizable, WindowExt};
use gpui_kit::{Anchor, App, Context, IntoElement, Window};
use rust_i18n::t;

/// Ghost "…" button opening the post's action menu.
pub(crate) fn render_overflow_menu(post: &Post, handlers: &PostCardHandlers) -> impl IntoElement {
    let (text, link, embed) = (post.content.clone(), post_permalink(post), embed_code(post));
    let handlers = handlers.clone();
    Button::new(format!("{OVERFLOW_BTN_ID_PREFIX}{}", post.id))
        .ghost()
        .xsmall()
        .icon(Icon::new(LucideIcon::Ellipsis))
        .tooltip(t!("post.menu_tooltip").to_string())
        .dropdown_menu_with_anchor(Anchor::TopRight, move |menu, window, cx| {
            let menu = with_actions_submenu(menu, handlers.on_post_action.as_ref(), window, cx);
            build_menu(menu, [&text, &link, &embed], &handlers, window, cx)
        })
}

fn build_menu(menu: PopupMenu, [text, link, embed]: [&str; 3], handlers: &PostCardHandlers, window: &mut Window, cx: &mut Context<PopupMenu>) -> PopupMenu {
    let menu = menu
        .item(copy_item(LucideIcon::Copy, t!("post.menu_copy_text"), text, t!("post.copied_text")))
        .item(copy_item(LucideIcon::Link2, t!("post.menu_copy_link"), link, t!("post.copied_link")))
        .item(copy_item(LucideIcon::Layers, t!("post.menu_copy_embed"), embed, t!("post.copied_embed")))
        .separator()
        .item(action_item(LucideIcon::MessageSquare, t!("post.menu_reply"), &handlers.on_reply))
        .item(action_item(LucideIcon::Edit, t!("post.menu_quote"), &handlers.on_quote))
        .item(action_item(LucideIcon::Compass, t!("post.menu_find_similar"), &handlers.on_find_similar));
    let menu = with_learn_submenu(menu, &handlers.learn, window, cx);
    let menu = crate::ui::features::thinking::credence::with_credence_item(menu, handlers.credence.as_ref());
    let menu = crate::ui::features::data::with_data_items(menu, handlers.data.as_ref(), window, cx);
    match &handlers.on_delete {
        Some(on_delete) => menu.separator().item(delete_item(on_delete.clone())),
        None => menu,
    }
}

fn copy_item(
    icon: LucideIcon,
    label: impl ToString,
    payload: &str,
    toast: impl ToString,
) -> PopupMenuItem {
    let (payload, toast) = (payload.to_string(), toast.to_string());
    PopupMenuItem::new(label.to_string())
        .icon(Icon::new(icon))
        .on_click(move |_, window, cx| copy_with_toast(payload.clone(), toast.clone(), window, cx))
}

fn action_item(icon: LucideIcon, label: impl ToString, action: &PostAction) -> PopupMenuItem {
    let action = action.clone();
    PopupMenuItem::new(label.to_string())
        .icon(Icon::new(icon))
        .on_click(move |_, window, cx| action(window, cx))
}

fn delete_item(on_delete: PostAction) -> PopupMenuItem {
    PopupMenuItem::new(t!("post.menu_delete").to_string())
        .icon(Icon::new(LucideIcon::Trash))
        .on_click(move |_, window, cx| confirm_delete(on_delete.clone(), window, cx))
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
