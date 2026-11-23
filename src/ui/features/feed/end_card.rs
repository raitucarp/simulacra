//! N4: the closing card of a finite feed. Once the day's budget is spent the stream ends
//! here: the feed is done for today, and asks what the reader learned.

use crate::i18n::t_count;
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::Icon;
use gpui_kit::*;
use rust_i18n::t;

const END_CARD_ID: &str = "feed-end-card";

/// The card closing a feed whose `budget` posts were all read today.
pub(super) fn render_end_card(budget: u32, theme: &ThemeTokens) -> AnyElement {
    let icon = div().size(theme.icon_size_2xl).text_color(theme.accent_brand).child(Icon::new(LucideIcon::Moon));
    let title = div().font_family(theme.font_family_display.clone()).text_size(theme.font_size_lg).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_primary).child(t!("feed.end.title").to_string());
    let read = div().text_size(theme.font_size_sm).text_color(theme.text_muted).child(t_count("feed.end.read", budget as usize));
    let question = div().text_size(theme.font_size_md).text_color(theme.text_secondary).child(t!("feed.end.question").to_string());
    div()
        .id(END_CARD_ID)
        .w_full()
        .px(theme.panel_padding)
        .py(theme.section_gap)
        .flex()
        .flex_col()
        .items_center()
        .gap(theme.spacing_sm)
        .border_t(theme.border_width_hairline)
        .border_color(theme.border_subtle)
        .bg(theme.bg_callout)
        .child(icon)
        .child(title)
        .child(read)
        .child(question)
        .into_any_element()
}
