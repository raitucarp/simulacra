//! The palaces: a card per palace (its rooms as thumbnails, what is placed) with edit, walk
//! and delete, and the form for a new one (a name and the first room's kind).

use super::super::super::panel::ReviewPanel;
use super::super::super::render::common::{block_title, caption, control_row, surface};
use super::super::super::text::tr;
use super::super::events::LearningEvent;
use super::super::render::{chip, framed_input, section_label};
use super::scene::room_art;
use crate::domain::learning::{Palace, RoomKind};
use crate::i18n::t_count;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::*;
use rust_i18n::t;

const KIND_ID_PREFIX: &str = "learning-palace-kind-";
const CREATE_ID: &str = "learning-palace-create";
const EDIT_ID_PREFIX: &str = "learning-palace-edit-";
const WALK_ID_PREFIX: &str = "learning-palace-walk-";
const DELETE_ID_PREFIX: &str = "learning-palace-delete-";
/// Room thumbnails shown per palace card.
const THUMBNAILS: usize = 4;

impl ReviewPanel {
    fn palace_card(&self, index: usize, palace: &Palace, cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        let button = |prefix: &'static str, icon: LucideIcon, label: &str| Button::new(ElementId::NamedInteger(prefix.into(), index as u64)).ghost().xsmall().icon(Icon::new(icon)).label(tr(label));
        let (edit_id, walk_id, delete_id) = (palace.id.clone(), palace.id.clone(), palace.id.clone());
        let edit = button(EDIT_ID_PREFIX, LucideIcon::Edit, "learning.palace.edit").on_click(cx.listener(move |this, _, _, cx| this.edit_palace(edit_id.clone(), cx)));
        let walk = button(WALK_ID_PREFIX, LucideIcon::Play, "learning.palace.walk").on_click(cx.listener(move |this, _, window, cx| this.start_walk(walk_id.clone(), window, cx)));
        let delete = Button::new(ElementId::NamedInteger(DELETE_ID_PREFIX.into(), index as u64)).ghost().xsmall().icon(Icon::new(LucideIcon::Trash)).tooltip(tr("learning.palace.delete"))
            .on_click(cx.listener(move |this, _, _, cx| this.emit_learning(LearningEvent::DeletePalace(delete_id.clone()), cx)));
        let thumbs = palace.rooms.iter().take(THUMBNAILS).map(|room| div().rounded(theme.radius_sm).overflow_hidden().child(room_art(room.kind, theme.learning_room_thumb_width)));
        let line = format!("{} · {}", t_count("learning.palace.rooms", palace.rooms.len()), t!("learning.palace.placed", placed = palace.placed(), total = palace.loci()));
        surface(theme)
            .child(div().text_size(theme.font_size_md).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_primary).child(palace.name.clone()))
            .child(caption(line, theme))
            .child(control_row(theme).children(thumbs))
            .child(control_row(theme).child(edit).child(walk).child(div().flex_1()).child(delete))
    }

    fn palace_form(&self, cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        let kinds = RoomKind::ALL.iter().enumerate().map(|(index, kind)| {
            let kind = *kind;
            chip(ElementId::NamedInteger(KIND_ID_PREFIX.into(), index as u64), tr(&format!("learning.palace.room.{}", kind.key())), self.learning.new_room_kind == kind, theme).on_click(cx.listener(move |this, _, _, cx| {
                this.learning.new_room_kind = kind;
                cx.notify();
            }))
        }).collect::<Vec<_>>();
        let create = Button::new(CREATE_ID).primary().small().icon(Icon::new(LucideIcon::Plus)).label(tr("learning.palace.create")).on_click(cx.listener(|this, _, window, cx| this.create_palace(window, cx)));
        surface(theme)
            .child(block_title(LucideIcon::MapPin, theme.accent_dialectic, tr("learning.palace.title"), theme))
            .child(caption(tr("learning.palace.intro"), theme))
            .child(section_label(tr("learning.palace.name_label"), theme))
            .child(control_row(theme).child(framed_input(&self.learn_fields.palace_name, theme)).child(create))
            .child(section_label(tr("learning.palace.first_room"), theme))
            .child(control_row(theme).children(kinds))
            .child(div().rounded(theme.radius_md).overflow_hidden().child(room_art(self.learning.new_room_kind, theme.learning_scene_width / 2.0)))
    }

    pub(super) fn render_palace_list(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &self.theme;
        let palaces = &self.data.learning.records.palaces;
        let cards: Vec<Div> = palaces.iter().enumerate().map(|(index, palace)| self.palace_card(index, palace, cx)).collect();
        div().w_full().max_w(theme.review_card_max_width).flex().flex_col().gap(theme.section_gap).children(cards).child(self.palace_form(cx))
    }
}
