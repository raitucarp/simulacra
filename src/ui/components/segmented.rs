//! Segmented control: a bordered track holding equally-sized icon + label pills.

use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::Icon;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;

/// One option of a segmented control.
pub struct Segment {
    pub id: ElementId,
    pub icon: LucideIcon,
    pub label: String,
    pub active: bool,
    /// Icon tint while this segment is active.
    pub active_icon_color: Rgba,
}

/// The rounded card-surface track that hosts [`segment_pill`]s.
pub fn segmented_track(theme: &ThemeTokens) -> Div {
    div()
        .w_full()
        .p(theme.spacing_2xs)
        .rounded(theme.radius_md)
        .bg(theme.bg_card)
        .border(theme.border_width_hairline)
        .border_color(theme.border_subtle)
        .flex()
        .items_center()
        .gap(theme.spacing_2xs)
}

/// An equally-flexing pill inside a [`segmented_track`]; brand-tinted when active.
pub fn segment_pill(
    segment: Segment,
    theme: &ThemeTokens,
    on_click: impl Fn(&mut Window, &mut App) + 'static,
) -> Stateful<Div> {
    let active = segment.active;
    div()
        .id(segment.id)
        .flex_1()
        .min_w_0()
        .py(theme.spacing_xs)
        .px(theme.spacing_xs)
        .rounded(theme.radius_sm)
        .cursor_pointer()
        .flex()
        .items_center()
        .justify_center()
        .gap(theme.spacing_xs)
        .when(active, |s| {
            s.bg(theme.accent_brand_subtle)
                .border(theme.border_width_hairline)
                .border_color(theme.accent_brand_border)
                .text_color(theme.text_primary)
        })
        .when(!active, |s| s.text_color(theme.text_muted))
        .on_click(move |_, w, cx| on_click(w, cx))
        .child(
            Icon::new(segment.icon)
                .flex_none()
                .size(theme.font_size_xs)
                .text_color(if active { segment.active_icon_color } else { theme.text_muted }),
        )
        .child(
            div()
                .text_size(theme.font_size_2xs)
                .font_weight(FontWeight::SEMIBOLD)
                .truncate()
                .child(segment.label),
        )
}
