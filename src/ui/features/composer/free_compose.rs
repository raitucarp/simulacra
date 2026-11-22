//! Free compose in the composer (T2): one editor where each line (or `-` / `*` bullet) is a
//! post and indentation nests a reply, a live count of the posts it makes, and a preview of
//! the split: every post in thread order, indented under the post it replies to, with its
//! length against the per-post limit.

use crate::domain::compose::ComposeLine;
use crate::domain::MAX_POST_CHARS;
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::{Icon, Selectable, Sizable};
use gpui_kit::{div, App, Div, ParentElement, Styled, Window};
use rust_i18n::t;
use std::rc::Rc;

const TOGGLE_ID: &str = "composer-free-compose";

/// Switches the composer in or out of free compose.
pub type FreeToggle = Rc<dyn Fn(&mut Window, &mut App) + 'static>;

/// The split being previewed.
#[derive(Clone, Debug, PartialEq)]
pub struct FreeComposeView {
    pub lines: Vec<ComposeLine>,
}

impl FreeComposeView {
    /// "3 posts": how many posts the text makes.
    pub fn count_text(&self) -> String {
        crate::i18n::t_count("composer.free_posts", self.lines.len())
    }
}

/// The footer toggle, lit while composing freely.
pub(super) fn free_button(on: bool, on_toggle: FreeToggle) -> Button {
    Button::new(TOGGLE_ID)
        .ghost()
        .small()
        .selected(on)
        .icon(Icon::new(LucideIcon::ListTree))
        .tooltip(t!("composer.free_compose_hint").to_string())
        .label(t!("composer.free_compose").to_string())
        .on_click(move |_, window, cx| on_toggle(window, cx))
}

fn line_row(line: &ComposeLine, theme: &ThemeTokens) -> Div {
    let over = line.is_over(MAX_POST_CHARS);
    let count_colour = if over { theme.accent_danger } else { theme.text_muted };
    let count = div().flex_none().font_family(theme.font_family_mono.clone()).text_size(theme.font_size_2xs).text_color(count_colour).child(t!("composer.char_counter", count = line.chars, max = MAX_POST_CHARS).to_string());
    let text = div().flex_1().min_w_0().pl(theme.spacing_xs).truncate().text_size(theme.font_size_xs).text_color(theme.text_primary).child(line.text.clone());
    div()
        .flex()
        .items_center()
        .gap(theme.inline_gap)
        .ml(theme.compose_preview_indent * line.depth as f32)
        .py(theme.spacing_3xs)
        .border_l(theme.border_width_threadline)
        .border_color(if over { theme.accent_danger } else { theme.border_subtle })
        .child(text)
        .child(count)
}

/// The preview under the editor: the count, then every post of the split.
pub(super) fn free_preview(view: &FreeComposeView, theme: &ThemeTokens) -> Div {
    let heading = div().text_size(theme.font_size_2xs).font_weight(gpui_kit::FontWeight::SEMIBOLD).text_color(theme.text_muted).child(view.count_text());
    let hint = div().text_size(theme.font_size_2xs).text_color(theme.text_muted).child(t!("composer.free_compose_hint").to_string());
    div()
        .w_full()
        .flex()
        .flex_col()
        .gap(theme.spacing_2xs)
        .px(theme.panel_padding)
        .py(theme.spacing_xs)
        .child(div().flex().items_center().justify_between().gap(theme.control_gap).child(heading).child(hint))
        .children(view.lines.iter().map(|line| line_row(line, theme)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::compose::compose_lines;

    #[test]
    fn the_preview_counts_the_posts_the_text_makes() {
        let view = FreeComposeView { lines: compose_lines("One\n  - a reply\nTwo") };
        assert_eq!(view.count_text(), crate::i18n::t_count("composer.free_posts", 3));
        assert_eq!(view.lines.iter().map(|l| l.depth).collect::<Vec<_>>(), vec![0, 1, 0]);
        let _preview = free_preview(&view, &ThemeTokens::default());
        for key in ["composer.free_compose", "composer.free_compose_hint"] {
            assert_ne!(t!(key, locale = "en"), key);
        }
    }
}
