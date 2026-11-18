//! Put it back in order: move the pieces up and down, check, see which sit right.

use super::super::panel::ReviewPanel;
use super::super::text::tr;
use super::common::{block_title, caption, control_row, surface};
use crate::domain::review::quiz::{OrderPuzzle, QuizKind};
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::{Disableable, Icon, Sizable};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use rust_i18n::t;

const UP_ID_PREFIX: &str = "review-order-up-";
const DOWN_ID_PREFIX: &str = "review-order-down-";
const CHECK_ID: &str = "review-order-check";
const ANOTHER_ID: &str = "review-order-another";

impl ReviewPanel {
    pub(super) fn render_order(&self, puzzle: &OrderPuzzle, (solved, rounds): (usize, usize), cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &self.theme;
        let progress = if rounds == 0 { tr("review.quiz.order.title") } else { t!("review.order.progress", solved = solved, rounds = rounds).to_string() };
        let rows: Vec<_> = puzzle.arrangement.iter().enumerate().map(|(slot, piece)| self.order_row(puzzle, slot, *piece, cx)).collect();
        div()
            .w_full()
            .max_w(theme.review_card_max_width)
            .mx_auto()
            .flex()
            .flex_col()
            .gap(theme.section_gap)
            .child(self.quiz_top(progress, if rounds == 0 { 0.0 } else { solved as f64 / rounds as f64 }, cx))
            .child(
                surface(theme)
                    .child(block_title(LucideIcon::ArrowUpDown, theme.accent_dialectic, puzzle.title.clone(), theme))
                    .child(caption(tr("review.order.instructions"), theme))
                    .child(div().flex().flex_col().gap(theme.spacing_xs).children(rows))
                    .child(self.order_footer(puzzle, cx)),
            )
    }

    fn order_row(&self, puzzle: &OrderPuzzle, slot: usize, piece: usize, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &self.theme;
        let last = puzzle.arrangement.len().saturating_sub(1);
        let border = match (puzzle.checked, puzzle.is_in_place(slot)) {
            (false, _) => theme.border,
            (true, true) => theme.accent_success,
            (true, false) => theme.accent_danger,
        };
        let up = Button::new(ElementId::NamedInteger(UP_ID_PREFIX.into(), slot as u64)).ghost().xsmall().icon(Icon::new(LucideIcon::ChevronUp)).disabled(puzzle.checked || slot == 0)
            .on_click(cx.listener(move |this, _, _, cx| this.move_piece(slot, true, cx)));
        let down = Button::new(ElementId::NamedInteger(DOWN_ID_PREFIX.into(), slot as u64)).ghost().xsmall().icon(Icon::new(LucideIcon::ChevronDown)).disabled(puzzle.checked || slot == last)
            .on_click(cx.listener(move |this, _, _, cx| this.move_piece(slot, false, cx)));
        div()
            .flex()
            .items_center()
            .gap(theme.inline_gap)
            .p(theme.spacing_sm)
            .rounded(theme.radius_md)
            .border(theme.border_width_hairline)
            .border_color(border)
            .bg(theme.bg_elevated)
            .child(div().flex_none().w(theme.key_cap_min_width).text_center().font_family(theme.font_family_mono.clone()).text_size(theme.font_size_xs).text_color(theme.text_muted).child((slot + 1).to_string()))
            .child(div().flex_1().min_w_0().text_size(theme.font_size_sm).line_height(relative(theme.line_height_body)).text_color(theme.text_primary).child(puzzle.pieces[piece].clone()))
            .when(puzzle.checked && !puzzle.is_in_place(slot), |row| row.child(caption(t!("review.order.belongs", position = piece + 1).to_string(), theme).flex_none()))
            .child(div().flex().flex_col().child(up).child(down))
    }

    fn order_footer(&self, puzzle: &OrderPuzzle, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &self.theme;
        if !puzzle.checked {
            let check = Button::new(CHECK_ID).primary().icon(Icon::new(LucideIcon::Check)).label(tr("review.order.check")).on_click(cx.listener(|this, _, _, cx| this.check_order(cx)));
            return control_row(theme).child(check);
        }
        let score = puzzle.score();
        let key = if score.is_perfect() { "review.order.perfect" } else { "review.order.score" };
        let summary = t!(key, in_place = score.in_place, total = score.total, pairs = score.pairs_in_order).to_string();
        let another = Button::new(ANOTHER_ID).primary().small().icon(Icon::new(LucideIcon::RotateCcw)).label(tr("review.order.another")).on_click(cx.listener(|this, _, _, cx| this.request_quiz(QuizKind::Order, cx)));
        control_row(theme).child(caption(summary, theme)).child(div().flex_1()).child(another)
    }
}
