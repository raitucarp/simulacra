//! An item's faces: a post as it appears in the stream (avatar, name, handle, time, body),
//! a vault passage as a quoted callout with its title and place, or the learner's own text.

use super::super::data::{PostView, ReviewData};
use super::super::text::{passage_place, tr};
use crate::domain::post::SourceLocator;
use crate::domain::review::item::{Anchor, Face};
use crate::domain::Post;
use crate::theme::ThemeTokens;
use crate::ui::atoms::render_profile_avatar;
use crate::ui::features::post::render_post_body;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;

const META_SEPARATOR: &str = "·";

/// Whether a face is asked or shown as the answer (the answer is emphasised).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FaceRole {
    Prompt,
    Answer,
}

/// A post exactly as the stream shows its author row and body.
pub fn render_post_face(view: &PostView, theme: &ThemeTokens) -> Div {
    let (post, author) = (&view.post, &view.author);
    let time = crate::ui::format::date_time(post.created_at);
    let meta = div()
        .flex()
        .flex_wrap()
        .items_center()
        .gap_x(theme.spacing_sm)
        .child(div().font_family(theme.font_family_display.clone()).text_size(theme.font_size_sm).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_primary).child(author.name.clone()))
        .child(div().text_size(theme.font_size_xs).text_color(theme.text_muted).child(author.handle.clone()))
        .child(div().font_family(theme.font_family_mono.clone()).text_size(theme.font_size_xs).text_color(theme.text_muted).child(format!("{META_SEPARATOR} {time}")));
    let linked: &[Post] = &view.linked;
    div()
        .w_full()
        .flex()
        .items_start()
        .gap(theme.spacing_md)
        .child(render_profile_avatar(author, theme.avatar_size_lg, theme))
        .child(div().flex_1().min_w_0().flex().flex_col().gap(theme.spacing_xs).child(meta).child(div().w_full().child(render_post_body(&post.content, linked, theme))))
}

/// A passage as a quoted callout: the text, then the document's title and place.
pub fn render_passage_face(title: &str, locator: Option<&SourceLocator>, text: &str, theme: &ThemeTokens) -> Div {
    let place = locator.map(passage_place).filter(|p| !p.is_empty());
    let caption = std::iter::once(title.to_string()).chain(place).collect::<Vec<_>>().join(&format!(" {META_SEPARATOR} "));
    div()
        .w_full()
        .flex()
        .flex_col()
        .gap(theme.spacing_sm)
        .pl(theme.card_padding)
        .py(theme.spacing_xs)
        .border_l(theme.border_width_threadline)
        .border_color(theme.accent_dialectic)
        .bg(theme.accent_dialectic_subtle)
        .rounded(theme.radius_sm)
        .when(!text.trim().is_empty(), |quote| {
            quote.child(div().text_size(theme.font_size_md).line_height(relative(theme.line_height_reading)).text_color(theme.text_primary).child(text.trim().to_string()))
        })
        .child(div().text_size(theme.font_size_xs).font_weight(FontWeight::SEMIBOLD).text_color(theme.accent_dialectic).child(caption))
}

/// The learner's own text: the prompt large, the answer emphasised.
fn render_written(text: &str, role: FaceRole, theme: &ThemeTokens) -> Div {
    let (color, weight) = match role {
        FaceRole::Prompt => (theme.text_primary, FontWeight::MEDIUM),
        FaceRole::Answer => (theme.accent_success, FontWeight::SEMIBOLD),
    };
    div().w_full().text_size(theme.font_size_lg).line_height(relative(theme.line_height_reading)).font_weight(weight).text_color(color).child(text.to_string())
}

fn render_missing(theme: &ThemeTokens) -> Div {
    div().text_size(theme.font_size_sm).text_color(theme.text_muted).child(tr("review.source.missing"))
}

/// `face` drawn for its `role`.
pub fn render_face(face: &Face, role: FaceRole, data: &ReviewData, theme: &ThemeTokens) -> Div {
    match face {
        Face::Source(Anchor::Post { id }) => data.post(id).map_or_else(|| render_missing(theme), |view| render_post_face(view, theme)),
        Face::Source(Anchor::Passage { document_id, locator, text }) => {
            let title = data.document_title(document_id).cloned().unwrap_or_else(|| tr("review.source.missing"));
            render_passage_face(&title, locator.as_ref(), text, theme)
        }
        Face::Written(text) => render_written(text, role, theme),
        Face::Map => div(),
    }
}
