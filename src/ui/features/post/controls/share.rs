//! Clipboard sharing of a thought: permalink, share text and the share button.

use super::super::consts::SHARE_BTN_ID_PREFIX;
use crate::domain::{Post, Profile};
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::notification::Notification;
use gpui_kit::component::{Icon, WindowExt};
use gpui_kit::{
    div, App, ClipboardItem, Div, InteractiveElement, ParentElement, Stateful,
    StatefulInteractiveElement, Styled, Window,
};
use rust_i18n::t;

/// Blank line separating the thought from its attribution.
const SHARE_PARAGRAPH_BREAK: &str = "\n\n";
/// Leading dash of the attribution line.
const SHARE_ATTRIBUTION_DASH: &str = "— ";
/// Line break before the permalink.
const SHARE_LINE_BREAK: &str = "\n";

/// Notification id so repeated copies replace the previous toast.
struct ClipboardNotice;

/// Stable local permalink of a thought: `simulacra://<network>/post/<id>`.
pub fn post_permalink(post: &Post) -> String {
    crate::domain::post::distill::post_link(post.network_id.as_str(), post.id.as_str())
}

/// Shareable text: the thought, an attribution line and its permalink.
pub fn share_text(post: &Post, author: &Profile) -> String {
    format!(
        "{}{SHARE_PARAGRAPH_BREAK}{SHARE_ATTRIBUTION_DASH}{} {}{SHARE_LINE_BREAK}{}",
        post.content.trim(),
        author.name,
        author.handle,
        post_permalink(post)
    )
}

/// Writes `text` to the system clipboard and confirms with a toast.
pub(crate) fn copy_with_toast(text: String, toast: String, window: &mut Window, cx: &mut App) {
    cx.write_to_clipboard(ClipboardItem::new_string(text));
    window.push_notification(Notification::success(toast).id::<ClipboardNotice>(), cx);
}

/// Share icon: copies the share text (thought, author, permalink) to the clipboard.
pub(crate) fn render_share_button(post: &Post, author: &Profile, theme: &ThemeTokens) -> Stateful<Div> {
    let hover_color = theme.text_primary;
    let text = share_text(post, author);
    div()
        .id(format!("{SHARE_BTN_ID_PREFIX}{}", post.id))
        .flex()
        .items_center()
        .py(theme.spacing_2xs)
        .cursor_pointer()
        .text_color(theme.text_muted)
        .hover(move |s| s.text_color(hover_color))
        .on_click(move |_, window, cx| {
            copy_with_toast(text.clone(), t!("post.shared").to_string(), window, cx);
        })
        .child(Icon::new(LucideIcon::Share2).size(theme.font_size_md))
}
