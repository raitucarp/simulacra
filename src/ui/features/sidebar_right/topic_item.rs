//! Trending topic item row molecule.

use crate::domain::TrendingTopic;
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::base::StyledExt;
use gpui_kit::component::Icon;
use gpui_kit::*;
use crate::i18n::t_count;

const HASH_PREFIX: char = '#';

fn render_tag_name(topic: &TrendingTopic, theme: &ThemeTokens) -> Div {
    div()
        .flex()
        .items_center()
        .gap(theme.spacing_xs)
        .child(div().w(theme.icon_size_sm).h(theme.icon_size_sm).text_color(theme.accent_brand).child(Icon::new(LucideIcon::Hash)))
        .child(
            div()
                .text_size(theme.font_size_label)
                .font_bold()
                .text_color(theme.text_primary)
                .child(topic.tag.trim_start_matches(HASH_PREFIX).to_string()),
        )
}

fn render_rank_and_tag(topic: &TrendingTopic, rank: usize, theme: &ThemeTokens) -> Div {
    div()
        .flex()
        .items_center()
        .gap(theme.spacing_sm_plus)
        .child(div().text_size(theme.font_size_label).font_bold().text_color(theme.text_muted).child(format!("{rank:02}")))
        .child(
            div()
                .flex()
                .flex_col()
                .child(render_tag_name(topic, theme))
                .child(div().text_size(theme.font_size_label).text_color(theme.text_muted).child(topic.category.clone())),
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
        .p(theme.spacing_sm)
        .rounded(theme.radius_sm)
        .hover(move |s| s.bg(hover_bg))
        .cursor_pointer()
        .on_click(move |_, window, cx| {
            on_click(&tag, window, cx);
        })
        .child(render_rank_and_tag(topic, rank, theme))
        .child(
            div()
                .text_size(theme.font_size_label)
                .text_color(theme.text_secondary)
                .child(t_count("sidebar.posts_count", topic.post_count)),
        )
}
