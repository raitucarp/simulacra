//! Prominent "+ New Thread" primary action (icon-only when the rail is collapsed).

use super::NewThreadCallback;
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::Icon;
use gpui_kit::{div, AnyElement, IntoElement, ParentElement, Styled};
use rust_i18n::t;

const NEW_THREAD_COLLAPSED_ID: &str = "new-thread-collapsed-btn";
const NEW_THREAD_EXPANDED_ID: &str = "new-thread-expanded-btn";

pub(super) fn render_new_thread_button(
    on_new: NewThreadCallback,
    is_collapsed: bool,
    theme: &ThemeTokens,
) -> AnyElement {
    if is_collapsed {
        div()
            .w_full()
            .flex()
            .justify_center()
            .py(theme.spacing_xs)
            .child(
                Button::new(NEW_THREAD_COLLAPSED_ID)
                    .primary()
                    .icon(Icon::new(LucideIcon::Plus))
                    .on_click(move |_, window, cx| (on_new)(window, cx)),
            )
            .into_any_element()
    } else {
        div()
            .w_full()
            .py(theme.spacing_xs)
            .child(
                Button::new(NEW_THREAD_EXPANDED_ID)
                    .primary()
                    .w_full()
                    .icon(Icon::new(LucideIcon::Plus))
                    .label(t!("composer.new_thread").to_string())
                    .on_click(move |_, window, cx| (on_new)(window, cx)),
            )
            .into_any_element()
    }
}
