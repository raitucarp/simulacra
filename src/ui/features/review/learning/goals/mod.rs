//! The goals tool: the goals with their progress and forecast, a goal's detail (metrics,
//! tasks, linked material), the new-goal form and the weekly review. Every change is a pure
//! edit of the goal, handed to the shell whole to be saved.

mod detail;
mod form;
mod links;
mod list;
mod weekly;
mod weekly_parts;

use super::super::events::{PickerMode, ReviewEvent};
use super::super::panel::ReviewPanel;
use super::events::LearningEvent;
use super::state::GoalsScreen;
use super::tutor::take_text;
use crate::domain::learning::goals::edit::{toggled_task, with_link, with_tasks};
use crate::domain::learning::{Goal, GoalId, GoalLink};
use crate::ui::responsive::LayoutClass;
use gpui_kit::{AnyElement, Context, IntoElement, Window};

impl ReviewPanel {
    /// The goals tool's current screen.
    pub(in crate::ui::features::review) fn render_goals(&self, class: LayoutClass, cx: &mut Context<Self>) -> AnyElement {
        match &self.learning.goals {
            GoalsScreen::List => self.render_goal_list(class, cx).into_any_element(),
            GoalsScreen::NewGoal => self.render_goal_form(cx).into_any_element(),
            GoalsScreen::Weekly => self.render_weekly(class, cx).into_any_element(),
            GoalsScreen::Goal(id) => match self.data.learning.goal(id) {
                Some(view) => self.render_goal_detail(view, cx).into_any_element(),
                None => self.render_goal_list(class, cx).into_any_element(),
            },
        }
    }

    /// The goal shown in the detail screen, if any.
    pub(in crate::ui::features::review) fn open_goal(&self) -> Option<&Goal> {
        match &self.learning.goals {
            GoalsScreen::Goal(id) => self.data.learning.goal(id).map(|view| &view.goal),
            _ => None,
        }
    }

    pub(in crate::ui::features::review) fn show_goals(&mut self, screen: GoalsScreen, cx: &mut Context<Self>) {
        self.learning.tool = super::LearnTool::Goals;
        self.learning.goals = screen;
        self.learning.goal_tried = false;
        self.show(super::super::panel::Screen::Learn, cx);
    }

    /// Opens the weekly review (the reminder's destination).
    pub fn open_weekly_review(&mut self, cx: &mut Context<Self>) {
        self.learning.weekly_unticked.clear();
        self.show_goals(GoalsScreen::Weekly, cx);
    }

    /// Saves the open goal changed by `change`.
    pub(in crate::ui::features::review) fn change_goal(&mut self, change: impl FnOnce(Goal) -> Goal, cx: &mut Context<Self>) {
        if let Some(goal) = self.open_goal().cloned() {
            self.emit_learning(LearningEvent::SaveGoal(change(goal)), cx);
        }
    }

    /// Enter in the concept field: links the typed concept to the open goal.
    pub fn add_goal_concept(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let name = take_text(&self.learn_fields.goal_concept, window, cx);
        if !name.is_empty() {
            self.change_goal(|goal| with_link(goal, GoalLink::concept(&name)), cx);
        }
    }

    /// Enter in the task field: adds the typed task to the open goal.
    pub fn add_goal_task(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let text = take_text(&self.learn_fields.task, window, cx);
        let today = self.data.today;
        if !text.is_empty() {
            self.change_goal(|goal| with_tasks(goal, &[text], today), cx);
        }
    }

    pub(in crate::ui::features::review) fn toggle_goal_task(&mut self, task: String, cx: &mut Context<Self>) {
        let today = self.data.today;
        self.change_goal(|goal| toggled_task(goal, &task, today), cx);
    }

    /// Opens the post picker to link a post or a thread to the open goal.
    pub(in crate::ui::features::review) fn pick_goal_post(&mut self, cx: &mut Context<Self>) {
        cx.emit(ReviewEvent::OpenPicker(PickerMode::Link));
    }

    /// Links `link` to the open goal and returns to it (from the picker).
    pub(in crate::ui::features::review) fn link_to_goal(&mut self, link: GoalLink, cx: &mut Context<Self>) {
        self.change_goal(|goal| with_link(goal, link), cx);
        self.show(super::super::panel::Screen::Learn, cx);
    }

    pub(in crate::ui::features::review) fn delete_goal(&mut self, id: GoalId, cx: &mut Context<Self>) {
        self.learning.goals = GoalsScreen::List;
        self.emit_learning(LearningEvent::DeleteGoal(id), cx);
    }
}
