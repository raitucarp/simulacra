//! Sheet header: the format family card, the file name, its facts and the parser's notes;
//! the reading progress and failure bodies.

use super::super::labels::{family_art, family_label, file_size, note_text};
use super::super::session::ImportStage;
use super::ImportSheet;
use crate::import::FormatFamily;
use crate::ui::illustration::{Illustration, IllustrationKind};
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::Icon;
use gpui_kit::*;
use rust_i18n::t;

const CLOSE_ID: &str = "import-sheet-close";
const CANCEL_ID: &str = "import-sheet-cancel";
const FACT_SEPARATOR: &str = " · ";

impl ImportSheet {
    /// The family whose card is shown (unknown while reading).
    fn family(&self) -> FormatFamily {
        self.session.loaded.as_ref().map_or(FormatFamily::Unknown, |file| file.doc.format.family())
    }

    /// `EPUB · 1.2 MB · 135 chapters · 2,450 paragraphs`; `None` until the file is read (the
    /// body says it is being read, or why it could not be: saying "large books take a few
    /// seconds" here too doubled it, and stayed after a failure).
    fn facts_line(&self) -> Option<String> {
        let file = self.session.loaded.as_ref()?;
        let doc = &file.doc;
        let mut facts = vec![family_label(doc.format.family()), doc.format.label().to_string(), file_size(doc.facts.size_bytes)];
        if doc.format.family().has_text() && doc.sections.len() > 1 {
            facts.push(t!("import.count.chapters", count = doc.sections.len()).to_string());
        }
        if doc.format.family().has_text() {
            facts.push(t!("import.count.paragraphs", count = doc.paragraph_count()).to_string());
        }
        Some(facts.join(FACT_SEPARATOR))
    }

    pub(super) fn header(&self) -> Div {
        let theme = &self.theme;
        let close = self.actions.on_close.clone();
        let notes = self.session.loaded.iter().flat_map(|file| file.doc.notes.iter()).map(|note| {
            div().text_size(theme.font_size_xs).text_color(theme.warning).child(note_text(note))
        });
        let text = div()
            .flex_1()
            .min_w_0()
            .flex()
            .flex_col()
            .gap(theme.spacing_2xs)
            .child(div().text_size(theme.font_size_lg).font_weight(FontWeight::BOLD).text_color(theme.text_primary).truncate().child(self.session.file_name.clone()))
            .children(self.facts_line().map(|facts| div().text_size(theme.font_size_xs).text_color(theme.text_muted).child(facts)))
            .children(notes);
        div()
            .flex_none()
            .px(theme.panel_padding)
            .py(theme.spacing_md)
            .border_b(theme.border_width_hairline)
            .border_color(theme.border)
            .flex()
            .items_center()
            .gap(theme.section_gap)
            .child(Illustration::asset(family_art(self.family()), IllustrationKind::TraitCard).width(theme.import_art_width))
            .child(text)
            .child(
                Button::new(CLOSE_ID)
                    .ghost()
                    .icon(Icon::new(LucideIcon::X).size(theme.icon_size_lg))
                    .tooltip(t!("import.close").to_string())
                    .on_click(move |_, window, cx| close(window, cx)),
            )
    }

    /// Reading in the background: a note and a cancel button.
    pub(super) fn progress(&self) -> Div {
        let theme = &self.theme;
        let cancel = self.actions.on_close.clone();
        let reading = matches!(self.session.stage, ImportStage::Reading);
        div()
            .p(theme.panel_padding)
            .flex()
            .flex_col()
            .items_center()
            .gap(theme.stack_gap)
            .child(div().text_size(theme.font_size_sm).text_color(theme.text_secondary).child(t!("import.reading", file = self.session.file_name.clone()).to_string()))
            .child(div().text_size(theme.font_size_xs).text_color(theme.text_muted).child(t!("import.reading_hint").to_string()))
            .children(reading.then(|| {
                Button::new(CANCEL_ID).ghost().label(t!("import.cancel").to_string()).on_click(move |_, window, cx| cancel(window, cx))
            }))
    }

    pub(super) fn failure(&self, reason: &str) -> Div {
        let theme = &self.theme;
        div()
            .p(theme.panel_padding)
            .flex()
            .flex_col()
            .gap(theme.stack_gap)
            .child(div().text_size(theme.font_size_sm).text_color(theme.accent_danger).child(t!("import.failed", reason = reason).to_string()))
    }
}
