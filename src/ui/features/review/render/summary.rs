//! After a session: how many cards, first-try recall, the grade spread, and what comes next.

use super::super::panel::{ReviewPanel, Screen};
use super::super::text::{grade_label, percent, tr};
use super::common::{block_title, caption, control_row, count_tile, legend_item, surface};
use super::grades::grade_color;
use crate::domain::review::quiz::QuizKind;
use crate::domain::review::stats::due_forecast;
use crate::domain::review::Grade;
use crate::i18n::t_count;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::*;

const BACK_ID: &str = "review-summary-back";
const STATS_ID: &str = "review-summary-stats";
const QUIZ_ID: &str = "review-summary-quiz";
/// Forecast days needed to read tomorrow's count.
const TOMORROW_WINDOW: usize = 2;

impl ReviewPanel {
    pub(super) fn render_summary(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &self.theme;
        let session = &self.session;
        let recall = session.first_try_recall().map_or_else(|| tr("review.none"), percent);
        let counts = session.grade_counts();
        let tiles = control_row(theme)
            .child(count_tile(session.done().to_string(), tr("review.summary.cards"), theme.accent_brand, theme))
            .child(count_tile(recall, tr("review.summary.first_try"), theme.accent_success, theme))
            .child(count_tile(counts[Grade::Again.index()].to_string(), tr("review.summary.again"), theme.accent_danger, theme));
        let tomorrow = due_forecast(&self.data.cards, self.data.today, TOMORROW_WINDOW)[1];
        let back = Button::new(BACK_ID).primary().small().icon(Icon::new(LucideIcon::ArrowLeft)).label(tr("review.back_today")).on_click(cx.listener(|this, _, _, cx| this.show(Screen::Today, cx)));
        let stats = Button::new(STATS_ID).ghost().small().icon(Icon::new(LucideIcon::ChartColumn)).label(tr("review.tab.stats")).on_click(cx.listener(|this, _, _, cx| this.show(Screen::Stats, cx)));
        surface(theme)
            .child(block_title(LucideIcon::Trophy, theme.accent_dialectic, tr("review.summary.title"), theme))
            .child(tiles)
            .child(self.grade_spread(counts))
            .child(caption(t_count("review.summary.tomorrow", tomorrow), theme))
            .child(caption(tr("review.summary.tip"), theme))
            .child(control_row(theme).child(back).child(stats).child(self.quiz_button(QUIZ_ID, QuizKind::Recall, cx)))
    }

    /// One bar split by grade, with a legend.
    fn grade_spread(&self, counts: [usize; 4]) -> impl IntoElement {
        let theme = &self.theme;
        let total = counts.iter().sum::<usize>().max(1) as f32;
        let segments = Grade::ALL.into_iter().filter(|g| counts[g.index()] > 0).map(|grade| {
            div().h_full().w(relative(counts[grade.index()] as f32 / total)).bg(grade_color(grade, theme))
        });
        let legend = Grade::ALL.into_iter().map(|grade| legend_item(grade_color(grade, theme), format!("{} {}", grade_label(grade), counts[grade.index()]), theme));
        div()
            .flex()
            .flex_col()
            .gap(theme.spacing_xs)
            .child(div().w_full().h(theme.review_swatch_size).flex().rounded(theme.radius_full).overflow_hidden().bg(theme.border_subtle).children(segments))
            .child(control_row(theme).children(legend))
    }
}
