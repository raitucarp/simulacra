//! The learn sheet's frame: a scrim, the elevated sheet with its header (what is being
//! learnt), the body and the footer.

use super::request::LearnSource;
use super::LearnSheet;
use crate::ui::atoms::ElevationExt;
use crate::ui::features::review::text::tr;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::*;

const BACKDROP_ID: &str = "learn-backdrop";
const SHEET_ID: &str = "learn-sheet";
const BODY_ID: &str = "learn-body";
const CLOSE_ID: &str = "learn-close";
const ESCAPE_KEY: &str = "escape";

impl Render for LearnSheet {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let Some(request) = self.request.clone() else { return div().into_any_element() };
        let theme = self.theme.clone();
        let sheet = self.render_sheet(&request.source, request.existing.is_some(), cx);
        div()
            .id(BACKDROP_ID)
            .track_focus(&self.focus)
            .on_key_down(cx.listener(|this, event: &KeyDownEvent, _, cx| {
                if event.keystroke.key == ESCAPE_KEY {
                    this.close(cx);
                }
            }))
            .absolute()
            .inset_0()
            .occlude()
            .bg(theme.overlay)
            .flex()
            .flex_col()
            .items_center()
            .px(theme.panel_padding)
            .pt(theme.palette_top_offset)
            .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, cx| this.close(cx)))
            .child(sheet)
            .into_any_element()
    }
}

impl LearnSheet {
    /// The elevated sheet (clicks inside never reach the scrim).
    fn render_sheet(&self, source: &LearnSource, editing: bool, cx: &mut Context<Self>) -> Stateful<Div> {
        let theme = self.theme.clone();
        let body = div().id(BODY_ID).flex_1().min_h_0().overflow_y_scroll().p(theme.card_padding).flex().flex_col().gap(theme.stack_gap).child(self.render_body(source, cx));
        div()
            .id(SHEET_ID)
            .w(theme.review_learn_sheet_width)
            .max_w_full()
            .max_h(theme.review_maker_max_height)
            .flex()
            .flex_col()
            .overflow_hidden()
            .rounded(theme.radius_lg)
            .bg(theme.bg_elevated)
            .border(theme.border_width_hairline)
            .border_color(theme.border)
            .elevation_shadow(&theme)
            .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
            .child(self.render_top(source, editing, cx))
            .child(body)
            .child(self.render_footer(source, cx))
    }

    fn render_top(&self, source: &LearnSource, editing: bool, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &self.theme;
        let title = match (editing, source) {
            (true, _) => tr("review.learn.edit_title"),
            (false, LearnSource::Post(_)) => tr("review.learn.post_title"),
            (false, LearnSource::Document { .. }) => tr("review.learn.passages_title"),
        };
        let close = Button::new(CLOSE_ID).ghost().xsmall().icon(Icon::new(LucideIcon::X)).tooltip(tr("review.learn.close")).on_click(cx.listener(|this, _, _, cx| this.close(cx)));
        div()
            .flex()
            .items_center()
            .gap(theme.inline_gap)
            .px(theme.card_padding)
            .py(theme.spacing_sm)
            .border_b(theme.border_width_hairline)
            .border_color(theme.border_subtle)
            .child(Icon::new(LucideIcon::GraduationCap).size(theme.icon_size_sm).text_color(theme.accent_brand_text))
            .child(div().flex_1().min_w_0().text_size(theme.font_size_sm).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_primary).child(title))
            .child(close)
    }
}
