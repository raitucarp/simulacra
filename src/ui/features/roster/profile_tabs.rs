//! sb F1, A2: the Profile / Posts (W7) / Beliefs / Recommended switch above a profile's
//! overview, one row for agents and personas alike, and the agent's Beliefs column.

use super::params::ProfileTab;
use crate::theme::ThemeTokens;
use crate::ui::components::{segment_pill, segmented_track, Segment};
use crate::ui::icons::LucideIcon;
use gpui_kit::*;
use rust_i18n::t;
use std::rc::Rc;

const PROFILE_ID: &str = "roster-tab-profile";
const POSTS_ID: &str = "roster-tab-posts";
const BELIEFS_ID: &str = "roster-tab-beliefs";
const RECOMMENDED_ID: &str = "roster-tab-recommended";

/// The pill switch, the same for agents and personas (A2), in the role's accent.
pub(super) fn profile_tabs(active: ProfileTab, is_agent: bool, theme: &ThemeTokens, on_switch: Rc<dyn Fn(ProfileTab, &mut Window, &mut App)>) -> Div {
    let accent = if is_agent { theme.accent_agent } else { theme.accent_user };
    let pill = |tab: ProfileTab, id: &'static str, icon: LucideIcon, key: &str| {
        let on_switch = on_switch.clone();
        let segment = Segment { id: id.into(), icon, label: t!(key).to_string(), active: active == tab, active_icon_color: accent };
        segment_pill(segment, theme, move |window, cx| on_switch(tab, window, cx))
    };
    segmented_track(theme)
        .max_w(theme.forge_trading_card_width)
        .child(pill(ProfileTab::Profile, PROFILE_ID, LucideIcon::User, "beliefs.tab.profile"))
        .child(pill(ProfileTab::Posts, POSTS_ID, LucideIcon::List, "roster.posts.tab"))
        .child(pill(ProfileTab::Beliefs, BELIEFS_ID, LucideIcon::Brain, "beliefs.tab.beliefs"))
        .child(pill(ProfileTab::Recommended, RECOMMENDED_ID, LucideIcon::Sparkles, "roster.recommended.tab"))
}

/// The Beliefs view in a reading-width column.
pub(super) fn beliefs_column(view: AnyView, theme: &ThemeTokens) -> Div {
    div().w_full().max_w(theme.reader_measure_width).child(view)
}
