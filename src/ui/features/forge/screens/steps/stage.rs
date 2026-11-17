//! The stage: the big portrait on the step's illustrated backdrop, the nameplate and the live
//! character sheet underneath.

use super::step_key;
use crate::domain::{Palette, Profile};
use crate::theme::ThemeTokens;
use crate::ui::atoms::render_profile_avatar;
use crate::ui::components::{meta_badge, BadgeColors};
use crate::ui::features::forge::text::tr;
use crate::ui::features::forge::view::{ForgeHandle, ForgeView};
use crate::ui::illustration::{Illustration, PortraitView};
use gpui_kit::component::scroll::ScrollableElement;
use gpui_kit::*;

const STAGE_ID: &str = "forge-stage";
const UNNAMED_KEY: &str = "forge.ui.unnamed";
/// The backdrop art spans this many portrait widths.
const BACKDROP_SPAN: f32 = 2.0;

/// The profile's avatar at `size`: its avatar mode when set, else the initial badge.
pub(in crate::ui::features::forge::screens) fn stage_portrait(profile: &Profile, size: Pixels, theme: &ThemeTokens) -> AnyElement {
    match profile.config.avatar.clone() {
        Some(mode) => PortraitView::new(mode, size, theme).into_any_element(),
        None => render_profile_avatar(profile, size, theme).into_any_element(),
    }
}

/// A gold badge naming the palette (or class) the draft started from.
pub(in crate::ui::features::forge::screens) fn palette_badge(palette: &Palette, theme: &ThemeTokens) -> Div {
    let colors = BadgeColors::new(theme.accent_dialectic, theme.accent_dialectic_subtle, theme.accent_dialectic_border);
    meta_badge(&tr(&palette.label_key()), colors, theme)
}

impl ForgeView {
    /// The draft's name, or "Unnamed thinker" while it has none.
    pub(in crate::ui::features::forge::screens) fn display_name(&self) -> String {
        Some(self.draft.profile().name.trim()).filter(|n| !n.is_empty()).map_or_else(|| tr(UNNAMED_KEY), str::to_string)
    }

    pub(super) fn stage(&self, handle: &ForgeHandle) -> impl IntoElement {
        let theme = &self.theme;
        div()
            .id(STAGE_ID)
            .flex_1()
            .min_w_0()
            .h_full()
            .overflow_y_scrollbar()
            .p(theme.spacing_xl)
            .flex()
            .flex_col()
            .items_center()
            .gap(theme.spacing_lg)
            .child(self.backdrop())
            .child(self.nameplate())
            .child(self.character_sheet(handle))
    }

    fn backdrop(&self) -> Div {
        let theme = &self.theme;
        let size = theme.forge_portrait_size;
        let art = Illustration::step(step_key(self.draft.step())).width(size * BACKDROP_SPAN);
        div()
            .relative()
            .w(size * BACKDROP_SPAN)
            .flex()
            .justify_center()
            .pt(theme.spacing_lg)
            .rounded(theme.radius_xl)
            .overflow_hidden()
            .bg(theme.bg_card)
            .border(theme.border_width_hairline)
            .border_color(theme.border_subtle)
            .child(div().absolute().top_0().left_0().opacity(theme.forge_backdrop_opacity).child(art))
            .child(div().pb(theme.spacing_lg).child(stage_portrait(self.draft.profile(), size, theme)))
    }

    fn nameplate(&self) -> Div {
        let theme = &self.theme;
        let profile = self.draft.profile();
        let name = self.display_name();
        let kind_colors = BadgeColors::new(theme.accent_brand, theme.accent_brand_subtle, theme.accent_brand_border);
        let badges = div()
            .flex()
            .gap(theme.spacing_xs)
            .child(meta_badge(&tr(&self.draft.kind().label_key()), kind_colors, theme))
            .children(self.palette().map(|palette| palette_badge(palette, theme)));
        div()
            .flex()
            .flex_col()
            .items_center()
            .gap(theme.spacing_xs)
            .child(div().font_family(theme.font_family_display.clone()).text_size(theme.font_size_xl).font_weight(FontWeight::BOLD).child(name))
            .children((!profile.handle.trim().is_empty()).then(|| {
                div().text_size(theme.font_size_sm).text_color(theme.text_muted).child(crate::state::forge::normalize_handle(&profile.handle))
            }))
            .child(badges)
    }
}
