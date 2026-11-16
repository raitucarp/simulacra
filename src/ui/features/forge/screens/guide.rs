//! Mnemo the guide owl: pose by context, a speech bubble with the tip, dismissible.

use crate::theme::ThemeTokens;
use crate::ui::features::forge::logic::{guide_pose, guide_tip_key, GuideMoment};
use crate::ui::features::forge::text::tr;
use crate::ui::features::forge::view::{ForgeHandle, ForgeScreen, ForgeView};
use crate::ui::icons::LucideIcon;
use crate::ui::illustration::Illustration;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::Sizable;
use gpui_kit::*;

const NAME_KEY: &str = "forge.guide.name";
const HIDE_KEY: &str = "forge.ui.guide_hide";
const SHOW_KEY: &str = "forge.ui.guide_show";
const HIDE_ID: &str = "forge-guide-hide";
const SHOW_ID: &str = "forge-guide-show";

impl ForgeView {
    /// What Mnemo is reacting to.
    pub(super) fn guide_moment(&self) -> GuideMoment {
        if self.guide.thinking {
            return GuideMoment::Waiting;
        }
        match self.screen {
            ForgeScreen::Celebrate => GuideMoment::Celebrating,
            ForgeScreen::Steps => GuideMoment::Step { step: self.draft.step(), fresh: self.guide.fresh_step },
            ForgeScreen::Start | ForgeScreen::Resume(_) => GuideMoment::Start,
        }
    }

    /// Mnemo with the tip for the current moment, or `None` while hidden.
    pub(super) fn guide_panel(&self, handle: &ForgeHandle) -> Option<Div> {
        if self.guide.hidden {
            return None;
        }
        let moment = self.guide_moment();
        let hide = handle.act(|this, window, cx| this.set_guide_hidden(true, window, cx));
        let close = Button::new(HIDE_ID)
            .ghost()
            .xsmall()
            .icon(LucideIcon::X)
            .tooltip(tr(HIDE_KEY))
            .on_click(move |_, window, cx| hide(window, cx));
        Some(guide_bubble(moment, &self.theme).child(div().flex_shrink_0().child(close)))
    }

    /// The "?" button that brings Mnemo back.
    pub(super) fn guide_show_button(&self, handle: &ForgeHandle) -> Option<Button> {
        if !self.guide.hidden {
            return None;
        }
        let show = handle.act(|this, window, cx| this.set_guide_hidden(false, window, cx));
        Some(
            Button::new(SHOW_ID)
                .ghost()
                .small()
                .icon(LucideIcon::HelpCircle)
                .tooltip(tr(SHOW_KEY))
                .on_click(move |_, window, cx| show(window, cx)),
        )
    }
}

/// The owl in its pose beside a speech bubble.
pub(super) fn guide_bubble(moment: GuideMoment, theme: &ThemeTokens) -> Div {
    let bubble = div()
        .flex_1()
        .min_w_0()
        .max_w(theme.forge_bubble_max_width)
        .p(theme.spacing_md)
        .rounded(theme.radius_lg)
        .bg(theme.bg_elevated)
        .border(theme.border_width_hairline)
        .border_color(theme.accent_brand_border)
        .flex()
        .flex_col()
        .gap(theme.spacing_2xs)
        .child(div().text_size(theme.font_size_xs).font_weight(FontWeight::BOLD).text_color(theme.accent_brand).child(tr(NAME_KEY)))
        .child(
            div()
                .text_size(theme.font_size_sm)
                .line_height(relative(theme.line_height_body))
                .child(tr(&guide_tip_key(moment))),
        );
    div()
        .w_full()
        .flex()
        .items_start()
        .gap(theme.spacing_sm)
        .child(Illustration::guide(guide_pose(moment)).width(theme.forge_guide_size))
        .child(bubble)
}
