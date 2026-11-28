//! The palace editor: the route of rooms (move, remove, add one of a kind) beside the selected
//! room's scene with its numbered loci, both as tall as the taller, and the selected locus's
//! panel across the full width below them.

use super::super::super::panel::ReviewPanel;
use super::super::super::render::common::{block_title, caption, control_row, surface};
use super::super::super::text::tr;
use super::super::render::{chip, section_label};
use super::super::state::PalaceScreen;
use super::scene::{render_scene, MarkerAction, MarkerState};
use crate::domain::learning::palace::edit::{moved_room, with_room, without_room, Direction};
use crate::domain::learning::{Palace, Room, RoomKind};
use crate::ui::icons::LucideIcon;
use crate::ui::responsive::LayoutClass;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;
use std::rc::Rc;

const BACK_ID: &str = "learning-palace-back";
const WALK_ID: &str = "learning-palace-walk";
const ROOM_ID_PREFIX: &str = "learning-room-";
const UP_ID_PREFIX: &str = "learning-room-up-";
const DOWN_ID_PREFIX: &str = "learning-room-down-";
const REMOVE_ID_PREFIX: &str = "learning-room-remove-";
const ADD_ID_PREFIX: &str = "learning-room-add-";
/// The scene's share of its full width on a compact window.
pub const COMPACT_SCENE: f32 = 0.6;

impl ReviewPanel {
    fn room_row(&self, index: usize, room: &Room, cx: &mut Context<Self>) -> Stateful<Div> {
        let theme = &self.theme;
        let selected = self.learning.palace_room.as_deref() == Some(room.id.as_str());
        let icon = |prefix: &'static str, icon: LucideIcon| Button::new(ElementId::NamedInteger(prefix.into(), index as u64)).ghost().xsmall().icon(Icon::new(icon));
        let (open, up, down, remove) = (room.id.clone(), room.id.clone(), room.id.clone(), room.id.clone());
        let placed = room.loci.iter().filter(|l| l.placement.is_some()).count();
        div()
            .id(ElementId::NamedInteger(ROOM_ID_PREFIX.into(), index as u64))
            .flex()
            .items_center()
            .gap(theme.inline_gap)
            .p(theme.spacing_xs)
            .rounded(theme.radius_sm)
            .cursor_pointer()
            .when(selected, |row| row.bg(theme.accent_brand_subtle))
            .on_click(cx.listener(move |this, _, _, cx| {
                (this.learning.palace_room, this.learning.palace_slot) = (Some(open.clone()), Some(0));
                cx.notify();
            }))
            .child(div().flex_1().min_w_0().flex().flex_col().child(div().truncate().text_size(theme.font_size_sm).text_color(theme.text_primary).child(format!("{}. {}", index + 1, room.name))).child(caption(format!("{placed}/{}", room.loci.len()), theme)))
            .child(icon(UP_ID_PREFIX, LucideIcon::ChevronUp).on_click(cx.listener(move |this, _, _, cx| this.change_palace(|p| moved_room(p, &up, Direction::Earlier), cx))))
            .child(icon(DOWN_ID_PREFIX, LucideIcon::ChevronDown).on_click(cx.listener(move |this, _, _, cx| this.change_palace(|p| moved_room(p, &down, Direction::Later), cx))))
            .child(icon(REMOVE_ID_PREFIX, LucideIcon::Trash).on_click(cx.listener(move |this, _, _, cx| this.change_palace(|p| without_room(p, &remove), cx))))
    }

    fn route_card(&self, palace: &Palace, cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        let rows: Vec<Stateful<Div>> = palace.rooms.iter().enumerate().map(|(index, room)| self.room_row(index, room, cx)).collect();
        let kinds = RoomKind::ALL.iter().enumerate().map(|(index, kind)| {
            let kind = *kind;
            let name = tr(&format!("learning.palace.room.{}", kind.key()));
            chip(ElementId::NamedInteger(ADD_ID_PREFIX.into(), index as u64), name.clone(), false, theme).on_click(cx.listener(move |this, _, _, cx| this.change_palace(|p| with_room(p, kind, &name), cx)))
        });
        surface(theme).child(block_title(LucideIcon::Compass, theme.accent_brand, tr("learning.palace.route"), theme)).children(rows).child(section_label(tr("learning.palace.add_room"), theme)).child(control_row(theme).children(kinds))
    }

    fn scene_card(&self, room: &Room, class: LayoutClass, cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        let selected = self.learning.palace_slot;
        let state_of = |slot: usize| MarkerState { placed: room.loci.iter().any(|l| l.slot == slot && l.placement.is_some()), highlighted: selected == Some(slot), shown: true };
        let entity = cx.entity().downgrade();
        let on_click: MarkerAction = Rc::new(move |slot, _, cx| {
            let _ = entity.update(cx, |panel, cx| {
                panel.learning.palace_slot = Some(slot);
                cx.notify();
            });
        });
        let width = if class.is_compact() { theme.learning_scene_width * COMPACT_SCENE } else { theme.learning_scene_width };
        surface(theme).items_center().child(render_scene(room, width, &state_of, Some(on_click), theme)).child(caption(tr("learning.palace.scene_hint"), theme))
    }

    pub(super) fn render_palace_editor(&self, palace: &Palace, class: LayoutClass, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &self.theme;
        let back = Button::new(BACK_ID).ghost().xsmall().icon(Icon::new(LucideIcon::ArrowLeft)).label(tr("learning.palace.all")).on_click(cx.listener(|this, _, _, cx| {
            this.learning.palace = PalaceScreen::List;
            cx.notify();
        }));
        let id = palace.id.clone();
        let walk = Button::new(WALK_ID).primary().small().icon(Icon::new(LucideIcon::Play)).label(tr("learning.palace.walk")).on_click(cx.listener(move |this, _, window, cx| this.start_walk(id.clone(), window, cx)));
        let head = surface(theme).child(control_row(theme).child(back).child(div().flex_1()).child(walk)).child(div().font_family(theme.font_family_display.clone()).text_size(theme.font_size_lg).font_weight(FontWeight::BOLD).text_color(theme.text_primary).child(palace.name.clone()));
        let room = self.learning.palace_room.as_ref().and_then(|id| palace.room(id)).or_else(|| palace.rooms.first());
        let side = if class.is_compact() { Pixels::ZERO } else { theme.learning_side_width };
        let route = div().flex().when(class.is_compact(), |d| d.w_full()).when(!class.is_compact(), |d| d.w(theme.learning_side_width).flex_none()).child(self.route_card(palace, cx).flex_1());
        let scene = room.map(|room| div().flex_1().min_w(side).flex().child(self.scene_card(room, class, cx).flex_1()));
        let locus = room.map(|room| self.render_locus_panel(palace, room, cx));
        let row = div().w_full().flex().flex_wrap().gap(theme.section_gap).child(route).children(scene);
        div().w_full().flex().flex_col().gap(theme.section_gap).child(head).child(row).children(locus)
    }
}
