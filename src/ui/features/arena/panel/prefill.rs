//! Filling the forms from outside: "Debate this" puts a post in the motion, "Run a thought
//! experiment on this" puts it in the premise, and "New debate" shows an empty setup.

use super::ArenaPanel;
use crate::domain::arena::debate::setup::MAX_MOTION_CHARS;
use crate::domain::arena::lab::scenario::MAX_PREMISE_CHARS;
use crate::domain::PostId;
use crate::ui::features::arena::tab::ArenaTab;
use gpui_kit::{Context, Window};

fn clipped(text: &str, max: usize) -> String {
    text.trim().chars().take(max).collect()
}

impl ArenaPanel {
    /// The Debate tab with its setup form showing.
    pub fn new_debate_tab(&mut self, cx: &mut Context<Self>) {
        self.tab = ArenaTab::Debate;
        self.open_debate = None;
        cx.notify();
    }

    /// The setup form with `text` (from `post`) as the motion.
    pub fn prefill_motion(&mut self, post: PostId, text: String, window: &mut Window, cx: &mut Context<Self>) {
        self.fields.motion.update(cx, |field, cx| field.set_value(clipped(&text, MAX_MOTION_CHARS), window, cx));
        self.draft.source_post = Some(post);
        self.new_debate_tab(cx);
    }

    /// The lab form with `text` as the premise, the rest left to write.
    pub fn prefill_premise(&mut self, text: String, window: &mut Window, cx: &mut Context<Self>) {
        self.fields.premise.update(cx, |field, cx| field.set_value(clipped(&text, MAX_PREMISE_CHARS), window, cx));
        self.lab.preset = None;
        self.open_experiment = None;
        self.tab = ArenaTab::Lab;
        cx.notify();
    }
}
