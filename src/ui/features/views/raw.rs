//! The raw view (W3): every post as the text it was written in, marks and all, under its
//! author and time, with "Copy all" putting the whole text on the clipboard. A locked post
//! shows (and copies) only its placeholder.

use super::common::{empty_note, locked_note, scroll_column};
use super::input::ViewInput;
use crate::domain::views::raw::{raw_blocks, raw_text, RawBlock, HEADING_SEPARATOR};
use crate::domain::Post;
use crate::i18n::t_count;
use crate::state::AppState;
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::*;
use rust_i18n::t;

/// How a block's time is written (the same in every language).
const TIME_FORMAT: &str = "%Y-%m-%d %H:%M";

/// "author · 2026-11-23 09:30".
fn heading(state: &AppState, post: &Post) -> String {
    let when = crate::time::local(post.created_at).strftime(TIME_FORMAT).to_string();
    format!("{}{HEADING_SEPARATOR}{when}", state.author_of(post).name)
}

/// The raw text of the input's posts, or why there is none.
pub(super) fn render_raw(input: &ViewInput<'_>) -> AnyElement {
    let theme = input.theme;
    let blocks = raw_blocks(input.posts.iter().copied(), |post| heading(input.state, post), &t!("posts.lock.locked"));
    if blocks.is_empty() {
        return empty_note("views.raw.empty", theme);
    }
    let toolbar = toolbar(input, raw_text(&blocks), blocks.len());
    let list = scroll_column(input, "raw").children(blocks.iter().map(|block| render_block(block, theme)));
    div().flex_1().min_h_0().w_full().flex().flex_col().child(toolbar).child(list).into_any_element()
}

fn toolbar(input: &ViewInput<'_>, text: String, count: usize) -> Div {
    let theme = input.theme;
    let copy = Button::new(input.id("raw-copy-all"))
        .ghost()
        .xsmall()
        .icon(Icon::new(LucideIcon::Copy))
        .label(t!("views.raw.copy_all").to_string())
        .on_click(move |_, _, cx| cx.write_to_clipboard(ClipboardItem::new_string(text.clone())));
    let count = div().text_size(theme.font_size_2xs).text_color(theme.text_muted).child(t_count("views.raw.count", count));
    div()
        .flex()
        .items_center()
        .justify_between()
        .gap(theme.control_gap)
        .px(theme.panel_padding)
        .py(theme.spacing_2xs)
        .border_b(theme.border_width_hairline)
        .border_color(theme.border_subtle)
        .child(count)
        .child(copy)
}

fn render_block(block: &RawBlock, theme: &ThemeTokens) -> Div {
    let heading = div().text_size(theme.font_size_2xs).text_color(theme.text_muted).font_family(theme.font_family_mono.clone()).child(block.heading.clone());
    let body = match block.locked {
        true => locked_note(theme),
        false => div()
            .font_family(theme.font_family_mono.clone())
            .text_size(theme.font_size_xs)
            .text_color(theme.text_primary)
            .flex()
            .flex_col()
            .children(block.text.lines().map(|line| div().min_h(theme.font_size_xs * theme.line_height_body).child(line.to_string()))),
    };
    div().flex().flex_col().gap(theme.spacing_3xs).child(heading).child(body)
}
