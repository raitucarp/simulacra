//! The selected locus: its landmark, what is placed there (a review item or a post), its
//! vivid cue (text, and a picture from image ideas), or the ways to place something.

use super::super::super::panel::ReviewPanel;
use super::super::super::render::common::{block_title, caption, control_row, surface};
use super::super::super::text::tr;
use super::super::events::LearningEvent;
use super::super::items::ItemAction;
use super::super::render::{framed_input, section_label};
use crate::domain::learning::palace::edit::{cleared, with_image};
use crate::domain::learning::{Locus, Palace, PlacedTarget, Placement, Room};
use crate::domain::review::quiz::piece_text;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::*;
use rust_i18n::t;
use std::path::PathBuf;
use std::rc::Rc;

const CLEAR_ID: &str = "learning-locus-clear";
const IDEAS_ID: &str = "learning-locus-ideas";
const NO_IMAGE_ID: &str = "learning-locus-no-image";
const POST_ID: &str = "learning-locus-post";
const ITEM_ID_PREFIX: &str = "learning-locus-item-";
/// Characters of a placed post shown.
const POST_CHARS: usize = 160;

/// The landmark's name, or the learner's own name for the locus.
pub fn locus_name(room: &Room, locus: &Locus) -> String {
    let place = room.kind.slots().get(locus.slot).map(|slot| tr(&format!("learning.palace.place.{}", slot.place))).unwrap_or_default();
    if locus.name.trim().is_empty() { place } else { locus.name.clone() }
}

impl ReviewPanel {
    /// What is placed, as text.
    pub(in crate::ui::features::review) fn placed_text(&self, target: &PlacedTarget) -> String {
        match target {
            PlacedTarget::Item { id } => self.data.item(id).map_or_else(|| tr("review.source.missing"), |item| self.item_line(item)),
            PlacedTarget::Post { id } => self.data.post_text(id).map_or_else(|| tr("review.source.missing"), |text| piece_text(&text, POST_CHARS)),
        }
    }

    fn placement_block(&self, placement: &Placement, palace: &Palace, room: &Room, slot: usize, cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        let (palace_id, room_id) = (palace.id.clone(), room.id.clone());
        let (clear_room, image_room) = (room.id.clone(), room.id.clone());
        let clear = Button::new(CLEAR_ID).ghost().xsmall().icon(Icon::new(LucideIcon::Eraser)).label(tr("learning.palace.clear")).on_click(cx.listener(move |this, _, _, cx| this.change_palace(|p| cleared(p, &clear_room, slot), cx)));
        let ideas = Button::new(IDEAS_ID).ghost().xsmall().icon(Icon::new(LucideIcon::WandSparkles)).label(tr("learning.palace.image_ideas")).on_click(cx.listener(move |this, _, _, cx| this.emit_learning(LearningEvent::LocusImage(palace_id.clone(), room_id.clone(), slot), cx)));
        let no_image = Button::new(NO_IMAGE_ID).ghost().xsmall().icon(Icon::new(LucideIcon::X)).tooltip(tr("learning.palace.remove_image")).on_click(cx.listener(move |this, _, _, cx| this.change_palace(|p| with_image(p, &image_room, slot, None), cx)));
        let kind = match placement.target { PlacedTarget::Item { .. } => "learning.palace.placed_item", PlacedTarget::Post { .. } => "learning.palace.placed_post" };
        let cue = (!placement.cue.is_empty()).then(|| div().text_size(theme.font_size_sm).text_color(theme.accent_dialectic).child(t!("learning.palace.cue_is", cue = placement.cue.clone()).to_string()));
        let picture = placement.image.clone().map(|path| control_row(theme).child(img(PathBuf::from(path)).size(theme.learning_cue_image_size).rounded(theme.radius_md)).child(no_image));
        div()
            .flex()
            .flex_col()
            .gap(theme.stack_gap)
            .child(section_label(tr(kind), theme))
            .child(div().text_size(theme.font_size_sm).line_height(relative(theme.line_height_body)).text_color(theme.text_primary).child(self.placed_text(&placement.target)))
            .children(cue)
            .child(section_label(tr("learning.palace.cue_label"), theme))
            .child(control_row(theme).child(framed_input(&self.learn_fields.cue, theme)))
            .children(picture)
            .child(control_row(theme).child(ideas).child(clear))
    }

    fn empty_locus_block(&self, palace: &Palace, cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        let post = Button::new(POST_ID).ghost().small().icon(Icon::new(LucideIcon::MessageSquare)).label(tr("learning.palace.place_post")).on_click(cx.listener(|this, _, _, cx| this.pick_locus_post(cx)));
        let placed: Vec<_> = palace.rooms.iter().flat_map(|r| &r.loci).filter_map(|l| match l.placement.as_ref().map(|p| &p.target) { Some(PlacedTarget::Item { id }) => Some(id.clone()), _ => None }).collect();
        let entity = cx.entity().downgrade();
        let on_item: ItemAction = Rc::new(move |id, _, cx| {
            let _ = entity.update(cx, |panel, cx| panel.place_item(id, cx));
        });
        div().flex().flex_col().gap(theme.stack_gap).child(caption(tr("learning.palace.empty_locus"), theme)).child(section_label(tr("learning.palace.place_item"), theme)).child(self.render_item_chooser(ITEM_ID_PREFIX, &placed, on_item, cx)).child(control_row(theme).child(post))
    }

    pub(super) fn render_locus_panel(&self, palace: &Palace, room: &Room, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &self.theme;
        let slot = self.learning.palace_slot.unwrap_or(0);
        let Some(locus) = room.loci.iter().find(|l| l.slot == slot) else { return surface(theme) };
        let title = t!("learning.palace.locus_title", number = slot + 1, name = locus_name(room, locus)).to_string();
        let body = match &locus.placement {
            Some(placement) => self.placement_block(placement, palace, room, slot, cx),
            None => self.empty_locus_block(palace, cx),
        };
        surface(theme).child(block_title(LucideIcon::MapPin, theme.accent_dialectic, title, theme)).child(body)
    }
}
