//! Import and capture state owned by the root view: the open import sheet, its inputs and
//! background task, files waiting their turn, the quick-capture box and the drag hint.

use super::view::AppView;
use crate::ui::features::import::{ImportInputs, ImportSession};
use gpui_kit::component::input::{InputEvent, InputState};
use gpui_kit::*;
use rust_i18n::t;
use std::collections::VecDeque;
use std::path::PathBuf;

/// The import sheet, quick capture and drop state.
pub struct ImportUi {
    pub session: Option<ImportSession>,
    pub inputs: ImportInputs,
    /// Reading the open file; dropping it cancels the read.
    pub task: Option<Task<()>>,
    /// Further dropped or picked files, opened one after another.
    pub queue: VecDeque<PathBuf>,
    pub capture_open: bool,
    pub capture_input: Entity<InputState>,
    /// Files are being dragged over the window.
    pub dragging: bool,
}

impl ImportUi {
    pub(super) fn new(window: &mut Window, cx: &mut Context<AppView>) -> (Self, Vec<Subscription>) {
        let mut input = |placeholder: String| cx.new(|cx| InputState::new(window, cx).placeholder(placeholder));
        let inputs = ImportInputs {
            title: input(t!("import.field.title").to_string()),
            creators: input(t!("import.field.creators_placeholder").to_string()),
            language: input(t!("import.field.language_placeholder").to_string()),
            date: input(t!("import.field.date_placeholder").to_string()),
            pseudonym: input(t!("import.pseudonym_placeholder").to_string()),
        };
        let capture_input = input(t!("capture.placeholder").to_string());
        let subscriptions = vec![
            cx.subscribe_in(&inputs.pseudonym, window, |_, _, _: &InputEvent, _, cx| cx.notify()),
            cx.subscribe_in(&capture_input, window, |view, _, event: &InputEvent, window, cx| {
                if matches!(event, InputEvent::PressEnter { .. }) {
                    view.save_capture(window, cx);
                }
            }),
        ];
        let ui = Self { session: None, inputs, task: None, queue: VecDeque::new(), capture_open: false, capture_input, dragging: false };
        (ui, subscriptions)
    }

    /// The trimmed text of `input`.
    pub(super) fn text(input: &Entity<InputState>, cx: &App) -> String {
        input.read(cx).value().trim().to_string()
    }
}
