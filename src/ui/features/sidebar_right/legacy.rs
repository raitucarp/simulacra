//! Right sidebar containing vector search, knowledge grounding, and active agent roster.

use std::rc::Rc;

use super::trending::{render_popular_tags_title, render_trending_list, trending_elements};
use crate::state::AppState;
use crate::theme::ThemeTokens;
use super::knowledge_card::render_knowledge_card;
use super::roster::render_persona_roster;
use gpui_kit::component::input::{Input, InputState};
use gpui_kit::component::scroll::ScrollableElement;
use gpui_kit::*;

const RIGHT_SIDEBAR_ID: &str = "right-sidebar-scroll-container";

/// Rounded bordered card surface shared by the search bar and trending card.
fn card_surface(theme: &ThemeTokens) -> Div {
    div()
        .w_full()
        .rounded_xl()
        .border_1()
        .border_color(theme.border)
        .bg(theme.bg_card)
        .shadow_sm()
}

fn render_trending_card(trending: Vec<AnyElement>, theme: &ThemeTokens) -> Div {
    card_surface(theme)
        .p_3p5()
        .flex()
        .flex_col()
        .gap_2p5()
        .child(render_popular_tags_title(theme))
        .child(render_trending_list(trending))
}

pub fn render_right_sidebar(
    state: &AppState,
    search_input: &Entity<InputState>,
    theme: &ThemeTokens,
    on_click_topic: impl Fn(&str, &mut Window, &mut App) + 'static + Clone,
) -> impl IntoElement {
    let trending = trending_elements(state, Rc::new(on_click_topic), theme);

    div()
        .id(RIGHT_SIDEBAR_ID)
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
        .child(card_surface(theme).p_1().child(Input::new(search_input).cleanable(true)))
        // Network Knowledge Grounding
        .child(render_knowledge_card(theme))
        // Trending Topics
        .child(render_trending_card(trending, theme))
        // Active Persona Roster
        .child(render_persona_roster(theme))
}
