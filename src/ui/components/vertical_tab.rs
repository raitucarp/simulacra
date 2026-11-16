//! Vertical navigation tab: icon tile, title, monospace subtitle and active indicator bar.

use super::hover_row::selectable_row;
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::Icon;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;

/// Content and state of one [`vertical_tab`].
pub struct VerticalTab {
    pub id: ElementId,
    pub icon: LucideIcon,
    pub title: String,
    pub subtitle: String,
    pub active: bool,
}

/// Renders a rail tab; the active tab is elevated and shows a brand indicator bar.
pub fn vertical_tab(
    tab: VerticalTab,
    theme: &ThemeTokens,
    on_click: impl Fn(&mut Window, &mut App) + 'static,
) -> Stateful<Div> {
    let active = tab.active;
    selectable_row(tab.id,active, theme.accent_brand_border, theme)
        .on_click(move |_, w, cx| on_click(w, cx))
        .flex()
        .items_center()
        .justify_between()
        .gap(theme.spacing_xs)
        .child(
            div()
                .flex()
                .items_center()
                .gap(theme.spacing_sm)
                .min_w_0()
                .flex_1()
                .child(icon_tile(tab.icon, active, theme))
                .child(tab_text(tab.title, tab.subtitle, active, theme)),
        )
        .when(active, |s| s.child(active_bar(theme)))
}

fn icon_tile(icon: LucideIcon, active: bool, theme: &ThemeTokens) -> Div {
    div()
        .w(theme.avatar_size_sm)
        .h(theme.avatar_size_sm)
        .rounded(theme.radius_sm)
        .bg(if active { theme.accent_brand_subtle } else { theme.bg_card })
        .flex()
        .items_center()
        .justify_center()
        .child(
            Icon::new(icon)
                .size(theme.font_size_xs)
                .text_color(if active { theme.accent_brand } else { theme.text_muted }),
        )
}

fn tab_text(title: String, subtitle: String, active: bool, theme: &ThemeTokens) -> Div {
    div()
        .flex_1()
        .min_w_0()
        .flex()
        .flex_col()
        .child(
            div()
                .text_size(theme.font_size_xs)
                .font_weight(if active { FontWeight::BOLD } else { FontWeight::MEDIUM })
                .text_color(if active { theme.text_primary } else { theme.text_secondary })
                .truncate()
                .child(title),
        )
        .child(
            div()
                .font_family(theme.font_family_mono.clone())
                .text_size(theme.font_size_2xs)
                .text_color(theme.text_muted)
                .truncate()
                .child(subtitle),
        )
}

fn active_bar(theme: &ThemeTokens) -> Div {
    div()
        .w(theme.border_width_threadline)
        .h(theme.spacing_lg)
        .rounded(theme.radius_full)
        .bg(theme.accent_brand)
}
