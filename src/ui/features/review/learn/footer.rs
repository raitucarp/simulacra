//! The learn sheet's footer: the AI suggestion and its status, "Add another passage" (for a
//! document) and "Save".

use super::request::LearnSource;
use super::{AiStatus, LearnSheet};
use crate::ui::features::review::text::tr;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::{Disableable, Icon, Sizable};
use gpui_kit::*;

const AI_ID: &str = "learn-ai";
const ADD_ID: &str = "learn-add";
const SAVE_ID: &str = "learn-save";

impl LearnSheet {
    pub(super) fn render_footer(&self, source: &LearnSource, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &self.theme;
        let waiting = self.ai == AiStatus::Waiting;
        let status = match &self.ai {
            AiStatus::Idle => None,
            AiStatus::Waiting => Some((tr("review.learn.ai_waiting"), theme.text_muted)),
            AiStatus::Suggested => Some((tr("review.learn.ai_done"), theme.accent_success)),
            AiStatus::Failed(message) => Some((message.clone(), theme.accent_danger)),
        };
        let ai = Button::new(AI_ID).ghost().small().icon(Icon::new(LucideIcon::WandSparkles)).label(tr("review.learn.ai")).loading(waiting).disabled(waiting)
            .on_click(cx.listener(|this, _, _, cx| this.request_ai(cx)));
        let add = matches!(source, LearnSource::Document { .. }).then(|| {
            Button::new(ADD_ID).ghost().small().icon(Icon::new(LucideIcon::Plus)).label(tr("review.learn.add")).on_click(cx.listener(|this, _, window, cx| {
                this.add_current(window, cx);
            }))
        });
        let save = Button::new(SAVE_ID).primary().small().icon(Icon::new(LucideIcon::Check)).label(tr("review.learn.save")).on_click(cx.listener(|this, _, window, cx| this.save(window, cx)));
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
            .children(add)
            .child(save)
    }
}
