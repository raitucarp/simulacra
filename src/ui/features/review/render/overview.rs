//! The "Today" screen: today's counts within the limits, the start button, and the quizzes.

use super::super::panel::ReviewPanel;
use super::super::text::{percent, tr};
use super::common::{block_title, caption, control_row, count_tile, surface};
use crate::domain::review::quiz::QuizKind;
use crate::ui::icons::LucideIcon;
use crate::ui::keymap::render_key_cap;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::{Disableable, Icon, Sizable};
use gpui_kit::*;
use rust_i18n::t;

const START_ID: &str = "review-start";
const MODE_ID_PREFIX: &str = "review-mode-";

fn mode_icon(kind: QuizKind) -> LucideIcon {
    match kind {
        QuizKind::Choice => LucideIcon::ListOrdered,
        QuizKind::Recall => LucideIcon::TextCursorInput,
        QuizKind::Order => LucideIcon::ArrowUpDown,
    }
}

impl ReviewPanel {
    pub(super) fn render_overview(&self, cx: &mut Context<Self>) -> impl IntoElement {
        if self.data.cards.is_empty() {
            return div().child(self.render_empty_deck(cx));
        }
        let today = if self.data.plan().queued() == 0 { self.render_caught_up(cx).into_any_element() } else { self.render_today(cx).into_any_element() };
        div().flex().flex_col().gap(self.theme.section_gap).child(today).child(self.render_modes(cx))
    }

    fn render_today(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &self.theme;
        let (plan, limits) = (self.data.plan(), self.data.limits);
        let tiles = control_row(theme)
            .child(count_tile(plan.queued_due.to_string(), tr("review.today.due"), theme.accent_brand, theme))
            .child(count_tile(plan.queued_new.to_string(), tr("review.today.new"), theme.info, theme))
            .child(count_tile(plan.answered.to_string(), tr("review.today.done"), theme.accent_success, theme));
        let retention = percent(self.data.policy.desired_retention);
        let limits = t!("review.today.limits", reviews = limits.reviews, new = limits.new_cards, retention = retention).to_string();
        let start = Button::new(START_ID)
            .primary()
            .icon(Icon::new(LucideIcon::Play))
            .label(t!("review.today.start", count = plan.queued()).to_string())
            .disabled(plan.queued() == 0)
            .on_click(cx.listener(|this, _, window, cx| this.start_session(window, cx)));
        surface(theme)
            .child(block_title(LucideIcon::Calendar, theme.accent_brand, tr("review.today.title"), theme))
            .child(tiles)
            .child(caption(limits, theme))
            .child(control_row(theme).child(start).child(caption(tr("review.today.keys_hint"), theme)).child(render_key_cap(tr("keymap.key.space"), theme)))
    }

    fn render_modes(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &self.theme;
        let tiles = QuizKind::ALL.into_iter().map(|kind| {
            let (title, body) = (tr(&format!("review.quiz.{}.title", kind.key())), tr(&format!("review.quiz.{}.description", kind.key())));
            div()
                .id(ElementId::Name(format!("{MODE_ID_PREFIX}{}", kind.key()).into()))
                .flex_1()
                .min_w(theme.review_count_tile_width)
                .flex()
                .flex_col()
                .gap(theme.spacing_xs)
                .p(theme.card_padding)
                .rounded(theme.radius_md)
                .bg(theme.bg_elevated)
                .border(theme.border_width_hairline)
                .border_color(theme.border_subtle)
                .cursor_pointer()
                .hover(|tile| tile.border_color(theme.accent_brand_border))
                .on_click(cx.listener(move |this, _, _, cx| this.request_quiz(kind, cx)))
                .child(div().flex().items_center().gap(theme.inline_gap).child(Icon::new(mode_icon(kind)).size(theme.icon_size_sm).text_color(theme.accent_dialectic)).child(div().text_size(theme.font_size_sm).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_primary).child(title)))
                .child(caption(body, theme))
        });
        surface(theme)
            .child(block_title(LucideIcon::Lightbulb, theme.accent_dialectic, tr("review.quiz.title"), theme))
            .child(caption(tr("review.quiz.why"), theme))
            .child(control_row(theme).items_start().children(tiles))
    }

    /// The small "Start" control used by the empty states and the summary.
    pub(super) fn quiz_button(&self, id: &'static str, kind: QuizKind, cx: &mut Context<Self>) -> Button {
        Button::new(id)
            .ghost()
            .small()
            .icon(Icon::new(mode_icon(kind)))
            .label(tr(&format!("review.quiz.{}.title", kind.key())))
            .on_click(cx.listener(move |this, _, _, cx| this.request_quiz(kind, cx)))
    }
}
