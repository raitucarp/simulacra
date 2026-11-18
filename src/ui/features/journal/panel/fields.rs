//! The page's text fields: the entry editor, the journal search, the check-in's note, tags
//! and sleep, the focus intention, custom length, goal and reflection, and the capsule's
//! note, opening day and reply.

use super::JournalPanel;
use gpui_kit::component::input::{InputEvent, InputState, TextareaState};
use gpui_kit::{App, AppContext, Context, Entity, Subscription, Window};
use rust_i18n::t;

/// Rows of the entry editor and the capsule note (shortest, tallest before scrolling).
const ENTRY_ROWS: (usize, usize) = (10, 28);
const NOTE_ROWS: (usize, usize) = (5, 16);

/// Text fields of the Journal page.
pub struct JournalFields {
    pub entry: Entity<TextareaState>,
    pub search: Entity<InputState>,
    pub note: Entity<InputState>,
    pub tags: Entity<InputState>,
    pub sleep: Entity<InputState>,
    pub intention: Entity<InputState>,
    pub minutes: Entity<InputState>,
    pub goal: Entity<InputState>,
    pub reflection: Entity<InputState>,
    pub capsule_body: Entity<TextareaState>,
    pub capsule_until: Entity<InputState>,
    pub reply: Entity<TextareaState>,
}

fn input(placeholder: &str, window: &mut Window, cx: &mut Context<JournalPanel>) -> Entity<InputState> {
    let placeholder = t!(placeholder).to_string();
    cx.new(|cx| InputState::new(window, cx).placeholder(placeholder))
}

fn textarea(placeholder: &str, rows: (usize, usize), window: &mut Window, cx: &mut Context<JournalPanel>) -> Entity<TextareaState> {
    let placeholder = t!(placeholder).to_string();
    cx.new(|cx| TextareaState::new(window, cx).placeholder(placeholder).auto_grow(rows.0, rows.1))
}

impl JournalFields {
    /// The fields, and the subscriptions that re-render on typing.
    pub fn new(window: &mut Window, cx: &mut Context<JournalPanel>) -> (Self, Vec<Subscription>) {
        let fields = Self {
            entry: textarea("journal.editor.placeholder", ENTRY_ROWS, window, cx),
            search: input("journal.search.placeholder", window, cx),
            note: input("journal.checkin.note_hint", window, cx),
            tags: input("journal.checkin.tags_hint", window, cx),
            sleep: input("journal.checkin.sleep_hint", window, cx),
            intention: input("journal.focus.intention_hint", window, cx),
            minutes: input("journal.focus.custom_hint", window, cx),
            goal: input("journal.focus.goal_hint", window, cx),
            reflection: input("journal.focus.reflection_hint", window, cx),
            capsule_body: textarea("journal.capsule.body_hint", NOTE_ROWS, window, cx),
            capsule_until: input("journal.capsule.until_hint", window, cx),
            reply: textarea("journal.capsule.reply_hint", NOTE_ROWS, window, cx),
        };
        let notify = |_: &mut JournalPanel, _: Entity<InputState>, event: &InputEvent, cx: &mut Context<JournalPanel>| {
            if matches!(event, InputEvent::Change) {
                cx.notify();
            }
        };
        let subscriptions = vec![cx.subscribe(&fields.search, notify), cx.subscribe(&fields.capsule_until, notify), cx.subscribe(&fields.minutes, notify)];
        (fields, subscriptions)
    }

    /// The trimmed text of `input`.
    pub fn text(input: &Entity<InputState>, cx: &App) -> String {
        input.read(cx).value().trim().to_string()
    }

    /// The text of a multi-line field, as written.
    pub fn area_text(input: &Entity<TextareaState>, cx: &App) -> String {
        input.read(cx).value().to_string()
    }
}
