//! "Resume your unfinished agent?" shown when the forge opens over an autosaved draft.

use super::common::{card, muted, title};
use super::guide::guide_bubble;
use crate::state::forge::AgentDraft;
use crate::ui::features::forge::logic::GuideMoment;
use crate::ui::features::forge::text::{tr, tr_with};
use crate::ui::features::forge::view::{ForgeHandle, ForgeView};
use crate::ui::icons::LucideIcon;
use crate::ui::illustration::PortraitView;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::*;

const TITLE_KEY: &str = "forge.ui.resume_title";
const BODY_KEY: &str = "forge.ui.resume_body";
const RESUME_KEY: &str = "forge.ui.resume";
const FRESH_KEY: &str = "forge.ui.start_fresh";
const UNNAMED_KEY: &str = "forge.ui.unnamed";
const ARG_NAME: &str = "name";
const ARG_STEP: &str = "step";
const RESUME_ID: &str = "forge-resume";
const FRESH_ID: &str = "forge-start-fresh";
/// The saved portrait is this fraction of the stage portrait.
const PREVIEW_SCALE: f32 = 0.5;

impl ForgeView {
    pub(super) fn resume_screen(&self, saved: &AgentDraft, handle: &ForgeHandle) -> AnyElement {
        let theme = &self.theme;
        let name = Some(saved.profile().name.trim()).filter(|n| !n.is_empty()).map_or_else(|| tr(UNNAMED_KEY), str::to_string);
        let body = tr_with(BODY_KEY, &[(ARG_NAME, name), (ARG_STEP, tr(&saved.step().label_key()))]);
        let (resume, fresh) = (handle.act(ForgeView::resume), handle.act(ForgeView::start_fresh));
        let portrait = saved.profile().config.avatar.clone().map(|mode| PortraitView::new(mode, theme.forge_portrait_size * PREVIEW_SCALE, theme));
        let buttons = div()
            .flex()
            .gap(theme.spacing_sm)
            .child(Button::new(RESUME_ID).primary().icon(LucideIcon::ArrowRight).label(tr(RESUME_KEY)).on_click(move |_, w, cx| resume(w, cx)))
            .child(Button::new(FRESH_ID).outline().label(tr(FRESH_KEY)).on_click(move |_, w, cx| fresh(w, cx)));
        div().size_full().flex().items_center().justify_center().child(
            card(theme)
                .w(theme.forge_trading_card_width)
                .items_center()
                .child(guide_bubble(GuideMoment::Start, theme))
                .children(portrait)
                .child(title(tr(TITLE_KEY), theme))
                .child(muted(body, theme))
                .child(buttons),
        )
        .into_any_element()
    }
}
