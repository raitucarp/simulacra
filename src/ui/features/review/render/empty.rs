//! Illustrated empty states: nothing learnt yet (and the ways to start), everything reviewed
//! for today (with when the next items come due), and a quiz without enough material.

use super::super::events::{PickerMode, ReviewEvent};
use super::super::panel::{ReviewPanel, Screen};
use super::super::text::tr;
use super::common::{caption, control_row};
use crate::domain::review::quiz::QuizKind;
use crate::domain::review::stats::due_forecast;
use crate::i18n::t_count;
use crate::ui::icons::LucideIcon;
use crate::ui::illustration::{EmptyKey, Illustration};
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::*;

/// Empty-state scenes (`assets/illustrations/empty/`, typed as `EmptyKey::ReviewEmpty` and
/// `EmptyKey::ReviewDone`).
pub const REVIEW_EMPTY_ART: &str = "illustrations/empty/review_empty.svg";
pub const REVIEW_DONE_ART: &str = "illustrations/empty/review_done.svg";
const EMPTY_PICK_ID: &str = "review-empty-pick";
const EMPTY_MAP_ID: &str = "review-empty-map";
const DONE_CHOICE_ID: &str = "review-done-choice";
const DONE_ORDER_ID: &str = "review-done-order";
const UNAVAILABLE_BACK_ID: &str = "review-unavailable-back";
/// Days ahead searched for the next due items.
const LOOKAHEAD_DAYS: usize = 365;

impl ReviewPanel {
    fn empty_state(&self, key: EmptyKey, title: String, hint: String) -> Div {
        let theme = &self.theme;
        div()
            .w_full()
            .py(theme.section_gap)
            .flex()
            .flex_col()
            .items_center()
            .gap(theme.stack_gap)
            .child(Illustration::empty(key).width(theme.review_empty_art_width))
            .child(div().font_family(theme.font_family_display.clone()).text_size(theme.font_size_md).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_primary).child(title))
            .child(div().max_w(theme.review_card_max_width).text_center().child(caption(hint, theme)))
    }

    pub(super) fn render_empty_deck(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let pick = Button::new(EMPTY_PICK_ID)
            .primary()
            .small()
            .icon(Icon::new(LucideIcon::GraduationCap))
            .label(tr("review.pick_posts"))
            .on_click(cx.listener(|_, _, _, cx| cx.emit(ReviewEvent::OpenPicker(PickerMode::Learn))));
        let map = Button::new(EMPTY_MAP_ID)
            .ghost()
            .small()
            .icon(Icon::new(LucideIcon::Share2))
            .label(tr("review.new_map"))
            .on_click(cx.listener(|_, _, _, cx| cx.emit(ReviewEvent::OpenPicker(PickerMode::Map))));
        let order = self.quiz_button(DONE_ORDER_ID, QuizKind::Order, cx);
        self.empty_state(EmptyKey::ReviewEmpty, tr("review.empty.title"), tr("review.empty.hint"))
            .child(control_row(&self.theme).justify_center().child(pick).child(map).child(order))
    }

    pub(super) fn render_caught_up(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let forecast = due_forecast(&self.data.items, self.data.today, LOOKAHEAD_DAYS);
        let next = forecast.iter().enumerate().skip(1).find(|(_, count)| **count > 0);
        let hint = match next {
            Some((days, count)) => format!("{} {}", t_count("review.done.next_items", *count), t_count("review.done.next_days", days)),
            None => tr("review.done.nothing_scheduled"),
        };
        let choice = self.quiz_button(DONE_CHOICE_ID, QuizKind::Choice, cx);
        let order = self.quiz_button(DONE_ORDER_ID, QuizKind::Order, cx);
        self.empty_state(EmptyKey::ReviewDone, tr("review.done.title"), hint)
            .child(caption(tr("review.done.practice"), &self.theme))
            .child(control_row(&self.theme).justify_center().child(choice).child(order))
    }

    pub(super) fn render_unavailable(&self, reason: &'static str, cx: &mut Context<Self>) -> impl IntoElement {
        let back = Button::new(UNAVAILABLE_BACK_ID)
            .ghost()
            .small()
            .icon(Icon::new(LucideIcon::ArrowLeft))
            .label(tr("review.back_today"))
            .on_click(cx.listener(|this, _, _, cx| this.show(Screen::Today, cx)));
        self.empty_state(EmptyKey::ReviewEmpty, tr("review.quiz.unavailable_title"), tr(reason)).child(back)
    }
}
