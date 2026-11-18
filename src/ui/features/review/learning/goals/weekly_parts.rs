//! The weekly review's steps: what was learnt (recalled items, the week's posts and
//! concepts), what comes due (with today's review), the weakest concepts, and the next
//! actions to tick and add to a goal.

use super::super::super::panel::ReviewPanel;
use super::super::super::render::common::{caption, control_row, surface};
use super::super::super::text::{percent, tr};
use super::super::events::LearningEvent;
use super::super::render::{bullet, chip, section_label};
use super::weekly::{action_label, WEEKLY_GAPS};
use crate::domain::learning::goals::edit::with_tasks;
use crate::domain::post::{WIKILINK_CLOSE, WIKILINK_OPEN};
use crate::domain::review::quiz::piece_text;
use crate::i18n::t_count;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::{Disableable, Icon, Sizable};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use rust_i18n::t;

const START_ID: &str = "learning-weekly-start";
const RADAR_ID: &str = "learning-weekly-radar";
const ADD_ID: &str = "learning-weekly-add";
const ACTION_ID_PREFIX: &str = "learning-weekly-action-";
const GOAL_ID_PREFIX: &str = "learning-weekly-goal-";
const CONCEPT_ID_PREFIX: &str = "learning-weekly-concept-";
/// Characters of a post shown in the look back.
const POST_CHARS: usize = 120;
/// Recalled items and posts listed in the look back.
const LOOK_BACK_ROWS: usize = 3;

impl ReviewPanel {
    pub(super) fn weekly_learnt(&self) -> Div {
        let theme = &self.theme;
        let digest = &self.data.learning.digest;
        let recalled = digest.recalled.iter().take(LOOK_BACK_ROWS).filter_map(|id| self.data.item(id)).map(|item| bullet(self.item_line(item), theme.accent_success, theme));
        let plain = |text: String| text.replace(WIKILINK_OPEN, "").replace(WIKILINK_CLOSE, "");
        let posts = digest.posts.iter().take(LOOK_BACK_ROWS).filter_map(|id| self.data.post_text(id)).map(|text| bullet(piece_text(&plain(text), POST_CHARS), theme.accent_dialectic, theme));
        let concepts = digest.concepts.iter().enumerate().map(|(index, (concept, count))| chip(ElementId::NamedInteger(CONCEPT_ID_PREFIX.into(), index as u64), format!("{concept} · {count}"), false, theme));
        let quiet = (digest.reviews == 0 && digest.posts_written == 0).then(|| caption(tr("learning.weekly.quiet"), theme));
        div()
            .flex()
            .flex_col()
            .gap(theme.stack_gap)
            .children(quiet)
            .when(!digest.recalled.is_empty(), |d| d.child(section_label(tr("learning.weekly.recalled"), theme)).children(recalled))
            .when(!digest.posts.is_empty(), |d| d.child(section_label(tr("learning.weekly.wrote"), theme)).children(posts))
            .when(!digest.concepts.is_empty(), |d| d.child(section_label(tr("learning.weekly.concepts"), theme)).child(control_row(theme).children(concepts)))
    }

    pub(super) fn weekly_look_ahead(&self, cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        let digest = &self.data.learning.digest;
        let start = Button::new(START_ID).primary().small().icon(Icon::new(LucideIcon::Play)).label(t!("review.today.start", count = digest.due_today).to_string()).disabled(digest.due_today == 0)
            .on_click(cx.listener(|this, _, window, cx| this.start_session(window, cx)));
        surface(theme)
            .child(self.step_title(2, LucideIcon::Calendar, "learning.weekly.look_ahead"))
            .child(caption(t!("learning.weekly.due", today = digest.due_today, week = digest.due_week).to_string(), theme))
            .child(control_row(theme).child(start))
    }

    pub(super) fn weekly_gaps(&self, cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        let weakest = self.data.learning.gaps.weakest(WEEKLY_GAPS);
        let rows = weakest.iter().map(|entry| bullet(t!("learning.weekly.gap", concept = entry.concept.clone(), gap = percent(entry.gap())).to_string(), theme.warning, theme));
        let radar = Button::new(RADAR_ID).ghost().small().icon(Icon::new(LucideIcon::Gauge)).label(tr("learning.weekly.open_radar")).on_click(cx.listener(|this, _, _, cx| this.open_gaps_for(None, cx)));
        let empty = weakest.is_empty().then(|| caption(tr("learning.gaps.empty"), theme));
        surface(theme).child(self.step_title(3, LucideIcon::Gauge, "learning.weekly.gaps")).children(empty).children(rows).child(control_row(theme).child(radar))
    }

    pub(super) fn step_title(&self, number: usize, icon: LucideIcon, title: &str) -> Div {
        let theme = &self.theme;
        super::super::super::render::common::block_title(icon, theme.accent_dialectic, t!("learning.weekly.step", number = number, title = tr(title)).to_string(), theme)
    }

    fn weekly_goal_chips(&self, cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        let chosen = self.learning.weekly_goal.clone().or_else(|| self.data.learning.goals.iter().find(|view| !view.goal.completed).map(|view| view.goal.id.clone()));
        let chips = self.data.learning.goals.iter().filter(|view| !view.goal.completed).enumerate().map(|(index, view)| {
            let id = view.goal.id.clone();
            chip(ElementId::NamedInteger(GOAL_ID_PREFIX.into(), index as u64), view.goal.title.clone(), chosen.as_ref() == Some(&id), theme).on_click(cx.listener(move |this, _, _, cx| {
                this.learning.weekly_goal = Some(id.clone());
                cx.notify();
            }))
        });
        control_row(theme).children(chips)
    }

    pub(super) fn weekly_next(&self, cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        let actions = self.weekly_actions();
        let rows = actions.iter().enumerate().map(|(index, action)| {
            let ticked = !self.learning.weekly_unticked.contains(&index);
            let icon = if ticked { LucideIcon::CircleCheck } else { LucideIcon::Minus };
            div().id(ElementId::NamedInteger(ACTION_ID_PREFIX.into(), index as u64)).flex().items_center().gap(theme.inline_gap).cursor_pointer()
                .on_click(cx.listener(move |this, _, _, cx| this.toggle_weekly_action(index, cx)))
                .child(Icon::new(icon).size(theme.icon_size_sm).text_color(if ticked { theme.accent_success } else { theme.text_muted }))
                .child(div().text_size(theme.font_size_sm).text_color(theme.text_primary).child(action_label(action)))
        }).collect::<Vec<_>>();
        let ticked = (0..actions.len()).filter(|i| !self.learning.weekly_unticked.contains(i)).count();
        let has_goal = self.data.learning.goals.iter().any(|view| !view.goal.completed);
        let add = Button::new(ADD_ID).primary().small().icon(Icon::new(LucideIcon::Plus)).label(t_count("learning.weekly.add_tasks", ticked)).disabled(ticked == 0 || !has_goal)
            .on_click(cx.listener(|this, _, _, cx| this.add_weekly_tasks(cx)));
        let hint = if has_goal { tr("learning.weekly.choose_goal") } else { tr("learning.weekly.no_goal") };
        surface(theme).child(self.step_title(4, LucideIcon::ListChecks, "learning.weekly.next")).children(rows).child(caption(hint, theme)).child(self.weekly_goal_chips(cx)).child(control_row(theme).child(add))
    }

    fn toggle_weekly_action(&mut self, index: usize, cx: &mut Context<Self>) {
        let unticked = &mut self.learning.weekly_unticked;
        match unticked.iter().position(|i| *i == index) {
            Some(at) => {
                unticked.remove(at);
            }
            None => unticked.push(index),
        }
        cx.notify();
    }

    /// Adds the ticked actions as tasks of the chosen goal.
    fn add_weekly_tasks(&mut self, cx: &mut Context<Self>) {
        let chosen = self.learning.weekly_goal.clone().or_else(|| self.data.learning.goals.iter().find(|view| !view.goal.completed).map(|view| view.goal.id.clone()));
        let Some(goal) = chosen.and_then(|id| self.data.learning.goal(&id)).map(|view| view.goal.clone()) else { return };
        let texts: Vec<String> = self.weekly_actions().iter().enumerate().filter(|(i, _)| !self.learning.weekly_unticked.contains(i)).map(|(_, action)| action_label(action)).collect();
        let goal = with_tasks(goal, &texts, self.data.today);
        self.learning.goals = super::super::state::GoalsScreen::Goal(goal.id.clone());
        self.emit_learning(LearningEvent::SaveGoal(goal), cx);
    }
}
