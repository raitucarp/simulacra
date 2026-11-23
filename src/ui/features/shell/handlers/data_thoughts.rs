//! N16–N20 in the shell: the one router of their events, and their open sheet over the shell
//! with the event that closes it.

use super::super::view::AppView;
use crate::ui::features::data::{render_glossary_sheet, render_lens_sheet, DataAction, DataEvent, GlossaryEvent, LensEvent, ThoughtEvent};
use gpui_kit::*;

impl AppView {
    /// Routes one N16–N20 event.
    pub(in crate::ui::features::shell) fn on_thought_event(&mut self, event: ThoughtEvent, window: &mut Window, cx: &mut Context<Self>) {
        match event {
            ThoughtEvent::Lens(event) => self.on_lens_event(event, window, cx),
            ThoughtEvent::Glossary(event) => self.on_glossary_event(event, window, cx),
        }
    }

    /// The open N16–N20 sheet and the event closing it, if one is open.
    pub(in crate::ui::features::shell) fn thought_overlay(&self, on_event: DataAction) -> Option<(Stateful<Div>, DataEvent)> {
        let (sheets, theme) = (&self.data.ideas.thoughts, &self.theme);
        if let Some(dialog) = &sheets.glossary {
            return Some((render_glossary_sheet(dialog, on_event, theme), DataEvent::Thought(ThoughtEvent::Glossary(GlossaryEvent::Close))));
        }
        let dialog = sheets.lens.as_ref()?;
        Some((render_lens_sheet(dialog, on_event, theme), DataEvent::Thought(ThoughtEvent::Lens(LensEvent::Close))))
    }
}
