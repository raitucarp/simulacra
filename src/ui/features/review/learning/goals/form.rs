//! The new-goal form: what to learn, why it matters, and a horizon for the target date.

use super::super::super::panel::ReviewPanel;
use super::super::super::render::common::{block_title, caption, control_row, surface};
use super::super::super::text::tr;
use super::super::events::LearningEvent;
use super::super::render::{chip, framed_input, section_label};
use super::super::state::{GoalsScreen, GOAL_HORIZONS};
use super::super::tutor::take_text;
use super::list::DATE_FORMAT;
use crate::domain::learning::goals::edit::{goal_problem, new_goal, GoalDraft};
use crate::domain::learning::GoalId;
use crate::domain::review::day::add_days;
use crate::ui::features::review::panel::PanelFields;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::*;
use rust_i18n::t;

const CREATE_ID: &str = "learning-goal-create";
const CANCEL_ID: &str = "learning-goal-cancel";
const HORIZON_ID_PREFIX: &str = "learning-goal-horizon-";

impl ReviewPanel {
    fn goal_draft(&self, cx: &App) -> GoalDraft {
        GoalDraft {
            title: PanelFields::text(&self.learn_fields.goal_title, cx),
            why: PanelFields::text(&self.learn_fields.goal_why, cx),
            target: add_days(self.data.today, self.learning.new_goal_days),
        }
    }

    /// Creates the typed goal and opens it.
    fn create_goal(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let draft = self.goal_draft(cx);
        let Some(goal) = new_goal(&draft, self.data.network_id.clone(), GoalId::generate(), self.data.today, jiff::Timestamp::now()) else { return cx.notify() };
        take_text(&self.learn_fields.goal_title, window, cx);
        take_text(&self.learn_fields.goal_why, window, cx);
        self.learning.goals = GoalsScreen::Goal(goal.id.clone());
        self.emit_learning(LearningEvent::SaveGoal(goal), cx);
    }

    pub(super) fn render_goal_form(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &self.theme;
        let horizons = GOAL_HORIZONS.iter().enumerate().map(|(index, days)| {
            let days = *days;
            let label = tr(&format!("learning.goals.horizon.{days}"));
            chip(ElementId::NamedInteger(HORIZON_ID_PREFIX.into(), index as u64), label, self.learning.new_goal_days == days, theme).on_click(cx.listener(move |this, _, _, cx| {
                this.learning.new_goal_days = days;
                cx.notify();
            }))
        }).collect::<Vec<_>>();
        let draft = self.goal_draft(cx);
        let problem = goal_problem(&draft, self.data.today).map(|problem| caption(tr(problem.message_key()), theme).text_color(theme.warning));
        let create = Button::new(CREATE_ID).primary().small().icon(Icon::new(LucideIcon::Check)).label(tr("learning.goals.create")).on_click(cx.listener(|this, _, window, cx| this.create_goal(window, cx)));
        let cancel = Button::new(CANCEL_ID).ghost().small().label(tr("learning.cancel")).on_click(cx.listener(|this, _, _, cx| this.show_goals(GoalsScreen::List, cx)));
        surface(theme)
            .max_w(theme.review_card_max_width)
            .child(block_title(LucideIcon::Target, theme.accent_brand, tr("learning.goals.new"), theme))
            .child(section_label(tr("learning.goals.title_label"), theme))
            .child(control_row(theme).child(framed_input(&self.learn_fields.goal_title, theme)))
            .child(section_label(tr("learning.goals.why_label"), theme))
            .child(control_row(theme).child(framed_input(&self.learn_fields.goal_why, theme)))
            .child(section_label(tr("learning.goals.target_label"), theme))
            .child(control_row(theme).children(horizons))
            .child(caption(t!("learning.goals.target_on", date = draft.target.strftime(DATE_FORMAT).to_string()).to_string(), theme))
            .children(problem)
            .child(control_row(theme).child(create).child(cancel))
    }
}
