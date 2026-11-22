//! Free compose in the composer (T2): one editor where each line (or `-` / `*` bullet) is a
//! post and indentation nests a reply, a live count of the posts it makes, and a preview of
//! the split: every post in thread order, indented under the post it replies to, with its
//! length against the per-post limit.

use crate::domain::compose::ComposeLine;
use crate::domain::MAX_POST_CHARS;
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use crate::ui::features::symbols::render_symbol_face;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::{Icon, Selectable, Sizable};
use gpui_kit::{div, App, Div, ParentElement, SharedString, Styled, Window};
use rust_i18n::t;
use std::rc::Rc;

const TOGGLE_ID: &str = "composer-free-compose";
const SPLIT_ID: &str = "composer-free-split";
const FACE_ID: &str = "composer-free-face";

/// Switches the composer in or out of free compose.
pub type FreeToggle = Rc<dyn Fn(&mut Window, &mut App) + 'static>;
/// Splits the free compose line at this raw line number (T3).
pub type SplitLine = Rc<dyn Fn(usize, &mut Window, &mut App) + 'static>;

/// The split being previewed.
#[derive(Clone)]
pub struct FreeComposeView {
    pub lines: Vec<ComposeLine>,
    /// Splits the raw line given over the limit into several (T3).
    pub on_split: SplitLine,
}

impl FreeComposeView {
    /// The raw lines of the posts over the per-post limit (T3).
    pub fn over_limit(&self) -> Vec<usize> {
        self.lines.iter().filter(|line| line.is_over(MAX_POST_CHARS)).map(|line| line.line).collect()
    }

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

/// "Split this line" on a line over the limit (T3): cut into posts that fit.
fn split_button(line: usize, on_split: SplitLine) -> Button {
    Button::new(SharedString::from(format!("{SPLIT_ID}-{line}")))
        .ghost()
        .xsmall()
        .icon(Icon::new(LucideIcon::Sparkles))
        .tooltip(t!("composer.char_warning").to_string())
        .label(t!("composer.split_line").to_string())
        .on_click(move |_, window, cx| on_split(line, window, cx))
}

fn line_row(line: &ComposeLine, on_split: &SplitLine, theme: &ThemeTokens) -> Div {
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
        .children(line.symbols.face().map(|face| div().pl(theme.spacing_xs).child(render_symbol_face(&format!("{FACE_ID}-{}", line.line), &face, theme.icon_size_sm, theme)))) // T5: the line's own symbols
        .child(text)
        .child(count)
        .children(over.then(|| split_button(line.line, on_split.clone()))) // T3: the line is flagged with its fix
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
        .children(view.lines.iter().map(|line| line_row(line, &view.on_split, theme)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::compose::compose_lines;

    #[test]
    fn the_preview_counts_the_posts_the_text_makes() {
        let view = FreeComposeView { lines: compose_lines("One\n  - a reply\nTwo"), on_split: Rc::new(|_, _, _| {}) };
        assert_eq!(view.count_text(), crate::i18n::t_count("composer.free_posts", 3));
        assert_eq!(view.lines.iter().map(|l| l.depth).collect::<Vec<_>>(), vec![0, 1, 0]);
        let _preview = free_preview(&view, &ThemeTokens::default());
        for key in ["composer.free_compose", "composer.free_compose_hint", "composer.split_line"] {
            assert_ne!(t!(key, locale = "en"), key);
        }
    }

    #[test]
    fn lines_over_the_limit_are_flagged_by_their_raw_line() {
        let long = "word ".repeat(crate::domain::MAX_POST_CHARS / 4);
        let view = FreeComposeView { lines: compose_lines(&format!("short\n\n  - {long}")), on_split: Rc::new(|_, _, _| {}) };
        assert_eq!(view.over_limit(), vec![2]);
        let _preview = free_preview(&view, &ThemeTokens::default());
    }
}
