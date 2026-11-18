//! The answer's buttons: copy, post as reply, save to vault and re-run.

use super::model::SheetEvent;
use super::sheet::SheetAction;
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::*;
use rust_i18n::t;

const COPY_ID: &str = "post-actions-copy";
const REPLY_ID: &str = "post-actions-reply";
const SAVE_ID: &str = "post-actions-save";
const RERUN_ID: &str = "post-actions-rerun";

/// The button row under a finished answer.
pub(super) fn render_footer(on_event: &SheetAction, theme: &ThemeTokens) -> Div {
    let button = |id: &'static str, icon: LucideIcon, label: String, event: SheetEvent| {
        let on_event = on_event.clone();
        Button::new(id).ghost().small().icon(Icon::new(icon)).label(label).on_click(move |_, window, cx| on_event(event.clone(), window, cx))
    };
    div()
        .flex()
        .flex_wrap()
        .items_center()
        .gap(theme.control_gap)
        .px(theme.panel_padding)
        .py(theme.spacing_sm)
        .border_t(theme.border_width_hairline)
        .border_color(theme.border_subtle)
        .child(button(COPY_ID, LucideIcon::Copy, t!("post_actions.copy").to_string(), SheetEvent::Copy))
        .child(button(REPLY_ID, LucideIcon::MessageSquare, t!("post_actions.reply").to_string(), SheetEvent::PostAsReply))
        .child(button(SAVE_ID, LucideIcon::BookMarked, t!("post_actions.save").to_string(), SheetEvent::SaveToVault))
        .child(div().flex_1())
        .child(button(RERUN_ID, LucideIcon::RefreshCw, t!("post_actions.rerun").to_string(), SheetEvent::Rerun))
}
