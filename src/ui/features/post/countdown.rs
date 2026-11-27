//! The countdown chip of a post that deletes itself (Y7): the time left in its largest whole
//! unit, with a tooltip saying what is deleted with it.

use crate::domain::post::auto_delete::Countdown;
use crate::domain::Post;
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::tooltip::Tooltip;
use gpui_kit::component::Icon;
use gpui_kit::{div, AnyElement, InteractiveElement, IntoElement, ParentElement, SharedString, StatefulInteractiveElement, Styled};
use jiff::Timestamp;
use rust_i18n::t;

const CHIP_ID_PREFIX: &str = "post-countdown-";

/// The time left, as the chip writes it (`3 d`, `5 h`, `12 min`).
pub fn countdown_text(left: Countdown) -> String {
    match left {
        Countdown::Minutes(count) => t!("data.auto_delete.minutes", count = count),
        Countdown::Hours(count) => t!("data.auto_delete.hours", count = count),
        Countdown::Days(count) => t!("data.auto_delete.days", count = count),
    }
    .to_string()
}

/// The chip of `post` while it waits to delete itself.
pub(crate) fn render_countdown_chip(post: &Post, theme: &ThemeTokens) -> Option<AnyElement> {
    let left = countdown_text(Countdown::until(post.lifecycle.delete_at?, Timestamp::now()));
    let chip = div()
        .id(SharedString::from(format!("{CHIP_ID_PREFIX}{}", post.id)))
        .flex_none()
        .flex()
        .items_center()
        .gap(theme.spacing_2xs)
        .text_size(theme.font_size_2xs)
        .text_color(theme.accent_danger)
        .child(Icon::new(LucideIcon::Timer).size(theme.icon_size_xs))
        .child(t!("data.auto_delete.chip", left = left).to_string())
        .tooltip(|window, cx| Tooltip::new(t!("data.auto_delete.chip_tooltip").to_string()).build(window, cx));
    Some(chip.into_any_element())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_unit_shows_its_number() {
        for (left, number) in [(Countdown::Minutes(12), "12"), (Countdown::Hours(5), "5"), (Countdown::Days(3), "3")] {
            assert!(countdown_text(left).contains(number), "{left:?}");
        }
    }
}
