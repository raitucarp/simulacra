//! The subject index: every vocabulary term from A to Z with its document count, its
//! broader term and scope note; a term opens the catalogue filtered by it.

use super::common::entrance;
use crate::domain::document::facets::Facet;
use crate::domain::document::vocabulary::{subject_index, IndexEntry, IndexLetter};
use crate::domain::document::SubjectId;
use crate::ui::features::vault::panel::VaultPanel;
use crate::ui::illustration::{Illustration, IllustrationKind};
use gpui_kit::component::scroll::ScrollableElement;
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use rust_i18n::t;
use std::collections::HashMap;

/// Banner of the index (`assets/illustrations/vault/subject_index.svg`, 320×120).
pub const SUBJECT_INDEX_ART: &str = "illustrations/vault/subject_index.svg";
const INDEX_SCROLL_ID: &str = "vault-subject-index";

impl VaultPanel {
    pub(super) fn render_subject_index(&self, cx: &mut Context<Self>) -> AnyElement {
        let theme = &self.theme;
        let counts: HashMap<SubjectId, usize> =
            self.facet_values(Facet::Subject).into_iter().map(|c| (SubjectId::new(c.key), c.count)).collect();
        let letters = subject_index(&self.data.library.subjects, &counts);
        let banner = div()
            .flex()
            .flex_wrap()
            .items_center()
            .gap(theme.section_gap)
            .child(Illustration::asset(SUBJECT_INDEX_ART, IllustrationKind::Step).width(theme.vault_banner_width))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(theme.spacing_xs)
                    .max_w(theme.reader_measure_width)
                    .child(div().font_family(theme.font_family_display.clone()).text_size(theme.font_size_xl).font_weight(FontWeight::BOLD).text_color(theme.text_primary).child(t!("vault.subject_index").to_string()))
                    .child(div().text_size(theme.font_size_sm).text_color(theme.text_secondary).child(t!("vault.subject_index_hint").to_string())),
            );
        let groups: Vec<AnyElement> = letters.into_iter().map(|letter| self.render_index_letter(letter, cx)).collect();
        let empty = groups.is_empty().then(|| div().flex().justify_center().child(crate::ui::components::art_state(crate::ui::components::ArtState::hint(crate::ui::illustration::Illustration::empty(crate::ui::illustration::EmptyKey::SubjectsNone).width(theme.art_empty_width), t!("vault.no_subjects").to_string()), theme)));
        let page = div()
            .id(INDEX_SCROLL_ID)
            .size_full()
            .overflow_y_scrollbar()
            .p(theme.panel_padding)
            .flex()
            .flex_col()
            .gap(theme.section_gap)
            .child(self.render_back_bar(cx))
            .child(banner)
            .children(groups)
            .when_some(empty, |p, e| p.child(e));
        entrance(div().size_full().child(page), INDEX_SCROLL_ID, self.data.reduced_motion, theme)
    }

    fn render_index_letter(&self, letter: IndexLetter, cx: &mut Context<Self>) -> AnyElement {
        let theme = &self.theme;
        div()
            .flex()
            .gap(theme.section_gap)
            .pt(theme.stack_gap)
            .border_t(theme.border_width_hairline)
            .border_color(theme.border_subtle)
            .child(
                div()
                    .w(theme.vault_letter_cell_size)
                    .flex_shrink_0()
                    .font_family(theme.font_family_display.clone())
                    .text_size(theme.font_size_xl)
                    .font_weight(FontWeight::BOLD)
                    .text_color(theme.accent_brand)
                    .child(letter.letter.to_string()),
            )
            .child(div().flex_1().min_w_0().flex().flex_col().gap(theme.spacing_xs).children(letter.entries.into_iter().map(|e| self.render_index_entry(e, cx))))
            .into_any_element()
    }

    fn render_index_entry(&self, entry: IndexEntry, cx: &mut Context<Self>) -> AnyElement {
        let theme = &self.theme;
        let hover = theme.hover_bg;
        let key = entry.id.as_str().to_string();
        let scope = self.data.library.subject(&entry.id).map(|s| s.scope_note.clone()).unwrap_or_default();
        let under = entry.broader.map(|broader| t!("vault.under", term = broader).to_string());
        div()
            .id(ElementId::Name(format!("index-{key}").into()))
            .flex()
            .flex_col()
            .gap(theme.spacing_2xs)
            .px(theme.spacing_sm)
            .py(theme.spacing_xs)
            .rounded(theme.radius_md)
            .cursor_pointer()
            .hover(move |s| s.bg(hover))
            .when(entry.count == 0, |row| row.opacity(theme.vault_muted_opacity))
            .on_click(cx.listener(move |this, _, window, cx| this.browse_only(Facet::Subject, &key, window, cx)))
            .child(
                div()
                    .flex()
                    .items_baseline()
                    .gap(theme.control_gap)
                    .child(div().text_size(theme.font_size_sm).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_primary).child(entry.label))
                    .children(under.map(|u| div().text_size(theme.font_size_xs).text_color(theme.text_muted).child(u)))
                    .child(div().flex_1())
                    .child(div().font_family(theme.font_family_mono.clone()).text_size(theme.font_size_xs).text_color(theme.text_muted).child(crate::i18n::t_count("vault.count.documents", entry.count))),
            )
            .when(!scope.is_empty(), |row| row.child(div().text_size(theme.font_size_xs).text_color(theme.text_secondary).child(scope)))
            .into_any_element()
    }
}
