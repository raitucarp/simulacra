//! "Cite this": the entry in each citation style, titles in italics, each with a copy button.

use super::super::common::{card, titled_section};
use crate::domain::document::citation::{cite, Citation, CitationSource, CitationStyle};
use crate::domain::GroundingDocument;
use crate::ui::features::vault::panel::VaultPanel;
use crate::ui::features::vault::text::style_label;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::*;
use rust_i18n::t;
use std::ops::Range;

/// Reference number shown in the numeric style of a single entry.
const SINGLE_REFERENCE: usize = 1;

/// The citation's text with the byte ranges of its emphasised parts.
fn styled(citation: &Citation) -> (String, Vec<Range<usize>>) {
    citation.parts.iter().fold((String::new(), Vec::new()), |(mut text, mut ranges), part| {
        let start = text.len();
        text.push_str(&part.text);
        if part.emphasis {
            ranges.push(start..text.len());
        }
        (text, ranges)
    })
}

impl VaultPanel {
    pub(super) fn render_citations(&self, doc: &GroundingDocument, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &self.theme;
        let source = CitationSource { title: &doc.title, record: &doc.record };
        let rows: Vec<AnyElement> = CitationStyle::ALL
            .iter()
            .map(|style| self.render_citation_row(*style, &cite(source, *style, SINGLE_REFERENCE), cx))
            .collect();
        div()
            .flex()
            .flex_col()
            .gap(theme.stack_gap)
            .child(titled_section(LucideIcon::Quote, t!("vault.cite").to_string(), theme))
            .child(card(theme).children(rows))
    }

    fn render_citation_row(&self, style: CitationStyle, citation: &Citation, cx: &mut Context<Self>) -> AnyElement {
        let theme = &self.theme;
        let (text, italic) = styled(citation);
        let copied = self.copied_style == Some(style);
        let highlight = HighlightStyle { font_style: Some(FontStyle::Italic), ..Default::default() };
        let copy_text = citation.text();
        let button = Button::new(ElementId::Name(format!("vault-copy-{}", style.key()).into()))
            .ghost()
            .xsmall()
            .icon(Icon::new(if copied { LucideIcon::Check } else { LucideIcon::Copy }))
            .label(if copied { t!("vault.copied") } else { t!("vault.copy") }.to_string())
            .on_click(cx.listener(move |this, _, _, cx| this.copy_citation(style, copy_text.clone(), cx)));
        div()
            .flex()
            .items_start()
            .gap(theme.control_gap)
            .child(
                div()
                    .flex_1()
                    .min_w_0()
                    .flex()
                    .flex_col()
                    .gap(theme.spacing_2xs)
                    .child(div().text_size(theme.font_size_2xs).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_muted).child(style_label(style)))
                    .child(
                        div()
                            .font_family(theme.font_family_body.clone())
                            .text_size(theme.font_size_sm)
                            .text_color(theme.text_primary)
                            .line_height(relative(theme.line_height_body))
                            .child(StyledText::new(text).with_highlights(italic.into_iter().map(|range| (range, highlight)))),
                    ),
            )
            .child(button)
            .into_any_element()
    }
}
