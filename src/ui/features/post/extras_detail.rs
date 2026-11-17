//! The inspector's "Behind this post" card: everything the engine recorded on the focused
//! post, in full: mood, the image prompt, each tool call with its arguments (failures marked)
//! and each source with where it lives (a web source shows its URL).

use super::extras::{mood_chip, source_chip, tool_icon, tool_name, OpenSourceAction};
use crate::domain::post::{PostExtras, SourceKind, SourceRef, ToolUse};
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::Icon;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::{div, Div, FontWeight, IntoElement, ParentElement, Styled};
use rust_i18n::t;

/// A captioned block of the detail card.
fn block(caption: String, body: impl IntoElement, theme: &ThemeTokens) -> Div {
    let caption = div().text_size(theme.font_size_2xs).font_weight(FontWeight::BOLD).text_color(theme.text_muted).child(caption);
    div().flex().flex_col().gap(theme.spacing_xs).child(caption).child(body)
}

/// One tool call: icon, name, its arguments and whether it failed.
fn tool_line(tool: &ToolUse, theme: &ThemeTokens) -> Div {
    let icon = if tool.failed { LucideIcon::CircleAlert } else { tool_icon(&tool.name) };
    let color = if tool.failed { theme.accent_danger } else { theme.accent_brand };
    let status = tool.failed.then(|| div().text_color(theme.accent_danger).child(t!("post.extras_failed").to_string()));
    div()
        .flex()
        .items_start()
        .gap(theme.spacing_xs)
        .text_size(theme.font_size_xs)
        .when(tool.failed, |line| line.opacity(theme.post_failed_tool_opacity))
        .child(div().text_color(color).child(Icon::new(icon).size(theme.icon_size_sm)))
        .child(
            div()
                .flex_1()
                .min_w_0()
                .flex()
                .flex_col()
                .child(div().flex().gap(theme.spacing_xs).child(div().font_weight(FontWeight::SEMIBOLD).text_color(theme.text_primary).child(tool_name(&tool.name))).children(status))
                .child(div().text_color(theme.text_secondary).child(tool.summary.clone())),
        )
}

/// One source chip, with its URL underneath for web pages.
fn source_line(source: &SourceRef, theme: &ThemeTokens, on_open: Option<OpenSourceAction>) -> Div {
    let url = (source.kind == SourceKind::Web).then(|| {
        div().font_family(theme.font_family_mono.clone()).text_size(theme.font_size_2xs).text_color(theme.text_muted).child(source.id.clone())
    });
    div().flex().flex_col().items_start().gap(theme.spacing_3xs).child(source_chip(source, theme, on_open)).children(url)
}

/// The detail card of `extras`, or `None` when nothing is recorded.
pub(crate) fn render_extras_detail(extras: &PostExtras, theme: &ThemeTokens, on_open: Option<&OpenSourceAction>) -> Option<Div> {
    if extras.is_empty() {
        return None;
    }
    let mood = extras.mood.as_deref().map(|mood| block(t!("post.extras_mood").to_string(), div().flex().child(mood_chip(mood, theme)), theme));
    let prompt = extras.image_prompt.clone().map(|prompt| {
        let line = div().flex().items_start().gap(theme.spacing_xs).text_size(theme.font_size_xs).text_color(theme.text_secondary).child(Icon::new(LucideIcon::Image).size(theme.icon_size_sm)).child(div().flex_1().min_w_0().child(prompt));
        block(t!("post.extras_image_prompt").to_string(), line, theme)
    });
    let tools = (!extras.tools.is_empty()).then(|| {
        let lines = extras.tools.iter().map(|tool| tool_line(tool, theme));
        block(t!("post.extras_tools_detail").to_string(), div().flex().flex_col().gap(theme.spacing_xs).children(lines), theme)
    });
    let sources = (!extras.sources.is_empty()).then(|| {
        let lines = extras.sources.iter().map(|source| source_line(source, theme, on_open.cloned()));
        block(t!("post.extras_sources_detail").to_string(), div().flex().flex_col().gap(theme.spacing_xs).children(lines), theme)
    });
    let title = div().flex().items_center().gap(theme.spacing_xs).text_size(theme.font_size_sm).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_primary).child(Icon::new(LucideIcon::Cpu).size(theme.icon_size_sm)).child(t!("post.extras_title").to_string());
    Some(
        div()
            .m(theme.spacing_md)
            .p(theme.card_padding)
            .flex()
            .flex_col()
            .gap(theme.spacing_md)
            .rounded(theme.radius_lg)
            .bg(theme.bg_card)
            .border(theme.border_width_hairline)
            .border_color(theme.border_subtle)
            .child(title)
            .children(mood)
            .children(prompt)
            .children(tools)
            .children(sources),
    )
}
