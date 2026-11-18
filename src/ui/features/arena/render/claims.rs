//! Which claims still stand: every claim of the transcript's argument map with its speaker
//! and its standing under the grounded semantics (stands, contested, defeated).

use super::super::kit::{block_title, caption, surface};
use super::super::panel::ArenaPanel;
use super::super::text::{standing_label, tr};
use crate::domain::arena::debate::{claim_standings, Debate};
use crate::domain::thinking::argument::{ArgumentMap, Standing};
use crate::theme::ThemeTokens;
use crate::ui::components::{meta_badge, BadgeColors};
use crate::ui::icons::LucideIcon;
use gpui_kit::*;

fn standing_colors(standing: Standing, theme: &ThemeTokens) -> BadgeColors {
    match standing {
        Standing::Accepted => BadgeColors::new(theme.accent_success, theme.accent_success_subtle, theme.accent_success_border),
        Standing::Undecided => BadgeColors::new(theme.accent_dialectic, theme.accent_dialectic_subtle, theme.accent_dialectic_border),
        Standing::Rejected => BadgeColors::new(theme.text_muted, theme.bg_elevated, theme.border_subtle),
    }
}

impl ArenaPanel {
    pub(super) fn render_claims(&self, debate: &Debate, map: &ArgumentMap) -> Div {
        let theme = &self.theme;
        let claims = claim_standings(debate, map);
        let rows = claims.iter().map(|claim| {
            let speaker = claim.speaker.as_ref().map(|agent| self.data.agent_name(agent)).unwrap_or_default();
            let text = div().flex_1().min_w_0().text_size(theme.font_size_sm).line_height(relative(theme.line_height_body)).text_color(theme.text_primary).child(claim.text.clone());
            div()
                .w_full()
                .flex()
                .items_start()
                .gap(theme.control_gap)
                .child(meta_badge(&standing_label(claim.standing), standing_colors(claim.standing, theme), theme).flex_none())
                .child(div().flex_1().min_w_0().flex().flex_col().gap(theme.spacing_3xs).child(text).child(caption(speaker, theme)))
        });
        let empty = claims.is_empty().then(|| caption(tr("arena.claims.none"), theme));
        surface(theme)
            .child(block_title(LucideIcon::Waypoints, theme.accent_success, tr("arena.claims.title"), theme))
            .child(caption(tr("arena.claims.hint"), theme))
            .children(rows)
            .children(empty)
    }
}
