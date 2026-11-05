//! Navigation item button molecule with 100% Lucide SVG icons.

use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::base::StyledExt;
use gpui_kit::component::Icon;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::{div, InteractiveElement, IntoElement, ParentElement, StatefulInteractiveElement, Styled, Window};

pub fn render_nav_button(
    label: &str,
    icon_name: LucideIcon,
    is_active: bool,
    is_collapsed: bool,
    badge: Option<usize>,
    theme: &ThemeTokens,
    on_click: impl Fn(&gpui_kit::ClickEvent, &mut Window, &mut gpui_kit::App) + 'static,
) -> impl IntoElement {
    let text_col = if is_active {
        theme.text_primary
    } else {
        theme.text_secondary
    };

    let bg_col = if is_active {
        theme.hover_bg
    } else {
        theme.bg_sidebar
    };

    let glyph_col = if is_active {
        theme.accent_agent
    } else {
        theme.text_muted
    };

    let hover_bg = theme.hover_bg;

    div()
        .id(format!("nav-btn-{}", label))
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
        .child(
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
                }),
        )
        .when(!is_collapsed, |this| {
            this.child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .when_some(badge, |this, count| {
                        if count > 0 {
                            this.child(
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
                                    .child(count.to_string()),
                            )
                        } else {
                            this
                        }
                    })
                    .when(is_active, |this| {
                        this.child(
                            div()
                                .w_1p5()
                                .h_1p5()
                                .rounded_full()
                                .bg(theme.accent_agent),
                        )
                    }),
            )
        })
}
