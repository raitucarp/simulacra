//! The Learn tab's text fields: the tutor's answer box and concept field, a goal's title,
//! why, concept and task fields, a palace's name, a locus cue and the placement search.
//! Enter in a one-line field applies it (Ctrl+Enter sends a tutor answer); the search and the
//! goal title redraw as typed.

use super::super::panel::ReviewPanel;
use gpui_kit::component::input::{InputEvent, InputState, TextareaState};
use gpui_kit::{AppContext, Context, Entity, Subscription, Window};
use rust_i18n::t;

/// Rows of the tutor's answer box (fewest, most).
const ANSWER_ROWS: (usize, usize) = (3, 10);

/// Text fields of the Learn tab.
pub struct LearningFields {
    pub answer: Entity<TextareaState>,
    pub concept: Entity<InputState>,
    pub goal_title: Entity<InputState>,
    pub goal_why: Entity<InputState>,
    pub goal_concept: Entity<InputState>,
    pub task: Entity<InputState>,
    pub palace_name: Entity<InputState>,
    pub cue: Entity<InputState>,
    pub place_search: Entity<InputState>,
}

fn input(placeholder: &str, window: &mut Window, cx: &mut Context<ReviewPanel>) -> Entity<InputState> {
    let placeholder = t!(placeholder).to_string();
    cx.new(|cx| InputState::new(window, cx).placeholder(placeholder))
}

type EnterAction = fn(&mut ReviewPanel, &mut Window, &mut Context<ReviewPanel>);

fn on_enter(field: &Entity<InputState>, action: EnterAction, window: &mut Window, cx: &mut Context<ReviewPanel>) -> Subscription {
    cx.subscribe_in(field, window, move |this: &mut ReviewPanel, _, event: &InputEvent, window, cx| {
        if matches!(event, InputEvent::PressEnter { .. }) {
            action(this, window, cx);
        }
    })
}

impl LearningFields {
    /// The fields and their subscriptions.
    pub fn new(window: &mut Window, cx: &mut Context<ReviewPanel>) -> (Self, Vec<Subscription>) {
        let placeholder = t!("learning.tutor.answer_hint").to_string();
        let (min_rows, max_rows) = ANSWER_ROWS;
        let fields = Self {
            answer: cx.new(|cx| TextareaState::new(window, cx).placeholder(placeholder).auto_grow(min_rows, max_rows)),
            concept: input("learning.tutor.concept_hint", window, cx),
            goal_title: input("learning.goals.title_hint", window, cx),
            goal_why: input("learning.goals.why_hint", window, cx),
            goal_concept: input("learning.goals.concept_hint", window, cx),
            task: input("learning.goals.task_hint", window, cx),
            palace_name: input("learning.palace.name_hint", window, cx),
            cue: input("learning.palace.cue_hint", window, cx),
            place_search: input("learning.palace.search_hint", window, cx),
        };
        let redraw = |field: &Entity<InputState>, cx: &mut Context<ReviewPanel>| {
            cx.subscribe(field, |_: &mut ReviewPanel, _, event: &InputEvent, cx| {
                if matches!(event, InputEvent::Change) {
                    cx.notify();
                }
            })
        };
        let (search, title) = (redraw(&fields.place_search, cx), redraw(&fields.goal_title, cx));
        let send = cx.subscribe_in(&fields.answer, window, |this: &mut ReviewPanel, _, event: &InputEvent, window, cx| {
            if matches!(event, InputEvent::PressEnter { secondary: true, .. }) {
                this.send_answer(window, cx);
            }
        });
        let subscriptions = vec![
            send,
            on_enter(&fields.concept, ReviewPanel::start_tutor_on_concept, window, cx),
            on_enter(&fields.goal_concept, ReviewPanel::add_goal_concept, window, cx),
            on_enter(&fields.task, ReviewPanel::add_goal_task, window, cx),
            on_enter(&fields.palace_name, ReviewPanel::create_palace, window, cx),
            on_enter(&fields.cue, ReviewPanel::save_cue, window, cx),
            search,
            title,
        ];
        (fields, subscriptions)
    }
}
