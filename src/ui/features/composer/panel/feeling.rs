//! The composer's feeling bar state (M2): whether it is open, the emojis picked (up to
//! three), the intensity, the short note and the emoji search. While it is open the
//! composer posts one feeling instead of the thread.

use super::super::feeling_bar::FeelingView;
use super::ComposerModalPanel;
use crate::domain::post::{FeelingProblem, Intensity, PostKind, MAX_FEELING_EMOJIS};
use gpui_kit::component::input::InputState;
use gpui_kit::*;
use rust_i18n::t;

/// The bar's state.
pub(super) struct FeelingPicker {
    pub open: bool,
    pub emojis: Vec<String>,
    pub intensity: Intensity,
    pub note: Entity<InputState>,
    pub search: Entity<InputState>,
}

impl FeelingPicker {
    pub fn new(window: &mut Window, cx: &mut Context<ComposerModalPanel>) -> Self {
        let (note_hint, search_hint) = (t!("composer.feeling.note_placeholder").to_string(), t!("composer.feeling.search_placeholder").to_string());
        let note = cx.new(|cx| InputState::new(window, cx).placeholder(note_hint));
        let search = cx.new(|cx| InputState::new(window, cx).placeholder(search_hint));
        Self { open: false, emojis: Vec::new(), intensity: Intensity::default(), note, search }
    }
}

/// `picked` with `emoji` added (up to [`MAX_FEELING_EMOJIS`]) or, when already there, removed.
pub(super) fn toggled(picked: &[String], emoji: &str) -> Vec<String> {
    if picked.iter().any(|e| e == emoji) {
        return picked.iter().filter(|e| *e != emoji).cloned().collect();
    }
    let mut next = picked.to_vec();
    if next.len() < MAX_FEELING_EMOJIS {
        next.push(emoji.to_string());
    }
    next
}

impl ComposerModalPanel {
    /// Opens the bar (focusing the emoji search) or closes it, forgetting the feeling.
    pub(super) fn toggle_feeling(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.feeling.open {
            self.reset_feeling(window, cx);
        } else {
            self.feeling.open = true;
            self.feeling.search.update(cx, |state, cx| state.focus(window, cx));
        }
        cx.notify();
    }

    /// Closes the bar and forgets the feeling.
    pub(super) fn reset_feeling(&mut self, window: &mut Window, cx: &mut App) {
        (self.feeling.open, self.feeling.emojis, self.feeling.intensity) = (false, Vec::new(), Intensity::default());
        for input in [&self.feeling.note, &self.feeling.search] {
            input.update(cx, |state, cx| state.set_value("", window, cx));
        }
    }

    pub(super) fn pick_feeling_emoji(&mut self, emoji: String, cx: &mut Context<Self>) {
        self.feeling.emojis = toggled(&self.feeling.emojis, &emoji);
        cx.notify();
    }

    pub(super) fn set_feeling_intensity(&mut self, intensity: Intensity, cx: &mut Context<Self>) {
        self.feeling.intensity = intensity;
        cx.notify();
    }

    /// The feeling to post, or why it cannot be; `None` while the bar is closed.
    pub(super) fn feeling_choice(&self, cx: &App) -> Option<Result<PostKind, FeelingProblem>> {
        let note = self.feeling.note.read(cx).value().to_string();
        self.feeling.open.then(|| PostKind::feeling(&self.feeling.emojis, self.feeling.intensity, &note))
    }

    /// The open bar as the dialog renders it.
    pub(super) fn feeling_view(&self, cx: &App) -> Option<FeelingView> {
        let query = self.feeling.search.read(cx).value().to_string();
        let problem = self.feeling_choice(cx).and_then(Result::err);
        self.feeling.open.then(|| FeelingView { emojis: self.feeling.emojis.clone(), intensity: self.feeling.intensity, note: self.feeling.note.clone(), search: self.feeling.search.clone(), query, problem })
    }
}

#[cfg(test)]
mod tests {
    use super::toggled;

    #[test]
    fn picking_adds_up_to_three_emojis_and_picking_again_drops_one() {
        let picked = ["😌", "🌿", "☕"].iter().fold(Vec::new(), |picked: Vec<String>, emoji| toggled(&picked, emoji));
        assert_eq!(picked, ["😌", "🌿", "☕"]);
        assert_eq!(toggled(&picked, "🎉"), picked, "a fourth is not added");
        assert_eq!(toggled(&picked, "🌿"), ["😌", "☕"]);
    }
}
