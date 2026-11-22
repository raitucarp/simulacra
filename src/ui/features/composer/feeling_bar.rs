//! The composer's feeling bar (M2): a footer toggle opens a row with the emojis picked (click
//! one to drop it), the five intensity steps, a short note and the emoji picker's search and
//! grid (the R5 picker's emoji, the smileys first). While it is open the composer posts one
//! feeling.

use super::modal::ModalActionCallback;
use crate::domain::emoji::{EmojiGroup, PickerMemory};
use crate::domain::post::{FeelingProblem, Intensity, MAX_FEELING_EMOJIS, MAX_FEELING_NOTE_CHARS};
use crate::theme::ThemeTokens;
use crate::ui::features::reactions::{picker_emoji, PickerTab};
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::input::{Input, InputState};
use gpui_kit::component::scroll::ScrollableElement;
use gpui_kit::component::{Icon, Selectable, Sizable};
use gpui_kit::*;
use rust_i18n::t;
use std::rc::Rc;

/// Picks (or drops) an emoji.
pub type EmojiCallback = Rc<dyn Fn(String, &mut Window, &mut App) + 'static>;
/// Picks an intensity.
pub type IntensityCallback = Rc<dyn Fn(Intensity, &mut Window, &mut App) + 'static>;

const BUTTON_ID: &str = "composer-feeling";
const CELL_ID: &str = "composer-feeling-emoji";
const PICKED_ID: &str = "composer-feeling-picked";
const STEP_ID: &str = "composer-feeling-step";
const GRID_ID: &str = "composer-feeling-grid";

/// The open bar.
#[derive(Clone)]
pub struct FeelingView {
    pub emojis: Vec<String>,
    pub intensity: Intensity,
    pub note: Entity<InputState>,
    pub search: Entity<InputState>,
    /// The search field's text.
    pub query: String,
    /// Why the feeling cannot be posted yet.
    pub problem: Option<FeelingProblem>,
}

/// The emoji the grid offers: the search results, or the smileys.
pub fn offered_emoji(query: &str) -> Vec<String> {
    picker_emoji(PickerTab::Group(EmojiGroup::SmileysEmotion), query, &PickerMemory::default())
}

/// The line explaining `problem`.
pub fn problem_text(problem: FeelingProblem) -> String {
    match problem {
        FeelingProblem::NoEmoji => t!("composer.feeling.problem.no_emoji").to_string(),
        FeelingProblem::TooManyEmojis => t!("composer.feeling.problem.too_many_emojis", count = MAX_FEELING_EMOJIS).to_string(),
        FeelingProblem::NoteTooLong => t!("composer.feeling.problem.note_too_long", count = MAX_FEELING_NOTE_CHARS).to_string(),
    }
}

/// The footer's feeling toggle, highlighted while the bar is open.
pub(super) fn feeling_button(open: bool, on_toggle: ModalActionCallback) -> Button {
    Button::new(BUTTON_ID).ghost().small().selected(open).icon(Icon::new(LucideIcon::HeartPulse)).tooltip(t!("composer.feeling.button").to_string()).on_click(move |_, window, cx| on_toggle(window, cx))
}

fn emoji_cell(id: SharedString, glyph: &str, on_pick: &EmojiCallback, theme: &ThemeTokens) -> Stateful<Div> {
    let (on_pick, emoji, hover) = (on_pick.clone(), glyph.to_string(), theme.hover_bg);
    div().id(id).size(theme.emoji_cell_size).flex().items_center().justify_center().rounded(theme.radius_sm).cursor_pointer().text_size(theme.font_size_lg).hover(move |style| style.bg(hover)).on_click(move |_, window, cx| on_pick(emoji.clone(), window, cx)).child(glyph.to_string())
}

fn steps(current: Intensity, on_step: &IntensityCallback) -> Div {
    let buttons = Intensity::ALL.map(|step| {
        let on_step = on_step.clone();
        Button::new(SharedString::from(format!("{STEP_ID}-{}", step.level()))).ghost().xsmall().selected(step == current).label(step.level().to_string()).on_click(move |_, window, cx| on_step(step, window, cx))
    });
    div().flex().items_center().children(buttons)
}

fn label(text: String, theme: &ThemeTokens) -> Div {
    div().flex_none().text_size(theme.font_size_xs).text_color(theme.text_muted).child(text)
}

/// The bar: picked emojis and intensity, the note, then the search and the grid.
pub(super) fn feeling_row(view: &FeelingView, on_pick: EmojiCallback, on_step: IntensityCallback, theme: &ThemeTokens) -> Div {
    let picked = view.emojis.iter().enumerate().map(|(index, glyph)| emoji_cell(SharedString::from(format!("{PICKED_ID}-{index}")), glyph, &on_pick, theme));
    let top = div().flex().items_center().gap(theme.control_gap).child(label(t!("composer.feeling.label").to_string(), theme)).child(div().flex().children(picked)).child(div().flex_1()).child(label(t!("composer.feeling.intensity").to_string(), theme)).child(steps(view.intensity, &on_step));
    let cells = offered_emoji(&view.query).into_iter().enumerate().map(|(index, glyph)| emoji_cell(SharedString::from(format!("{CELL_ID}-{index}")), &glyph, &on_pick, theme));
    let grid = div().id(GRID_ID).h(theme.emoji_picker_grid_height).overflow_y_scrollbar().child(div().flex().flex_wrap().children(cells));
    let problem = view.problem.map(|problem| div().text_size(theme.font_size_xs).text_color(theme.accent_danger).child(problem_text(problem)));
    div()
        .w_full()
        .flex()
        .flex_col()
        .gap(theme.spacing_xs)
        .px(theme.panel_padding)
        .py(theme.spacing_xs)
        .border_t(theme.border_width_hairline)
        .border_color(theme.border_subtle)
        .child(top)
        .child(Input::new(&view.note).small())
        .child(Input::new(&view.search).small().cleanable(true))
        .child(grid)
        .children(problem)
}

#[cfg(test)]
mod tests {
    use super::{offered_emoji, problem_text, FeelingProblem};

    #[test]
    fn every_feeling_text_is_translated_and_the_grid_offers_smileys() {
        for key in ["composer.feeling.button", "composer.feeling.label", "composer.feeling.intensity", "composer.feeling.note_placeholder", "composer.feeling.search_placeholder"] {
            assert_ne!(rust_i18n::t!(key, locale = "en"), key);
        }
        for problem in [FeelingProblem::NoEmoji, FeelingProblem::TooManyEmojis, FeelingProblem::NoteTooLong] {
            assert!(!problem_text(problem).starts_with("composer."), "{problem:?}");
        }
        assert!(!offered_emoji("").is_empty());
        assert!(offered_emoji("").len() > offered_emoji("cry").len());
    }
}
