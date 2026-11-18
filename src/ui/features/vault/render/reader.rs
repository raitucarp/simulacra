//! The reader: the text in a comfortable measure and leading, headings with breathing room,
//! quotations as callouts, and progress saved as the reader scrolls.

use super::common::{entrance, meta_line, progress_bar};
use crate::domain::document::record::{ReadingStatus, PROGRESS_COMPLETE};
use crate::domain::document::text::{blocks, BlockKind, TextBlock};
use crate::domain::preferences::ReadingWidth;
use crate::ui::features::vault::panel::VaultPanel;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::*;
use rust_i18n::t;

const READER_SCROLL_ID: &str = "vault-reader";
const READER_BACK_ID: &str = "vault-reader-back";
const READER_DONE_ID: &str = "vault-reader-done";
/// Children of the scrolled column before the first text block (the title block).
const LEADING_CHILDREN: usize = 1;

impl VaultPanel {
    pub(super) fn render_reader(&mut self, id: &str, cx: &mut Context<Self>) -> AnyElement {
        let Some(doc) = self.data.document(id).cloned() else { return div().into_any_element() };
        if let Some(block) = self.pending_block.take() {
            self.reader_scroll.scroll_to_item(block + LEADING_CHILDREN);
        }
        let theme = &self.theme;
        let progress = self.reader_progress.max(doc.record.reading.progress);
        let top = self.render_reader_bar(&doc, progress, cx);
        let scroller = div()
            .id(READER_SCROLL_ID)
            .flex_1()
            .min_h_0()
            .w_full()
            .overflow_y_scroll()
            .track_scroll(&self.reader_scroll)
            .on_scroll(cx.listener(|this, _, _, cx| this.track_reading(cx)))
            .flex()
            .flex_col()
            .items_center()
            .px(theme.panel_padding)
            .py(theme.section_gap)
            .gap(theme.paragraph_break_height)
            .children(self.render_reader_column(&doc));
        let page = div()
            .size_full()
            .flex()
            .flex_col()
            .child(top)
            .child(progress_bar(progress.min(PROGRESS_COMPLETE), theme))
            .child(scroller);
        entrance(page, ElementId::Name(format!("vault-reader-{id}").into()), self.data.reduced_motion, &self.theme)
    }

    /// The line length the reading-width preference asks for.
    fn measure(&self) -> Pixels {
        match self.data.reading_width {
            ReadingWidth::Narrow => self.theme.reader_measure_narrow_width,
            ReadingWidth::Comfortable => self.theme.reader_measure_width,
            ReadingWidth::Wide => self.theme.reader_measure_wide_width,
        }
    }

    /// Back, the title, the progress and "mark as read".
    fn render_reader_bar(&self, doc: &crate::domain::GroundingDocument, progress: u8, cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        let finished = doc.record.reading.status == ReadingStatus::Read;
        let done_id = doc.id.clone();
        div()
            .flex()
            .items_center()
            .gap(theme.control_gap)
            .px(theme.panel_padding)
            .py(theme.spacing_sm)
            .child(Button::new(READER_BACK_ID).ghost().small().icon(Icon::new(LucideIcon::ArrowLeft)).label(t!("vault.back").to_string()).on_click(cx.listener(|this, _, _, cx| this.back(cx))))
            .child(div().flex_1().min_w_0().truncate().text_size(theme.font_size_sm).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_secondary).child(doc.title.clone()))
            .child(div().font_family(theme.font_family_mono.clone()).text_size(theme.font_size_xs).text_color(theme.text_muted).child(format!("{progress}%")))
            .child(
                Button::new(READER_DONE_ID)
                    .ghost()
                    .small()
                    .icon(Icon::new(LucideIcon::Check))
                    .label(if finished { t!("vault.mark_unread") } else { t!("vault.mark_read") }.to_string())
                    .on_click(cx.listener(move |this, _, _, cx| this.mark_read(done_id.clone(), !finished, cx))),
            )
    }

    /// The title block, then one child per text block (so a block index scrolls into view).
    fn render_reader_column(&self, doc: &crate::domain::GroundingDocument) -> Vec<AnyElement> {
        let theme = &self.theme;
        let measure = self.measure();
        let measured = |el: Div| el.w_full().max_w(measure).into_any_element();
        let creators = doc.record.authors().iter().map(|c| c.name.clone()).collect::<Vec<_>>().join(", ");
        let heading = div()
            .flex()
            .flex_col()
            .gap(theme.spacing_sm)
            .pb(theme.section_gap)
            .child(div().font_family(theme.font_family_display.clone()).text_size(theme.font_size_xl).font_weight(FontWeight::BOLD).text_color(theme.text_primary).child(doc.display_title()))
            .child(meta_line(vec![creators, doc.record.date.clone()], theme).text_size(theme.font_size_sm));
        let body = blocks(&doc.content);
        let art = || crate::ui::illustration::Illustration::spot(crate::ui::illustration::SpotKey::Reading).width(theme.art_notice_width);
        let empty = body.is_empty().then(|| crate::ui::components::art_state(crate::ui::components::ArtState::hint(art(), t!("vault.no_text").to_string()), theme));
        std::iter::once(heading)
            .chain(body.into_iter().map(|block| self.render_block(block)))
            .chain(empty)
            .map(measured)
            .collect()
    }

    fn render_block(&self, block: TextBlock) -> Div {
        let theme = &self.theme;
        let prose = div().font_family(theme.font_family_body.clone()).line_height(relative(theme.line_height_reading));
        match block.kind {
            BlockKind::Heading(level) => {
                let size = match level {
                    1 => theme.font_size_lg,
                    2 => theme.font_size_md,
                    _ => theme.font_size_base,
                };
                div()
                    .pt(theme.section_gap)
                    .font_family(theme.font_family_display.clone())
                    .text_size(size)
                    .font_weight(FontWeight::BOLD)
                    .text_color(theme.text_primary)
                    .child(block.text)
            }
            BlockKind::Paragraph => prose.text_size(theme.font_size_md).text_color(theme.text_primary).child(block.text),
            BlockKind::Quote => prose
                .pl(theme.card_padding)
                .py(theme.spacing_sm)
                .border_l(theme.border_width_threadline)
                .border_color(theme.accent_dialectic)
                .bg(theme.bg_callout)
                .rounded(theme.radius_md)
                .text_size(theme.font_size_md)
                .italic()
                .text_color(theme.text_primary)
                .child(block.text),
        }
    }
}
