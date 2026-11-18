//! The inspector header's view switch (sb C2): post cards or the text-only outline.

use super::inspector_outline::{InspectorCallback, InspectorCommand, InspectorMode};
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::{Icon, Selectable, Sizable};
use gpui_kit::{div, Div, ParentElement, Styled};
use rust_i18n::t;

const CARDS_ID: &str = "inspector-mode-cards";
const OUTLINE_ID: &str = "inspector-mode-outline";
const ARGUMENT_ID: &str = "inspector-mode-argument";

/// Two small icon buttons, the active one in the accent.
pub(crate) fn render_mode_toggle(mode: InspectorMode, on_command: &InspectorCallback, theme: &ThemeTokens) -> Div {
    let button = |id: &'static str, target: InspectorMode, icon: LucideIcon, tooltip: String| {
        let on_command = on_command.clone();
        let color = if mode == target { theme.accent_brand } else { theme.text_muted };
        Button::new(id)
            .ghost()
            .xsmall()
            .selected(mode == target)
            .icon(Icon::new(icon).text_color(color))
            .tooltip(tooltip)
            .on_click(move |_, window, cx| on_command(InspectorCommand::SetMode(target), window, cx))
    };
    div()
        .flex_none()
        .flex()
        .items_center()
        .gap(theme.spacing_3xs)
        .child(button(CARDS_ID, InspectorMode::Cards, LucideIcon::LayoutGrid, t!("feed.outline.show_cards").to_string()))
        .child(button(OUTLINE_ID, InspectorMode::Outline, LucideIcon::ListTree, t!("feed.outline.show_outline").to_string()))
        .child(button(ARGUMENT_ID, InspectorMode::Argument, LucideIcon::Waypoints, t!("thinking.argument.show").to_string()))
}
