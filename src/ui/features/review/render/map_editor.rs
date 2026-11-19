//! The connection map editor: a title, the points on a ring with their reference
//! connections (click two points to join or separate them), the relation and direction new
//! connections get, concepts to add, and the lists of connections and points.

use super::super::data::PostViews;
use super::super::panel::{MapEditor, ReviewPanel};
use super::super::text::tr;
use super::common::{block_title, caption, control_row, surface};
use super::map::{point_chip, MapCanvas, PointAction, PointChip, Stroke};
use crate::domain::review::connections::ConnectionMap;
use crate::ui::components::choice_chip;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::button::{Button, ButtonVariants};
use gpui_kit::component::input::{Input, InputState};
use gpui_kit::component::{Icon, Sizable};
use gpui_kit::*;
use std::rc::Rc;

const CANVAS_ID: &str = "review-map-editor";
const DIRECTED_ID: &str = "review-map-directed";
const ADD_CONCEPT_ID: &str = "review-map-add-concept";
const SAVE_ID: &str = "review-map-save";
const CANCEL_ID: &str = "review-map-cancel";

impl ReviewPanel {
    /// The chips of `map`'s points (`extra` holds posts the page's data may lack).
    pub(super) fn map_chips(&self, map: &ConnectionMap, extra: Option<&PostViews>) -> Vec<PointChip> {
        let post = |id: &str| extra.and_then(|posts| posts.get(id)).or_else(|| self.data.post(id)).cloned();
        let title = |id: &str| self.data.document_title(id).cloned();
        map.points.iter().map(|point| point_chip(point, &post, &title)).collect()
    }

    pub(super) fn field(&self, label: String, input: &Entity<InputState>) -> Div {
        let theme = &self.theme;
        let frame = div().w_full().rounded(theme.radius_md).border(theme.border_width_hairline).border_color(theme.border).bg(theme.input_bg).px(theme.spacing_sm).child(Input::new(input).bordered(false));
        div().flex_1().min_w(theme.review_count_tile_width).flex().flex_col().gap(theme.spacing_xs).child(caption(label, theme)).child(frame)
    }

    pub(super) fn render_map_editor(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &self.theme;
        let Some(editor) = &self.editor else { return surface(theme) };
        let entity = cx.entity().downgrade();
        let on_point: PointAction = Rc::new(move |point, _, cx| {
            let _ = entity.update(cx, |panel, cx| panel.click_editor_point(point, cx));
        });
        let strokes = editor.map.edges.iter().map(|edge| Stroke { from: edge.from, to: edge.to, color: theme.accent_brand, curved: false, arrow: edge.directed, label: edge.label.clone() }).collect();
        let canvas = MapCanvas {
            id: CANVAS_ID.into(),
            chips: self.map_chips(&editor.map, Some(&editor.posts)),
            positions: editor.positions(),
            strokes,
            pending: editor.pending,
            on_point: Some(on_point),
            theme: theme.clone(),
        };
        let problem = editor.problem.map(|problem| div().text_size(theme.font_size_xs).text_color(theme.accent_danger).child(tr(problem.message_key())));
        surface(theme)
            .child(block_title(LucideIcon::Share2, theme.accent_brand, tr(if editor.existing.is_some() { "review.map.edit_title" } else { "review.map.new_title" }), theme))
            .child(caption(tr("review.map.intro"), theme))
            .child(control_row(theme).child(self.field(tr("review.map.title"), &self.fields.title)))
            .child(self.editor_tools(editor, cx))
            .child(canvas)
            .child(self.render_map_lists(editor, cx))
            .children(problem)
            .child(self.editor_footer(cx))
    }

    /// The relation and direction of new connections, and the concept field.
    fn editor_tools(&self, editor: &MapEditor, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &self.theme;
        let entity = cx.entity().downgrade();
        let directed = choice_chip(DIRECTED_ID, &tr("review.map.one_way"), editor.directed, theme, move |_, cx| {
            let _ = entity.update(cx, |panel, cx| panel.toggle_directed(cx));
        });
        let add = Button::new(ADD_CONCEPT_ID).ghost().small().icon(Icon::new(LucideIcon::Plus)).label(tr("review.map.add_concept"))
            .on_click(cx.listener(|this, _, window, cx| this.add_concept(window, cx)));
        control_row(theme)
            .items_end()
            .child(self.field(tr("review.map.relation"), &self.fields.relation))
            .child(directed)
            .child(self.field(tr("review.map.concept_field"), &self.fields.concept))
            .child(add)
    }

    fn editor_footer(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &self.theme;
        let cancel = Button::new(CANCEL_ID).ghost().small().label(tr("review.map.cancel")).on_click(cx.listener(|this, _, _, cx| this.cancel_map(cx)));
        let save = Button::new(SAVE_ID).primary().small().icon(Icon::new(LucideIcon::Check)).label(tr("review.map.save")).on_click(cx.listener(|this, _, _, cx| this.save_map(cx)));
        control_row(theme).child(caption(tr("review.map.save_hint"), theme)).child(div().flex_1()).child(cancel).child(save)
    }
}
