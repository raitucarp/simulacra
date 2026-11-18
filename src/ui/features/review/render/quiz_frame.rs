//! What every quiz shares: the progress row with "Finish", and the closing screen with the
//! score, the confidence calibration and a way to go again.

use super::super::panel::{ReviewPanel, Screen};
use super::super::text::{percent, tr};
use super::common::{block_title, caption, control_row, progress_bar, surface};
use crate::domain::review::quiz::QuizKind;
use crate::domain::review::stats::calibration;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::*;
use rust_i18n::t;

const FINISH_ID: &str = "review-quiz-finish";
const AGAIN_ID: &str = "review-quiz-again";
const BACK_ID: &str = "review-quiz-back";

impl ReviewPanel {
    /// "Question 3 of 10", the progress bar and "Finish".
    pub(super) fn quiz_top(&self, progress: String, share: f64, cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        let finish = Button::new(FINISH_ID).ghost().xsmall().icon(Icon::new(LucideIcon::X)).label(tr("review.quiz.finish")).on_click(cx.listener(|this, _, _, cx| this.show(Screen::Today, cx)));
        div()
            .flex()
            .items_center()
            .gap(theme.control_gap)
            .child(caption(progress, theme).flex_none())
            .child(div().flex_1().child(progress_bar(share, theme.accent_dialectic, theme)))
            .child(finish)
    }

    /// How sure answers were right, as one caption (empty before any rated answer).
    fn calibration_caption(&self) -> Option<String> {
        let rows: Vec<String> = calibration(&self.data.log)
            .into_iter()
            .filter_map(|row| row.accuracy().map(|share| t!("review.calibration.row", confidence = tr(&format!("review.confidence.{}", row.confidence.key())), share = percent(share)).to_string()))
            .collect();
        (!rows.is_empty()).then(|| t!("review.calibration.summary", rows = rows.join(" · ")).to_string())
    }

    /// The end of a quiz of `kind`.
    pub(super) fn render_quiz_done(&self, kind: QuizKind, score: String, cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        let again = Button::new(AGAIN_ID).primary().small().icon(Icon::new(LucideIcon::RotateCcw)).label(tr("review.quiz.again")).on_click(cx.listener(move |this, _, _, cx| this.request_quiz(kind, cx)));
        let back = Button::new(BACK_ID).ghost().small().icon(Icon::new(LucideIcon::ArrowLeft)).label(tr("review.back_today")).on_click(cx.listener(|this, _, _, cx| this.show(Screen::Today, cx)));
        surface(theme)
            .child(block_title(LucideIcon::Trophy, theme.accent_dialectic, tr(&format!("review.quiz.{}.title", kind.key())), theme))
            .child(div().font_family(theme.font_family_display.clone()).text_size(theme.font_size_lg).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_primary).child(score))
            .children(self.calibration_caption().filter(|_| kind == QuizKind::Choice).map(|text| caption(text, theme)))
            .child(caption(tr(&format!("review.quiz.{}.done_tip", kind.key())), theme))
            .child(control_row(theme).child(again).child(back))
    }
}
