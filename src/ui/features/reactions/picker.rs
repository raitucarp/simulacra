//! The picker's body: search field, skin tones, tabs and the emoji grid (or the search
//! results, or a line saying there is nothing to show).

use super::grid::{emoji_grid, tab_row, tone_row};
use super::{PickerTab, PickerView, ReactionControls};
use crate::domain::emoji::{group_emoji, search_emoji, EmojiGroup, PickerMemory};
use crate::theme::ThemeTokens;
use gpui_kit::component::input::Input;
use gpui_kit::component::Sizable;
use gpui_kit::{div, Div, FontWeight, IntoElement, ParentElement, Styled};
use rust_i18n::t;

/// Locale key of a group's name.
fn group_key(group: EmojiGroup) -> &'static str {
    match group {
        EmojiGroup::SmileysEmotion => "reactions.group.smileys_emotion",
        EmojiGroup::PeopleBody => "reactions.group.people_body",
        EmojiGroup::AnimalsNature => "reactions.group.animals_nature",
        EmojiGroup::FoodDrink => "reactions.group.food_drink",
        EmojiGroup::TravelPlaces => "reactions.group.travel_places",
        EmojiGroup::Activities => "reactions.group.activities",
        EmojiGroup::Objects => "reactions.group.objects",
        EmojiGroup::Symbols => "reactions.group.symbols",
        EmojiGroup::Flags => "reactions.group.flags",
    }
}

/// The tab's name (its tooltip and the heading over the grid).
pub fn tab_label(tab: PickerTab) -> String {
    match tab {
        PickerTab::Recent => t!("reactions.picker.recent").to_string(),
        PickerTab::Group(group) => t!(group_key(group)).to_string(),
    }
}

/// The emoji the picker shows, in the remembered skin tone: the search results while
/// `query` has text, else the open tab's emoji.
pub fn picker_emoji(tab: PickerTab, query: &str, memory: &PickerMemory) -> Vec<String> {
    if !query.trim().is_empty() {
        return search_emoji(query).into_iter().map(|emoji| memory.toned(emoji).to_string()).collect();
    }
    match tab {
        PickerTab::Recent => memory.recent.clone(),
        PickerTab::Group(group) => group_emoji(group).into_iter().map(|emoji| memory.toned(emoji).to_string()).collect(),
    }
}

/// What the grid area says when it has nothing to show.
fn empty_line(tab: PickerTab, query: &str) -> String {
    match (query.trim().is_empty(), tab) {
        (false, _) => t!("reactions.picker.no_results").to_string(),
        (true, PickerTab::Recent) => t!("reactions.picker.no_recent").to_string(),
        (true, PickerTab::Group(_)) => String::new(),
    }
}

/// The picker's body for `controls`' post; `query` is the search field's text.
pub(super) fn picker_content(controls: &ReactionControls, picker: &PickerView, query: &str, theme: &ThemeTokens) -> Div {
    let shown = picker_emoji(picker.tab, query, &picker.memory);
    let heading = if query.trim().is_empty() { tab_label(picker.tab) } else { t!("reactions.picker.results").to_string() };
    let heading = div().text_size(theme.font_size_2xs).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_muted).child(heading);
    let body = if shown.is_empty() {
        div().h(theme.emoji_picker_grid_height).flex().items_center().justify_center().text_size(theme.font_size_xs).text_color(theme.text_muted).child(empty_line(picker.tab, query)).into_any_element()
    } else {
        emoji_grid(&controls.post_id, &shown, &controls.on_event, theme)
    };
    div()
        .w(theme.emoji_picker_width)
        .flex()
        .flex_col()
        .gap(theme.stack_gap)
        .child(Input::new(&picker.search).small().cleanable(true))
        .child(tab_row(picker.tab, &controls.on_event, theme))
        .child(heading)
        .child(body)
        .child(tone_row(picker.memory.tone, &controls.on_event, theme))
}
