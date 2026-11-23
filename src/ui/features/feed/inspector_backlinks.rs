//! N1: "Referred to by" under the focused post: every thought that embeds, quotes or links to
//! it, newest first, each with its author and opening words; a click focuses its thread. A
//! locked thought shows only its placeholder excerpt.

use super::inspector::InspectorContext;
use crate::domain::post::refs::Backlink;
use crate::domain::Post;
use crate::i18n::t_count;
use gpui_kit::*;
use rust_i18n::t;

const ROW_ID: &str = "inspector-backlink-";

/// The section, or nothing when no thought refers to `post`.
pub(super) fn render_backlinks(ctx: &InspectorContext<'_>, post: &Post) -> Option<Div> {
    let links = ctx.state.backlinks(&post.id);
    if links.is_empty() {
        return None;
    }
    let theme = ctx.theme;
    let heading = div().text_size(theme.font_size_xs).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_primary).child(t_count("feed.backlinks.title", links.len()));
    let section = div()
        .mx(theme.panel_padding)
        .my(theme.spacing_sm)
        .p(theme.spacing_sm)
        .rounded(theme.radius_md)
        .border(theme.border_width_hairline)
        .border_color(theme.border_subtle)
        .flex()
        .flex_col()
        .gap(theme.spacing_3xs)
        .child(heading)
        .children(links.iter().map(|link| row(ctx, link)));
    Some(section)
}

fn row(ctx: &InspectorContext<'_>, link: &Backlink) -> Stateful<Div> {
    let theme = ctx.theme;
    let author = ctx.state.get_post(&link.from).map(|from| ctx.state.author_of(from).name.clone()).unwrap_or_default();
    let label = t!(&format!("post.refs_by.{}", link.kind.key()), excerpt = ctx.state.resolve_wikilink_fast(link.from.as_str())).to_string();
    let (on_select, id, hover) = (ctx.callbacks.on_select.clone(), link.from.clone(), theme.hover_bg);
    div()
        .id(SharedString::from(format!("{ROW_ID}{}", link.from)))
        .px(theme.spacing_xs)
        .py(theme.spacing_3xs)
        .rounded(theme.radius_sm)
        .cursor_pointer()
        .hover(move |row| row.bg(hover))
        .on_click(move |_, window, cx| on_select(id.clone(), window, cx))
        .flex()
        .items_center()
        .gap(theme.inline_gap)
        .text_size(theme.font_size_xs)
        .child(div().font_weight(FontWeight::MEDIUM).text_color(theme.text_secondary).child(author))
        .child(div().min_w_0().truncate().text_color(theme.text_muted).child(label))
}
