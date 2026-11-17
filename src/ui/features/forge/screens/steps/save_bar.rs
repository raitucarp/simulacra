//! The sticky "Unsaved changes · Discard / Save" bar (Save disabled with the reason while
//! errors block it).

use crate::ui::features::forge::logic::{save_bar_state, SaveBarState};
use crate::ui::features::forge::text::{tr, tr_count};
use crate::ui::features::forge::view::{ForgeHandle, ForgeView};
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::Icon;
use gpui_kit::component::Disableable;
use gpui_kit::*;

const UNSAVED_KEY: &str = "forge.ui.unsaved";
const BLOCKED_KEY: &str = "forge.ui.save_blocked";
const DISCARD_KEY: &str = "forge.ui.discard";
const SAVE_KEY: &str = "forge.ui.save";
const DISCARD_ID: &str = "forge-discard";
const SAVE_ID: &str = "forge-save";

impl ForgeView {
    /// The bar, while the draft has unsaved changes.
    pub(super) fn save_bar(&self, handle: &ForgeHandle) -> Option<Div> {
        let state = save_bar_state(self.draft.is_dirty(), &self.issues);
        if !state.is_visible() {
            return None;
        }
        let theme = &self.theme;
        let reason = match state {
            SaveBarState::Blocked { errors } => Some(tr_count(BLOCKED_KEY, errors, &[])),
            SaveBarState::Ready | SaveBarState::Hidden => None,
        };
        let (discard, save) = (handle.act(ForgeView::discard), handle.act(ForgeView::save));
        let mut save_button = Button::new(SAVE_ID).primary().icon(LucideIcon::Check).label(tr(SAVE_KEY)).disabled(!state.can_save());
        if let Some(reason) = reason.clone() {
            save_button = save_button.tooltip(reason);
        }
        let status = div()
            .flex_1()
            .flex()
            .items_center()
            .gap(theme.spacing_sm)
            .child(div().text_color(theme.warning).child(Icon::new(LucideIcon::CircleAlert).size(theme.font_size_md)))
            .child(div().text_size(theme.font_size_sm).font_weight(FontWeight::SEMIBOLD).child(tr(UNSAVED_KEY)))
            .children(reason.map(|text| div().text_size(theme.font_size_xs).text_color(theme.text_muted).child(text)));
        Some(
            div()
                .flex_shrink_0()
                .flex()
                .items_center()
                .gap(theme.spacing_sm)
                .px(theme.panel_padding)
                .py(theme.spacing_sm)
                .border_t(theme.border_width_hairline)
                .border_color(theme.border_strong)
                .bg(theme.bg_elevated)
                .child(status)
                .child(Button::new(DISCARD_ID).ghost().label(tr(DISCARD_KEY)).on_click(move |_, w, cx| discard(w, cx)))
                .child(save_button.on_click(move |_, w, cx| save(w, cx))),
        )
    }
}
