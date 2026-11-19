//! The browse screen: header, toolbar, active filters, then the shelf or the table.

use super::common::META_SEPARATOR;
use crate::i18n::t_count;
use crate::ui::features::vault::panel::VaultPanel;
use crate::ui::features::vault::screen::{BrowseLayout, Screen};
use crate::ui::features::vault::VaultEvent;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::scroll::ScrollableElement;
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use rust_i18n::t;

const BROWSE_SCROLL_ID: &str = "vault-browse";
const ADD_BUTTON_ID: &str = "vault-add";
const IMPORT_BUTTON_ID: &str = "vault-import";
const INDEX_BUTTON_ID: &str = "vault-subject-index";

impl VaultPanel {
    pub(super) fn render_browse(&self, compact: bool, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &self.theme;
        let body = if self.data.documents.is_empty() {
            self.render_empty_vault(cx).into_any_element()
        } else if self.results.is_empty() {
            self.render_no_matches(cx).into_any_element()
        } else {
            match self.layout {
                BrowseLayout::Shelf => self.render_shelf(cx).into_any_element(),
                BrowseLayout::Table => self.render_table(compact, cx).into_any_element(),
            }
        };
        div()
            .id(BROWSE_SCROLL_ID)
            .size_full()
            .overflow_y_scrollbar()
            .p(theme.panel_padding)
            .flex()
            .flex_col()
            .gap(theme.section_gap)
            .child(self.render_browse_header(compact, cx))
            .when(!self.data.documents.is_empty(), |d| d.child(self.render_toolbar(compact, cx)))
            .children(self.render_active_filters(cx))
            .child(body)
    }

    fn render_browse_header(&self, compact: bool, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &self.theme;
        let summary = [
            t_count("vault.count.documents", self.data.documents.len()),
            t_count("vault.count.subjects", self.data.library.subjects.len()),
            t_count("vault.count.collections", self.data.library.collections_of(&self.data.network_id).len()),
        ]
        .join(META_SEPARATOR);
        let title = div()
            .flex()
            .flex_col()
            .gap(theme.spacing_2xs)
            .child(
                div()
                    .font_family(theme.font_family_display.clone())
                    .text_size(theme.font_size_lg)
                    .font_weight(FontWeight::BOLD)
                    .text_color(theme.text_primary)
                    .child(t!("vault.title").to_string()),
            )
            .child(div().text_size(theme.font_size_xs).text_color(theme.text_muted).child(summary));
        div().flex().flex_wrap().items_center().justify_between().gap(theme.control_gap).child(title).child(self.render_header_actions(compact, cx))
    }

    fn render_header_actions(&self, compact: bool, cx: &mut Context<Self>) -> impl IntoElement {
        let label = |key: &str| (!compact).then(|| t!(key).to_string());
        let index = Button::new(INDEX_BUTTON_ID)
            .ghost()
            .small()
            .icon(Icon::new(LucideIcon::Library))
            .tooltip(t!("vault.subject_index").to_string())
            .when_some(label("vault.subject_index"), |b, l| b.label(l))
            .on_click(cx.listener(|this, _, _, cx| this.go(Screen::SubjectIndex, cx)));
        let import = Button::new(IMPORT_BUTTON_ID)
            .ghost()
            .small()
            .icon(Icon::new(LucideIcon::FileText))
            .tooltip(t!("vault.import_file").to_string())
            .when_some(label("vault.import_file"), |b, l| b.label(l))
            .on_click(cx.listener(|_, _, _, cx| cx.emit(VaultEvent::Import)));
        let add = Button::new(ADD_BUTTON_ID)
            .primary()
            .small()
            .icon(Icon::new(LucideIcon::Plus))
            .label(t!("vault.add_document").to_string())
            .on_click(cx.listener(|this, _, window, cx| this.open_editor(None, window, cx)));
        div().flex().items_center().gap(self.theme.control_gap).child(index).child(import).child(add)
    }
}
