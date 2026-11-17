//! Illustrated empty states: an empty vault (with the two ways in) and a search that found
//! nothing (with the way back).

use crate::ui::features::vault::panel::VaultPanel;
use crate::ui::features::vault::VaultEvent;
use crate::ui::icons::LucideIcon;
use crate::ui::illustration::{Illustration, IllustrationKind};
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::*;
use rust_i18n::t;

/// Empty-state scenes (`assets/illustrations/empty/`, typed as `EmptyKey::VaultEmpty` and
/// `EmptyKey::VaultNoMatches`).
pub const EMPTY_VAULT_ART: &str = "illustrations/empty/vault_empty.svg";
pub const NO_MATCHES_ART: &str = "illustrations/empty/vault_no_matches.svg";
const EMPTY_ADD_ID: &str = "vault-empty-add";
const EMPTY_IMPORT_ID: &str = "vault-empty-import";
const EMPTY_CLEAR_ID: &str = "vault-empty-clear";

impl VaultPanel {
    fn empty_state(&self, art: &str, title: String, hint: String) -> Div {
        let theme = &self.theme;
        div()
            .w_full()
            .py(theme.section_gap)
            .flex()
            .flex_col()
            .items_center()
            .gap(theme.stack_gap)
            .child(Illustration::asset(art, IllustrationKind::Empty).width(theme.vault_empty_art_width))
            .child(div().font_family(theme.font_family_display.clone()).text_size(theme.font_size_md).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_primary).child(title))
            .child(div().max_w(theme.reader_measure_width).text_size(theme.font_size_sm).text_color(theme.text_secondary).text_center().child(hint))
    }

    pub(super) fn render_empty_vault(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let add = Button::new(EMPTY_ADD_ID)
            .primary()
            .small()
            .icon(Icon::new(LucideIcon::Plus))
            .label(t!("vault.add_document").to_string())
            .on_click(cx.listener(|this, _, window, cx| this.open_editor(None, window, cx)));
        let import = Button::new(EMPTY_IMPORT_ID)
            .ghost()
            .small()
            .icon(Icon::new(LucideIcon::FileText))
            .label(t!("vault.import_file").to_string())
            .on_click(cx.listener(|_, _, _, cx| cx.emit(VaultEvent::Import)));
        self.empty_state(EMPTY_VAULT_ART, t!("vault.empty_title").to_string(), t!("vault.empty_hint").to_string())
            .child(div().flex().gap(self.theme.control_gap).child(add).child(import))
    }

    pub(super) fn render_no_matches(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let clear = Button::new(EMPTY_CLEAR_ID)
            .ghost()
            .small()
            .icon(Icon::new(LucideIcon::X))
            .label(t!("vault.clear_search").to_string())
            .on_click(cx.listener(|this, _, window, cx| {
                this.search_input.update(cx, |input, cx| input.set_value("", window, cx));
                this.clear_filters(cx);
            }));
        self.empty_state(NO_MATCHES_ART, t!("vault.no_matches_title").to_string(), t!("vault.no_matches_hint").to_string()).child(clear)
    }
}
