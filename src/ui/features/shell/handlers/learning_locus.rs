//! Memory palace cue pictures (sb E2): image ideas opened on a locus (D3's helper, through
//! one hook) and the drawn picture put on the locus's cue.

use super::super::view::AppView;
use crate::domain::learning::{PalaceId, PlacedTarget};
use gpui_kit::*;

/// Characters of a cue shown in the image ideas sheet.
const CUE_CHARS: usize = 60;

impl AppView {
    /// Image ideas for the cue of a palace locus (D3's helper, one hook): the picture of what
    /// is placed there, from its cue or its text.
    pub(super) fn open_image_ideas_for_locus(&mut self, palace: PalaceId, room: String, slot: usize, window: &mut Window, cx: &mut Context<Self>) {
        let placement = self.review.learning.records.palace(&palace).and_then(|p| p.room(&room)).and_then(|r| r.loci.iter().find(|l| l.slot == slot)).and_then(|l| l.placement.clone());
        let Some(placement) = placement else { return };
        let about = if placement.cue.is_empty() { self.placed_text(&placement.target) } else { placement.cue.clone() };
        let own_engine = self.state.current_profile().map(|p| p.config.engine.image.clone()).unwrap_or_default();
        let open = super::image_ideas::IdeasOpen {
            target: crate::domain::image_prompt::ImageTarget::Post,
            destination: crate::ui::features::image_prompt::IdeasDestination::Locus(palace, room, slot),
            about: crate::state::semantic::hits::shortened(&about, CUE_CHARS),
            subject: crate::domain::image_prompt::subject_from_post(&about),
            own_engine,
        };
        self.open_image_ideas(open, window, cx);
    }

    /// The text of what is placed (an item's prompt, a post).
    fn placed_text(&self, target: &PlacedTarget) -> String {
        let text_of = |id: &str| self.state.get_post_by_id(id).map(|post| post.content.clone());
        match target {
            PlacedTarget::Item { id } => self.review.deck.item(id).map(|item| item.prompt_text(&text_of)).unwrap_or_default(),
            PlacedTarget::Post { id } => text_of(id).unwrap_or_default(),
        }
    }

    /// Puts a drawn picture on a locus's cue.
    pub(in crate::ui::features::shell) fn set_locus_image(&mut self, palace: &PalaceId, room: &str, slot: usize, path: &std::path::Path, cx: &mut Context<Self>) {
        let Some(found) = self.review.learning.records.palace(palace).cloned() else { return };
        let image = Some(path.to_string_lossy().into_owned());
        self.store_palace(crate::domain::learning::palace::edit::with_image(found, room, slot, image), cx);
    }
}
