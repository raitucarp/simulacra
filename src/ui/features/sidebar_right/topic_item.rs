//! Trending topic item row molecule.

use crate::domain::TrendingTopic;
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::base::StyledExt;
use gpui_kit::component::Icon;
use gpui_kit::*;
use rust_i18n::t;

const HASH_PREFIX: char = '#';

fn render_tag_name(topic: &TrendingTopic, theme: &ThemeTokens) -> Div {
    div()
        .flex()
        .items_center()
        .gap_1()
        .child(div().w_3().h_3().text_color(theme.accent_brand).child(Icon::new(LucideIcon::Hash)))
        .child(
            div()
                .text_xs()
                .font_bold()
                .text_color(theme.text_primary)
                .child(topic.tag.trim_start_matches(HASH_PREFIX).to_string()),
        )
}

fn render_rank_and_tag(topic: &TrendingTopic, rank: usize, theme: &ThemeTokens) -> Div {
    div()
        .flex()
        .items_center()
        .gap_2p5()
        .child(div().text_xs().font_bold().text_color(theme.text_muted).child(format!("{rank:02}")))
        .child(
            div()
                .flex()
                .flex_col()
                .child(render_tag_name(topic, theme))
                .child(div().text_xs().text_color(theme.text_muted).child(topic.category.clone())),
        )
}

pub fn render_topic_item(
    topic: &TrendingTopic,
    rank: usize,
    theme: &ThemeTokens,
    on_click: impl Fn(&str, &mut Window, &mut App) + 'static,
) -> impl IntoElement {
    let hover_bg = theme.hover_bg;
    let tag = topic.tag.clone();

    div()
        .id(format!("trending-topic-{}", topic.tag))
        .w_full()
        .flex()
        .items_center()
        .justify_between()
        .p_2()
        .rounded_md()
        .hover(move |s| s.bg(hover_bg))
        .cursor_pointer()
        .on_click(move |_, window, cx| {
            on_click(&tag, window, cx);
        })
        .child(render_rank_and_tag(topic, rank, theme))
        .child(
            div()
                .text_xs()
                .text_color(theme.text_secondary)
                .child(t!("sidebar.posts_count", count = topic.post_count).to_string()),
        )
}
