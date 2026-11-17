//! The encyclopedic entry of a document: title block and actions, abstract, filing
//! (subjects, collections, classification), contents, citations, related entries and
//! backlinks; the cover and the metadata table sit beside them on wide windows.

mod citations;
mod connections;
mod filing;
mod metadata;
mod title;

use super::common::entrance;
use super::cover_view::cover;
use crate::domain::GroundingDocument;
use crate::ui::features::vault::panel::VaultPanel;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::scroll::ScrollableElement;
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::*;
use rust_i18n::t;

const ENTRY_SCROLL_ID: &str = "vault-entry";
const BACK_BUTTON_ID: &str = "vault-back";

impl VaultPanel {
    pub(super) fn render_entry(&self, id: &str, compact: bool, cx: &mut Context<Self>) -> AnyElement {
        let Some(doc) = self.data.document(id) else { return div().into_any_element() };
        let theme = &self.theme;
        let main = div()
            .flex_1()
            .min_w_0()
            .flex()
            .flex_col()
            .gap(theme.section_gap)
            .child(self.render_title_block(doc, cx))
            .children(self.render_abstract(doc))
            .child(self.render_filing(doc, cx))
            .children(self.render_contents(doc, cx))
            .child(self.render_citations(doc, cx))
            .child(self.render_related(doc, cx))
            .child(self.render_backlinks(doc, cx));
        let side = self.render_entry_side(doc, compact);
        let columns = if compact {
            div().flex().flex_col().gap(theme.section_gap).child(side).child(main)
        } else {
            div().flex().items_start().gap(theme.section_gap).child(main).child(side)
        };
        let page = div()
            .id(ENTRY_SCROLL_ID)
            .size_full()
            .overflow_y_scrollbar()
            .p(theme.panel_padding)
            .flex()
            .flex_col()
            .gap(theme.section_gap)
            .child(self.render_back_bar(cx))
            .child(columns);
        entrance(div().size_full().child(page), ElementId::Name(format!("vault-entry-{id}").into()), self.data.reduced_motion, theme)
    }

    /// "← Back" to wherever the entry was opened from.
    pub(super) fn render_back_bar(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let button = Button::new(BACK_BUTTON_ID)
            .ghost()
            .small()
            .icon(Icon::new(LucideIcon::ArrowLeft))
            .label(t!("vault.back").to_string())
            .on_click(cx.listener(|this, _, _, cx| this.back(cx)));
        div().flex().child(button)
    }

    /// Cover and metadata (beside the entry, or above it on compact windows).
    fn render_entry_side(&self, doc: &GroundingDocument, compact: bool) -> impl IntoElement {
        let theme = &self.theme;
        let side = div().flex().flex_col().gap(theme.section_gap).child(cover(doc, theme.vault_entry_cover_width, theme));
        let side = if compact { side.w_full().items_center() } else { side.w(theme.vault_entry_side_width).flex_shrink_0() };
        side.child(self.render_metadata(doc))
    }
}
