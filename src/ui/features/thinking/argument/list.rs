//! The statements in display order: each row shows its role, text, cited post and standing;
//! drag a row by its handle onto another to move it there (the order sets the canvas
//! order within each tier); click a row to edit it.

use super::super::labels::{role_colour, role_label, standing_colour, tr};
use super::{ArgumentEvent, ArgumentPanel, ArgumentThread};
use crate::domain::thinking::argument::{ArgNode, Standing};
use crate::theme::ThemeTokens;
use crate::ui::icons::LucideIcon;
use gpui_kit::component::Icon;
use gpui_kit::*;

const ROW_ID: &str = "argument-row";
const SOURCE_ID: &str = "argument-row-source";

/// The row being dragged, and its preview under the pointer.
#[derive(Clone)]
pub struct DraggedNode {
    pub id: String,
    pub text: String,
    pub theme: ThemeTokens,
}

impl Render for DraggedNode {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        let theme = &self.theme;
        div()
            .max_w(theme.thinking_chip_width)
            .px(theme.spacing_sm)
            .py(theme.spacing_xs)
            .rounded(theme.radius_md)
            .bg(theme.bg_elevated)
            .border(theme.border_width_hairline)
            .border_color(theme.accent_brand)
            .text_size(theme.font_size_2xs)
            .text_color(theme.text_primary)
            .truncate()
            .child(self.text.clone())
    }
}

impl ArgumentPanel {
    pub(super) fn render_list(&self, thread: &ArgumentThread, cx: &mut Context<Self>) -> Div {
        let theme = &self.theme;
        let title = div().text_size(theme.font_size_2xs).font_weight(FontWeight::SEMIBOLD).text_color(theme.text_muted).child(tr("thinking.argument.statements"));
        let rows = thread.map.nodes.iter().enumerate().map(|(i, node)| {
            let standing = self.report.standing.get(i).copied().unwrap_or(Standing::Undecided);
            let source = thread.posts.iter().position(|(id, _)| *id == node.source);
            self.row(i, node, standing, source, cx)
        });
        div().w_full().flex().flex_col().gap(theme.spacing_3xs).child(title).children(rows)
    }

    fn row(&self, index: usize, node: &ArgNode, standing: Standing, source: Option<usize>, cx: &mut Context<Self>) -> Stateful<Div> {
        let theme = &self.theme;
        let dragged = DraggedNode { id: node.id.clone(), text: node.text.clone(), theme: theme.clone() };
        let (selected, tint) = (self.edited_id() == Some(node.id.as_str()), theme.accent_brand_subtle);
        let handle = div().flex_none().text_color(theme.text_muted).cursor_grab().child(Icon::new(LucideIcon::GripVertical).size(theme.icon_size_xs));
        let role = div().flex_none().w(theme.thinking_handle_width).flex().items_center().gap(theme.spacing_xs).child(div().flex_none().size(theme.insights_swatch_size).rounded(theme.radius_full).bg(role_colour(node.role, theme))).child(div().truncate().text_color(theme.text_muted).child(role_label(node.role)));
        let cited = source.map(|at| self.source_link(index, at, node, cx));
        let mark = div().flex_none().size(theme.insights_swatch_size).rounded(theme.radius_full).border(theme.border_width_threadline).border_color(standing_colour(standing, theme));
        div()
            .id(ElementId::NamedInteger(ROW_ID.into(), index as u64))
            .w_full()
            .flex()
            .items_center()
            .gap(theme.spacing_xs)
            .px(theme.spacing_xs)
            .py(theme.spacing_3xs)
            .rounded(theme.radius_sm)
            .border(theme.border_width_hairline)
            .border_color(if selected { theme.accent_brand_border } else { theme.bg_transparent })
            .text_size(theme.font_size_2xs)
            .hover(|row| row.bg(theme.hover_bg))
            .on_drag(dragged, |dragged, _, _, cx| cx.new(|_| dragged.clone()))
            .drag_over::<DraggedNode>(move |row, _, _, _| row.bg(tint))
            .on_drop(cx.listener(move |panel, dragged: &DraggedNode, _, cx| panel.move_node(dragged.id.clone(), index, cx)))
            .on_click(cx.listener(move |panel, _, window, cx| panel.pick_node(index, window, cx)))
            .child(handle)
            .child(role)
            .child(div().flex_1().min_w_0().truncate().text_color(theme.text_primary).child(node.text.clone()))
            .children(cited)
            .child(mark)
    }

    /// "P2": opens the post the statement cites.
    fn source_link(&self, index: usize, at: usize, node: &ArgNode, cx: &mut Context<Self>) -> Stateful<Div> {
        let theme = &self.theme;
        let post = node.source.clone();
        div()
            .id(ElementId::NamedInteger(SOURCE_ID.into(), index as u64))
            .flex_none()
            .px(theme.spacing_xs)
            .rounded(theme.radius_sm)
            .bg(theme.hover_bg)
            .text_color(theme.text_secondary)
            .cursor_pointer()
            .on_click(cx.listener(move |panel, _, _, cx| {
                cx.stop_propagation();
                panel.request(ArgumentEvent::OpenPost(post.clone()), cx);
            }))
            .child(format!("P{}", at + 1))
    }
}

