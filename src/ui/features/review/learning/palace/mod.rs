//! The palace tool: the palaces, the editor (rooms along the route, a room's scene with its
//! numbered loci, what is placed at the selected locus and its cue) and the walk-through.
//! Every change is a pure edit of the palace, handed to the shell whole to be saved.

mod editor;
mod list;
mod locus;
mod scene;
mod walk;

use super::super::events::{PickerMode, ReviewEvent};
use super::super::panel::{ReviewPanel, Screen};
use super::events::LearningEvent;
use super::state::PalaceScreen;
use super::tutor::take_text;
use crate::domain::learning::palace::edit::{new_palace, placed, with_cue};
use crate::domain::learning::{Palace, PalaceId, PlacedTarget};
use crate::domain::review::ItemId;
use crate::ui::responsive::LayoutClass;
use gpui_kit::{AnyElement, Context, IntoElement, Window};

impl ReviewPanel {
    pub(in crate::ui::features::review) fn render_palace(&self, class: LayoutClass, cx: &mut Context<Self>) -> AnyElement {
        match &self.learning.palace {
            PalaceScreen::List => self.render_palace_list(cx).into_any_element(),
            PalaceScreen::Edit(id) => match self.data.learning.records.palace(id) {
                Some(palace) => self.render_palace_editor(palace, class, cx).into_any_element(),
                None => self.render_palace_list(cx).into_any_element(),
            },
            PalaceScreen::Walk(walk) => self.render_walk(walk, class, cx).into_any_element(),
        }
    }

    /// The palace open in the editor.
    pub(in crate::ui::features::review) fn open_palace(&self) -> Option<&Palace> {
        match &self.learning.palace {
            PalaceScreen::Edit(id) => self.data.learning.records.palace(id),
            PalaceScreen::Walk(walk) => self.data.learning.records.palace(&walk.palace_id),
            PalaceScreen::List => None,
        }
    }

    pub(in crate::ui::features::review) fn edit_palace(&mut self, id: PalaceId, cx: &mut Context<Self>) {
        let first_room = self.data.learning.records.palace(&id).and_then(|p| p.rooms.first().map(|room| room.id.clone()));
        self.learning.palace = PalaceScreen::Edit(id);
        (self.learning.palace_room, self.learning.palace_slot) = (first_room, Some(0));
        cx.notify();
    }

    /// Saves the open palace changed by `change`.
    pub(in crate::ui::features::review) fn change_palace(&mut self, change: impl FnOnce(Palace) -> Palace, cx: &mut Context<Self>) {
        if let Some(palace) = self.open_palace().cloned() {
            self.emit_learning(LearningEvent::SavePalace(change(palace)), cx);
        }
    }

    /// Enter in the name field (or the button): a new palace with a first room.
    pub fn create_palace(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let name = take_text(&self.learn_fields.palace_name, window, cx);
        if name.is_empty() {
            return;
        }
        let kind = self.learning.new_room_kind;
        let room = super::super::text::tr(&format!("learning.palace.room.{}", kind.key()));
        let palace = new_palace(PalaceId::generate(), self.data.network_id.clone(), &name, (kind, &room), jiff::Timestamp::now());
        let id = palace.id.clone();
        self.emit_learning(LearningEvent::SavePalace(palace.clone()), cx);
        self.learning.palace = PalaceScreen::Edit(id);
        (self.learning.palace_room, self.learning.palace_slot) = (palace.rooms.first().map(|room| room.id.clone()), Some(0));
    }

    /// The selected locus (room id, slot).
    pub(in crate::ui::features::review) fn selected_locus(&self) -> Option<(String, usize)> {
        Some((self.learning.palace_room.clone()?, self.learning.palace_slot?))
    }

    pub(in crate::ui::features::review) fn place_item(&mut self, id: ItemId, cx: &mut Context<Self>) {
        let Some((room, slot)) = self.selected_locus() else { return };
        self.change_palace(|palace| placed(palace, &room, slot, PlacedTarget::Item { id }), cx);
    }

    /// Opens the picker to place a post at the selected locus.
    pub(in crate::ui::features::review) fn pick_locus_post(&mut self, cx: &mut Context<Self>) {
        let Some(((room, slot), palace)) = self.selected_locus().zip(self.open_palace().map(|p| p.id.clone())) else { return };
        self.learning.pending_place = Some((palace, room, slot));
        cx.emit(ReviewEvent::OpenPicker(PickerMode::Place));
    }

    /// A post picked in the picker goes to the pending locus; back to the editor.
    pub(in crate::ui::features::review) fn place_post(&mut self, id: String, cx: &mut Context<Self>) {
        let Some((palace_id, room, slot)) = self.learning.pending_place.take() else { return };
        if let Some(palace) = self.data.learning.records.palace(&palace_id).cloned() {
            self.emit_learning(LearningEvent::SavePalace(placed(palace, &room, slot, PlacedTarget::Post { id })), cx);
        }
        self.learning.palace = PalaceScreen::Edit(palace_id);
        self.show(Screen::Learn, cx);
    }

    /// Enter in the cue field: saves the cue of the selected locus.
    pub fn save_cue(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let cue = take_text(&self.learn_fields.cue, window, cx);
        if let Some((room, slot)) = self.selected_locus() {
            self.change_palace(|palace| with_cue(palace, &room, slot, &cue), cx);
        }
    }
}
