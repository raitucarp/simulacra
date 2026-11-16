//! The trading-card character sheet of the Review step: portrait, name, class, key traits
//! as illustrated chips and the personality radar.

use super::super::super::common::{card, title};
use super::super::sheet::radar;
use super::super::{class_badge, stage_portrait};
use crate::domain::TraitOption;
use crate::state::forge::{personality_radar, ForgeKind, TraitField, TraitValue, TraitVisitor};
use crate::theme::ThemeTokens;
use crate::ui::features::forge::text::trait_label;
use crate::ui::features::forge::view::ForgeView;
use crate::ui::illustration::Illustration;
use gpui_kit::*;

/// Traits shown as chips on an agent's card.
const AGENT_KEY_TRAITS: [TraitField; 5] = [
    TraitField::Mbti,
    TraitField::Enneagram,
    TraitField::Temperament,
    TraitField::DictionRegister,
    TraitField::HumorStyle,
];
/// Traits shown as chips on a persona's card (personas skip the psychology steps).
const PERSONA_KEY_TRAITS: [TraitField; 3] = [TraitField::Gender, TraitField::DictionRegister, TraitField::EmojiUsage];
/// The card portrait is this fraction of the stage portrait.
const CARD_PORTRAIT_SCALE: f32 = 0.75;
const CARD_RADAR_ID: &str = "forge-card-radar";

/// One illustrated chip for a trait value.
struct TraitChip<'a> {
    theme: &'a ThemeTokens,
}

impl TraitVisitor for TraitChip<'_> {
    type Output = Div;

    fn visit<T: TraitOption>(self, current: T, _: fn(T) -> TraitValue) -> Div {
        let theme = self.theme;
        div()
            .w(theme.forge_compact_card_width)
            .flex()
            .flex_col()
            .items_center()
            .gap(theme.spacing_2xs)
            .p(theme.spacing_xs)
            .rounded(theme.radius_md)
            .bg(theme.bg_elevated)
            .child(Illustration::trait_card(current).width(theme.forge_part_thumb_size))
            .child(div().text_size(theme.font_size_2xs).font_weight(FontWeight::SEMIBOLD).text_center().child(trait_label(current)))
    }
}

impl ForgeView {
    pub(super) fn trading_card(&self) -> Div {
        let theme = &self.theme;
        let profile = self.draft.profile();
        let traits: &[TraitField] = match self.draft.kind() {
            ForgeKind::Agent => &AGENT_KEY_TRAITS,
            ForgeKind::Persona => &PERSONA_KEY_TRAITS,
        };
        let chips: Vec<Div> = traits.iter().map(|field| field.read(&profile.config).visit(TraitChip { theme })).collect();
        card(theme)
            .items_center()
            .border_color(theme.accent_dialectic_border)
            .child(stage_portrait(profile, theme.forge_portrait_size * CARD_PORTRAIT_SCALE, theme))
            .child(title(self.display_name(), theme))
            .child(div().text_size(theme.font_size_sm).text_color(theme.text_muted).child(crate::state::forge::normalize_handle(&profile.handle)))
            .children(self.class.map(|class| class_badge(class, theme)))
            .child(div().flex().flex_wrap().justify_center().gap(theme.spacing_xs).children(chips))
            .child(radar(CARD_RADAR_ID, personality_radar(&profile.config), theme))
    }
}
