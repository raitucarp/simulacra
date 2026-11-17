//! Daily Life (agents): a calendar-like page. Body clock (sun or moon), when they are online
//! (a clock ring plus the hour picker), how often and how spontaneously they post, a week
//! strip replayed through the real scheduler, and what fills their days.

mod clock;
mod ring;
mod week;

use super::super::super::common::{card, heading, message};
use super::trait_block::explained;
use crate::agent::schedule::{cadence_rule, peak_window, WeekPreview};
use crate::domain::agent_config::AgentConfig;
use crate::domain::HumanChronotype;
use crate::state::forge::{DraftEdit, HourField, ScoreField, TextField, ToggleField, TraitField, TraitSet};
use crate::ui::features::forge::text::{tr, tr_with};
use crate::ui::features::forge::view::{ForgeHandle, ForgeView};
use crate::ui::features::forge::widgets::{ExplainerButton, HourRangePicker, ToggleCard};
use crate::ui::icons::LucideIcon;
use gpui_kit::component::Icon;
use gpui_kit::*;

const BODY_CLOCK_KEY: &str = "forge.ui.daily.body_clock";
const PEAK_KEY: &str = "forge.ui.daily.peak";
const ONLINE_TITLE_KEY: &str = "forge.ui.daily.online_title";
const RHYTHM_KEY: &str = "forge.ui.daily.rhythm";
const WEEK_KEY: &str = "forge.ui.daily.week_title";
const ACTIVE_DAYS_KEY: &str = "forge.ui.daily.active_days_title";
/// Weekday cards per row until the grid is measured (preset width keeps "Wednesday" whole).
const DAY_COLUMNS: usize = 4;
const DAYS_KEY: &str = "forge.ui.daily.days";
const HOURS_KEY: &str = "forge.ui.daily.hours";
const ARG_START: &str = "start";
const ARG_END: &str = "end";

/// What the week strip's headline says.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum WeekSummary {
    /// "Posts on their own" is off.
    Off,
    /// The cadence is manual-only.
    Manual,
    /// About this many posts in the simulated week.
    Posts(u32),
}

pub(super) fn week_summary(cfg: &AgentConfig, week: &WeekPreview) -> WeekSummary {
    if !cfg.is_active {
        WeekSummary::Off
    } else if cadence_rule(cfg.posting_cadence).is_none() {
        WeekSummary::Manual
    } else {
        WeekSummary::Posts(week.total())
    }
}

/// Sun for the daytime chronotypes, moon for the evening and night ones.
pub(super) fn chronotype_icon(chronotype: HumanChronotype) -> LucideIcon {
    match chronotype {
        HumanChronotype::EarlyBirdLark | HumanChronotype::DaytimeDiurnal => LucideIcon::Sun,
        HumanChronotype::TwilightContemplative | HumanChronotype::NightOwlInsomniac => LucideIcon::Moon,
    }
}

impl ForgeView {
    pub(in super::super) fn daily_life_page(&self, handle: &ForgeHandle) -> AnyElement {
        let theme = &self.theme;
        let profile = self.draft.profile();
        div()
            .flex()
            .flex_col()
            .gap(theme.section_gap)
            .child(card(theme).child(heading(tr(BODY_CLOCK_KEY), theme)).child(self.trait_block(TraitField::Chronotype, handle)).child(self.peak_line()))
            .child(self.online_card(handle))
            .child(
                card(theme)
                    .child(heading(tr(RHYTHM_KEY), theme))
                    .child(self.trait_block(TraitField::PostingCadence, handle))
                    .child(self.score_row(ScoreField::SpontaneityRate, handle)),
            )
            .child(card(theme).child(heading(tr(ACTIVE_DAYS_KEY), theme)).child(self.trait_set_block(
                TraitSet::ActiveDays,
                (DAY_COLUMNS, theme.forge_preset_card_width),
                handle,
            )))
            .child(card(theme).child(heading(tr(WEEK_KEY), theme)).child(week::week_strip(&profile.config, self.draft.id(), theme)))
            .child(
                card(theme)
                    .child(heading(tr(DAYS_KEY), theme))
                    .child(self.text_row(TextField::Hobbies, handle))
                    .child(self.text_row(TextField::DailyRoutine, handle))
                    .child(self.text_row(TextField::PreferredTopics, handle)),
            )
            .into_any_element()
    }

    /// "Most alert 17:00 – 00:00" with a sun or moon.
    fn peak_line(&self) -> Div {
        let theme = &self.theme;
        let chronotype = self.draft.profile().config.chronotype;
        let peak = peak_window(chronotype);
        let args = [(ARG_START, format!("{:02}:00", peak.start)), (ARG_END, format!("{:02}:00", peak.end))];
        div()
            .flex()
            .items_center()
            .gap(theme.spacing_xs)
            .text_color(theme.accent_dialectic)
            .child(Icon::new(chronotype_icon(chronotype)).size(theme.font_size_md))
            .child(message(tr_with(PEAK_KEY, &args), theme.accent_dialectic, theme))
    }

    /// "Active hours" with the hour fields' explainer.
    fn hours_caption(&self) -> Div {
        let theme = &self.theme;
        div()
            .flex()
            .items_center()
            .gap(theme.spacing_xs)
            .child(div().text_size(theme.font_size_xs).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_secondary).child(tr(HOURS_KEY)))
            .children(ExplainerButton::for_field(HourField::ActiveHoursStart.path(), theme))
    }

    fn online_card(&self, handle: &ForgeHandle) -> Div {
        let theme = &self.theme;
        let profile = self.draft.profile();
        let toggle_edit = handle.edit(|on: bool| DraftEdit::Toggle(ToggleField::IsActive, on));
        let toggle = ToggleCard::new(ToggleField::IsActive, profile.config.is_active, theme, move |on, w, cx| toggle_edit(on, w, cx));
        let hour_edit = handle.edit(|(field, hour): (HourField, u8)| DraftEdit::Hour(field, hour));
        let picker_edit = hour_edit.clone();
        let picker = HourRangePicker::new(&profile.config, theme, move |change, w, cx| picker_edit(change, w, cx));
        let start = HourField::ActiveHoursStart.path();
        card(theme)
            .child(heading(tr(ONLINE_TITLE_KEY), theme))
            .child(explained(ToggleField::IsActive.path(), toggle, theme))
            .child(self.hours_caption())
            .child(div().flex().justify_center().child(ring::clock_ring(&profile.config, theme, hour_edit)))
            .child(picker)
            .children(self.issue_lines(start))
            .children(self.issue_lines(HourField::ActiveHoursEnd.path()))
    }
}

#[cfg(test)]
mod tests;
