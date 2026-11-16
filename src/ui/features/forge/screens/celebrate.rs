//! The celebration after "Bring to life": Mnemo cheers next to the new thinker.

use super::common::{card, muted, title};
use super::guide::guide_bubble;
use super::steps::stage_portrait;
use crate::ui::features::forge::logic::GuideMoment;
use crate::ui::features::forge::text::{tr, tr_with};
use crate::ui::features::forge::view::{ForgeHandle, ForgeView};
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::*;

const TITLE_KEY: &str = "forge.ui.celebrate_title";
const BODY_KEY: &str = "forge.ui.celebrate_body";
const DONE_KEY: &str = "forge.ui.back_to_roster";
const AGAIN_KEY: &str = "forge.ui.forge_another";
const ARG_NAME: &str = "name";
const DONE_ID: &str = "forge-celebrate-done";
const AGAIN_ID: &str = "forge-celebrate-again";

impl ForgeView {
    pub(super) fn celebrate_screen(&self, handle: &ForgeHandle) -> AnyElement {
        let theme = &self.theme;
        let profile = self.draft.profile();
        let (done, again) = (handle.act(ForgeView::close), handle.act(ForgeView::forge_another));
        let buttons = div()
            .flex()
            .gap(theme.spacing_sm)
            .child(Button::new(DONE_ID).primary().icon(LucideIcon::Check).label(tr(DONE_KEY)).on_click(move |_, w, cx| done(w, cx)))
            .child(Button::new(AGAIN_ID).outline().icon(LucideIcon::Sparkles).label(tr(AGAIN_KEY)).on_click(move |_, w, cx| again(w, cx)));
        div()
            .size_full()
            .flex()
            .items_center()
            .justify_center()
            .child(
                card(theme)
                    .w(theme.forge_trading_card_width)
                    .items_center()
                    .child(stage_portrait(profile, theme.forge_portrait_size, theme))
                    .child(title(tr_with(TITLE_KEY, &[(ARG_NAME, profile.name.clone())]), theme))
                    .child(muted(tr(BODY_KEY), theme))
                    .child(guide_bubble(GuideMoment::Celebrating, theme))
                    .child(buttons),
            )
            .into_any_element()
    }
}
