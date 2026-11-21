//! Reactions in the shell (epic R): the controls every card gets, and the picker's events
//! (open, close, tab, skin tone, pick), each reaction saved to the vault as it happens.

use super::super::binder::Binder;
use super::super::view::AppView;
use crate::ui::features::reactions::{PickerView, ReactionEvent, ReactionSource};
use gpui_kit::*;
use std::rc::Rc;

impl AppView {
    /// What the feed needs to draw reaction controls and the picker.
    pub(in crate::ui::features::shell) fn reaction_source(&self, bind: &Binder) -> ReactionSource {
        let host = &self.reactions;
        let picker = PickerView { search: host.search.clone(), tab: host.tab, memory: Rc::new(host.memory.clone()) };
        ReactionSource { open: host.open.clone(), picker, on_event: Rc::new(bind.with(Self::on_reaction_event)) }
    }

    pub(in crate::ui::features::shell) fn on_reaction_event(&mut self, event: ReactionEvent, window: &mut Window, cx: &mut Context<Self>) {
        match event {
            ReactionEvent::OpenPicker(post) => {
                self.reactions.open = Some(post);
                self.reactions.search.update(cx, |input, cx| input.set_value("", window, cx));
            }
            ReactionEvent::ClosePicker => self.reactions.open = None,
            ReactionEvent::SelectTab(tab) => self.reactions.tab = tab,
            ReactionEvent::SetTone(tone) => {
                self.reactions.memory.tone = tone;
                self.reactions.save_memory(&self.state);
            }
            ReactionEvent::Toggle(post, emoji) => {
                if self.state.toggle_reaction(&post, &emoji).is_some() {
                    self.feed.stream.remeasure();
                }
            }
            ReactionEvent::Pick(post, emoji) => {
                if self.state.toggle_reaction(&post, &emoji).is_some() {
                    self.reactions.memory.used(&emoji);
                    self.reactions.save_memory(&self.state);
                    self.feed.stream.remeasure();
                }
                self.reactions.open = None;
            }
        }
        cx.notify();
    }
}
