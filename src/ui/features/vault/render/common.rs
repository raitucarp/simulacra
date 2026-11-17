//! Small building blocks shared by the vault screens: headings, tags, meta lines, the
//! progress bar and the calm entrance of a screen.

use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::Icon;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use std::time::Duration;

/// Middle dot between meta items ("1677 · Book").
pub const META_SEPARATOR: &str = " · ";

/// A quiet small-caps section heading.
pub fn section_heading(text: impl Into<SharedString>, theme: &ThemeTokens) -> Div {
    div()
        .font_family(theme.font_family_display.clone())
        .text_size(theme.font_size_2xs)
        .font_weight(FontWeight::BOLD)
        .text_color(theme.text_muted)
        .child(text.into().to_uppercase())
}

/// A heading with an icon and an optional trailing element.
pub fn titled_section(icon: LucideIcon, title: impl Into<SharedString>, theme: &ThemeTokens) -> Div {
    div()
        .flex()
        .items_center()
        .gap(theme.inline_gap)
        .text_color(theme.text_muted)
        .child(Icon::new(icon).size(theme.icon_size_sm))
        .child(section_heading(title, theme))
}

/// A clickable tag (subject, collection, classification value).
pub fn tag(id: impl Into<ElementId>, icon: Option<LucideIcon>, label: String, theme: &ThemeTokens) -> Stateful<Div> {
    let hover = theme.hover_bg;
    div()
        .id(id)
        .flex()
        .items_center()
        .gap(theme.inline_gap)
        .px(theme.spacing_sm)
        .py(theme.spacing_2xs)
        .rounded(theme.radius_full)
        .bg(theme.bg_elevated)
        .border(theme.border_width_hairline)
        .border_color(theme.border_subtle)
        .text_size(theme.font_size_xs)
        .text_color(theme.text_secondary)
        .cursor_pointer()
        .hover(move |s| s.bg(hover))
        .when_some(icon, |s, icon| s.child(Icon::new(icon).size(theme.icon_size_xs)))
        .child(label)
}

/// A non-interactive label in the brand tint (document kind, status).
pub fn badge(icon: LucideIcon, label: String, theme: &ThemeTokens) -> Div {
    div()
        .flex()
        .items_center()
        .gap(theme.inline_gap)
        .px(theme.spacing_sm)
        .py(theme.spacing_2xs)
        .rounded(theme.radius_full)
        .bg(theme.accent_brand_subtle)
        .text_size(theme.font_size_2xs)
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(theme.accent_brand)
        .child(Icon::new(icon).size(theme.icon_size_xs))
        .child(label)
}

/// Muted items joined by middle dots (empty items skipped).
pub fn meta_line(items: Vec<String>, theme: &ThemeTokens) -> Div {
    let text = items.into_iter().filter(|item| !item.trim().is_empty()).collect::<Vec<_>>().join(META_SEPARATOR);
    div().text_size(theme.font_size_xs).text_color(theme.text_muted).child(text)
}

/// A thin progress bar `percent` full.
pub fn progress_bar(percent: u8, theme: &ThemeTokens) -> Div {
    let fraction = f32::from(percent.min(100)) / 100.0;
    div()
        .w_full()
        .h(theme.vault_progress_height)
        .rounded(theme.radius_full)
        .bg(theme.border_subtle)
        .child(div().h_full().w(relative(fraction)).rounded(theme.radius_full).bg(theme.accent_success))
}

/// A card surface.
pub fn card(theme: &ThemeTokens) -> Div {
    div()
        .p(theme.card_padding)
        .rounded(theme.radius_lg)
        .border(theme.border_width_hairline)
        .border_color(theme.border)
        .bg(theme.bg_card)
        .flex()
        .flex_col()
        .gap(theme.stack_gap)
}

/// `element` fading in over the vault's calm duration (instantly with reduced motion).
pub fn entrance(element: Div, id: impl Into<ElementId>, reduced_motion: bool, theme: &ThemeTokens) -> AnyElement {
    if reduced_motion {
        return element.into_any_element();
    }
    let duration = Duration::from_millis(theme.vault_motion_duration_ms as u64);
    element.with_animation(id, Animation::new(duration).with_easing(ease_in_out), |el, delta| el.opacity(delta)).into_any_element()
}
