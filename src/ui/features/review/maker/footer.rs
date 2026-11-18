//! The card maker's footer: the AI draft and its status, "Add another" and "Save cards".

use super::{AiStatus, FlashcardMaker};
use crate::ui::features::review::text::tr;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::{Disableable, Icon, Sizable};
use gpui_kit::*;
use rust_i18n::t;

const AI_ID: &str = "maker-ai";
const ADD_ID: &str = "maker-add";
const SAVE_ID: &str = "maker-save";

impl FlashcardMaker {
    pub(super) fn render_footer(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &self.theme;
        let waiting = self.ai == AiStatus::Waiting;
        let status = match &self.ai {
            AiStatus::Idle => None,
            AiStatus::Waiting => Some((tr("review.maker.ai_waiting"), theme.text_muted)),
            AiStatus::Added(count) => Some((t!("review.maker.ai_added", count = count).to_string(), theme.accent_success)),
            AiStatus::Failed(message) => Some((message.clone(), theme.accent_danger)),
        };
        let ai = Button::new(AI_ID).ghost().small().icon(Icon::new(LucideIcon::WandSparkles)).label(tr("review.maker.ai")).loading(waiting).disabled(waiting).on_click(cx.listener(|this, _, _, cx| this.request_ai(cx)));
        let add = Button::new(ADD_ID).ghost().small().icon(Icon::new(LucideIcon::Plus)).label(tr("review.maker.add")).on_click(cx.listener(|this, _, window, cx| {
            this.add_current(window, cx);
        }));
        let save = Button::new(SAVE_ID).primary().small().icon(Icon::new(LucideIcon::Check)).label(tr("review.maker.save")).on_click(cx.listener(|this, _, window, cx| this.save(window, cx)));
        let status = status.map(|(text, color)| div().flex_1().min_w_0().truncate().text_size(theme.font_size_2xs).text_color(color).child(text));
        div()
            .flex()
            .items_center()
            .gap(theme.control_gap)
            .px(theme.card_padding)
            .py(theme.spacing_sm)
            .border_t(theme.border_width_hairline)
            .border_color(theme.border_subtle)
            .child(ai)
            .children(status)
            .child(div().flex_1())
            .child(add)
            .child(save)
    }
}
