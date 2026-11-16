//! [`PersonalityPanel`]: the "balanced build" switch with its points meter, then the Big Five
//! and HEXACO sliders. Whether the budget applies is view-only state kept in keyed element
//! state (it starts on when the draft already fits the budget).

use super::budget::{BudgetMeter, BIG_FIVE, HEXACO};
use crate::domain::agent_config::AgentConfig;
use crate::state::forge::attributes::is_balanced_build;
use crate::state::forge::ScoreField;
use crate::theme::ThemeTokens;
use crate::ui::features::forge::screens::common::{card, heading, muted};
use crate::ui::features::forge::text::{tr, tr_count, tr_with};
use crate::ui::features::forge::widgets::{ScoreSlider, ToggleCard, ValueCallback};
use gpui_kit::*;

const PANEL_ID: &str = "forge-personality-panel";
const BALANCED_ID: &str = "forge-balanced-build";
const BALANCED_KEY: &str = "forge.ui.attributes.balanced";
const BALANCED_HINT_KEY: &str = "forge.ui.attributes.balanced_hint";
const POINTS_KEY: &str = "forge.ui.attributes.points_left";
const OVER_KEY: &str = "forge.ui.attributes.over_budget";
const FREE_KEY: &str = "forge.ui.attributes.free_build";
const BIG_FIVE_KEY: &str = "forge.ui.attributes.big_five";
const BIG_FIVE_HINT_KEY: &str = "forge.ui.attributes.big_five_hint";
const HEXACO_KEY: &str = "forge.ui.attributes.hexaco";
const HEXACO_HINT_KEY: &str = "forge.ui.attributes.hexaco_hint";
const ARG_LEFT: &str = "left";
const ARG_TOTAL: &str = "total";

/// Whether the balanced-build budget caps the sliders.
struct BuildMode {
    balanced: bool,
}

/// The personality sliders of one config; `on_score` gets each released value.
#[derive(IntoElement)]
pub(super) struct PersonalityPanel {
    pub cfg: AgentConfig,
    pub theme: ThemeTokens,
    pub on_score: ValueCallback<(ScoreField, u8)>,
}

impl PersonalityPanel {
    fn meter(&self, balanced: bool) -> Div {
        let theme = &self.theme;
        let meter = BudgetMeter::of(&self.cfg);
        let over = meter.over_by().filter(|_| balanced);
        let color = if over.is_some() { theme.warning } else { theme.accent_brand };
        let line = match (balanced, over) {
            (false, _) => muted(tr(FREE_KEY), theme),
            (true, Some(over)) => muted(tr_count(OVER_KEY, usize::from(over), &[]), theme).text_color(theme.warning),
            (true, None) => {
                let args = [(ARG_LEFT, meter.remaining().to_string()), (ARG_TOTAL, meter.budget.to_string())];
                muted(tr_with(POINTS_KEY, &args), theme)
            }
        };
        let bar = div()
            .w_full()
            .h(theme.forge_skill_bar_height)
            .rounded(theme.radius_full)
            .bg(theme.bg_elevated)
            .child(div().h_full().w(relative(meter.fill())).rounded(theme.radius_full).bg(color));
        div().flex().flex_col().gap(theme.spacing_xs).child(line).children(balanced.then_some(bar))
    }

    fn budget_card(&self, balanced: bool, mode: Entity<BuildMode>) -> Div {
        let toggle = ToggleCard::custom(BALANCED_ID, tr(BALANCED_KEY), balanced, &self.theme, move |on, _, cx| {
            mode.update(cx, |mode, cx| {
                mode.balanced = on;
                cx.notify();
            });
        })
        .hint(Some(tr(BALANCED_HINT_KEY)));
        card(&self.theme).child(toggle).child(self.meter(balanced))
    }

    fn slider_card(&self, title_key: &str, hint_key: &str, fields: &[ScoreField], balanced: bool) -> Div {
        let theme = &self.theme;
        let sliders = fields.iter().map(|&field| {
            let on_score = self.on_score.clone();
            ScoreSlider::new(field, &self.cfg, theme, move |value, w, cx| on_score((field, value), w, cx)).budgeted(balanced)
        });
        card(theme).gap(theme.spacing_lg).child(heading(tr(title_key), theme)).child(muted(tr(hint_key), theme)).children(sliders)
    }
}

impl RenderOnce for PersonalityPanel {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let initially = is_balanced_build(&self.cfg);
        let mode = window.use_keyed_state(PANEL_ID, cx, |_, _| BuildMode { balanced: initially });
        let balanced = mode.read(cx).balanced;
        div()
            .flex()
            .flex_col()
            .gap(self.theme.spacing_lg)
            .child(self.budget_card(balanced, mode))
            .child(self.slider_card(BIG_FIVE_KEY, BIG_FIVE_HINT_KEY, &BIG_FIVE, balanced))
            .child(self.slider_card(HEXACO_KEY, HEXACO_HINT_KEY, &HEXACO, balanced))
    }
}
