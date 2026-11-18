//! What a point of a map says: a post's author and its opening words, a passage's document
//! and words, or a concept's name, colour-coded like the Mind Graph's nodes.

use super::super::super::data::PostView;
use super::super::super::text::tr;
use crate::domain::review::connections::MapPoint;
use crate::domain::review::quiz::piece_text;
use crate::theme::ThemeTokens;
use gpui_kit::*;

/// Characters of a post or passage shown on its chip.
const CHIP_CHARS: usize = 90;

/// The kind of a point (its colour).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChipKind {
    Post,
    Passage,
    Concept,
}

/// One point as its chip shows it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PointChip {
    pub kind: ChipKind,
    pub title: String,
    pub body: String,
}

/// The chip of `point`; `post` and `title` look up posts and document titles.
pub fn point_chip(point: &MapPoint, post: &dyn Fn(&str) -> Option<PostView>, title: &dyn Fn(&str) -> Option<String>) -> PointChip {
    match point {
        MapPoint::Post { id } => match post(id) {
            Some(view) => PointChip { kind: ChipKind::Post, title: view.author.name.clone(), body: piece_text(&view.post.content, CHIP_CHARS) },
            None => PointChip { kind: ChipKind::Post, title: tr("review.source.missing"), body: String::new() },
        },
        MapPoint::Passage { document_id, text, .. } => PointChip {
            kind: ChipKind::Passage,
            title: title(document_id).unwrap_or_else(|| tr("review.source.missing")),
            body: if text.trim().is_empty() { tr("review.map.whole_document") } else { piece_text(text, CHIP_CHARS) },
        },
        MapPoint::Concept { label } => PointChip { kind: ChipKind::Concept, title: label.clone(), body: tr("review.map.concept") },
    }
}

fn kind_color(kind: ChipKind, theme: &ThemeTokens) -> Rgba {
    match kind {
        ChipKind::Post => theme.accent_agent,
        ChipKind::Passage => theme.accent_user,
        ChipKind::Concept => theme.accent_alt,
    }
}

/// The chip box (positioned by the canvas); `pending` marks the first point of a pair.
pub fn render_chip(chip: &PointChip, id: ElementId, pending: bool, theme: &ThemeTokens) -> Stateful<Div> {
    let color = kind_color(chip.kind, theme);
    let (border, background) = if pending { (theme.accent_brand, theme.accent_brand_subtle) } else { (theme.border, theme.bg_card) };
    div()
        .id(id)
        .w(theme.review_map_point_width)
        .h(theme.review_map_point_height)
        .flex()
        .flex_col()
        .gap(theme.spacing_3xs)
        .px(theme.spacing_sm)
        .py(theme.spacing_xs)
        .rounded(theme.radius_md)
        .bg(background)
        .border(if pending { theme.border_width_threadline } else { theme.border_width_hairline })
        .border_color(border)
        .hover(|chip| chip.border_color(theme.accent_brand_border))
        .overflow_hidden()
        .child(
            div()
                .flex()
                .items_center()
                .gap(theme.spacing_xs)
                .child(div().flex_none().size(theme.review_swatch_size).rounded(theme.radius_full).bg(color))
                .child(div().min_w_0().truncate().text_size(theme.font_size_xs).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_primary).child(chip.title.clone())),
        )
        .child(div().text_size(theme.font_size_2xs).line_height(relative(theme.line_height_body)).text_color(theme.text_secondary).line_clamp(2).child(chip.body.clone()))
}
