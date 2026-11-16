//! A single selectable language row inside the language popover.

use super::SelectLangCallback;
use crate::i18n::Language;
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::base::StyledExt;
use gpui_kit::component::Icon;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::{
    div, Div, InteractiveElement, ParentElement, Stateful, StatefulInteractiveElement, Styled,
};
use std::rc::Rc;

/// Element id prefix of each language row (suffixed with the row index).
const ITEM_ID_PREFIX: &str = "lang-item-";

/// Renders one language row; clicking it selects that language.
pub(super) fn render_language_item(
    idx: usize,
    lang: Language,
    is_active: bool,
    on_select: Rc<SelectLangCallback>,
    theme: &ThemeTokens,
) -> Stateful<Div> {
    let hover_bg = theme.hover_bg;
    div()
        .id(format!("{ITEM_ID_PREFIX}{idx}"))
        .px(theme.spacing_sm_plus)
        .py(theme.spacing_xs_plus)
        .rounded(theme.radius_sm)
        .cursor_pointer()
        .flex()
        .items_center()
        .justify_between()
        .when(is_active, |this| this.bg(hover_bg))
        .hover(move |s| s.bg(hover_bg))
        .on_click(move |_, window, cx| {
            on_select(lang, window, cx);
        })
        .child(render_label(lang, is_active, theme))
        .child(
            div()
                .text_size(theme.font_size_label)
                .text_color(theme.text_muted)
                .child(lang.code().to_uppercase()),
        )
}

/// Check-mark slot followed by the language's native name.
fn render_label(lang: Language, is_active: bool, theme: &ThemeTokens) -> Div {
    let accent_agent = theme.accent_agent;
    let label_color = if is_active { accent_agent } else { theme.text_primary };
    div()
        .flex()
        .items_center()
        .gap(theme.spacing_sm)
        .child(
            div()
                .w(theme.icon_size_lg)
                .h(theme.icon_size_lg)
                .flex()
                .items_center()
                .justify_center()
                .when(is_active, |this| {
                    this.text_color(accent_agent).child(Icon::new(LucideIcon::Check))
                }),
        )
        .child(
            div()
                .text_size(theme.font_size_label)
                .font_medium()
                .text_color(label_color)
                .child(lang.native_label()),
        )
}
