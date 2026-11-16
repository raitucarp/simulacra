//! Horizontal pill tab (icon + label) used in sub-tab bars.

use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::Icon;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;

/// Content and state of one [`pill_tab`].
pub struct PillTab {
    pub id: ElementId,
    pub icon: LucideIcon,
    pub label: String,
    pub active: bool,
}

/// Renders a bordered pill; the active pill is brand-tinted with a bold label.
pub fn pill_tab(
    tab: PillTab,
    theme: &ThemeTokens,
    on_click: impl Fn(&mut Window, &mut App) + 'static,
) -> Stateful<Div> {
    let active = tab.active;
    let hover_bg = theme.hover_bg;
    div()
        .id(tab.id)
        .px(theme.spacing_md)
        .py(theme.spacing_xs)
        .rounded(theme.radius_md)
        .cursor_pointer()
        .border(theme.border_width_hairline)
        .when(active, |s| {
            s.bg(theme.accent_brand_subtle)
                .border_color(theme.accent_brand)
                .text_color(theme.text_primary)
        })
        .when(!active, |s| {
            s.bg(theme.bg_card)
                .border_color(theme.border_subtle)
                .text_color(theme.text_secondary)
                .hover(move |h| h.bg(hover_bg))
        })
        .on_click(move |_, w, cx| on_click(w, cx))
        .flex()
        .items_center()
        .gap(theme.spacing_xs)
        .child(
            Icon::new(tab.icon)
                .size(theme.font_size_xs)
                .text_color(if active { theme.accent_brand } else { theme.text_muted }),
        )
        .child(pill_label(tab.label, active, theme))
}

fn pill_label(label: String, active: bool, theme: &ThemeTokens) -> Div {
    div()
        .text_size(theme.font_size_xs)
        .font_weight(if active { FontWeight::BOLD } else { FontWeight::MEDIUM })
        .child(label)
}
