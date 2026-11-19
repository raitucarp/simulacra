//! The walk-through: each stop shows its room and the locus (only that marker); the learner
//! recalls what is placed there, reveals it (with its cue and picture) and grades it. A
//! graded item is an FSRS review in mode `palace` (the review keys work here too).

use super::super::super::events::ReviewEvent;
use super::super::super::panel::ReviewPanel;
use super::super::super::render::common::{caption, control_row, progress_bar, surface};
use super::super::super::render::face::{render_face, render_post_face, FaceRole};
use super::super::super::render::grades::render_grade_buttons;
use super::super::super::text::{percent, tr};
use super::super::render::chip;
use super::super::state::PalaceScreen;
use super::editor::COMPACT_SCENE;
use super::locus::locus_name;
use super::scene::{render_scene, MarkerState};
use crate::domain::learning::{Palace, PalaceId, PlacedTarget, Walk, WalkScope, WalkStop};
use crate::domain::review::schedule::preview;
use crate::domain::review::{Grade, ReviewItem};
use crate::ui::icons::LucideIcon;
use crate::ui::responsive::LayoutClass;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::*;
use rust_i18n::t;
use std::rc::Rc;

const END_ID: &str = "learning-walk-end";
const REVEAL_ID: &str = "learning-walk-reveal";
const SCOPE_ID_PREFIX: &str = "learning-walk-scope-";

impl ReviewPanel {
    /// The active review item of a placed target (a post's item, if it is being learnt).
    fn item_of(&self, target: &PlacedTarget) -> Option<ReviewItem> {
        match target {
            PlacedTarget::Item { id } => self.data.item(id).filter(|item| item.is_active()).cloned(),
            PlacedTarget::Post { id } => self.data.items.iter().find(|item| item.is_active() && item.is_on_post(id)).cloned(),
        }
    }

    /// Starts a walk over palace `id` in the chosen scope and takes the keyboard.
    pub(in crate::ui::features::review) fn start_walk(&mut self, id: PalaceId, window: &mut Window, cx: &mut Context<Self>) {
        let Some(palace) = self.data.learning.records.palace(&id) else { return };
        let stops = crate::domain::learning::plan_walk(palace, self.learning.walk_scope, &|target| self.item_of(target), self.data.today);
        self.learning.palace = PalaceScreen::Walk(Walk::new(id, stops));
        self.focus.focus(window, cx);
        cx.notify();
    }

    pub(in crate::ui::features::review) fn reveal_walk(&mut self, cx: &mut Context<Self>) {
        if let PalaceScreen::Walk(walk) = &self.learning.palace {
            self.learning.palace = PalaceScreen::Walk(walk.clone().reveal());
            cx.notify();
        }
    }

    /// Grades the revealed stop; its item's answer is applied by the shell.
    pub(in crate::ui::features::review) fn grade_walk(&mut self, grade: Grade, cx: &mut Context<Self>) {
        let PalaceScreen::Walk(walk) = &self.learning.palace else { return };
        let (walk, answer) = walk.clone().grade(grade);
        self.learning.palace = PalaceScreen::Walk(walk);
        if let Some(answer) = answer {
            cx.emit(ReviewEvent::Answered(answer));
        }
        cx.notify();
    }

    fn walk_scope_chips(&self, palace: &PalaceId, cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        let chips = [WalkScope::All, WalkScope::Due].into_iter().enumerate().map(|(index, scope)| {
            let (id, key) = (palace.clone(), if scope == WalkScope::All { "learning.walk.all" } else { "learning.walk.due" });
            chip(ElementId::NamedInteger(SCOPE_ID_PREFIX.into(), index as u64), tr(key), self.learning.walk_scope == scope, theme).on_click(cx.listener(move |this, _, window, cx| {
                this.learning.walk_scope = scope;
                this.start_walk(id.clone(), window, cx);
            }))
        });
        control_row(theme).children(chips)
    }

    fn walk_revealed(&self, stop: &WalkStop, placement_cue: (String, Option<String>), cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        let item = stop.item.as_ref().and_then(|id| self.data.item(id));
        let face = match (&stop.target, item) {
            (_, Some(item)) => div().flex().flex_col().gap(theme.stack_gap).child(render_face(&item.faces().prompt, FaceRole::Prompt, &self.data, theme)).child(render_face(&item.faces().answer, FaceRole::Answer, &self.data, theme)),
            (PlacedTarget::Post { id }, None) => self.data.post(id).map_or_else(|| caption(self.placed_text(&stop.target), theme), |view| render_post_face(view, theme)),
            (PlacedTarget::Item { .. }, None) => caption(self.placed_text(&stop.target), theme),
        };
        let (cue, image) = placement_cue;
        let entity = cx.entity().downgrade();
        let on_grade = Rc::new(move |grade, _: &mut Window, cx: &mut App| {
            let _ = entity.update(cx, |panel, cx| panel.grade_walk(grade, cx));
        });
        let intervals = item.map(|item| preview(&item.schedule, self.data.today, &self.data.policy));
        div()
            .w_full()
            .flex()
            .flex_col()
            .gap(theme.stack_gap)
            .children((!cue.is_empty()).then(|| div().text_size(theme.font_size_sm).text_color(theme.accent_dialectic).child(t!("learning.palace.cue_is", cue = cue).to_string())))
            .children(image.map(|path| img(std::path::PathBuf::from(path)).size(theme.learning_cue_image_size).rounded(theme.radius_md)))
            .child(face)
            .child(render_grade_buttons(intervals, None, on_grade, theme))
    }

    fn walk_stop(&self, palace: &Palace, walk: &Walk, stop: &WalkStop, class: LayoutClass, cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        let Some(room) = palace.rooms.get(stop.room) else { return surface(theme) };
        let Some(locus) = room.loci.get(stop.locus) else { return surface(theme) };
        let current = locus.slot;
        let state_of = move |slot: usize| MarkerState { placed: slot == current && walk.revealed, highlighted: slot == current, shown: slot == current };
        let width = if class.is_compact() { theme.learning_walk_scene_width * COMPACT_SCENE } else { theme.learning_walk_scene_width };
        let place = t!("learning.walk.where", room = room.name.clone(), place = locus_name(room, locus)).to_string();
        let cue = locus.placement.as_ref().map(|p| (p.cue.clone(), p.image.clone())).unwrap_or_default();
        let below = match walk.revealed {
            true => self.walk_revealed(stop, cue, cx),
            false => {
                let reveal = Button::new(REVEAL_ID).primary().icon(Icon::new(LucideIcon::Eye)).label(tr("learning.walk.reveal")).on_click(cx.listener(|this, _, _, cx| this.reveal_walk(cx)));
                div().flex().flex_col().gap(theme.stack_gap).child(caption(tr("learning.walk.recall"), theme)).child(control_row(theme).child(reveal))
            }
        };
        let details = div().flex_1().min_w_0().flex().flex_col().gap(theme.stack_gap).child(div().text_size(theme.font_size_md).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_primary).child(place)).child(below);
        let scene = render_scene(room, width, &state_of, None, theme);
        match class.is_compact() {
            true => surface(theme).child(scene).child(details),
            false => surface(theme).child(div().w_full().flex().items_start().gap(theme.section_gap).child(scene).child(details)),
        }
    }

    pub(super) fn render_walk(&self, walk: &Walk, class: LayoutClass, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &self.theme;
        let Some(palace) = self.data.learning.records.palace(&walk.palace_id) else { return div() };
        let id = palace.id.clone();
        let end = Button::new(END_ID).ghost().xsmall().icon(Icon::new(LucideIcon::X)).label(tr("learning.walk.end")).on_click(cx.listener(move |this, _, _, cx| this.edit_palace(id.clone(), cx)));
        let total = walk.stops.len().max(1);
        let top = control_row(theme).child(caption(t!("learning.walk.progress", done = (walk.position + 1).min(walk.stops.len()), total = walk.stops.len()).to_string(), theme)).child(div().flex_1().child(progress_bar(walk.position as f64 / total as f64, theme.accent_dialectic, theme))).child(end);
        let body = match walk.current() {
            Some(stop) => self.walk_stop(palace, walk, stop, class, cx),
            None if walk.stops.is_empty() => surface(theme).child(caption(tr("learning.walk.nothing"), theme)),
            None => surface(theme).child(caption(walk.recalled_share().map_or_else(|| tr("learning.walk.done"), |share| t!("learning.walk.score", share = percent(share)).to_string()), theme)),
        };
        div().w_full().max_w(theme.review_page_max_width).flex().flex_col().gap(theme.section_gap).child(top).child(self.walk_scope_chips(&palace.id, cx)).child(body)
    }
}
