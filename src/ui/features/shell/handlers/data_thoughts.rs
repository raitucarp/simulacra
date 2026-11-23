//! N16–N20 in the shell: the one router of their events, and their open sheet over the shell
//! with the event that closes it.

use super::super::view::AppView;
use crate::ui::features::data::{render_glossary_sheet, render_lens_sheet, render_mind_sheet, DataAction, DataEvent, GlossaryEvent, LensEvent, MindEvent, ThoughtEvent};
use gpui_kit::*;

impl AppView {
    /// Routes one N16–N20 event.
    pub(in crate::ui::features::shell) fn on_thought_event(&mut self, event: ThoughtEvent, window: &mut Window, cx: &mut Context<Self>) {
        match event {
            ThoughtEvent::Lens(event) => self.on_lens_event(event, window, cx),
            ThoughtEvent::Glossary(event) => self.on_glossary_event(event, window, cx),
            ThoughtEvent::Questions(event) => self.on_questions_event(event, window, cx),
            ThoughtEvent::Mind(event) => self.on_mind_event(event, window, cx),
        }
    }

    /// The open N16–N20 sheet and the event closing it, if one is open.
    pub(in crate::ui::features::shell) fn thought_overlay(&self, on_event: DataAction) -> Option<(Stateful<Div>, DataEvent)> {
        if let Some(found) = self.questions_page(on_event.clone()) {
            return Some(found); // N18
        }
        let (sheets, theme) = (&self.data.ideas.thoughts, &self.theme);
        if let Some(dialog) = &sheets.mind {
            return Some((render_mind_sheet(dialog, on_event, theme), DataEvent::Thought(ThoughtEvent::Mind(MindEvent::Close))));
        }
        if let Some(dialog) = &sheets.glossary {
            return Some((render_glossary_sheet(dialog, on_event, theme), DataEvent::Thought(ThoughtEvent::Glossary(GlossaryEvent::Close))));
        }
        let dialog = sheets.lens.as_ref()?;
        Some((render_lens_sheet(dialog, on_event, theme), DataEvent::Thought(ThoughtEvent::Lens(LensEvent::Close))))
    }
}
