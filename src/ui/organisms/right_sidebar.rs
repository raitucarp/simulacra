//! Right sidebar organism containing vector search, knowledge grounding, and active agent roster.

use std::rc::Rc;

use crate::state::AppState;
use crate::theme::ThemeTokens;
use crate::ui::molecules::{render_knowledge_card, render_persona_roster, render_topic_item};
use gpui_kit::base::StyledExt;
use gpui_kit::component::input::{Input, InputState};
use gpui_kit::component::scroll::ScrollableElement;
use gpui_kit::*;
use rust_i18n::t;

pub fn render_right_sidebar(
    state: &AppState,
    search_input: &Entity<InputState>,
    theme: &ThemeTokens,
    on_click_topic: impl Fn(&str, &mut Window, &mut App) + 'static + Clone,
) -> impl IntoElement {
    let on_topic = Rc::new(on_click_topic);

    let mut trending_elements = Vec::new();
    for (idx, topic) in state.trending.iter().enumerate() {
        let cb = on_topic.clone();
        trending_elements.push(
            render_topic_item(topic, idx + 1, theme, move |tag, w, cx| {
                cb(tag, w, cx);
            })
            .into_any_element(),
        );
    }

    div()
        .id("right-sidebar-scroll-container")
        .w_full()
        .h_full()
        .overflow_y_scrollbar()
        .bg(theme.bg_sidebar)
        .border_l_1()
        .border_color(theme.border)
        .flex()
        .flex_col()
        .gap_4()
        .p_3p5()
        // Vector Search Bar
        .child(
            div()
                .w_full()
                .rounded_xl()
                .border_1()
                .border_color(theme.border)
                .bg(theme.bg_card)
                .p_1()
                .shadow_sm()
                .child(Input::new(search_input).cleanable(true)),
        )
        // Network Knowledge Grounding
        .child(render_knowledge_card(theme))
        // Trending Topics
        .child(
            div()
                .w_full()
                .p_3p5()
                .rounded_xl()
                .border_1()
                .border_color(theme.border)
                .bg(theme.bg_card)
                .shadow_sm()
                .flex()
                .flex_col()
                .gap_2p5()
                .child(
                    div()
                        .text_xs()
                        .font_bold()
                        .text_color(theme.text_muted)
                        .child(t!("sidebar.popular_tags").to_string()),
                )
                .child(div().flex().flex_col().gap_1().children(trending_elements)),
        )
        // Active Persona Roster
        .child(render_persona_roster(theme))
}
