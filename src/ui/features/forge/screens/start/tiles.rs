//! The "other ways to begin" tiles: a blank sheet, the dice and describing them in words.

use crate::theme::ThemeTokens;
use crate::ui::features::forge::text::tr;
use crate::ui::features::forge::view::{ForgeHandle, ForgeView};
use crate::ui::features::forge::widgets::{ActionCallback, KeyboardStop};
use crate::ui::icons::LucideIcon;
use gpui_kit::component::Icon;
use gpui_kit::*;

const SCRATCH_ID: &str = "forge-tile-scratch";
const ROLL_ID: &str = "forge-tile-roll";
const DESCRIBE_ID: &str = "forge-tile-describe";
const SCRATCH_TITLE: &str = "forge.ui.start.scratch_title";
const SCRATCH_BODY: &str = "forge.ui.start.scratch_body";
const ROLL_TITLE: &str = "forge.ui.start.roll_title";
const ROLL_BODY: &str = "forge.ui.start.roll_body";
const DESCRIBE_TITLE: &str = "forge.ui.start.describe_title";
const DESCRIBE_BODY: &str = "forge.ui.start.describe_body";

impl ForgeView {
    pub(super) fn begin_tiles(&self, handle: &ForgeHandle) -> Div {
        let theme = &self.theme;
        div()
            .flex()
            .gap(theme.stack_gap)
            .child(tile(SCRATCH_ID, LucideIcon::Plus, SCRATCH_TITLE, SCRATCH_BODY, theme, handle.act(ForgeView::begin_blank)))
            .child(tile(ROLL_ID, LucideIcon::Dices, ROLL_TITLE, ROLL_BODY, theme, handle.act(ForgeView::roll)))
            .child(tile(DESCRIBE_ID, LucideIcon::Sparkles, DESCRIBE_TITLE, DESCRIBE_BODY, theme, handle.act(ForgeView::toggle_describe)))
    }
}

/// One "way to begin" tile: icon, title and a line of explanation.
fn tile(id: &'static str, icon: LucideIcon, title_key: &str, body_key: &str, theme: &ThemeTokens, on_click: ActionCallback) -> Stateful<Div> {
    let hover_bg = theme.bg_elevated;
    let click = on_click.clone();
    div()
        .id(id)
        .flex_1()
        .min_h(theme.forge_tile_min_height)
        .p(theme.card_padding)
        .flex()
        .flex_col()
        .gap(theme.spacing_xs)
        .rounded(theme.radius_xl)
        .border(theme.border_width_hairline)
        .border_color(theme.border_subtle)
        .bg(theme.bg_card)
        .cursor_pointer()
        .hover(move |style| style.bg(hover_bg))
        .on_click(move |_, window, cx| click(window, cx))
        .keyboard_stop(theme, on_click)
        .child(div().text_color(theme.accent_brand_text).child(Icon::new(icon).size(theme.font_size_xl)))
        .child(div().text_size(theme.font_size_md).font_weight(FontWeight::SEMIBOLD).child(tr(title_key)))
        .child(div().text_size(theme.font_size_xs).text_color(theme.text_muted).child(tr(body_key)))
}
