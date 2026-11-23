//! N4: the closing card of a finite feed. Once the day's budget is spent the stream ends
//! here and asks what the reader learned; the answer goes out as a one-line post (Enter or
//! "Post"), and the card then shows it for the rest of the day.

use crate::domain::PostId;
use crate::i18n::t_count;
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::input::{Input, InputState};
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::*;
use rust_i18n::t;
use std::rc::Rc;

const END_CARD_ID: &str = "feed-end-card";
const POST_ID: &str = "feed-end-post";
const ANSWER_ID: &str = "feed-end-answer";

/// What the closing card needs from the shell.
#[derive(Clone)]
pub struct EndCard {
    /// The one-line answer field.
    pub input: Entity<InputState>,
    /// Today's answer, once posted: its id and its text.
    pub answer: Option<(PostId, String)>,
    pub on_answer: Rc<dyn Fn(&mut Window, &mut App) + 'static>,
    pub on_open: Rc<dyn Fn(PostId, &mut Window, &mut App) + 'static>,
}

/// The answer field and "Post", or today's answer (opening its post).
fn answer_row(end: &EndCard, theme: &ThemeTokens) -> AnyElement {
    if let Some((id, text)) = end.answer.clone() {
        let (on_open, hover) = (end.on_open.clone(), theme.hover_bg);
        let line = div().text_size(theme.font_size_sm).text_color(theme.text_primary).child(t!("feed.end.answered", answer = text).to_string());
        return div().id(ANSWER_ID).px(theme.spacing_sm).py(theme.spacing_xs).rounded(theme.radius_sm).cursor_pointer().hover(move |row| row.bg(hover)).on_click(move |_, window, cx| on_open(id.clone(), window, cx)).child(line).into_any_element();
    }
    let on_answer = end.on_answer.clone();
    let post = Button::new(POST_ID).primary().small().icon(Icon::new(LucideIcon::Send)).label(t!("feed.end.post").to_string()).on_click(move |_, window, cx| on_answer(window, cx));
    div().w_full().max_w(theme.reader_measure_width).flex().items_center().gap(theme.control_gap).child(div().flex_1().min_w_0().child(Input::new(&end.input))).child(post).into_any_element()
}

/// The card closing a feed whose `budget` posts were all read today.
pub(super) fn render_end_card(budget: u32, end: Option<&EndCard>, theme: &ThemeTokens) -> AnyElement {
    let icon = div().size(theme.icon_size_2xl).text_color(theme.accent_brand).child(Icon::new(LucideIcon::Moon));
    let title = div().font_family(theme.font_family_display.clone()).text_size(theme.font_size_lg).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_primary).child(t!("feed.end.title").to_string());
    let read = div().text_size(theme.font_size_sm).text_color(theme.text_muted).child(t_count("feed.end.read", budget as usize));
    let question = div().text_size(theme.font_size_md).text_color(theme.text_secondary).child(t!("feed.end.question").to_string());
    div()
        .id(END_CARD_ID)
        .w_full()
        .px(theme.panel_padding)
        .py(theme.section_gap)
        .flex()
        .flex_col()
        .items_center()
        .gap(theme.spacing_sm)
        .border_t(theme.border_width_hairline)
        .border_color(theme.border_subtle)
        .bg(theme.bg_callout)
        .child(icon)
        .child(title)
        .child(read)
        .child(question)
        .children(end.map(|end| answer_row(end, theme)))
        .into_any_element()
}
