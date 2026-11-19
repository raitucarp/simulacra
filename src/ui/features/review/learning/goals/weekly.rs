//! The weekly review: a guided ritual in four steps. Look back (what was recalled, written
//! and practised this week), look ahead (what comes due), look at the gaps (the weakest
//! concepts), then choose next actions, which become tasks of a goal.

use super::super::super::panel::ReviewPanel;
use super::super::super::render::common::{caption, control_row, count_tile, surface};
use super::super::super::text::{percent, tr};
use super::super::state::GoalsScreen;
use super::list::DATE_FORMAT;
use crate::domain::learning::weekly::{suggest_actions, NextAction, WeekWindow};
use crate::ui::icons::LucideIcon;
use crate::ui::responsive::LayoutClass;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::*;
use rust_i18n::t;

const BACK_ID: &str = "learning-weekly-back";
/// Weak concepts shown in the gaps step.
pub const WEEKLY_GAPS: usize = 3;
const NONE: &str = "–";

/// The label of a suggested action (also the text of the task it becomes).
pub fn action_label(action: &NextAction) -> String {
    let key = format!("learning.weekly.action.{}", action.key());
    match action {
        NextAction::ReviewDue { count } => t!(key.as_str(), count = count).to_string(),
        _ => t!(key.as_str(), concept = action.concept().unwrap_or_default()).to_string(),
    }
}

impl ReviewPanel {
    /// The week's suggested actions.
    pub(super) fn weekly_actions(&self) -> Vec<NextAction> {
        suggest_actions(self.data.learning.digest.due_week, self.data.learning.gaps.weakest(WEEKLY_GAPS))
    }

    fn weekly_look_back(&self) -> Div {
        let theme = &self.theme;
        let digest = &self.data.learning.digest;
        let tiles = [
            (digest.reviews.to_string(), tr("learning.weekly.tile.reviews"), theme.accent_brand),
            (digest.recall_share.map_or_else(|| NONE.to_string(), percent), tr("learning.weekly.tile.recall"), theme.accent_success),
            (digest.items_started.to_string(), tr("learning.weekly.tile.started"), theme.info),
            (digest.posts_written.to_string(), tr("learning.weekly.tile.posts"), theme.accent_dialectic),
            (digest.tutor_sessions.to_string(), tr("learning.weekly.tile.sessions"), theme.warning),
        ];
        surface(theme)
            .child(self.step_title(1, LucideIcon::RotateCcw, "learning.weekly.look_back"))
            .child(control_row(theme).children(tiles.into_iter().map(|(value, label, color)| count_tile(value, label, color, theme))))
            .child(self.weekly_learnt())
    }

    pub(super) fn render_weekly(&self, class: LayoutClass, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &self.theme;
        let window = WeekWindow::ending(self.data.today);
        let range = t!("learning.weekly.range", from = window.start.strftime(DATE_FORMAT).to_string(), to = window.end.strftime(DATE_FORMAT).to_string()).to_string();
        let back = Button::new(BACK_ID).ghost().xsmall().icon(Icon::new(LucideIcon::ArrowLeft)).label(tr("learning.goals.all")).on_click(cx.listener(|this, _, _, cx| this.show_goals(GoalsScreen::List, cx)));
        let head = surface(theme)
            .child(control_row(theme).child(back))
            .child(div().font_family(theme.font_family_display.clone()).text_size(theme.font_size_lg).font_weight(FontWeight::BOLD).text_color(theme.text_primary).child(tr("learning.weekly.title")))
            .child(caption(format!("{range} · {}", tr("learning.weekly.intro")), theme));
        let columns = div().w_full().flex().gap(theme.section_gap).flex_wrap().child(div().flex_1().min_w(if class.is_compact() { Pixels::ZERO } else { theme.learning_side_width }).child(self.weekly_look_ahead(cx))).child(div().flex_1().min_w(if class.is_compact() { Pixels::ZERO } else { theme.learning_side_width }).child(self.weekly_gaps(cx)));
        div()
            .w_full()
            .flex()
            .flex_col()
            .gap(theme.section_gap)
            .child(head)
            .child(self.weekly_look_back())
            .child(columns)
            .child(self.weekly_next(cx))
    }
}
