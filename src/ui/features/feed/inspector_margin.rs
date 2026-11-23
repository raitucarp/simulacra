//! N2: "In the margin" under the focused post: the replies answering one of its passages, in
//! the order of those passages, each with the words it answers and its own opening words; a
//! click focuses the reply. A locked post's notes show only the placeholder.

use super::inspector::InspectorContext;
use crate::domain::post::anchor::AnchorPlace;
use crate::domain::Post;
use crate::i18n::t_count;
use crate::state::store::MarginNote;
use gpui_kit::*;
use rust_i18n::t;

const NOTE_ID: &str = "inspector-margin-";

/// The section, or nothing when no reply answers a passage of `post`.
pub(super) fn render_margin_notes(ctx: &InspectorContext<'_>, post: &Post) -> Option<Div> {
    let notes = ctx.state.margin_notes(&post.id);
    if notes.is_empty() {
        return None;
    }
    let theme = ctx.theme;
    let heading = div().text_size(theme.font_size_xs).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_primary).child(t_count("feed.margin.title", notes.len()));
    let section = div()
        .mx(theme.panel_padding)
        .my(theme.spacing_sm)
        .flex()
        .flex_col()
        .gap(theme.spacing_xs)
        .child(heading)
        .children(notes.iter().map(|note| render_note(ctx, note)));
    Some(section)
}

fn render_note(ctx: &InspectorContext<'_>, note: &MarginNote) -> Stateful<Div> {
    let theme = ctx.theme;
    let reply = ctx.state.get_post(&note.reply);
    let author = reply.map(|reply| ctx.state.author_of(reply).name.clone()).unwrap_or_default();
    let excerpt = ctx.state.resolve_wikilink_fast(note.reply.as_str());
    let quote_colour = if note.place == AnchorPlace::Gone { theme.border_subtle } else { theme.accent_dialectic };
    let quote = div().pl(theme.spacing_xs).border_l(theme.border_width_threadline).border_color(quote_colour).italic().text_size(theme.font_size_xs).text_color(theme.text_secondary).child(t!("post.margin.quote", quote = note.quote.clone()).to_string());
    let changed = (note.place == AnchorPlace::Gone).then(|| div().text_size(theme.font_size_2xs).text_color(theme.text_muted).child(t!("post.margin.changed").to_string()));
    let answer = div().flex().gap(theme.inline_gap).text_size(theme.font_size_xs).child(div().font_weight(FontWeight::MEDIUM).text_color(theme.text_primary).child(author)).child(div().min_w_0().truncate().text_color(theme.text_muted).child(excerpt));
    let (on_select, id, hover) = (ctx.callbacks.on_select.clone(), note.reply.clone(), theme.hover_bg);
    div()
        .id(SharedString::from(format!("{NOTE_ID}{}", note.reply)))
        .p(theme.spacing_xs)
        .rounded(theme.radius_sm)
        .bg(theme.bg_callout)
        .cursor_pointer()
        .hover(move |row| row.bg(hover))
        .on_click(move |_, window, cx| on_select(id.clone(), window, cx))
        .flex()
        .flex_col()
        .gap(theme.spacing_3xs)
        .child(quote)
        .children(changed)
        .child(answer)
}
