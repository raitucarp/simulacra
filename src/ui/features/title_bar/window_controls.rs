//! Frameless window controls (minimize, maximize/restore, close).

use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::Icon;
use gpui_kit::{
    div, Div, InteractiveElement, ParentElement, Rgba, Stateful, Styled,
    WindowControlArea,
};

const WIN_CONTROL_MIN_ID: &str = "win-control-min";
const WIN_CONTROL_MAX_ID: &str = "win-control-max";
const WIN_CONTROL_CLOSE_ID: &str = "win-control-close";

/// Min, Max/Restore and Close controls (hover fill and its readable icon color per control), right-aligned in the title bar.
pub(super) fn render_window_controls(is_maximized: bool, theme: &ThemeTokens) -> Div {
    let max_icon = if is_maximized {
        LucideIcon::Minimize2
    } else {
        LucideIcon::Maximize2
    };
    let hover = (theme.bg_window_control_hover, theme.text_primary);
    div()
        .flex()
        .items_center()
        .gap_1()
        .pl_2()
        // Minimize
        .child(window_control(WIN_CONTROL_MIN_ID, WindowControlArea::Min, LucideIcon::Minus, hover, theme))
        // Maximize / Restore
        .child(window_control(WIN_CONTROL_MAX_ID, WindowControlArea::Max, max_icon, hover, theme))
        // Close
        .child(window_control(
            WIN_CONTROL_CLOSE_ID,
            WindowControlArea::Close,
            LucideIcon::X,
            (theme.bg_window_control_close_hover, theme.on_danger),
            theme,
        ))
}

fn window_control(
    id: &'static str,
    area: WindowControlArea,
    icon: LucideIcon,
    (hover_bg, hover_text): (Rgba, Rgba),
    theme: &ThemeTokens,
) -> Stateful<Div> {
    div()
        .id(id)
        .w(theme.win_control_width)
        .h(theme.win_control_height)
        .rounded_sm()
        .flex()
        .items_center()
        .justify_center()
        .text_color(theme.text_secondary)
        .hover(move |s| s.bg(hover_bg).text_color(hover_text))
        .window_control_area(area)
        .child(Icon::new(icon))
}
