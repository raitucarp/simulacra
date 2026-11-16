//! Navigation item button molecule with 100% Lucide SVG icons.

use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::base::StyledExt;
use gpui_kit::component::Icon;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::{
    div, Div, InteractiveElement, IntoElement, ParentElement, Rgba, StatefulInteractiveElement,
    Styled, Window,
};

/// Element id prefix of each nav button (suffixed with its label).
const NAV_BUTTON_ID_PREFIX: &str = "nav-btn-";

pub fn render_nav_button(
    label: &str,
    icon_name: LucideIcon,
    is_active: bool,
    is_collapsed: bool,
    badge: Option<usize>,
    theme: &ThemeTokens,
    on_click: impl Fn(&gpui_kit::ClickEvent, &mut Window, &mut gpui_kit::App) + 'static,
) -> impl IntoElement {
    let bg_col = if is_active { theme.hover_bg } else { theme.bg_sidebar };
    let hover_bg = theme.hover_bg;

    div()
        .id(format!("{NAV_BUTTON_ID_PREFIX}{}", label))
        .w_full()
        .px_2p5()
        .py_2()
        .rounded_md()
        .bg(bg_col)
        .cursor_pointer()
        .hover(move |s| s.bg(hover_bg))
        .on_click(on_click)
        .flex()
        .items_center()
        .when(is_collapsed, |this| this.justify_center())
        .when(!is_collapsed, |this| this.justify_between())
        .child(render_leading(label, icon_name, is_active, is_collapsed, theme))
        .when(!is_collapsed, |this| this.child(render_trailing(badge, is_active, theme)))
}

/// Icon glyph plus (when expanded) the truncated label.
fn render_leading(
    label: &str,
    icon_name: LucideIcon,
    is_active: bool,
    is_collapsed: bool,
    theme: &ThemeTokens,
) -> Div {
    let text_col = if is_active { theme.text_primary } else { theme.text_secondary };
    let glyph_col: Rgba = if is_active { theme.accent_agent } else { theme.text_muted };
    div()
        .flex()
        .items_center()
        .gap_2p5()
        .min_w_0()
        .child(
            div()
                .w_5()
                .flex()
                .items_center()
                .justify_center()
                .text_color(glyph_col)
                .child(Icon::new(icon_name)),
        )
        .when(!is_collapsed, |this| {
            this.child(
                div()
                    .font_family(theme.font_family_ui.clone())
                    .text_xs()
                    .font_medium()
                    .text_color(text_col)
                    .min_w_0()
                    .truncate()
                    .child(label.to_string()),
            )
        })
}

/// Optional count badge plus the active-state dot.
fn render_trailing(badge: Option<usize>, is_active: bool, theme: &ThemeTokens) -> Div {
    div()
        .flex()
        .items_center()
        .gap_2()
        .when_some(badge.filter(|count| *count > 0), |this, count| {
            this.child(render_badge(count, theme))
        })
        .when(is_active, |this| {
            this.child(div().w_1p5().h_1p5().rounded_full().bg(theme.accent_agent))
        })
}

fn render_badge(count: usize, theme: &ThemeTokens) -> Div {
    div()
        .px_1p5()
        .py_0p5()
        .rounded_md()
        .bg(theme.bg_card)
        .border_1()
        .border_color(theme.border)
        .font_family(theme.font_family_mono.clone())
        .text_xs()
        .font_medium()
        .text_color(theme.text_secondary)
        .child(count.to_string())
}
