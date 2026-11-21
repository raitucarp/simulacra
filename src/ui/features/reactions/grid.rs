//! The picker's clickable parts: emoji cells in a scrolling grid, the tab strip (one emoji
//! per group) and the skin-tone swatches.

use super::picker::tab_label;
use super::{send, PickerTab, ReactionAction, ReactionEvent};
use crate::domain::emoji::{group_emoji, SkinTone};
use crate::domain::PostId;
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::scroll::ScrollableElement;
use gpui_kit::component::{Icon, Selectable, Sizable};
use gpui_kit::*;
use rust_i18n::t;

const GRID_ID: &str = "emoji-picker-grid";
const CELL_ID: &str = "emoji-picker-cell";
const TAB_ID: &str = "emoji-picker-tab";
const TONE_ID: &str = "emoji-picker-tone";
/// The emoji each skin-tone swatch shows, in the tone.
const TONE_SAMPLE: &str = "\u{270B}";

/// One emoji cell: picking it reacts with it and closes the picker.
fn cell(index: usize, post: &PostId, glyph: &str, on_event: &ReactionAction, theme: &ThemeTokens) -> Stateful<Div> {
    let pick = send(on_event, ReactionEvent::Pick(post.clone(), glyph.to_string()));
    let hover = theme.hover_bg;
    div()
        .id(SharedString::from(format!("{CELL_ID}-{index}")))
        .size(theme.emoji_cell_size)
        .flex()
        .items_center()
        .justify_center()
        .rounded(theme.radius_sm)
        .cursor_pointer()
        .text_size(theme.font_size_lg)
        .hover(move |style| style.bg(hover))
        .on_click(move |_, window, cx| pick(window, cx))
        .child(glyph.to_string())
}

/// The scrolling grid of `shown` emoji.
pub(super) fn emoji_grid(post: &PostId, shown: &[String], on_event: &ReactionAction, theme: &ThemeTokens) -> AnyElement {
    let cells = shown.iter().enumerate().map(|(index, glyph)| cell(index, post, glyph, on_event, theme));
    let grid = div().flex().flex_wrap().children(cells);
    div().id(GRID_ID).h(theme.emoji_picker_grid_height).overflow_y_scrollbar().child(grid).into_any_element()
}

/// The tab strip: a clock for the recent emoji, then each group's first emoji.
pub(super) fn tab_row(open: PickerTab, on_event: &ReactionAction, theme: &ThemeTokens) -> Div {
    let tabs = PickerTab::all().enumerate().map(|(index, tab)| {
        let select = send(on_event, ReactionEvent::SelectTab(tab));
        let button = Button::new(SharedString::from(format!("{TAB_ID}-{index}"))).ghost().xsmall().selected(tab == open).tooltip(tab_label(tab));
        let button = match tab {
            PickerTab::Recent => button.icon(Icon::new(LucideIcon::Clock)),
            PickerTab::Group(group) => button.label(group_emoji(group).first().map_or("", |emoji| emoji.glyph)),
        };
        button.on_click(move |_, window, cx| select(window, cx))
    });
    div().flex().flex_wrap().items_center().gap(theme.spacing_2xs).children(tabs)
}

/// Locale key of a tone's name (`None` is the default yellow).
fn tone_key(tone: Option<SkinTone>) -> &'static str {
    match tone {
        None => "reactions.picker.tone.default",
        Some(SkinTone::Light) => "reactions.picker.tone.light",
        Some(SkinTone::MediumLight) => "reactions.picker.tone.medium_light",
        Some(SkinTone::Medium) => "reactions.picker.tone.medium",
        Some(SkinTone::MediumDark) => "reactions.picker.tone.medium_dark",
        Some(SkinTone::Dark) => "reactions.picker.tone.dark",
    }
}

/// The skin-tone swatches, the chosen one selected.
pub(super) fn tone_row(chosen: Option<SkinTone>, on_event: &ReactionAction, theme: &ThemeTokens) -> Div {
    let sample = crate::domain::emoji::find_emoji(TONE_SAMPLE);
    let tones = std::iter::once(None).chain(SkinTone::ALL.into_iter().map(Some));
    let swatches = tones.enumerate().map(|(index, tone)| {
        let pick = send(on_event, ReactionEvent::SetTone(tone));
        let glyph = sample.map_or(TONE_SAMPLE, |emoji| emoji.in_tone(tone));
        Button::new(SharedString::from(format!("{TONE_ID}-{index}")))
            .ghost()
            .xsmall()
            .selected(tone == chosen)
            .label(glyph)
            .tooltip(t!(tone_key(tone)).to_string())
            .on_click(move |_, window, cx| pick(window, cx))
    });
    let label = div().flex_none().text_size(theme.font_size_2xs).text_color(theme.text_muted).child(t!("reactions.picker.skin_tone").to_string());
    div().flex().items_center().gap(theme.spacing_2xs).child(label).children(swatches)
}

#[cfg(test)]
pub(super) fn tone_keys() -> Vec<&'static str> {
    std::iter::once(None).chain(SkinTone::ALL.into_iter().map(Some)).map(tone_key).collect()
}
