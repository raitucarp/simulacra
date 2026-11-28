//! Custom rows the kit's plain item cannot draw: a destructive item in the theme's danger
//! colour, and a disabled item that says why it cannot be used.

use super::text::MenuText;
use gpui_kit::component::menu::PopupMenuItem;
use gpui_kit::component::ActiveTheme;
use gpui_kit::{div, ParentElement, Styled};

/// A row whose label is drawn in the danger colour.
pub(super) fn destructive_row(label: &MenuText) -> PopupMenuItem {
    let label = label.text.clone();
    PopupMenuItem::element(move |_, cx| div().flex_1().text_color(cx.theme().danger).child(label.clone()))
}

/// A row with a second, muted line under its label.
pub(super) fn detail_row(label: &MenuText, detail: &MenuText) -> PopupMenuItem {
    let (label, detail) = (label.text.clone(), detail.text.clone());
    PopupMenuItem::element(move |_, cx| div().flex_1().flex().flex_col().child(label.clone()).child(div().text_color(cx.theme().muted_foreground).child(detail.clone())))
}

/// A greyed-out row with its reason (when it has one) under the label.
pub(super) fn disabled_row(label: &MenuText, reason: &MenuText) -> PopupMenuItem {
    let (label, reason) = (label.text.clone(), reason.text.clone());
    PopupMenuItem::element(move |_, cx| {
        let muted = cx.theme().muted_foreground;
        let reason = (!reason.is_empty()).then(|| div().text_color(muted).child(reason.clone()));
        div().flex_1().flex().flex_col().text_color(muted).child(label.clone()).children(reason)
    })
}
