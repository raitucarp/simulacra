//! Composer preview: a draft rendered by the feed's own body renderer (marks, spoilers,
//! code blocks, links), and the button that switches between editing and previewing.

use crate::domain::Post;
use crate::theme::ThemeTokens;
use crate::ui::features::post::render_post_body;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::{Icon, Selectable, Sizable};
use gpui_kit::{div, AnyElement, App, IntoElement, ParentElement, SharedString, Styled, Window};
use rust_i18n::t;
use std::rc::Rc;

/// Element id prefix of the preview toggle (suffixed with the composer's scope).
const PREVIEW_BUTTON_ID: &str = "composer-preview";

/// Switches a composer between editing and previewing.
pub type PreviewCallback = Rc<dyn Fn(&mut Window, &mut App) + 'static>;

/// One draft as it will read once posted (wikilinks show their raw target in a preview).
pub(super) fn render_preview(text: &str, theme: &ThemeTokens) -> AnyElement {
    if text.trim().is_empty() {
        return div().text_size(theme.font_size_sm).text_color(theme.text_muted).child(t!("composer.preview_empty").to_string()).into_any_element();
    }
    let no_posts: &[Post] = &[];
    div().w_full().min_w_0().py(theme.spacing_2xs).child(render_post_body(text.trim(), no_posts, theme)).into_any_element()
}

/// The preview toggle: an eye while editing, a pencil while previewing.
pub(super) fn preview_button(scope: &'static str, previewing: bool, on_toggle: PreviewCallback) -> Button {
    let (icon, tooltip) = if previewing { (LucideIcon::Pencil, t!("composer.preview_hide")) } else { (LucideIcon::Eye, t!("composer.preview_show")) };
    Button::new(SharedString::from(format!("{PREVIEW_BUTTON_ID}-{scope}")))
        .ghost()
        .small()
        .selected(previewing)
        .icon(Icon::new(icon))
        .tooltip(tooltip.to_string())
        .on_click(move |_, window, cx| on_toggle(window, cx))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preview_renders_marked_and_empty_drafts_and_its_labels_are_translated() {
        let theme = ThemeTokens::default();
        let _marked = render_preview("**bold** ||secret||\n```rust\nfn x() {}\n```", &theme);
        let _empty = render_preview("   ", &theme);
        for key in ["composer.preview_empty", "composer.preview_show", "composer.preview_hide"] {
            assert_ne!(rust_i18n::t!(key, locale = "en"), key);
        }
        let toggle: PreviewCallback = Rc::new(|_, _| {});
        let _button = preview_button("test", true, toggle);
    }
}
