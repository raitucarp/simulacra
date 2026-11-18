//! Post attachment helpers. The image galleries, link cards and video cards themselves are
//! rendered by `ui::features::media` (sb C3); this keeps the illustrated stand-in they share.

use crate::domain::engine::ImageEngineKind;
use crate::theme::ThemeTokens;
use crate::ui::illustration::Illustration;
use gpui_kit::{div, AnyElement, IntoElement, ParentElement, Styled};
use rust_i18n::t;

/// An illustrated stand-in for an attached image whose file is gone or unreadable.
pub(crate) fn missing_image(theme: &ThemeTokens) -> impl Fn() -> AnyElement + 'static {
    let (width, padding, color, size) = (theme.forge_option_card_width, theme.spacing_md, theme.text_muted, theme.font_size_xs);
    move || {
        div()
            .size_full()
            .p(padding)
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap(padding)
            .child(Illustration::trait_card(ImageEngineKind::None).width(width))
            .child(div().text_size(size).text_color(color).child(t!("post.image_missing").to_string()))
            .into_any_element()
    }
}
