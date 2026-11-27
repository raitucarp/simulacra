//! Link previews: the compact card (image, title, description, site), the skeleton shown
//! while it loads, and the plain chip shown when previews are off or the fetch failed.

use super::consts::{DESCRIPTION_LINES, LINK_CARD_ID, LINK_CHIP_ID, META_SEPARATOR, SHOWN_SCHEMES, SKELETON_SHORT_LINE, TITLE_LINES};
use super::image_tile::round_corners;
use crate::domain::media::{display_domain, Corners, PreviewEntry};
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::Icon;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::{div, img, Div, FontWeight, InteractiveElement, IntoElement, MouseButton, ObjectFit, ParentElement, Stateful, StatefulInteractiveElement, Styled, StyledImage};
use rust_i18n::t;
use std::path::PathBuf;

/// The corners of a card a left-hand thumbnail touches.
const LEFT_CORNERS: Corners = [true, false, true, false];

/// A bordered surface that opens `url` in the system browser (without selecting the post).
pub(super) fn opens_url(id: &'static str, url: &str, theme: &ThemeTokens) -> Stateful<Div> {
    let (url, hover_border) = (url.to_string(), theme.border_strong);
    div()
        .id(id)
        .w_full()
        .min_w_0()
        .rounded(theme.radius_lg)
        .overflow_hidden()
        .bg(theme.bg_card)
        .border(theme.border_width_hairline)
        .border_color(theme.border_subtle)
        .hover(move |s| s.border_color(hover_border))
        .cursor_pointer()
        .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
        .on_click(move |_, _, cx| cx.open_url(&url))
}

/// The preview card of `url`.
pub(super) fn link_card(url: &str, entry: &PreviewEntry, theme: &ThemeTokens) -> Stateful<Div> {
    let preview = entry.preview.clone().unwrap_or_default();
    let text = div()
        .flex_1()
        .min_w_0()
        .p(theme.card_padding)
        .flex()
        .flex_col()
        .justify_center()
        .gap(theme.spacing_2xs)
        .child(site_row(url, &preview.site_name, entry.icon_path.as_deref(), theme))
        .when(!preview.title.is_empty(), |col| col.child(title_text(&preview.title, theme)))
        .when(!preview.description.is_empty(), |col| {
            col.child(div().text_size(theme.font_size_xs).text_color(theme.text_secondary).line_clamp(DESCRIPTION_LINES).text_ellipsis().child(preview.description.clone()))
        });
    let thumb = entry.image_path.as_ref().map(|path| {
        let still = round_corners(img(PathBuf::from(path)), LEFT_CORNERS, theme.radius_lg).absolute().inset_0().size_full().object_fit(ObjectFit::Cover);
        div().relative().flex_none().w(theme.link_card_thumb_size).bg(theme.bg_elevated).child(still)
    });
    opens_url(LINK_CARD_ID, url, theme).flex().items_stretch().min_h(theme.link_card_thumb_size).children(thumb).child(text)
}

/// A card title.
pub(super) fn title_text(title: &str, theme: &ThemeTokens) -> Div {
    div()
        .text_size(theme.font_size_sm)
        .font_weight(FontWeight::SEMIBOLD)
        .text_color(theme.text_primary)
        .line_clamp(TITLE_LINES).text_ellipsis()
        .child(title.to_string())
}

/// Site icon (or a globe) and the site name with its domain.
fn site_row(url: &str, site_name: &str, icon_path: Option<&str>, theme: &ThemeTokens) -> Div {
    let domain = display_domain(url);
    let label = if site_name.is_empty() || site_name.eq_ignore_ascii_case(&domain) { domain } else { format!("{site_name}{META_SEPARATOR}{domain}") };
    let icon = match icon_path {
        Some(path) => img(PathBuf::from(path)).size(theme.link_card_icon_size).rounded(theme.radius_xs).into_any_element(),
        None => Icon::new(LucideIcon::Globe).size(theme.link_card_icon_size).into_any_element(),
    };
    div()
        .flex()
        .items_center()
        .gap(theme.inline_gap)
        .text_size(theme.font_size_xs)
        .text_color(theme.text_muted)
        .child(div().flex_none().child(icon))
        .child(div().min_w_0().truncate().child(label))
}

/// Grey bars in the card's shape while the preview loads.
pub(super) fn link_skeleton(theme: &ThemeTokens) -> Div {
    let bar = |width| div().h(theme.font_size_sm).w(width).max_w_full().rounded(theme.radius_sm).bg(theme.hover_bg);
    div()
        .w_full()
        .flex()
        .items_center()
        .rounded(theme.radius_lg)
        .overflow_hidden()
        .border(theme.border_width_hairline)
        .border_color(theme.border_subtle)
        .child(div().flex_none().size(theme.link_card_thumb_size).bg(theme.hover_bg))
        .child(
            div()
                .flex_1()
                .min_w_0()
                .p(theme.card_padding)
                .flex()
                .flex_col()
                .gap(theme.stack_gap)
                .child(bar(theme.media_skeleton_line_width))
                .child(bar(theme.media_skeleton_line_width * SKELETON_SHORT_LINE)),
        )
}

/// A plain chip with the link (or "Video" and the domain) that opens it.
pub(super) fn link_chip(url: &str, is_video: bool, theme: &ThemeTokens) -> Stateful<Div> {
    let (icon, text) = if is_video {
        (LucideIcon::Play, format!("{}{META_SEPARATOR}{}", t!("media.video"), display_domain(url)))
    } else {
        let shown = SHOWN_SCHEMES.iter().find_map(|scheme| url.strip_prefix(scheme)).unwrap_or(url);
        (LucideIcon::Link2, shown.trim_end_matches('/').to_string())
    };
    opens_url(LINK_CHIP_ID, url, theme)
        .w_auto()
        .max_w_full()
        .rounded(theme.radius_full)
        .px(theme.spacing_sm)
        .py(theme.spacing_2xs)
        .flex()
        .items_center()
        .gap(theme.inline_gap)
        .text_size(theme.font_size_xs)
        .text_color(theme.accent_brand_text)
        .child(Icon::new(icon).size(theme.icon_size_sm))
        .child(div().min_w_0().truncate().child(text))
}
