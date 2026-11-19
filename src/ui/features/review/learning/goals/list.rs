//! The goals list: a card per goal (title, why, target, forecast, progress against the time
//! spent, the main numbers), with "New goal" and "Weekly review" above.

use super::super::super::panel::ReviewPanel;
use super::super::super::render::common::{block_title, caption, control_row, surface};
use super::super::super::text::{percent, tr};
use super::super::data::GoalView;
use super::super::state::GoalsScreen;
use crate::domain::learning::Forecast;
use crate::domain::review::day::days_between;
use crate::i18n::t_count;
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use crate::ui::responsive::LayoutClass;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::*;
use rust_i18n::t;

const NEW_ID: &str = "learning-goals-new";
const WEEKLY_ID: &str = "learning-goals-weekly";
const GOAL_ID_PREFIX: &str = "learning-goal-";
/// How a target date is written.
pub const DATE_FORMAT: &str = "%Y-%m-%d";

/// The accent of a forecast.
pub fn forecast_color(forecast: Forecast, theme: &ThemeTokens) -> Rgba {
    match forecast {
        Forecast::Achieved => theme.accent_success,
        Forecast::OnTrack => theme.info,
        Forecast::AtRisk => theme.warning,
        Forecast::Overdue => theme.accent_danger,
    }
}

/// A forecast badge.
pub fn forecast_badge(forecast: Forecast, theme: &ThemeTokens) -> Div {
    let color = forecast_color(forecast, theme);
    div().flex_none().px(theme.spacing_sm).py(theme.spacing_3xs).rounded(theme.radius_full).border(theme.border_width_hairline).border_color(color).text_size(theme.font_size_2xs).font_weight(FontWeight::SEMIBOLD).text_color(color).child(tr(&format!("learning.forecast.{}", forecast.key())))
}

/// Progress (filled) against the time spent (a tick), on one track.
pub fn progress_track(score: f64, expected: f64, color: Rgba, theme: &ThemeTokens) -> Div {
    let tick = div().absolute().top_0().bottom_0().left(relative(expected.clamp(0.0, 1.0) as f32)).w(theme.border_width_threadline).bg(theme.text_muted);
    let fill = div().h_full().w(relative(score.clamp(0.0, 1.0) as f32)).rounded(theme.radius_full).bg(color);
    div().relative().w_full().h(theme.review_progress_height).rounded(theme.radius_full).bg(theme.border_subtle).child(fill).child(tick)
}

impl ReviewPanel {
    pub(super) fn goal_numbers(&self, view: &GoalView) -> String {
        let progress = &view.progress;
        let retention = progress.retention.map(|r| t!("learning.goals.retention", value = percent(r)).to_string());
        let parts = [Some(t_count("learning.goals.items", progress.items)), retention, Some(t_count("learning.goals.posts", progress.new_posts)), Some(t!("learning.goals.tasks", done = progress.tasks_done, total = progress.tasks_total).to_string())];
        parts.into_iter().flatten().collect::<Vec<_>>().join(" · ")
    }

    pub(super) fn target_line(&self, view: &GoalView) -> String {
        let date = view.goal.target.strftime(DATE_FORMAT).to_string();
        match view.goal.target >= self.data.today {
            true => t!("learning.goals.target_in", date = date, days = days_between(self.data.today, view.goal.target)).to_string(),
            false => t!("learning.goals.target_past", date = date).to_string(),
        }
    }

    fn goal_card(&self, index: usize, view: &GoalView, cx: &mut Context<Self>) -> Stateful<Div> {
        let theme = &self.theme;
        let id = view.goal.id.clone();
        let head = div().flex().items_start().gap(theme.control_gap).child(div().flex_1().min_w_0().text_size(theme.font_size_md).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_primary).child(view.goal.title.clone())).child(forecast_badge(view.forecast, theme));
        surface(theme)
            .id(ElementId::NamedInteger(GOAL_ID_PREFIX.into(), index as u64))
            .cursor_pointer()
            .hover(|card| card.bg(theme.bg_elevated))
            .on_click(cx.listener(move |this, _, _, cx| this.show_goals(GoalsScreen::Goal(id.clone()), cx)))
            .child(head)
            .child(caption(view.goal.why.clone(), theme))
            .child(progress_track(view.progress.score, view.expected, forecast_color(view.forecast, theme), theme))
            .child(caption(format!("{} · {}", self.target_line(view), self.goal_numbers(view)), theme))
    }

    pub(super) fn render_goal_list(&self, class: LayoutClass, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &self.theme;
        let new = Button::new(NEW_ID).primary().small().icon(Icon::new(LucideIcon::Plus)).label(tr("learning.goals.new")).on_click(cx.listener(|this, _, _, cx| this.show_goals(GoalsScreen::NewGoal, cx)));
        let weekly = Button::new(WEEKLY_ID).ghost().small().icon(Icon::new(LucideIcon::Calendar)).label(tr("learning.weekly.open")).on_click(cx.listener(|this, _, _, cx| this.open_weekly_review(cx)));
        let goals = &self.data.learning.goals;
        let cards = goals.iter().enumerate().map(|(index, view)| self.goal_card(index, view, cx).min_w(if class.is_compact() { Pixels::ZERO } else { theme.learning_side_width }).flex_1()).collect::<Vec<_>>();
        let empty = goals.is_empty().then(|| caption(tr("learning.goals.empty"), theme));
        div()
            .w_full()
            .flex()
            .flex_col()
            .gap(theme.section_gap)
            .child(control_row(theme).child(block_title(LucideIcon::Target, theme.accent_brand, tr("learning.goals.title"), theme)).child(div().flex_1()).child(weekly).child(new))
            .children(empty)
            .child(div().w_full().flex().flex_wrap().gap(theme.section_gap).children(cards))
    }
}
