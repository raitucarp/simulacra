//! A goal's detail: its forecast and progress, the measured numbers as tiles, its tasks
//! (tick, add) and its linked material, with "Mark reached", "Gap radar" and "Delete".

use super::super::super::panel::ReviewPanel;
use super::super::super::render::common::{block_title, caption, control_row, count_tile, surface};
use super::super::super::text::{percent, tr};
use super::super::data::GoalView;
use super::super::render::{framed_input, section_label};
use super::super::state::GoalsScreen;
use super::super::tutor::ladder::level_label;
use super::list::{forecast_badge, forecast_color, progress_track};
use crate::domain::learning::Goal;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::*;
use rust_i18n::t;

const BACK_ID: &str = "learning-goal-back";
const REACHED_ID: &str = "learning-goal-reached";
const RADAR_ID: &str = "learning-goal-radar";
const DELETE_ID: &str = "learning-goal-delete";
const TASK_ID_PREFIX: &str = "learning-goal-task-";
const NONE: &str = "–";

impl ReviewPanel {
    fn goal_tiles(&self, view: &GoalView) -> Div {
        let theme = &self.theme;
        let p = &view.progress;
        let tiles = [
            (p.items.to_string(), tr("learning.goals.tile.items"), theme.accent_brand),
            (p.retention.map_or_else(|| NONE.to_string(), percent), tr("learning.goals.tile.retention"), theme.accent_success),
            (p.new_posts.to_string(), tr("learning.goals.tile.posts"), theme.info),
            (p.sessions.to_string(), tr("learning.goals.tile.sessions"), theme.accent_dialectic),
            (p.best_level.map_or_else(|| NONE.to_string(), level_label), tr("learning.goals.tile.level"), theme.accent_dialectic),
            (percent(p.score), tr("learning.goals.tile.score"), forecast_color(view.forecast, theme)),
        ];
        control_row(theme).children(tiles.into_iter().map(|(value, label, color)| count_tile(value, label, color, theme)))
    }

    fn goal_tasks(&self, goal: &Goal, cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        let rows = goal.tasks.iter().enumerate().map(|(index, task)| {
            let id = task.id.clone();
            let (icon, color) = if task.done { (LucideIcon::CircleCheck, theme.accent_success) } else { (LucideIcon::Minus, theme.text_muted) };
            div()
                .id(ElementId::NamedInteger(TASK_ID_PREFIX.into(), index as u64))
                .flex()
                .items_center()
                .gap(theme.inline_gap)
                .cursor_pointer()
                .on_click(cx.listener(move |this, _, _, cx| this.toggle_goal_task(id.clone(), cx)))
                .child(Icon::new(icon).size(theme.icon_size_sm).text_color(color))
                .child(div().flex_1().min_w_0().text_size(theme.font_size_sm).text_color(if task.done { theme.text_muted } else { theme.text_primary }).child(task.text.clone()))
        });
        let empty = goal.tasks.is_empty().then(|| caption(tr("learning.goals.no_tasks"), theme));
        surface(theme)
            .child(block_title(LucideIcon::ListChecks, theme.accent_success, tr("learning.goals.tasks_title"), theme))
            .children(empty)
            .children(rows)
            .child(control_row(theme).child(framed_input(&self.learn_fields.task, theme)))
    }

    fn goal_actions(&self, goal: &Goal, cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        let back = Button::new(BACK_ID).ghost().xsmall().icon(Icon::new(LucideIcon::ArrowLeft)).label(tr("learning.goals.all")).on_click(cx.listener(|this, _, _, cx| this.show_goals(GoalsScreen::List, cx)));
        let (id, completed) = (goal.id.clone(), goal.completed);
        let reached = Button::new(REACHED_ID).ghost().xsmall().icon(Icon::new(LucideIcon::Trophy)).label(tr(if completed { "learning.goals.reopen" } else { "learning.goals.reached" }))
            .on_click(cx.listener(move |this, _, _, cx| this.change_goal(|goal| Goal { completed: !completed, ..goal }, cx)));
        let radar_id = goal.id.clone();
        let radar = Button::new(RADAR_ID).ghost().xsmall().icon(Icon::new(LucideIcon::Gauge)).label(tr("learning.goals.radar")).on_click(cx.listener(move |this, _, _, cx| this.open_gaps_for(Some(radar_id.clone()), cx)));
        let delete = Button::new(DELETE_ID).ghost().xsmall().icon(Icon::new(LucideIcon::Trash)).tooltip(tr("learning.goals.delete")).on_click(cx.listener(move |this, _, _, cx| this.delete_goal(id.clone(), cx)));
        control_row(theme).child(back).child(div().flex_1()).child(radar).child(reached).child(delete)
    }

    pub(super) fn render_goal_detail(&self, view: &GoalView, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &self.theme;
        let goal = &view.goal;
        let expected = t!("learning.goals.expected", done = percent(view.progress.score), expected = percent(view.expected)).to_string();
        let head = surface(theme)
            .child(self.goal_actions(goal, cx))
            .child(div().flex().items_start().gap(theme.control_gap).child(div().flex_1().min_w_0().font_family(theme.font_family_display.clone()).text_size(theme.font_size_lg).font_weight(FontWeight::BOLD).text_color(theme.text_primary).child(goal.title.clone())).child(forecast_badge(view.forecast, theme)))
            .child(caption(goal.why.clone(), theme))
            .child(progress_track(view.progress.score, view.expected, forecast_color(view.forecast, theme), theme))
            .child(caption(format!("{} · {expected}", self.target_line(view)), theme));
        div()
            .w_full()
            .max_w(theme.review_card_max_width)
            .flex()
            .flex_col()
            .gap(theme.section_gap)
            .child(head)
            .child(section_label(tr("learning.goals.measured"), theme))
            .child(self.goal_tiles(view))
            .child(self.goal_tasks(goal, cx))
            .child(self.goal_links(goal, cx))
    }
}
