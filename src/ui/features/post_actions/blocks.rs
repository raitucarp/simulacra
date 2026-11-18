//! An answer's blocks: section headings, prose, bullets, concept relations and scored claims.

use crate::domain::post_actions::{ActionOutput, ConceptRelation, OutputBlock};
use crate::theme::ThemeTokens;
use gpui_kit::*;

/// Percent of a full confidence bar.
const FULL_PERCENT: f32 = 100.0;
const BULLET: &str = "•";
const ARROW: &str = "→";

/// The answer, block by block.
pub(super) fn render_blocks(output: &ActionOutput, theme: &ThemeTokens) -> Div {
    div()
        .flex()
        .flex_col()
        .gap(theme.spacing_xs)
        .font_family(theme.font_family_body.clone())
        .children(output.blocks.iter().map(|block| render_block(block, theme)))
}

fn render_block(block: &OutputBlock, theme: &ThemeTokens) -> AnyElement {
    match block {
        OutputBlock::Heading(text) => div()
            .pt(theme.spacing_xs)
            .text_size(theme.font_size_xs)
            .font_weight(FontWeight::SEMIBOLD)
            .text_color(theme.text_primary)
            .child(text.clone())
            .into_any_element(),
        OutputBlock::Paragraph(text) => prose(theme).child(text.clone()).into_any_element(),
        OutputBlock::Item(text) => bulleted(div().child(text.clone()), theme).into_any_element(),
        OutputBlock::Relation(relation) => bulleted(render_relation(relation, theme), theme).into_any_element(),
        OutputBlock::Score { label, percent } => render_score(label, *percent, theme).into_any_element(),
    }
}

fn prose(theme: &ThemeTokens) -> Div {
    div().text_size(theme.font_size_sm).line_height(relative_line(theme)).text_color(theme.text_primary)
}

fn relative_line(theme: &ThemeTokens) -> DefiniteLength {
    relative(theme.line_height_body)
}

fn bulleted(content: Div, theme: &ThemeTokens) -> Div {
    prose(theme)
        .flex()
        .items_start()
        .gap(theme.inline_gap)
        .child(div().flex_none().text_color(theme.accent_brand).child(BULLET))
        .child(content.flex_1().min_w_0())
}

/// "A → relation → B" with the concepts emphasised.
fn render_relation(relation: &ConceptRelation, theme: &ThemeTokens) -> Div {
    let concept = |text: &str| div().font_weight(FontWeight::SEMIBOLD).child(text.to_string());
    let link = |text: &str| div().text_color(theme.text_muted).child(text.to_string());
    div()
        .flex()
        .flex_wrap()
        .gap(theme.inline_gap)
        .child(concept(&relation.from))
        .child(link(ARROW))
        .child(link(&relation.relation))
        .child(link(ARROW))
        .child(concept(&relation.to))
}

/// A claim with its confidence bar and percentage.
fn render_score(label: &str, percent: u8, theme: &ThemeTokens) -> Div {
    let width = theme.post_actions_score_bar_width;
    let filled = width * (f32::from(percent) / FULL_PERCENT);
    let bar = div()
        .flex_none()
        .w(width)
        .h(theme.similarity_bar_height)
        .rounded(theme.radius_full)
        .bg(theme.hover_bg)
        .child(div().h_full().w(filled).rounded(theme.radius_full).bg(theme.accent_dialectic));
    let readout = div().flex_none().w(theme.similarity_readout_width).font_family(theme.font_family_mono.clone()).text_size(theme.font_size_2xs).text_color(theme.text_secondary).child(format!("{percent}%"));
    prose(theme).flex().items_center().gap(theme.inline_gap).child(bar).child(readout).child(div().flex_1().min_w_0().child(label.to_string()))
}
