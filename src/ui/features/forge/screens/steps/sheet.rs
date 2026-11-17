//! The live character sheet on the stage: Big Five radar, intelligence skill bars with
//! level names, and a sample post in the draft's voice.

use super::super::common::{card, heading};
use crate::domain::agent_config::AgentConfig;
use crate::state::forge::fields::MAX_SCORE;
use crate::state::forge::{intelligence_radar, personality_radar, voice_sample, QuotientLevel, RadarPoint};
use crate::theme::ThemeTokens;
use crate::ui::atoms::render_profile_avatar;
use crate::ui::features::forge::text::tr;
use crate::ui::features::forge::view::{ForgeHandle, ForgeView};
use gpui_kit::component::chart::RadarChart;
use gpui_kit::*;

const PERSONALITY_KEY: &str = "forge.ui.sheet.personality";
const INTELLIGENCE_KEY: &str = "forge.ui.sheet.intelligence";
const VOICE_KEY: &str = "forge.ui.sheet.voice_sample";
const RADAR_ID: &str = "forge-sheet-radar";
/// The sheet is as wide as this many portraits (matching the backdrop).
const SHEET_SPAN: f32 = 2.0;

/// A radar of `points` (normalized to 0..=1), shown and hovered as 0..=100 scores.
pub(super) fn radar(id: &'static str, points: Vec<RadarPoint>, theme: &ThemeTokens) -> Div {
    let scale = f64::from(MAX_SCORE);
    let chart = RadarChart::new(points)
        .id(id)
        .value(move |point: &RadarPoint| (f64::from(point.value) * scale).round())
        .label(|point: &RadarPoint| tr(&point.label_key))
        .label_color(theme.text_muted)
        .max_value(scale)
        .stroke(theme.accent_brand)
        .fill(theme.accent_brand_subtle)
        .dot();
    div().w_full().h(theme.forge_radar_size).child(chart)
}

fn skill_bar(point: &RadarPoint, cfg: &AgentConfig, theme: &ThemeTokens) -> Div {
    let score = *point.field.of(cfg);
    let level = tr(&QuotientLevel::from_score(score).label_key());
    let fill = relative(f32::from(score) / f32::from(MAX_SCORE));
    div()
        .flex()
        .flex_col()
        .gap(theme.spacing_2xs)
        .child(
            div()
                .flex()
                .justify_between()
                .text_size(theme.font_size_xs)
                .child(div().text_color(theme.text_secondary).child(tr(&point.label_key)))
                .child(div().text_color(theme.accent_brand).font_weight(FontWeight::SEMIBOLD).child(level)),
        )
        .child(
            div()
                .w_full()
                .h(theme.forge_skill_bar_height)
                .rounded(theme.radius_full)
                .bg(theme.bg_elevated)
                .child(div().h_full().w(fill).rounded(theme.radius_full).bg(theme.accent_brand)),
        )
}

impl ForgeView {
    pub(super) fn character_sheet(&self, _: &ForgeHandle) -> Div {
        let theme = &self.theme;
        let cfg = &self.draft.profile().config;
        let bars: Vec<Div> = intelligence_radar(cfg).iter().map(|point| skill_bar(point, cfg, theme)).collect();
        div()
            .w(theme.forge_portrait_size * SHEET_SPAN)
            .flex()
            .flex_col()
            .gap(theme.spacing_md)
            .child(card(theme).child(heading(tr(PERSONALITY_KEY), theme)).child(radar(RADAR_ID, personality_radar(cfg), theme)))
            .child(card(theme).child(heading(tr(INTELLIGENCE_KEY), theme)).children(bars))
            .child(self.voice_bubble())
    }

    fn voice_bubble(&self) -> Div {
        let theme = &self.theme;
        let profile = self.draft.profile();
        let bubble = div()
            .flex_1()
            .min_w_0()
            .p(theme.card_padding)
            .rounded(theme.radius_lg)
            .bg(theme.bg_callout)
            .border_l(theme.border_width_threadline)
            .border_color(theme.accent_brand)
            .text_size(theme.font_size_sm)
            .line_height(relative(theme.line_height_body))
            .child(voice_sample(&self.draft));
        card(theme)
            .child(heading(tr(VOICE_KEY), theme))
            .child(div().flex().items_start().gap(theme.spacing_sm).child(render_profile_avatar(profile, theme.avatar_size_md, theme)).child(bubble))
    }
}
