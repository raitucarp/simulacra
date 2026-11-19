//! The page's text fields: the typed answer of free recall, the picker's search, and the map
//! editor's title, relation and concept fields.

use super::ReviewPanel;
use gpui_kit::component::input::{InputEvent, InputState};
use gpui_kit::{AppContext, Context, Entity, Subscription, Window};
use rust_i18n::t;

/// Text fields of the Review page.
pub struct PanelFields {
    pub answer: Entity<InputState>,
    pub search: Entity<InputState>,
    pub title: Entity<InputState>,
    pub relation: Entity<InputState>,
    pub concept: Entity<InputState>,
}

fn input(placeholder: &str, window: &mut Window, cx: &mut Context<ReviewPanel>) -> Entity<InputState> {
    let placeholder = t!(placeholder).to_string();
    cx.new(|cx| InputState::new(window, cx).placeholder(placeholder))
}

impl PanelFields {
    /// The fields and their subscriptions (Enter checks a recall answer or adds a concept;
    /// typing filters the picker).
    pub fn new(window: &mut Window, cx: &mut Context<ReviewPanel>) -> (Self, Vec<Subscription>) {
        let fields = Self {
            answer: input("review.recall.placeholder", window, cx),
            search: input("review.picker.search", window, cx),
            title: input("review.map.title_hint", window, cx),
            relation: input("review.map.relation_hint", window, cx),
            concept: input("review.map.concept_hint", window, cx),
        };
        let answer = cx.subscribe_in(&fields.answer, window, |this: &mut ReviewPanel, _, event: &InputEvent, window, cx| {
            if matches!(event, InputEvent::PressEnter { .. }) {
                this.check_recall(window, cx);
            }
        });
        let concept = cx.subscribe_in(&fields.concept, window, |this: &mut ReviewPanel, _, event: &InputEvent, window, cx| {
            if matches!(event, InputEvent::PressEnter { .. }) {
                this.add_concept(window, cx);
            }
        });
        let search = cx.subscribe(&fields.search, |_: &mut ReviewPanel, _, event: &InputEvent, cx| {
            if matches!(event, InputEvent::Change) {
                cx.notify();
            }
        });
        (fields, vec![answer, concept, search])
    }

    /// The trimmed text of `input`.
    pub fn text(input: &Entity<InputState>, cx: &gpui_kit::App) -> String {
        input.read(cx).value().trim().to_string()
    }
}
